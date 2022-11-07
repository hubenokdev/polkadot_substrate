
//I think this should be part of the cryptos module and brough into the withdraw module
//same for the eth.rs file.
pub(crate) mod withdraw {
    use crate::{U256, Vec};
    use scale_info::prelude::string::String;
	use serde::__private::ToString;
	use sp_std::vec;
	
    pub(crate) fn crypto_req_data<'a>(address: &str, amt_value: U256) -> Option<(&'a str, Vec<String>)> {
        let precision = 1000000000000000000.00;
		let amt = amt_value.as_u128();
		let amt1 = amt as f64/precision;
		let amount_value = libm::floor(amt1 * 10000.0) / 10000.0;
		let symbol = "EOS".to_string();
		let amount_value_str = amount_value.to_string();
		let amt_eos = amount_value_str + " " + &symbol;
		log::info!("amount for withdraw {:?}", amt_eos);
		let body = serde_json::json!({
			"account": address,
			"quantity": amt_eos,
		});
		let body_str = serde_json::to_string(&body).unwrap();
		let payload = vec!(body_str);
		Some((crate::global_constants::_EOS_WITHDRAW_REQUEST_API_ENDPOINT, payload))
    }
}

use crate::{U256, UsdRate, Error, Config};
use crate::*;
use crate::cryptos::utility::{VtbcUsdRate}; 
///EosUsdRate: This have features of conversion  of Eos->Usd and Usd->Eos
pub struct EosUsdRate<T>(T);
impl<T: Config> EosUsdRate<T> {
  
    /// fn convert_usd_to_eos: This function take one argument of U256 type.
    /// It have the logic to convert provided Usd value to equivalent Eos.
    /// Here, T::CryptoPrecision::get() - represent 18decimals such as 1_000_000_000_000_000_000
    pub fn convert_usd_to_eos(amount_usd: U256) -> Result<U256, Error<T>>   {
        
        let rate = <UsdRate<T>>::get().eos;
        let res = match amount_usd.checked_mul(T::CryptoPrecision::get()).ok_or(Error::<T>::NumberIsTooBigForU256)?.checked_div(rate){
            Some(res) => res,
            _ => return Err(Error::<T>::UsdEosConversionFailed),
        };
        
        Ok(res)
    }

    /// fn convert_eos_to_usd: This function take one argument of U256 type.
    /// It have the logic to convert provided Eos value to equivalent Usd.
    /// Here, T::CryptoPrecision::get() - represent 18decimals such as 1_000_000_000_000_000_000
    pub fn convert_eos_to_usd(amount_eos: U256) -> Result<U256, Error<T>>   {
        
        let rate = <UsdRate<T>>::get().eos;
        let res = match amount_eos.checked_mul(rate).ok_or(Error::<T>::NumberIsTooBigForU256)?.checked_div(T::CryptoPrecision::get()){
            Some(res) => res,
            _ => return Err(Error::<T>::EosUsdConversionFailed),
        };

        Ok(res)
    }

    /// fn convert_eos_to_vtbc: This function take one argument of U256 type.
    /// It have the logic to convert provided Eos value to equivalent Vtbc.
    /// Here, T::CryptoPrecision::get() - represent 18decimals such as 1_000_000_000_000_000_000
    pub fn convert_eos_to_vtbc(amount_eos: U256) -> Result<U256, Error<T>> {

        let amt_usd = Self::convert_eos_to_usd(amount_eos)?;

        VtbcUsdRate::<T>::convert_usd_to_vtbc(amt_usd)
    }

    /// fn convert_vtbc_to_eos: This function take one argument of U256 type.
    /// It have the logic to convert provided Vtbc value to equivalent Eos.
    /// Here, T::CryptoPrecision::get() - represent 18decimals such as 1_000_000_000_000_000_000
    pub fn convert_vtbc_to_eos(amount_vtbc: U256) -> Result<U256, Error<T>> {

        let amt_usd = VtbcUsdRate::<T>::convert_vtbc_to_usd(amount_vtbc)?;
        
        Self::convert_usd_to_eos(amt_usd)
    }
}

// mod	accountpowerup {
    

// #[derive(Deserialize, serde::Serialize)]
// pub struct EosBuyRamRequestJson<'a> {
// 	pub receiver: &'a str,
// 	pub bytes: u64,
// }

// #[derive(Deserialize, Serialize)]
// pub struct EosPowerupRequestJson<'a> {
// 	pub receiver: &'a str,
// 	pub max_pay: &'a str,
// }
// 	impl<T: Config> Pallet<T> {
// 		pub fn buy_ram_in_eos_account(eos_user_acc: &str) -> Result<(), Error<T>> {
						
