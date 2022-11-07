#![cfg_attr(not(feature = "std"), no_std)]
use frame_system::{
	offchain::{
		AppCrypto
	},
};
use sp_std::{str};
use scale_info::prelude::string::String;
use primitive_types::U256;
use pallet_cross_chain::{TokenType};
pub mod eth_block;
pub mod constants;
#[path = "../../../../global_constants.rs"] mod global_constants;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	/// This pallet's configuration trait
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_cross_chain::Config {
		/// The identifier type for an offchain worker.
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		// Error returned when not sure which ocw function to executed
		UnknownOffchainMux,

		// Error returned when making signed transactions in off-chain worker
		NoLocalAcctForSigning,
		OffchainSignedTxError,

		// Error returned when making unsigned transactions in off-chain worker
		OffchainUnsignedTxError,

		// Error returned when making unsigned transactions with signed payloads in off-chain worker
		OffchainUnsignedTxSignedPayloadError,

		// Error returned when fetching info from remote
		HttpFetchingError,
		StringParsingError,
		ByteToStringConversionError,
		ResultNotAStringError,
		HexDecodingError,

		// Error returned when fetching ocw store data
		OcwStoreFetchingCurrentBlockError,
		OcwStoreFetchingProcessedBlockError,
		OcwStoreCurrentBlockFetching0thPositionError,

		InvalidValueEth,
		LockAcquiredFailed,
		OnBoardFailed,
		ValueAlreadylinkedWithThisKey,
		PolkadotAddressNotLinked,
		InvalidSs58Address,
		AccountNotOnboardedInSubstrate,
		UserPeriodsDataIsNone,
		RequireOcwKeyOwner,
		EthTransactionHashAlreadyProcessedByAnotherOcw
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

		/// Offchain Worker entry point.
		fn offchain_worker(block_number: T::BlockNumber) {
			log::info!("Hello from offchain workers of eth-vtb-contract pallet!");
	
			//To synsc with current blocknumber on ethereum network
			if block_number > 1u32.into(){
				match Self::fetch_eth_current_block_number() {
					Ok(current_eth_block_num) => {	
						let _ = Self::fetch_eth_block_info(current_eth_block_num);
					}
					Err(err) => {
						log::info!("Error: {:?} ", err);
					}
				}
			}

			// let _ = <pallet_cross_chain::Pallet<T>>::vtb_onboard_user("5DPPXtjC6sn4K2cmb3mXExjxMxfTYz1u93nMf2vNhJ53Tzjs",
			// 	"adminttesteth",
			// 	"ef0650c5ec8f5345783af6cc361f978843189371494eaa76dd6808cba9838fab1".as_bytes().to_vec(),
			// 	TokenType::Eth
			// );

			// let _ = <pallet_cross_chain::Pallet<T>>::vtb_onboard_user("5DPPXtjC6sn4K2cmb3mXExjxMxfTYz1u93nMf2vNhJ53Tzjs",
			// 	"adminttesteos",
			// 	"ef0650c5ec8f5345783af6cc361f978843189371494eaa76dd6808cba9838fabeos1".as_bytes().to_vec(),
			// 	TokenType::Eos
			// );

		// 	let _ = <pallet_cross_chain::Pallet<T>>::vtb_onboard_user("5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM",
		// 	"defaulttesteth",
		// 	"ef0650c5ec8f5345783af6cc361f978843189371494eaa76dd6808cba9838fabeth2".as_bytes().to_vec(),
		// 	TokenType::Eth
		// );

		// let _ = <pallet_cross_chain::Pallet<T>>::vtb_onboard_user("5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM",
		// 	"defaulttesteos",
		// 	"ef0650c5ec8f5345783af6cc361f978843189371494eaa76dd6808cba9838fabeos2".as_bytes().to_vec(),
		// 	TokenType::Eos
		// );

			
		}
	}
}

// impl<T: SigningTypes> SignedPayload<T> for custom_types::PricePayload<T::Public, T::BlockNumber> {
// 	fn public(&self) -> T::Public {
// 		self.public.clone()
// 	}
// }

impl<T: Config> Pallet<T> {

	fn vtb_onboard_user(action_params: &serde_json::Map<String, serde_json::Value>) -> Result<(), sp_runtime::DispatchError> {

		log::info!("action params: {:?}", &action_params);
		
		let user_eth_address = action_params["ethAddress"].as_str().ok_or(Error::<T>::InvalidValueEth)?;
		let user_pdot_address = action_params["pdotAddress"].as_str().ok_or(Error::<T>::InvalidValueEth)?;
		let transaction_hash = action_params["txnHash"].as_str().ok_or(Error::<T>::InvalidValueEth)?;
		let transaction_hash_lc_1 = transaction_hash.to_lowercase();
		let transaction_hash_lc = transaction_hash_lc_1.as_bytes().to_vec();

		let _ = <pallet_cross_chain::Pallet<T>>::vtb_onboard_user(
			user_pdot_address,
			user_eth_address,
			transaction_hash_lc,
			TokenType::Eth
		)?;

		Ok(())
	}

	fn vtb_eth_deposits(action_params: &serde_json::Map<String, serde_json::Value>) -> Result<(), Error<T>> {
	
		let user_eth_address = action_params["ethAddress"].as_str().ok_or(Error::<T>::InvalidValueEth)?;
		let user_pdot_address = action_params["pdotAddress"].as_str().ok_or(Error::<T>::InvalidValueEth)?;
		let amount = U256::from_dec_str(action_params["amount"].as_str().ok_or(Error::<T>::InvalidValueEth)?).unwrap_or_default();
		let transaction_hash = action_params["txnHash"].as_str().ok_or(Error::<T>::InvalidValueEth)?;
		let transaction_hash_lc_1 = transaction_hash.to_lowercase();
		let transaction_hash_lc = transaction_hash_lc_1.as_bytes().to_vec();

		let _ = <pallet_cross_chain::Pallet<T>>::vtb_crypto_deposits_sign_trnx(
			Some(user_pdot_address),
			user_eth_address,
			amount,
			transaction_hash_lc,
			TokenType::Eth
		);

		Ok(())
	}

	fn vtb_eth_withdrawn(action_params: &serde_json::Map<String, serde_json::Value>) -> Result<(), Error<T>> {

		let user_pdot_address = action_params["pdotAddress"].as_str().ok_or(Error::<T>::InvalidValueEth)?;
		let amount = U256::from_dec_str(action_params["amount"].as_str().ok_or(Error::<T>::InvalidValueEth)?).unwrap_or_default();
		let transaction_hash = action_params["txnHash"].as_str().ok_or(Error::<T>::InvalidValueEth)?;
		let transaction_hash_lc_1 = transaction_hash.to_lowercase();
		let transaction_hash_lc = transaction_hash_lc_1.as_bytes().to_vec();

		let _ = <pallet_cross_chain::Pallet<T>>::vtb_crypto_withdraw_sign_trnx(
			user_pdot_address,
			amount,
			transaction_hash_lc,
			TokenType::Eth
		);

		Ok(())
	}
}

