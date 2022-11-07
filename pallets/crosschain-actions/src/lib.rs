#![cfg_attr(not(feature = "std"), no_std)]
use frame_system::{
	offchain::{
		AppCrypto, CreateSignedTransaction, SendSignedTransaction,
		SignedPayload, Signer, SigningTypes,
	},
};
use sp_std::{
	vec::Vec, prelude::*, str,
	collections::{vec_deque::VecDeque}
};
use primitive_types::U256;
pub use weights::WeightInfo;
pub use pallet_vtbdex::TokenType;
mod types;
mod vtb_offchain;
mod actions;
mod constants;
pub mod weights;
#[path = "../../../global_constants.rs"] mod global_constants;

pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	/// This pallet's configuration trait
	#[pallet::config]
	pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config + pallet_vtbdex::Config {
		/// The identifier type for an offchain worker.
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The overarching dispatch call type.
		type Call: From<Call<Self>>;

		/// The overarching WeightInfo type
		type WeightInfo: WeightInfo;

	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// A public part of the pallet.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/*************************** Start extrinsic which is signed only by ocw *****************************/
		///	fn submit_signed_user_new_wallet_record(origin: OriginFor<T>, 
		///	token_type: TokenType,
		///	info: glbl_type::Wallet, 
		///	pdot_address: T::AccountId, 
		///	transaction_hash: Vec<u8>,
		/// ) -> DispatchResult
		/// 
		/// This is extrinsic signed via Ocw to create new user wallet.
		/// On successful completion of the extrinsic it emit event to notify user and the system.
		/// Event name: CreateWallet(Option<T::AccountId>, TokenType, glbl_type::Wallet).
		#[pallet::weight(<T as Config>::WeightInfo::submit_signed_user_new_wallet_record())]
		pub fn submit_signed_user_new_wallet_record(origin: OriginFor<T>,
			token_type: TokenType, 
			mut info: pallet_vtbdex::UserType, 
			pdot_address: T::AccountId, 
			transaction_hash: Vec<u8>,
		) -> DispatchResult {
			log::info!("Extrinsic for submit new wallet with address: {:?}", pdot_address);
			let signer_address = ensure_signed(origin)?;
		
			ensure!(!<pallet_vtbdex::WalletStorage<T>>::is_user_exist(&pdot_address), Error::<T>::PolkadotAddressAlreadylinked);
			ensure!(!<TransactionHashList<T>>::get(&token_type).contains(&transaction_hash), Error::<T>::TransactionHashAlreadyExist);
			let crypto_details = info.crypto_addresses.get(&token_type).ok_or(Error::<T>::InvalidTokenType)?;
			let crypto_address = crypto_details.crypto_address.as_ref().ok_or(Error::<T>::CrytoAddressNotLinked)?;
			<pallet_vtbdex::WalletStorage<T>>::insert_new_wallet_record(&pdot_address.encode(), &crypto_address, info.clone());
			
			Self::initialize_user_for_distribution(&pdot_address, crypto_address, token_type)?;
			<TransactionHashList<T>>::mutate(&token_type, |tx_list| {
				tx_list.push_back(transaction_hash)
			});

			Self::deposit_event(Event::CreateWallet {
				signer: Some(signer_address), 
				token_type,
				user_wallet_data: info 
			});

			Ok(())
		}

		/// fn submit_signed_user_wallet_add_new_crypto(origin: OriginFor<T>, 
		/// token_type: TokenType
		///	pdot_address: T::AccountId, 
		///	info: glbl_type::CryptoDetail, 
		///	transaction_hash: Vec<u8>,
		/// ) -> DispatchResult
		/// 
		/// This is extrinsic signed via Ocw to add new cryptos in wallet.
		/// On successful completion of the extrinsic it emit event to notify user and the system.
		/// Event name: AddedNewCrypto(Option<T::AccountId>, TokenType, T::AccountId, glbl_type::CryptoDetail).
		#[pallet::weight(<T as Config>::WeightInfo::submit_signed_user_wallet_add_new_crypto())]
		pub fn submit_signed_user_wallet_add_new_crypto(origin: OriginFor<T>, 
			token_type: TokenType,
			pdot_address: T::AccountId, 
			info: pallet_vtbdex::UserCryptoBasedData, 
			transaction_hash: Vec<u8>,
		) -> DispatchResult {
			let signer_add = ensure_signed(origin)?;

			ensure!(<pallet_vtbdex::WalletStorage<T>>::is_user_exist(&pdot_address), Error::<T>::PolkadotAddressNotLinked);
			ensure!(!<TransactionHashList<T>>::get(&token_type).contains(&transaction_hash), Error::<T>::TransactionHashAlreadyExist);
			<pallet_vtbdex::WalletStorage<T>>::append_or_replace_user_wallet_crypto(&info, &pdot_address, token_type)?;
			let crypto_address = info.crypto_address.as_ref().ok_or(Error::<T>::CrytoAddressNotLinked)?;
			Self::initialize_user_for_distribution(&pdot_address, crypto_address, token_type)?;
			<TransactionHashList<T>>::mutate(&token_type, |tx_list| {
				tx_list.push_back(transaction_hash)
			});
			Self::deposit_event(Event::AddedNewCrypto {
				signer: Some(signer_add), 
				token_type, 
				user_address: pdot_address, 
				crypto_data: info
			});

			Ok(())
		}

		/// fn crypto_deposit_success(origin: OriginFor<T>, 
		///	pdot_address: T::AccountId, 
		///	balance: U256, 
		///	token_type: TokenType, 
		///	transaction_hash: Vec<u8>
		/// ) -> DispatchResult 
		/// 
		/// This is extrinsic signed via Ocw to update crypto deposit balance.
		/// On successful completion of the extrinsic it emit event to notify user and the system.
		/// Event name: DepositSuccess(Option<T::AccountId>, TokenType, T::AccountId, Vec<u8>, U256).
		#[pallet::weight(<T as Config>::WeightInfo::crypto_deposit_success())]
		pub fn crypto_deposit_success(origin: OriginFor<T>, 
			token_type: TokenType, 
			pdot_address: Option<T::AccountId>, 
			deposit_amount: U256, 	
			transaction_hash: Vec<u8>,
			crypto_detail: Option<pallet_vtbdex::UserCryptoBasedData>, 
			new_wallet: Option<pallet_vtbdex::UserType>, 
		) -> DispatchResult {
			let signer_add = ensure_signed(origin)?;
			ensure!(!<TransactionHashList<T>>::get(&token_type).contains(&transaction_hash), Error::<T>::TransactionHashAlreadyExist);

			let mut account_key = Vec::new();
			Self::check_and_create_wallet_storage(&mut account_key, token_type, &pdot_address, crypto_detail, new_wallet)?;
			let res = <pallet_vtbdex::Pallet<T>>::add_update_balance(&token_type, &account_key, deposit_amount, U256::from(0_u8));
			match res {
				Ok(()) => {
					<pallet_vtbdex::Pallet<T>>::add_circulation_token_balance(&token_type, deposit_amount)?;
					let user = <pallet_vtbdex::WalletStorage<T>>::get_wallet_record(&account_key);

					<TransactionHashList<T>>::mutate(&token_type, |tx_list| {
						tx_list.push_back(transaction_hash.clone())
					});
					Self::deposit_event(Event::UpdateBalance {
						signer: Some(signer_add.clone()),
						token_type,
						user_wallet_data: user
					});
																	
					Self::deposit_event(Event::DepositSuccess {
						signer: Some(signer_add),
						token_type,
						user_address: pdot_address, 
						transaction_id: transaction_hash, 
						amount: deposit_amount
					});

					Ok(())
				},
				Err(err) =>  Err(frame_support::dispatch::DispatchError::from(err)), 

			}

		}

		/// fn crypto_withdraw_success(origin: OriginFor<T>, 
		///	pdot_address: T::AccountId, 
		///	balance: U256, 
		///	token_type: TokenType, 
		///	transaction_hash: Vec<u8>
		/// ) -> DispatchResult
		/// 
		/// This is extrinsic signed via Ocw to update crypto withdraw balance.
		/// On successful completion of the extrinsic it emit event to notify user and the system.
		/// Event name: WithdrawSuccess(Option<T::AccountId>, TokenType, T::AccountId, Vec<u8>, U256).
		/// crypto_withdraw_success
		#[pallet::weight(<T as Config>::WeightInfo::crypto_withdraw_success())]
		pub fn crypto_withdraw_success(origin: OriginFor<T>, 
			token_type: TokenType, 
			pdot_address: T::AccountId, 
			withdrawn_amount: U256, 
			transaction_hash: Vec<u8>
		) -> DispatchResult {
			let signer_add = ensure_signed(origin)?;
		
			ensure!(<pallet_vtbdex::WalletStorage<T>>::is_user_exist(&pdot_address), Error::<T>::PolkadotAddressNotLinked);
			ensure!(!<TransactionHashList<T>>::get(&token_type).contains(&transaction_hash), Error::<T>::TransactionHashAlreadyExist);
			match <pallet_vtbdex::Pallet<T>>::update_blocked_user_state_balance(&pdot_address, &transaction_hash) {
				Ok(_) => {
					<TransactionHashList<T>>::mutate(&token_type, |tx_list| {
						tx_list.push_back(transaction_hash.clone())
					});
					Self::deposit_event(Event::WithdrawSuccess {
						signer: Some(signer_add), 
						token_type, 
						user_address: pdot_address, 
						transaction_id: transaction_hash, 
						amount: withdrawn_amount
					});

					Ok(())
				},
				Err(err) => Err(err)
			}
		}

		/*************************** End of extrinsic which is signed only by ocw ** ***************************/
	}

	/// Events for the pallet.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event generated when a new user is registered in Vtbdex system.
		CreateWallet {
			signer: Option<T::AccountId>, 
			token_type: TokenType, 
			user_wallet_data: pallet_vtbdex::UserType
		},
		/// Event generated when a new crypto is linked in existing user address.
		AddedNewCrypto {
			signer: Option<T::AccountId>, 
			token_type: TokenType, 
			user_address: T::AccountId, 
			crypto_data: pallet_vtbdex::UserCryptoBasedData
		},
		/// Event generated when User wallet balance is updated
		UpdateBalance {
			signer: Option<T::AccountId>, 
			token_type: TokenType, 
			user_wallet_data: pallet_vtbdex::UserType
		},
		/// Event generated when a withdraw request is completed in Vtbdex system.
		WithdrawSuccess {
			signer: Option<T::AccountId>, 
			token_type: TokenType, 
			user_address: T::AccountId, 
			transaction_id: Vec<u8>, 
			amount: U256
		},
		/// Event generated when a Deposit Crypto request is completed in Vtbdex system.
		DepositSuccess {
			signer: Option<T::AccountId>, 
			token_type: TokenType, 
			user_address: Option<T::AccountId>, 
			transaction_id: Vec<u8>, 
			amount: U256
		},
	}

	/// Storage to keep record of transaction_hash of the events present on Crypto(Eth/Eos) network.
	#[pallet::storage]
	#[pallet::getter( fn tran_hash_list)]
	pub type TransactionHashList<T> = StorageMap<_, Blake2_128Concat, TokenType, VecDeque<Vec<u8>>, ValueQuery>;

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error returned when no account is available to sign transaction
		NoLocalAcctForSigning,
		/// Error returned when facing issue while sending transaction from OCW
		OffchainSignedTxError,
		/// Error returned when fetching info from remote
		HttpFetchingError,
		/// Error returned when Facing issue with String Parsing
		StringParsingError,
		/// Error returned when facing issue while converting from Bytes to String
		ByteToStringConversionError,	
		/// Error returned when fetching data from Ocw-storage for current block failed
		OcwStoreFetchingCurrentBlockError,
		/// Error returned when fetching data from Ocw-storage for last Processed block failed
		OcwStoreFetchingProcessedBlockError,
		/// Error returned when fetching data from Ocw-storage for current block at 0th position failed
		OcwStoreCurrentBlockFetching0thPositionError,
		/// Error returned when Sign transaction for Onboarding process failed
		OnBoardFailed,
		/// Error emitted when the given transaction hash is already listened by another OCW or the hash exist in the system
		TransactionHashAlreadyExist,
		/// Error emitted when PolkadotAddress is not known to Vtbdex system.
		PolkadotAddressNotLinked,
		/// Error emitted when PolkadotAddress is already attached to Vtbdex system
		PolkadotAddressAlreadylinked,
		/// Error emitted when token type object is not present in Wallet
		InvalidTokenType,
		/// Error emitted when Crypto address is not present in field in Wallet 
		CrytoAddressNotLinked
	}

}

/// Implement methods for Struct type VtbContractPayload
impl<T: SigningTypes> SignedPayload<T> for types::VtbContractPayload<T::Public, T::BlockNumber> {
	fn public(&self) -> T::Public {
		self.public.clone()
	}
}