// 			let req_obj = custom_types::EosBuyRamRequestJson {
// 				receiver: eos_user_acc,
// 				bytes: 5024
// 			};

// 			let req_obj_str: &str = &serde_json::to_string(&req_obj).unwrap();
// 			let request_body = vec!(req_obj_str);

// 			let mut lock = StorageLock::<BlockAndTime<frame_system::Pallet<T>>>::with_block_and_time_deadline(
// 				b"vtbdex::buyram-eos-lock",
// 				constants::LOCK_BLOCK_EXPIRATION,
// 				rt_offchain::Duration::from_millis(constants::LOCK_TIMEOUT_EXPIRATION),
// 			);
			
// 			if let Ok(_guard) = lock.try_lock() {
// 				match Self::fetch_n_parse_post_request(global_constants::_EOS_BUY_RAM_IN_EOS_ACCOUNT_API, request_body) {
// 					Ok(response) => {
// 						log::info!("buy_ram response: {:?}", response);
// 					}
// 					Err(err) => {
// 						return Err(err);
// 					}
// 				}	
// 			};

// 			Ok(())
// 		}

// 		pub(super) fn do_powerup(time_stamp: T::Moment) -> DispatchResult {

// 			const RECENTLY_SENT: () = ();
		
// 			let val = StorageValueRef::persistent(b"powerup::last_send_timestamp");
// 			let res = val.mutate(|last_send: Result<Option<T::Moment>, StorageRetrievalError>| {
// 				match last_send {
// 					// If we already have a value in storage and the block number is recent enough
// 					// we avoid sending another transaction at this time.
// 					Ok(Some(last_time_stamp)) if time_stamp < last_time_stamp + T::PowerupDayInterval::get() =>
// 						Err(RECENTLY_SENT),
// 					// In everrust crate for logarithm of 10y other case we attempt to acquire the lock and send a transaction.
// 					_ => Ok(time_stamp),
// 				}
// 			});

// 			match res {
// 				// The value has been set correctly, which means we can safely send a transaction now.
// 				Ok(_block_number) => {
// 					let _ = Self::powerup_eos_account(global_constants::_EOS_SUBSTRATE_ACCOUNT); //resouces needed to sign powerup actions for the contract contract
// 					let _ = Self::powerup_eos_account(global_constants::_EOS_CONTRACT_ACCOUNT); //resouces needed to sign inline actions of contract
// 					Ok(())
// 				},
// 				// We are in the grace period, we should not send a transaction this time.
// 				Err(MutateStorageError::ValueFunctionFailed(RECENTLY_SENT)) => Err(frame_support::dispatch::DispatchError::from(<Error<T>>::InSameGracePeriodForPowerup)),
// 				// We wanted to send a transaction, but failed to write the block number (acquire a
// 				// lock). This indicates that another offchain worker that was running concurrently
// 				// most likely executed the same logic and succeeded at writing to storage.
// 				// Thus we don't really want to send the transaction, knowing that the other run
// 				// already did.
// 				Err(MutateStorageError::ConcurrentModification(_)) => Err(frame_support::dispatch::DispatchError::from(<Error<T>>::LockAcquiredFailed)),
// 			}
// 		} 

// 		fn powerup_eos_account(eos_user_acc: &str) -> Result<(), Error<T>> {

// 			let mut lock = StorageLock::<BlockAndTime<frame_system::Pallet<T>>>::with_block_and_time_deadline(
// 				b"vtbdex::withdraw-account-powereup-lock",
// 				10,
// 				rt_offchain::Duration::from_millis(6000),
// 			);
				
// 			let req_obj = custom_types::EosPowerupRequestJson {
// 				receiver: eos_user_acc,
// 				max_pay: global_constants::_EOS_EOSIO_MAX_PAYMENT
// 			};

// 			let req_obj_str: &str = &serde_json::to_string(&req_obj).unwrap();
// 			let request_body = vec!(req_obj_str);

// 			if let Ok(_guard) = lock.try_lock() {

// 				match Self::fetch_n_parse_post_request(global_constants::_EOS_POWERUP_ACCOUNT_API, request_body) {
// 					Ok(res) => {
// 						log::info!("Powerup eos account response: {:?}", res);
// 					}
// 					Err(err) => {
// 						return Err(err);
// 					}
// 				}
// 			}
// 			else {
// 				log::info!("Lock is acquired by another ocw worker");
// 			}
// 			Ok(())
// 		}
// 	}
// }