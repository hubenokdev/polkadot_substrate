#![recursion_limit="256"]

#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
pub use frame_support::traits::Get;
use frame_support::{
	ensure
};
use frame_system::{
	offchain::{
		AppCrypto, CreateSignedTransaction, SendSignedTransaction,
		SignedPayload, Signer, SigningTypes,
	},
};
use sp_runtime::{
	offchain as rt_offchain,
	offchain::{
		storage::{MutateStorageError, StorageRetrievalError, StorageValueRef},
		storage_lock::{BlockAndTime, StorageLock},
	},
	transaction_validity::{
		InvalidTransaction, TransactionSource, TransactionValidity, ValidTransaction,
	},
};
use scale_info::prelude::string::String;
use sp_std::{
	collections::{vec_deque::VecDeque},
	prelude::*, str, vec::Vec
};
use sp_io::offchain_index;
use primitive_types::U256;
use sp_core::crypto::AccountId32; 
use sp_std::borrow::ToOwned;
use sp_std::convert::TryInto;
pub mod custom_types;
mod constants;
mod cryptos;
pub use cryptos::types::TokenType;
mod trade;
use trade::types::{OrderBookStruct};
pub use trade::types::TradeType;
use crate::withdraw::types::WithdrawClaim;

mod apr;
mod ipfs;
mod distribution;
use distribution::{
	distribute::InitializeDistributionReq,
	claim::{ClaimTokenReq, ClaimTokenTrait}
};
mod utils;
mod users;
pub use users::{WalletStorage};
pub use users::types::{UserCryptoBasedData, User as UserType, Balances};

mod withdraw;
use withdraw::{
	types::{WithdrawCryptoReq},
	withdraw_trait::WithdrawCrypto 
};

mod vtbt;
use vtbt::{VtbtErc20Req, VTBTErc20};

// mod migrated_user_period_balance_update;
pub use pallet::*;
pub use pallet_usd_rate::UsdRate;
use serde::__private::ToString;

#[path = "../../../global_constants.rs"] mod global_constants;

type WithdrawRequest<T> = WithdrawClaim<<T as frame_system::Config>::AccountId, <T as pallet_timestamp::Config>::Moment, <T as frame_system::Config>::BlockNumber>;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

pub mod migration;

// pub mod weights;
pub mod weights;
use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// This pallet's configuration trait
	#[pallet::config]
	pub trait Config: CreateSignedTransaction<Call<Self>> + pallet_timestamp::Config + frame_system::Config + pallet_usd_rate::Config + pallet_vtbc_token::Config + pallet_vtbt::Config {
		/// The identifier type for an offchain worker.
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event> + IsType<<Self as pallet_vtbc_token::Config>::Event>;
		/// The overarching dispatch call type.
		type Call: From<Call<Self>>;
		/// The constant type CryptoPrecision as 18 decimal
		#[pallet::constant]
		type CryptoPrecision: Get<U256>;
		/// The constant type VtbcPrecision as 18 decimal
		#[pallet::constant]
		type VtbcPrecision: Get<U256>;
		/// The constant type MinUsd as $5
		#[pallet::constant]
		type MinUsd: Get<U256>;
		/// The constant type IpfsPeriod once per day
		#[pallet::constant]
		type IpfsTimestamp: Get<Self::Moment>;
		#[pallet::constant]
		type HourlyPeriod: Get<Self::Moment>;
		/// The constant type DistributionPeriod as 30 days
		#[pallet::constant]
		type DistributionTimestamp: Get<Self::Moment>;
		#[pallet::constant]
		type MinDepositBalanceToPayFee: Get<U256>;
		#[pallet::constant]
		type VtbErc20AssetId: Get<Self::AssetId>;
		#[pallet::constant]
		type SlotDuration: Get<Self::Moment>;
		#[pallet::constant]
		type PendingWithdrawMinTime: Get<Self::Moment>;
		#[pallet::constant]
		type PowerupDayInterval: Get<Self::Moment>;

		type WeightInfo: WeightInfo;

		//type AccountStore: StoredValue<Self::AccountId, custom_types::FeeCollectorAccount<Self::AccountId>>;
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

		fn on_initialize(n: T::BlockNumber) -> Weight {
			// Anything that needs to be done at the start of the block.
			if n > 1u32.into() {
				// This block will execute only when sytstem requres restarte.
				// if !Self::is_init() {
				// 	Self::initialize_apr_contants();
				// 	let time_stamp = <pallet_timestamp::Pallet<T>>::now();
	
				// 	let res = InitializeDistributionReq::<T>::init_distribution(time_stamp);
				// 	match res {
				// 		Ok(_res) => {
				// 			<InitDistribution<T>>::put(true);	
				// 		},
				// 		Err(_err) => (), 
				// 	}
				// };

				let time_stamp = <pallet_timestamp::Pallet<T>>::now();
				if Self::is_init() && time_stamp > Self::distribute(Self::get_current_period()).inittimestamp + T::DistributionTimestamp::get() {
					let _ = InitializeDistributionReq::<T>::check_and_initialize_distribution(time_stamp);
				}

				// if Self::is_init() && n == 21u32.into() {
				// 	let _ = Self::initiate_initialize_values_for_apr(U256::from(5_004_572_707_450_340_349_u128));
				// }
			}
			0
		}

		/// Offchain Worker entry point.
		fn offchain_worker(block_number: T::BlockNumber) {
		
			log::info!("Hello from offchain workers of vtbdex pallet!");

			let globals = Self::get_globals();	
			let apr_rate = AprEstimate::<T>::get(globals.current_year);
			if apr_rate.start_rate > U256::from(0_u64) && apr_rate.ta <= U256::from(0_u64) && block_number > 22u32.into() {
				let _ = apr::HourlyIncrease::<T>::calculate_ta_in_year_start();
			}
			// Reading back the off-chain indexing value. It is exactly the same as reading from
			// ocw local storage.
			WithdrawCryptoReq::<T::AccountId, T::BlockNumber>::check_for_withdraw_indexed_crypto_data::<T>(block_number);
		
		    let time_stamp = <pallet_timestamp::Pallet<T>>::now();
			let last_apr_timestamp = Self::updated_time_list().apr_timestamp;
			let rate = UsdRate::<T>::get();
			if time_stamp >= last_apr_timestamp + T::HourlyPeriod::get() && rate.vtbc_current_price > U256::from(0_u64) {
				let _res = apr::HourlyIncrease::<T>::calc_hourly_vtbc_rate(time_stamp);
			}
		
			if time_stamp >= T::IpfsTimestamp::get() {
				let _res = Self::check_ipfs_period(time_stamp);
			}

			let _time_stamp = <pallet_timestamp::Pallet<T>>::now();
			//	let _ = Self::do_powerup(time_stamp);

			//let _ = Self::fetch_estimated_gas_price(block_number);
		}

	}
	// Declare a pallet origin (this is optional).
	//
	// The macro accept type alias or struct or enum, it checks generics are consistent.
	#[pallet::origin]
	pub struct Origin<T>(PhantomData<T>);

	/// A public part of the pallet.
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		/*************************** Start Root extrinsic** ***************************/

		/// Root Extrinsic to Set fee coollector account
		/// This extrinsic is signed via the sudo user.
		/// It take one parameters of AccountId.
		/// The given AccountId must have wallet created in runtime.
		/// And the account must be linked with all deposit cryptos, such as (Eth, Eos, etc.)
		/// It initialize the system.
		/// It initialize the Vtbt token(This will be [performed only first time when the system started]).
		#[pallet::weight(<T as Config>::WeightInfo::set_vtbdex_fee_collector_account())]
		pub fn set_vtbdex_fee_collector_account(origin: OriginFor<T>, account_address: T::AccountId) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(<UserWallet<T>>::contains_key(&account_address.encode()), Error::<T>::PolkadotAddressNotLinked);
			let user = <UserWallet<T>>::get(&account_address.encode());

			ensure!(user.crypto_addresses.get(&TokenType::Eth) != None, Error::<T>::EthAddressNotLinkedWithPolkadotAccount);
			ensure!(user.crypto_addresses.get(&TokenType::Eos) != None, Error::<T>::EosAddressNotLinkedWithPolkadotAccount);

			let fees = custom_types::FeeCollectorAccount {
				fee_collector_address: account_address.clone(),
				fee: U256::from(1_000_000_000_000_000_000_u128),
			};
			VtbdexTransactionFee::<T>::put(fees);
			<VtbSystemRunning<T>>::put(true);
			let _ =  <pallet_vtbt::Pallet<T>>::initialize_vtbt_token(T::VtbErc20AssetId::get(), account_address.clone());

			Self::deposit_event(Event::SetFeeCollectorAddress {
				fee_collector: account_address,
			});

			Ok(())
		}

		/// Root Extrinsic for Set fee
		/// This extrinsic is signed via the sudo user.
		/// It take one parameters of U256 for fee_amount.
		/// It mutate the VtbdexTransactionFee state to set fee.
		#[pallet::weight(<T as Config>::WeightInfo::set_vtbdex_transaction_fee())]
		pub fn set_vtbdex_transaction_fee(origin: OriginFor<T>, fee: U256) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(<VtbSystemRunning<T>>::get(), Error::<T>::VtbdexSystemIsStopped);

			VtbdexTransactionFee::<T>::mutate(|fee_collector_opt| {
				let fee_collector = fee_collector_opt.as_mut().ok_or(Error::<T>::Unknown)?;
				fee_collector.fee = fee;

				Self::deposit_event(Event::SetFeeAmountInUsd {
					fee_amount: fee
				});
				Ok(())
			})
		}

		/// Root Extrinsic to unblock blocked user due to withdraw failure
		/// This extrinsic is signed via the sudo user.
		/// It take two parameters of AccountId & Vec<u8>.
		/// This extrinsic is to verify and return the blocked withdraw amount in user account.
		#[pallet::weight(<T as Config>::WeightInfo::check_and_return_withdraw_pending_amount())]
		pub fn check_and_return_withdraw_pending_amount(origin: OriginFor<T>, user_add: T::AccountId, id: Vec<u8>) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(<VtbSystemRunning<T>>::get(), Error::<T>::VtbdexSystemIsStopped);
			ensure!(<UserWallet<T>>::contains_key(&user_add.encode()), Error::<T>::PolkadotAddressNotLinked);

			let time_stamp = <pallet_timestamp::Pallet<T>>::now();
			let user_pdot_address = user_add.clone();
			BlockedUserWallet::<T>::mutate(&user_pdot_address, |list_opt| {
				let list = list_opt.as_mut().ok_or(Error::<T>::NumberIsTooLowForU256)?;
				match list.iter().position(|u| u.id == id) {
					Some(index) => {
						let copy_list = list.clone();
						let user_data = copy_list.get(index).ok_or(Error::<T>::NumberIsTooLowForU256)?;
						log::info!("user_data {:?}",user_data);
						log::info!("time_stamp {:?}",time_stamp);

						ensure!(time_stamp > user_data.timestamp + T::PendingWithdrawMinTime::get(), Error::<T>::WithdrawPendingAmountCanBeReturnedAfter24Hours);
						let total_amount = user_data.withdraw_amount; // Only return amount
						let _ = Self::add_update_balance(&user_data.token_type, &user_add.encode(), total_amount, U256::from(0_u64));
						Self::add_circulation_token_balance(&user_data.token_type, total_amount)?;
						list.remove(index);
						Self::deposit_event(Event::PendingWithdrawReturned {
							user: user_add, 
							amount: total_amount, 
							id
						});
						Ok(())
					},
					None =>  Err(frame_support::dispatch::DispatchError::from(<Error::<T>>::ClaimOrBlockedRequestDoesNotExist))
				}
			})
		}

		/// Root Extrinsic to unblock blocked user due to withdraw failure
		/// This extrinsic is signed via the sudo user.
		/// It take two parameters of AccountId & Vec<u8>.
		/// This extrinsic is to verify and remove the blocked withdraw amount from pending list.
		#[pallet::weight(<T as Config>::WeightInfo::check_and_remove_from_pending_list())]
		pub fn check_and_remove_from_pending_list(origin: OriginFor<T>, user: T::AccountId, id: Vec<u8>) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(<VtbSystemRunning<T>>::get(), Error::<T>::VtbdexSystemIsStopped);
			ensure!(<UserWallet<T>>::contains_key(&user.encode()), Error::<T>::PolkadotAddressNotLinked);

			let user_pdot_address = user.clone();
			BlockedUserWallet::<T>::mutate(&user_pdot_address, |list_opt| {
				let list = list_opt.as_mut().ok_or(Error::<T>::NumberIsTooLowForU256)?;
				match list.iter().position(|u| u.id == id ) {
					Some(index) => {
						let time_stamp = <pallet_timestamp::Pallet<T>>::now();
						let user_data = list.get(index).ok_or(Error::<T>::NumberIsTooLowForU256)?;
						log::info!("user_data {:?}",user_data);
						log::info!("time_stamp {:?}",time_stamp);
						ensure!(time_stamp > user_data.timestamp + T::PendingWithdrawMinTime::get(), Error::<T>::WithdrawPendingAmountCanBeReturnedAfter24Hours);
						list.remove(index);
						Self::deposit_event(Event::PendingWithdrawRejected { user, id });
						Ok(())
					},
					None => Err(frame_support::dispatch::DispatchError::from("<Error::<T>>::ClaimOrBlockedRequestDoesNotExist"))
				}
			})
		}

		/// Root Extrinsic to stop vtbdex functionality
		/// This extrinsic is signed via the sudo user.
		/// This extrinsic is to stop the functionality of Vtbdex system
		/// such as Buy/Sell/Cancel_order/Withdraw/Redeem
		#[pallet::weight(<T as Config>::WeightInfo::stop_vtbdex_functionality())]
		pub fn stop_vtbdex_functionality(origin: OriginFor<T>, ) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(<VtbSystemRunning<T>>::get(), Error::<T>::VtbdexSystemIsStopped);
			<VtbSystemRunning<T>>::put(false);
			Self::deposit_event(Event::VtbdexSystemStopped());
			
			Ok(())
		}

		/// Root Extrinsic to resume vtbdex functionality
		/// This extrinsic is signed via the sudo user.
		/// This extrinsic is to resume the functionality of Vtbdex system.
		/// such as Buy/Sell/Cancel_order/Withdraw/Redeem
		#[pallet::weight(<T as Config>::WeightInfo::resume_vtbdex_functionality())]
		pub fn resume_vtbdex_functionality(origin: OriginFor<T>) -> DispatchResult {
			ensure_root(origin)?;
			ensure!(!<VtbSystemRunning<T>>::get(), Error::<T>::VtbDexSystemIsRunning);
			<VtbSystemRunning<T>>::put(true);

			Self::deposit_event(Event::VtbdexSystemResume());
			Ok(())
		}

		#[pallet::weight(<T as Config>::WeightInfo::resume_vtbdex_functionality())]
		pub fn stop_system_functionality(origin: OriginFor<T>, stop_for: custom_types::StopData) -> DispatchResult {
			ensure_root(origin)?;

			log::info!("stop for: {:?}", stop_for);
			// StopEnabled::<T>::put(stop_for);
			// ensure!(!<VtbSystemRunning<T>>::get(), Error::<T>::VtbDexSystemIsRunning);
			<VtbSystemRunning<T>>::put(true);

			Self::deposit_event(Event::VtbdexSystemResume());
			Ok(())
		}

		/*************************** End of Root extrinsic** ***************************/

		/*************************** Start public extrinsic ** ***************************/

		///fn buy_vtbc(origin: OriginFor<T>, token_type: TokenType, crypto_amount: U256) -> DispatchResult;
		///This function take three argument as Origin, TokenType, and amount of Crypto.
		///It check and verify the user balance and than it start the buy_vtbc transaction.
		///There are two check before starting buy vtbc process, 
		///One user must be onboarded with linked cryptos.
		///VtbSystem must be running
		///This is signed extrinc via the user
		///
		///  successful completion it emit list of events based on the execution of each process.
		///But the initial event will be BuyVtbcRequested,
		///BuyVtbcRequested(T::AccountId, TokenType, U256),
		#[pallet::weight((<T as Config>::WeightInfo::buy_vtbc(), Pays::No))]
		pub fn buy_vtbc(origin: OriginFor<T>, token_type: TokenType, crypto_amount: U256) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(<VtbSystemRunning<T>>::get(), Error::<T>::VtbdexSystemIsStopped);
			ensure!(<UserWallet<T>>::contains_key(&who.encode()), Error::<T>::PolkadotAddressNotLinked);
			let req_data = trade::types::TradeRequest::new(token_type, 
					TradeType::Buy, 
					who, 
					crypto_amount);

			let res = trade::buyvtbc::BuyVtbc::<T>::initiate_buy_vtbc(req_data);	
			match res {
				Ok(_) => {
					log::info!("=============Buy vtbc Extrinsic Completed =============");
					Ok(())
				},
				Err(err) => Err(err), 
			}
		}

		///fn cancel_buy_vtbc_order(origin: OriginFor<T>, order_id: Vec<u8>, token_type: TokenType) -> DispatchResult;
		///This function take three argument as Origin, OrderId, and TokenType.
		///It check and verify the user balance and than it start the cancel_order transaction.
		///There are three check before starting cancel buy order process, 
		///One user must be onboarded with linked cryptos.
		///VtbSystem must be running
		///OrderId must exist in the system
		///This is signed extrinc via the user
		///On successfull completion event emitted is BuyOrderRefunded(Vec<u8>, T::AccountId, TokenType, U256)
		#[pallet::weight((<T as Config>::WeightInfo::cancel_buy_vtbc_order(), Pays::No))]
		pub fn cancel_buy_vtbc_order(origin: OriginFor<T>, order_id: Vec<u8>, token_type: TokenType) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(<VtbSystemRunning<T>>::get(), Error::<T>::VtbdexSystemIsStopped);
			ensure!(<UserWallet<T>>::contains_key(&who.encode()), Error::<T>::PolkadotAddressNotLinked);
			//ensure!(UserIdListNMap::<T>::contains_key((&TradeType::Buy, &token_type, &who, &order_id)), Error::<T>::OrderIdDoesNotExist);
			let res = trade::cancelorder::CancelOrder::<T>::initiate_cancel_order(&who, &order_id, token_type, TradeType::Buy);
			match res {
				Ok(res) => {
					Self::deposit_event(Event::CanceledOrder {
						trade_type: TradeType::Buy, 
						user: who, 
						order_id, 
						amount: res
					});

					Ok(())
				},
				Err(err) => Err(err), 
			}
		}


		///fn sell_vtbc(origin: OriginFor<T>, token_type: TokenType, volume_vtbc: U256) -> DispatchResult ;
		///This function take three argument as Origin, TokenType & Vtbc amount.
		///It check and verify the user balance and than it start the sell_vtbc transaction.
		///There are two check before starting cancel sell order process, 
		///One user must be onboarded with linked cryptos.
		///VtbSystem must be running
		///This is signed extrinc via the user
		///Event emitted via this extrinsic is SellVtbcRequested(T::AccountId,U256)
		#[pallet::weight((<T as Config>::WeightInfo::sell_vtbc(), Pays::No))]
		pub fn sell_vtbc(origin: OriginFor<T>, token_type: TokenType, volume_vtbc: U256) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(<VtbSystemRunning<T>>::get(), Error::<T>::VtbdexSystemIsStopped);
			ensure!(<UserWallet<T>>::contains_key(&who.encode()), Error::<T>::PolkadotAddressNotLinked);

			let req_data = trade::types::TradeRequest::new(token_type, 
				TradeType::Sell, 
				who, 
				volume_vtbc);

		    let res = trade::sellvtbc::SellVtbc::<T>::initiate_sell_vtbc(req_data);
			match res {
				Ok(_) => {
					log::info!("===========================Sell vtbc Extrinsic Completed ======================================");
					Ok(())
				},
				Err(err) => Err(err), 
			}
		}


		///fn cancel_sell_vtbc_order(origin: OriginFor<T>, order_id: Vec<u8>, token_type: TokenType) -> DispatchResult;
		///This function take three argument as Origin, OrderId, and TokenType.
		///It check and verify the user balance and than it start the cancel order transaction.
		///There are three check before starting cancel sell order process, 
		///One user must be onboarded with linked cryptos.
		///VtbSystem must be running
		///OrderId must exist in the system
		///This is signed extrinc via the user
		#[pallet::weight((<T as Config>::WeightInfo::cancel_sell_vtbc_order(), Pays::No))]
		pub fn cancel_sell_vtbc_order(origin: OriginFor<T>, order_id: Vec<u8>, token_type: TokenType) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(<VtbSystemRunning<T>>::get(), Error::<T>::VtbdexSystemIsStopped);
			ensure!(<UserWallet<T>>::contains_key(&who.encode()), Error::<T>::PolkadotAddressNotLinked);
		//	ensure!(UserIdListNMap::<T>::contains_key((&TradeType::Sell, &token_type, &who, &order_id)), Error::<T>::OrderIdDoesNotExist);
			let res = trade::cancelorder::CancelOrder::<T>::initiate_cancel_order(&who, &order_id, token_type, TradeType::Sell);
			match res {
				Ok(res) => {
					Self::deposit_event(Event::CanceledOrder {
						trade_type: TradeType::Sell, 
						user: who, 
						order_id, 
						amount: res
					});
					Ok(())
				},
				Err(err) => Err(err), 
			}
		}

		/// fn withdraw_initiate(origin: OriginFor<T>, token_type: TokenType, amount: U256) -> DispatchResult;
		/// This is signed extrinsic signed by the user for withdraw cryptos from the Vtbsystem.
		/// This extrinsic has two check,
		/// One System must be running & second User address must be linked.
		/// When both the check passes withdraw_initiate process started by the system
		#[pallet::weight((<T as Config>::WeightInfo::withdraw_initiate(), Pays::No))]
		pub fn withdraw_initiate(origin: OriginFor<T>, token_type: TokenType, amount: U256) -> DispatchResult {
			let who = ensure_signed(origin)?;

			ensure!(<VtbSystemRunning<T>>::get(), Error::<T>::VtbdexSystemIsStopped);
			ensure!(<UserWallet<T>>::contains_key(&who.encode()), Error::<T>::PolkadotAddressNotLinked);

			let mut req_data = WithdrawCryptoReq::new(who, token_type, amount);
			let res = WithdrawCrypto::<T>::initiate_withdraw(&mut req_data);
			match res {
				Ok(_res) => Ok(()),
				Err(err) => Err(err), 
			}
		}

		///fn initiate_convert_vtbc_to_vtbt_substrate(origin: OriginFor<T>, amount: U256) -> DispatchResult;
		///This extrinsic is designed to sign via the user.
		///This is designed for Vtbt ERC20 token in substrate.
		///This extrinsic is used to convert Vtbc to Vtbt token.
		#[pallet::weight((<T as Config>::WeightInfo::initiate_convert_vtbc_to_vtbt_substrate(), Pays::No))]
		pub fn initiate_convert_vtbc_to_vtbt_substrate(origin: OriginFor<T>, amount: U256) -> DispatchResult {
			let signer_add = ensure_signed(origin.clone())?;

			ensure!(<VtbSystemRunning<T>>::get(), Error::<T>::VtbdexSystemIsStopped);
			ensure!(<UserWallet<T>>::contains_key(&signer_add.encode()), Error::<T>::PolkadotAddressNotLinked);
			let mut req = VtbtErc20Req::<T::AccountId, OriginFor<T>>::new(origin, signer_add, amount, None);
			VTBTErc20::<T>::initiate_convert_vtbc_to_vtbt_erc20substrate_token(&mut req)?;	
			Ok(())
		}

		///fn initiate_transfer_of_vtbt_substrate(origin: OriginFor<T>, to_address: T::AccountId, amount: U256)) -> DispatchResult;
		///This extrinsic is designed to sign via the user.
		///This is designed for Vtbt ERC20 token in substrate.
		///This extrinsic is used to convert Vtbt to Vtbc token.
		#[pallet::weight((<T as Config>::WeightInfo::initiate_convert_vtbt_to_vtbc_substrate(), Pays::No))]
		pub fn initiate_convert_vtbt_to_vtbc_substrate(origin: OriginFor<T>, amount: U256) -> DispatchResult {
			let signer_add = ensure_signed(origin.clone())?;

			ensure!(<VtbSystemRunning<T>>::get(), Error::<T>::VtbdexSystemIsStopped);
			ensure!(<UserWallet<T>>::contains_key(&signer_add.encode()), Error::<T>::PolkadotAddressNotLinked);

			let req = VtbtErc20Req::new(origin, signer_add, amount, None);
			VTBTErc20::<T>::initiate_convert_vtbt_to_vtbc_erc20substrate_token(&req)?;	

			Ok(())
		}

		///fn initiate_transfer_of_vtbt_substrate(origin: OriginFor<T>, to_address: T::AccountId, amount: U256)) -> DispatchResult;
		///This extrinsic is designed to sign via the user.
		///This is designed for Vtbt ERC20 token in substrate.
		///This extrinsic is used to transfer Vtbt to anoter user.
		#[pallet::weight((<T as Config>::WeightInfo::initiate_transfer_of_vtbt_substrate(), Pays::No))]
		pub fn initiate_transfer_of_vtbt_substrate(origin: OriginFor<T>, to_address: T::AccountId, amount: U256) -> DispatchResult {
			let signer_add = ensure_signed(origin.clone())?; //from_address is signer

			ensure!(<VtbSystemRunning<T>>::get(), Error::<T>::VtbdexSystemIsStopped);
			ensure!(<UserWallet<T>>::contains_key(&signer_add.encode()), Error::<T>::PolkadotAddressNotLinked);
			ensure!(<UserWallet<T>>::contains_key(&to_address.encode()), Error::<T>::PolkadotAddressNotLinked);
			ensure!(signer_add != to_address, Error::<T>::BothFromAndToCanNotBeSame);
			let req = VtbtErc20Req::new(origin, signer_add, amount, Some(to_address));
			VTBTErc20::<T>::initiate_transfer_of_vtbt_erc20substrate_token(&req)?;	

			Ok(())
		}

		///fn initiate_transfer_from_of_vtbt_substrate(origin: OriginFor<T>, from_address: T::AccountId, to_address: T::AccountId, amount: U256) -> DispatchResult;
		///This extrinsic is designed to sign via the user.
		///This is designed for Vtbt ERC20 token in substrate.
		///When one user want to transfer Vtbt on behalf of another user, he can use transfer_from.
		#[pallet::weight((<T as Config>::WeightInfo::initiate_transfer_from_of_vtbt_substrate(), Pays::No))]
		pub fn initiate_transfer_from_of_vtbt_substrate(origin: OriginFor<T>, from_address: T::AccountId, to_address: T::AccountId, amount: U256) -> DispatchResult {
			let signer = ensure_signed(origin.clone())?;

			ensure!(<VtbSystemRunning<T>>::get(), Error::<T>::VtbdexSystemIsStopped);
			ensure!(<UserWallet<T>>::contains_key(&signer.encode()), Error::<T>::PolkadotAddressNotLinked);
			ensure!(<UserWallet<T>>::contains_key(&from_address.encode()), Error::<T>::PolkadotAddressNotLinked);
			ensure!(<UserWallet<T>>::contains_key(&to_address.encode()), Error::<T>::PolkadotAddressNotLinked);
			ensure!(from_address != to_address, Error::<T>::BothFromAndToCanNotBeSame);

			let req = VtbtErc20Req::new(origin, from_address, amount, Some(to_address));
			VTBTErc20::<T>::initiate_transfer_from_of_vtbt_erc20substrate_token(&req, signer)?;	

			Ok(())
		}

		/// fn claim_distribution(origin: OriginFor<T>, token_type: TokenType) -> DispatchResult;
		/// This is signed extrinsic signed by the user to claim distributable balance of the closed period.
		/// It takes one argument of TokenType enum.
		/// This extrinsic has two check,
		/// One System must be running & second User address must be linked.
		/// When both the check passes claim process started by the system
		/// On successfull completion of the extrinsic it emit event ClaimedSuccess
		/// ClaimedSuccess(T::AccountId, TokenType, U256 )
		#[pallet::weight((<T as Config>::WeightInfo::claim_distribution(), Pays::No))]
		pub fn claim_distribution(origin: OriginFor<T>, token_type: TokenType) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(<VtbSystemRunning<T>>::get(), Error::<T>::VtbdexSystemIsStopped);

			ensure!(<UserWallet<T>>::contains_key(&who.encode()), Error::<T>::PolkadotAddressNotLinked);
			let claim_req = ClaimTokenReq::new(who, Some(token_type));
			let res = ClaimTokenTrait::<T>::initiate_claim_distribution(claim_req);	
			match res {
				Ok(_res) => Ok(()),
				Err(err) => Err(err), 
			}
		}

		/// fn claim_all_distribution(origin: OriginFor<T>) -> DispatchResult;
		/// This is signed extrinsic signed by the user to claim distributable balance of the closed period.
		/// This extrinsic has two check,
		/// One System must be running & second User address must be linked.
		/// When both the check passes, it start claim process for each distributable token.
		/// On successfull completion of the extrinsic it emit event ClaimedSuccess
		/// ClaimedSuccess(T::AccountId, TokenType, U256 )
		#[pallet::weight((<T as Config>::WeightInfo::claim_all_distribution(), Pays::No))]
		pub fn claim_all_distribution(origin: OriginFor<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			ensure!(<VtbSystemRunning<T>>::get(), Error::<T>::VtbdexSystemIsStopped);

			ensure!(<UserWallet<T>>::contains_key(&who.encode()), Error::<T>::PolkadotAddressNotLinked);
			let mut claim_req = ClaimTokenReq::new(who, None);
			let res = ClaimTokenTrait::<T>::initiate_claim_all_distribution(&mut claim_req);	
			match res {
				Ok(_res) => Ok(()),
				Err(err) => Err(err), 
			}
		}

		/// fn check_claim_distribution(origin: OriginFor<T>, address: T::AccountId, period: u64) -> DispatchResult;
		/// This is unsigned extrinsic executed by the user to check distributable balance of the closed period.
		/// This extrinsic has three check,
		/// One System must be running, second User address must be linked & third period must be closed.
		/// When all the check passes, it start to calculate availble claim balance for each distributable token.
		/// On successfull completion of the extrinsic it update the storage which will list the available claim balance.
		#[pallet::weight((<T as Config>::WeightInfo::check_claim_distribution(), Pays::No))]
		pub fn check_claim_distribution(origin: OriginFor<T>, address: T::AccountId, period: u64) -> DispatchResult {
			ensure_none(origin)?;
			ensure!(<VtbSystemRunning<T>>::get(), Error::<T>::VtbdexSystemIsStopped);
			ensure!(<Distribution<T>>::get(period).closed, Error::<T>::DistributionPeriodIsStillOpen);
			ensure!(<UserWallet<T>>::contains_key(&address.encode()), Error::<T>::PolkadotAddressNotLinked);
			let mut claim_req = ClaimTokenReq::new(address, None);
			let res = ClaimTokenTrait::<T>::check_claim_distribution_initiate(&mut claim_req);
			match res {
				Ok(_res) => {
					Ok(())
				},
				Err(err) => Err(err), 
			}
		}
		/*************************** End of public extrinsic ** ***************************/

		/*************************** Start extrinsic which is signed only by ocw ** ***************************/

		//Note: As system is running and stable, so make this part of code comment.
		// Uncomment it only when system restart required.
		// /// Extrinsic signed by Ocw to initialize initial values of apr
		// /// This extrinsic is designed to sign only via Ocw.
		// /// This set the start price of the system on the system start.
		// /// This function must be called only once in entire life cycle of Vtbdex system.
		// /// 
		// /// 
		#[pallet::weight(<T as Config>::WeightInfo::initialize_values_for_apr())]
		pub fn initialize_values_for_apr(origin: OriginFor<T>, vtbc_start_price: U256) -> DispatchResult {
			let _who = ensure_signed(origin)?;			// Call::initialize_value_for_year_ta(U256::from(ta_18 as u128), final_price));
			
			let res = apr::InitializeApr::<T>::initiate_initialize_values_for_apr(vtbc_start_price);
			match res {
				Ok(_) => Ok(()),
				Err(err) => Err(frame_support::dispatch::DispatchError::from(err)), 
			}
		}

		/// Extrinsic signed by ocw to initialize value for ta.
		/// This extrinsic is designed to sign only via Ocw.
		/// This function set the value for contant value ta for algorithmic calculation.
		/// This constant ta is being calculated once in a year at the year start of the chain.
		/// Also it set the year start price of the Vtbc
		#[pallet::weight(<T as Config>::WeightInfo::initialize_value_for_year_ta())]
		pub fn initialize_value_for_year_ta(origin: OriginFor<T>, ta: U256, elpased_price: U256) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			
			let globals = Self::get_globals();	
			AprEstimate::<T>::mutate(globals.current_year, |obj|{
				obj.ta = ta;
			});
			UsdRate::<T>::mutate(|get_usd_rate| {
				get_usd_rate.vtbc_current_price = elpased_price;
				get_usd_rate.vtbc_last_apr_rate = elpased_price;
			});
			<UsdVtbcH<T>>::put(elpased_price);
			Ok(())
		}

		/// Extrinsic signed by Ocw to submit new vtbc rate
		/// This extrinsic is designed to sign only via Ocw.
		/// This extrinsic is signed to update and notify about the Vtbc price increase. 
		/// This extrinsic is executed per hour, as per the apr algorithm,
		/// the price must be increasing at each hour.
		/// On successful increment of price, emit event HourlyChanges to notify.
		#[pallet::weight(<T as Config>::WeightInfo::submit_vtbc_hourly_rate())]
		pub fn submit_vtbc_hourly_rate(origin: OriginFor<T>, rate: U256, time_stamp: T::Moment, apr_calculated_rate: U256, hours: u32, remaining_hours: u32) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			let last_apr_timestamp = Self::updated_time_list().apr_timestamp;
			ensure!(time_stamp >= last_apr_timestamp + T::HourlyPeriod::get(), Error::<T>::HourluCalculationRecentlySentByAnotherOcw);
			
			if rate > U256::from(0_u64) {
				let mut old_rate = U256::from(0_u64);
				UsdRate::<T>::mutate(|get_usd_rate| {
					old_rate = get_usd_rate.vtbc_last_apr_rate;
					get_usd_rate.vtbc_current_price = rate;
					get_usd_rate.vtbc_last_apr_rate = rate;
				});
				let old_time_stamp: T::Moment = UpdatedTimeList::<T>::get().apr_timestamp; 
				UpdatedTimeList::<T>::mutate(|obj| {
					obj.apr_timestamp = time_stamp;
				});
				<TxnAffectedVtbcPrice<T>>::put(U256::from(0_u8));
				<UsdVtbcH<T>>::put(apr_calculated_rate);
				Self::deposit_event(Event::IncreaseHourlyAccuralRate {
					old_vtbc_rate: old_rate, 
					old_timestamp: old_time_stamp, 
					new_vtbc_rate: rate, 
					new_timestamp: time_stamp
				});
			}
			apr::InitializeApr::<T>::hourly_hours_changes(hours);
			Self::deposit_event(Event::HourlyChanges());
			if remaining_hours > 0 {
				apr::InitializeApr::<T>::hourly_hours_changes(remaining_hours);
				Self::deposit_event(Event::HourlyChanges());
			}

			Ok(())
		}

		/// Extrinsic signed by Ocw to to give withdraw in progress message
		/// This extrinsic is designed to sign only via Ocw.
		/// This extrinsic is signed to notify user about the withdraw In progress.
		/// This update the transaction_hash in the runtime state of BlockedUserWallet struct.
		/// On successful completion, it emit event to notify user.
		/// WithdrawInProgress(pdot_address, crypto_address, token_type, crypto_amount, transaction_hash))
		#[pallet::weight(<T as Config>::WeightInfo::withdraw_inprogress())]
		pub fn withdraw_inprogress(origin: OriginFor<T>, pdot_address: T::AccountId, crypto_address: Vec<u8>, transaction_hash: Vec<u8>, id: Vec<u8>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			BlockedUserWallet::<T>::mutate(&pdot_address, |list_opt| {
				let list = list_opt.as_mut().ok_or(Error::<T>::NumberIsTooLowForU256)?;
				match list.iter().position(|u| u.id == id && u.transaction_hash.is_empty() ) {
				
					Some(index) => {
						list.get_mut(index).ok_or(Error::<T>::NoneValue)?.transaction_hash = transaction_hash.clone();
						let list_res = list.get(index).ok_or(Error::<T>::NumberIsTooLowForU256)?;
						let crypto_amount = list_res.withdraw_amount;
						Self::deposit_event(Event::WithdrawInProgress {
							user: pdot_address.clone(), 
							crypto_address, 
							token_type: list_res.token_type, 
							amount: crypto_amount, 
							transaction_id: transaction_hash
						});

						Ok(())
					},
					None => {
						log::info!("User does not exist in blocked user list");
						Err(frame_support::dispatch::DispatchError::from("User does not exist in blocked user list"))
					}
				}
			})
		}

		/// Extrinsic signed by Ocw to give failed msg for withdraw transaction
		/// This extrinsic is designed to sign only via Ocw.
		/// This extrinsic is signed to notify user about the withdraw failed.
		#[pallet::weight(<T as Config>::WeightInfo::withdraw_failed())]
		pub fn withdraw_failed(origin: OriginFor<T>, pdot_address: T::AccountId, cryto_address: Vec<u8>, msg: Vec<u8>, amount: U256, id: Vec<u8>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			BlockedUserWallet::<T>::mutate(&pdot_address, |list_opt| {
				log::info!("list from wf {:?}",list_opt);
				let list = list_opt.as_mut().ok_or(Error::<T>::NumberIsTooLowForU256)?;

				match list.iter().position(|u| u.id == id && u.transaction_hash.is_empty() && u.withdraw_amount == amount ) {
					Some(index) => {
						let user_data = list.get(index).ok_or(Error::<T>::NumberIsTooLowForU256)?;
						let token_type = user_data.token_type;
						let total_amount = user_data.withdraw_amount; //return only withdraw amount to user
						let _ = Self::add_update_balance(&user_data.token_type, &pdot_address.encode(), total_amount, U256::from(0_u64));
						Self::add_circulation_token_balance(&user_data.token_type, user_data.withdraw_amount)?;
						list.remove(index);
						
						Self::deposit_event(Event::WithdrawFailed { 
							user: pdot_address.clone(), 
							crypto_address: cryto_address, 
							token_type, 
							amount: total_amount, 
							msg
						});

						Ok(())
					},
					None => {
						log::info!("User does not exist in blocked user list");
						Err(frame_support::dispatch::DispatchError::from("User does not exist in blocked user list"))
					}
				}
			})
		}

		/// Extrinsic signed by Ocw to give failed msg for withdraw transaction
		/// This extrinsic is designed to sign only via Ocw.
		/// This extrinsic is signed to notify user about the withdraw failed due to api timeout.
		#[pallet::weight(<T as Config>::WeightInfo::withdraw_failed_due_to_time_out())]
		pub fn withdraw_failed_due_to_time_out(origin: OriginFor<T>, pdot_address: T::AccountId, cryto_address: Vec<u8>, msg: Vec<u8>, id: Vec<u8>) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			
			BlockedUserWallet::<T>::mutate(&pdot_address, |list_opt| {
				log::info!("list from wto {:?}",list_opt);
				let list = list_opt.as_mut().ok_or(Error::<T>::NumberIsTooLowForU256)?;

				match list.iter().position(|u| u.id == id && u.transaction_hash.is_empty() ) {
					Some(index) => {
						list.get_mut(index).ok_or(Error::<T>::NoneValue)?.transaction_hash = "got time out from network, needs to verify by admin".as_bytes().to_vec();
						let user_data = list.get(index).ok_or(Error::<T>::NumberIsTooLowForU256)?;
						let token_type = user_data.token_type;
						let crypto_amount = user_data.withdraw_amount;
						Self::deposit_event(Event::WithdrawFailed { 
							user: pdot_address.clone(), 
							crypto_address: cryto_address, 
							token_type, 
							amount: crypto_amount, 
							msg
						});
							
						Ok(())
					},
					None => {
						log::info!("User does not exist in blocked user list");
						Err(frame_support::dispatch::DispatchError::from("User does not exist in blocked user list"))
					}
				}
			})
		}
				
		/// Extrinsic signed by Ocw to submit new estimated gas price for set IPFS on ethereum
		/// This extrinsic is designed to sign only via Ocw.
		/// This set the fetched gas price for ipfs
		#[pallet::weight(<T as Config>::WeightInfo::submit_new_estimated_price())]
		pub fn submit_new_estimated_price(origin: OriginFor<T>, gas_price: U256) -> DispatchResult {
			let who = ensure_signed(origin)?;

			<EstimatedGasPrice<T>>::put(gas_price);
			Self::deposit_event(Event::UpdateNewEstimatedGasPrice {
				signer: Some(who), 
				amount: gas_price
			});
			Ok(())
		}
		
		/// Extrinsic signed by Ocw to submit new estimated gas price for set IPFS on ethereum
		/// This extrinsic is designed to sign only via Ocw.
		/// This charge the gas-fee to set-ipfs call during the process of withdraw.
		#[pallet::weight(<T as Config>::WeightInfo::charge_set_ipfs_trnx_price())]
		pub fn charge_set_ipfs_trnx_price(origin: OriginFor<T>, address: T::AccountId, trnx_price: U256, ipfs_hash: Vec<u8>, token_type: TokenType, trnx_hash: Vec<u8>) -> DispatchResult {
			let who = ensure_signed(origin)?;

			Self::sub_update_balance(&token_type, &address.encode(), trnx_price, U256::from(0_u64))?;
			//Self::pay_trnx_fee_for_withdraw(&token_type, &address, trnx_price, "SetIpfs ETH transaction price")?;
			Self::deposit_event(Event::ChargeIpfsSnapshotGasPrice{
				signer: Some(who), 
				user: address, 
				amount: trnx_price, 
				ipfs_hash, 
				token_type, 
				transaction_id: trnx_hash,
			});
			Ok(())
		}
		/*************************** End of extrinsic which is signed only by ocw ** ***************************/

	}

	/// Events for the pallet.
	#[pallet::event]
	#[pallet::generate_deposit(pub fn deposit_event)]

	pub enum Event<T: Config> {
		/// Event generated when new stop request is accepted.
		VtbdexSystemStopped(),

		/// Event generated when new resume request is accepted.
		VtbdexSystemResume(),

		/// Event generated when set new fee collector address request is accepted.
		SetFeeCollectorAddress {fee_collector: T::AccountId},

		/// Event generated when set new fee amount request is accepted.
		SetFeeAmountInUsd {fee_amount: U256},

		/// Event generated when new withdraw request is accepted.
		WithdrawInitiated {
			user: T::AccountId, 
			crypto_address: Vec<u8>, 
			amount: U256, 
			id: Vec<u8> 
		},

		/// Event generated when withdraw request processed from ocw indexed queue & is sent on EthNetwork.
		WithdrawInProgress {
			user: T::AccountId, 
			crypto_address: Vec<u8>, 
			token_type: TokenType, 
			amount: U256, 
			transaction_id: Vec<u8>
		},

		/// Event generated when withdraw request is failed from Eth/Eos Network.
		WithdrawFailed {
			user: T::AccountId, 
			crypto_address: Vec<u8>, 
			token_type: TokenType, 
			amount: U256, 
			msg: Vec<u8>
		},

		/// Event generated when new buyvtbc request is accepted.
		BuyVtbcRequested {
			buyer: T::AccountId, 
			token_type: TokenType, 
			crypto_amount: U256
		},

		/// Event generated when buy from reserve taken place.
		BuyVtbcFromReserve {
			buyer: T::AccountId, 
			token_type: TokenType, 
			crypto_amount: U256, 
			vtbc_amount: U256
		},

		/// Event generated when fullfilled buyorder from selllist(using FIFO)
		BuyVtbcFromSellOrder {
			buyer: T::AccountId, 
			token_type: TokenType, 
			seller: Option<T::AccountId>, 
			order_id: Vec<u8>, 
			crypto_amount: U256, 
			vtbc_amount: U256
		},

		/// Event generated when reserve balance is 0, so new BuyOrder is opened.
		OpenedNewBuyVtbcOrder {
			buyer: T::AccountId, 
			token_type: TokenType, 
			order_id: Vec<u8>, 
			crypto_amount: U256
		},
	
		/// Event generated when new sell_vtbc request is accepted.
		SellVtbcRequested {seller: T::AccountId, vtbc_amount: U256},

		/// Event generated when opened sellOrder
		OpenSellOrder {
			seller: T::AccountId, 
			order_id: Vec<u8>, 
			token_type: TokenType, 
			vtbc_amount: U256
		},
		///Event generated when buy order is fulfilled
		SellVtbcToFillBuyOrder {
			seller: T::AccountId, 
			token_type: TokenType, 
			buyer: T::AccountId, 
			order_id: Vec<u8>, 
			vtbc_amount: U256, 
			crypto_amount: U256
		},

		/// Event generated when trade (Buy/Sell) order is canceled.
        CanceledOrder {
			trade_type: TradeType, 
			user: T::AccountId, 
			order_id: Vec<u8>, 
			amount: U256
		},
		/// Event generated when order is canceled & amount is refunded.
		OrderRefunded {
			trade_type: TradeType, 
			order_id: Vec<u8>, 
			user: T::AccountId, 
			token_type: TokenType, 
			amount: U256
		},

		///Event generated when evere user is charged with Substrate transaction fee.
		TransactionSuccessFee {
			user: T::AccountId, 
			reason: Vec<u8>, 
			token_type: TokenType, 
			amount: U256
		},

		/// Event generated when reserve balance is 0, so new BuyOrder is opened.
		ReserveBalanceIsZero {amount: U256},
	
		/// Event generated Vtbc price increase due to buy transaction.
		IncreasedByTransaction {transaction_count: u64, amount: U256},
		
		/// Event generated Vtbc price increase due to buy transaction.
		IncreasedVtbRateDueToTransaction {
			old_vtbc_rate: U256, 
			new_vtbc_rate: U256
		},

		/// Event generated when Vtbc price increased as per hourly algorithm,
		IncreaseHourlyAccuralRate {
			old_vtbc_rate: U256, 
			old_timestamp: T::Moment, 
			new_vtbc_rate: U256, 
			new_timestamp: T::Moment
		},

		HourlyChanges(),
		NewDistributionPeriodAdded(u64, T::Moment),

		///Event generated when Vtbt token mint process initiated.
		MintSubstrateErc20VTBtInitiated {
			user: T::AccountId, 
			vtbt_amount: U256
		},
		///Event generated when conversion of Vtbc to Vtbt completed successfully.
		ConvertVtbcToVtbtSuccess {
			user: T::AccountId, 
			vtbc_amount: U256,
			vtbt_amount: U256
		},

		///Event generated when Vtbt token mint process initiated.
		BurnSubstrateErc20VTBtInitiated {
			user: T::AccountId, 
			vtbt_amount: U256
		},
		///Event generated when conversion of Vtbt to Vtbc completed successfully.
		ConvertVtbtToVtbcSuccess {
			user: T::AccountId, 
			vtbt_amount: U256, 
			vtbc_amount: U256
		},

		///Event generated when transfer of Vtbt initiated
		TransferSubstrateErc20VTBtInitiated {
			sender_address: T::AccountId, 
			receiver_address: T::AccountId, 
			vtbt_amount: U256 
		},

		///Event generated when transfer of Vtbt completed successfully
		TransferVtbtErc20Success {
			sender_address: T::AccountId, 
			receiver_address: T::AccountId, 
			vtbt_amount: U256 
		},

		///Event generated when Claim distributable token completed successfuly
		ClaimedSuccess {
			user: T::AccountId, 
			token_type: TokenType, 
			claimed_amount: U256 
		},

		/// Event generated when withdraw request is failed from CryptoNetwork.
		PendingWithdrawReturned {
			user: T::AccountId,
			amount: U256, 
			id: Vec<u8>
		},

		/// Event generated when pending withdraw amount is invalid
		PendingWithdrawRejected {
			user: T::AccountId, 
			id: Vec<u8>
		},

		///Event generated when user is charged to pay gasfee for ipfs snapshot.
		ChargeIpfsSnapshotGasPrice {
			signer: Option<T::AccountId>,
			user: T::AccountId, 
			amount: U256, 
			ipfs_hash: Vec<u8>, 
			token_type: TokenType, 
			transaction_id: Vec<u8>
		},

		///Event generated when Ocw fetch new estimated gas price for setIpfs() call in Ethereum network,
		UpdateNewEstimatedGasPrice {
			signer: Option<T::AccountId>, 
			amount: U256
		},
	}

	// Defines the Wallet when new user transaction will be accepted.
	// This storage entry defines when new transaction is going to be accepted.
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	#[pallet::storage]
	pub(super) type UserWallet<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		Vec<u8>,
		users::types::User,
		ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn order_book_nmap)]
	pub(super) type OrderBookNMap <T: Config> = StorageNMap<
		Key = (NMapKey<Blake2_128Concat, TradeType>,
		NMapKey<Blake2_128Concat, TokenType>,
		NMapKey<Twox64Concat, u64>),
		Value = OrderBookStruct<T::AccountId>,
		QueryKind = OptionQuery,
	>;

	// #[pallet::storage]
	// pub(super) type OrderIndexedNMap <T: Config> = StorageNMap<
	// 	Key = (NMapKey<Blake2_128Concat, TradeType>,
	// 	NMapKey<Blake2_128Concat, TokenType>),
	// 	Value = Indexed,
	// 	QueryKind = ValueQuery,
	// >;

	#[pallet::storage]
	pub(super) type OrderIndexedNMap <T: Config> = StorageNMap<
		Key = (NMapKey<Blake2_128Concat, TradeType>,
		NMapKey<Blake2_128Concat, TokenType>),
		Value = Vec<u64>,
		QueryKind = ValueQuery,
	>;

	#[pallet::storage]
	#[pallet::getter( fn distribute)]
	pub(super) type Distribution<T: Config> = StorageMap<_, 
	Blake2_128Concat, u64, 
	distribution::types::Distribution<T::Moment>, 
	ValueQuery>;

	/// Store of Total orders count till date when new order request is accepted.
	#[pallet::storage]
	#[pallet::getter( fn total_orders_till_date)]
	pub(super) type TotalOrdersCountTillDate<T: Config> = StorageValue<_, U256, ValueQuery>; 

	#[pallet::storage]
	#[pallet::getter( fn total_sells_journals)]
	pub(super) type TotalSellsJournal<T> = StorageValue<_, U256, ValueQuery>;
	
	#[pallet::storage]
	#[pallet::getter( fn running_status)]
	pub(super) type VtbSystemRunning<T: Config> = StorageValue<_, bool, ValueQuery>; 

	#[pallet::storage]
	#[pallet::getter( fn is_vtbc_start_rate)]
	pub(super) type VtbcStartRate<T: Config> = StorageValue<_, bool, ValueQuery>; 

	#[pallet::storage]
	#[pallet::getter( fn get_circulation_value)]
	pub(super) type Circulation<T> = StorageMap<_, Blake2_128Concat, TokenType, U256, ValueQuery>;


	// #[pallet::storage]
	// // #[pallet::getter( fn get_circulation_value)]
	// pub(super) type StopEnabled<T> = StorageValue<_, 
	// custom_types::StopData, ValueQuery>;
	
	#[pallet::storage]
	#[pallet::getter( fn get_globals)]
	pub(super) type Globals<T: Config> = StorageValue<_, custom_types::Globals, ValueQuery>;
	
	/// Store of current period going on
	#[pallet::storage]
	#[pallet::getter( fn get_current_period)]
	pub(super) type Period<T: Config> = StorageValue<_, u64, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter( fn get_txn_affected_vtbc_price)]
	pub(super) type TxnAffectedVtbcPrice<T: Config> = StorageValue<_, U256, ValueQuery>; 

	#[pallet::storage]
	#[pallet::getter( fn is_init)]
	pub(super) type InitDistribution<T: Config> = StorageValue<_, bool, ValueQuery>;
	
	#[pallet::storage]
	pub(super) type ClaimToken<T: Config> = StorageDoubleMap<_, 
		Blake2_128Concat, T::AccountId, 
		Blake2_128Concat, TokenType, 
		custom_types::ToClaimUserBalance, 
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter( fn blocked_account)]
	pub(super) type BlockedUserWallet<T: Config> =  StorageMap<_, 
		Blake2_128Concat, T::AccountId, 
		VecDeque<WithdrawRequest<T>>,
		OptionQuery, 
		GetDefault
	>;
	
	#[pallet::storage]
	#[pallet::getter( fn vtb_dex_transaction_fee)]
	pub(super) type VtbdexTransactionFee<T: Config> = StorageValue<_, custom_types::FeeCollectorAccount<T::AccountId>, OptionQuery, GetDefault>;

	#[pallet::storage]
	#[pallet::getter( fn updated_time_list)]
	pub(super) type UpdatedTimeList<T: Config> = StorageValue<_, custom_types::FutureEstimatedTimeList<T::Moment>, ValueQuery>;
	
	#[pallet::storage]
	#[pallet::getter( fn apr_estimate)]
	pub(super) type AprEstimate<T: Config> = StorageMap<_, Blake2_128Concat, u32, custom_types::AprEstimation, ValueQuery>;
	
	#[pallet::storage]
	#[pallet::getter( fn usd_apr_upto_last_hour_rate)]
	pub(super) type UsdVtbcH<T> =  StorageValue<_, U256, ValueQuery>; 

	#[pallet::storage]
	#[pallet::getter( fn ipfs_gas_price)]
	pub(super) type EstimatedGasPrice<T> = StorageValue<_, U256, ValueQuery>;
	
	#[pallet::type_value]
	pub fn DefaultIntegerAs0<T: Config>() -> u64 { 0 } 

	#[pallet::storage]
	pub(super) type WithdrawCountRecord <T: Config> = StorageNMap<
		Key = (NMapKey<Blake2_128Concat, TokenType>,
		NMapKey<Blake2_128Concat, T::BlockNumber>),
		Value = u64,
		QueryKind = ValueQuery,
		OnEmpty =  DefaultIntegerAs0<T>
	>;

	// Below list of storage is for storage migration

	// To migrate storage data to v2
	#[pallet::type_value]
	pub fn StorageVersionValue<T: Config>() -> migration::types::StorageVersion { migration::types::StorageVersion::V1_0_0 } 

	#[pallet::storage]
	pub type PalletStorageVersion<T: Config> = StorageMap<_, Blake2_128Concat, migration::types::MigrationType, migration::types::StorageVersion, ValueQuery, StorageVersionValue<T>>; 


	#[pallet::validate_unsigned]
	impl<T: Config> frame_support::unsigned::ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

		fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {

			match call {
				Call::check_claim_distribution  { address, period } => {
					ValidTransaction::with_tag_prefix("vtbdex/validate_unsigned/check_claim_distribution")
					.priority(period + 100)
					.and_provides(address)
					.and_provides(period)
					.longevity(5)
					.propagate(true)
					.build()
				},

				_ => InvalidTransaction::Call.into(),
			}
		}
	}

	//// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		Unknown,
		NoneValue,
		NumberIsTooBigForU256,
		NumberIsTooLowForU256,
		UserPeriodBalDoesNotExist,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		// Error returned when making signed transactions in off-chain worker
		NoLocalAcctForSigning,
		OffchainSignedTxError,
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
		LockAcquiredFailed,
		OnBoardFailed,
		ValueAlreadylinkedWithThisKey,
		PolkadotAddressNotLinked,
		InsufficientFunds,
		UserWalletIsLocked,
		AmountIsLessThanMinUsd25,
		EthUsdConversionFailed,
		UsdEthConversionFailed,
		EosUsdConversionFailed,
		UsdEosConversionFailed,
		EthVtbcConversionFailed,
		BuyVtbcFailed,

		// unwrap error failed
		UnwrapWithNoneFailed,
		DistributionPeriodIsStillOpen,
		OpenNewBuyOrderFailedDueToAmountLessThenMinUsd,
		OpenNewSellOrderFailedDueToAmountLessThenMinUsd,
		InvalidTokenType,
		SellVtbcFailed,
		InSameaccuralPeriod,
		CryptoamountIsInsufficientToBuyVtbc,
		AlreadyInitialized,
		InvalidSs58Address,
		UserDoesNotHaveLinkedCryptoAddress,
		UserDoesNotHaveLinkedEthAddress,
		InsufficientFundsInToken,
		InsufficientFundsToPayFees,
		CancelOrderFailed,
		ErrorInMatchinDistributableAssetsArm,
		InsufficientFundsToPayFee,
		MatchArms,
		OrderIdDoesNotExist,
		VtbDexSystemIsRunning,
		VtbdexSystemIsStopped,
		EthAddressNotLinkedWithPolkadotAccount,
		EosAddressNotLinkedWithPolkadotAccount,
		RequireOcwKeyOwner,
		VtbcUsdConversionFailed,
		ClaimOrBlockedRequestDoesNotExist,
		ClaimOnlyAllowedAfter7DaysOfInitiatedCompletesForSecurity,
		AmountNotMatchingWithRequest,
		HttpFetchingErrorTimeoutError,
		HttpFetchingErrorBadGateWay,
		HttpFetchingErrorOther,
		ResultNotAValidNumber,
		HourluCalculationRecentlySentByAnotherOcw,
		BothFromAndToCanNotBeSame,
		InSameGracePeriodForPowerup,
		WithdrawPendingAmountCanBeReturnedAfter24Hours,
		InSameGracePeriod,
		InChargeGasPrice,
		InvalidResponseCode,
		InsufficientFundToPayIpfsGasFee,
		ErrorInMatchingTokenType,
		CryptoAddressNotLinked,
		CallIsEmpty
	}
	
	impl <T: Config> Pallet<T> {
		pub fn moment_to_u32(input: T::Moment) -> Option<u32> {
			sp_std::convert::TryInto::<u32>::try_into(input).ok()
		}
		pub fn u32_to_moment(input: u64) -> Option<T::Moment> {
			sp_std::convert::TryInto::<T::Moment>::try_into(input).ok()
		}
	}
}

// use pallet::{Config, Pallet, Error, Event};
impl<T: SigningTypes> SignedPayload<T> for custom_types::VtbdexPayload<T::Public, T::BlockNumber> {
	fn public(&self) -> T::Public {
		self.public.clone()
	}
}
