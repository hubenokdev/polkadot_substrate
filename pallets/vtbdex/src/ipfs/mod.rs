//! mod ipfs: This module is responsible to capture each runtime state and send entire data in json file to ipfs and submit 
//! the ipfs hash to eth-contract method setIpfs.
//! It capture all the essential state such as sellorders, buyorders, User balance, If any blocked uers balance, 
//! Other state such as usdrate of the token.
//! Sending data to ipfs will be regultaed by runtime, via specifying the time.
//! But usually the plan to take the snapshot once per day.
mod misc;
mod order;
mod user;
// mod migrated_data;

// use migrated_data::MigratedData;
use misc::MiscData;
use user::UserData;
use order::OrderData;
use crate::{
	Pallet, Error, Config, 
	StorageRetrievalError, StorageValueRef, MutateStorageError,
	Get, U256, global_constants, String, Vec, ToString,
};
use sp_std::vec;
use sp_runtime::SaturatedConversion;
use serde_json::{Value, Map};

impl<T: Config> Pallet<T> {
	
	/// fn check_ipfs_period: This is called by Offchain worker context.
	/// This function is responsible to check for the recent transaction for Ipfs, if the transaction is recently sent,
	/// so it will ignore or else it will acquire the lock and make the transaction
    pub(crate) fn check_ipfs_period(block_number: T::Moment) -> Result<(Vec<u8>, U256, Vec<u8>), Error<T>> {

        /// A friendlier name for the error that is going to be returned in case we are in the grace
		/// period.
		const RECENTLY_SENT: () = ();

		let val = StorageValueRef::persistent(b"ipfs::last_send::wallet::data1");
		let res = val.mutate(|last_send: Result<Option<T::Moment>, StorageRetrievalError>| {
			match last_send {
				// If we already have a value in storage and the block number is recent enough
				// we avoid sending another transaction at this time.
				Ok(Some(block)) if block_number < block + T::IpfsTimestamp::get() =>
					Err(RECENTLY_SENT),
				// In every other case we attempt to acquire the lock and send a transaction.
				_ => Ok(block_number),
			}
		});
		match res {
			// The value has been set correctly, which means we can safely send a transaction now.
			Ok(block_number) => {
			    
                Self::merge_all_ipfs_data_and_send("Ipfs Due to Daily Record", block_number)
			},
			// We are in the grace period, we should not send a transaction this time.
			Err(MutateStorageError::ValueFunctionFailed(RECENTLY_SENT)) => Err(<Error<T>>::InSameaccuralPeriod),
			Err(MutateStorageError::ConcurrentModification(_)) => Err(<Error<T>>::LockAcquiredFailed),
		}
    }

	/// fn merge_all_ipfs_data_and_send -> This task of this function is to create json object which will have all the essential
	/// runtime state for snapshot backup to ipfs
	fn merge_all_ipfs_data_and_send(reason: &str, block_number: T::Moment) -> Result<(Vec<u8>, U256, Vec<u8>), Error<T>> {

		let mut final_ipfs_data = Map::new();

		// Capture and put the data of order book( Buy/Sell)
		<OrderData<T>>::orders_data(&mut final_ipfs_data)?;
		//Capture and put the data of Users balances
		<UserData<T>>::wallet_data_ipfs_call(&mut final_ipfs_data)?;
		//Capture and put the data of blockedUser balance due to failed/pending withdraw
		<UserData<T>>::blocked_users_wallet(&mut final_ipfs_data)?;
		//Capture and put the data of migrated users balance
		// <MigratedData<T>>::migrated_users_ipfs_call(&mut final_ipfs_data)?;
		//Capture and put the data of migrated seller orders
		// <MigratedData<T>>::user_hodldex_selllist_data(&mut final_ipfs_data)?;
		//Capture and put the data of AprEstimated data for each year
		<MiscData<T>>::apr_estimate_data(&mut final_ipfs_data)?;
		//Capture and put the data for Globals runtime state 
		<MiscData<T>>::globals_data_to_map(&mut final_ipfs_data)?;
		//Capture and put the data for Circulation runtime state 
		<MiscData<T>>::circulation_data_to_map(&mut final_ipfs_data)?;
		//Capture and put the data for VTBT balances and total supply runtime state 
		<MiscData<T>>::vtbt_supply_data(&mut final_ipfs_data)?;
		//Capture and put the data for Other miscallenous state runtime state 
		<MiscData<T>>::other_state_to_map(&mut final_ipfs_data)?;
		//Capture and put the data for usdrate of the tokens runtime state 
		<MiscData<T>>::usd_rate_data_to_map(&mut final_ipfs_data)?;
		//Capture and put the data for Substrate feecollector runtime state 
		<MiscData<T>>::fee_collector_data_to_map(&mut final_ipfs_data)?;
		//Capture and put the data for Distribution runtime state from oth period to current
		<MiscData<T>>::distribution_data_to_map(&mut final_ipfs_data)?;

		// This is to recognize when the ipfs data is taken
		let mut others = Map::new();
		others.insert("reason".to_string(), Value::String(reason.to_string()));
		others.insert("Timestamp".to_string(), Value::from(block_number.saturated_into::<u64>()));
	
		final_ipfs_data.insert("Reason of Update".to_string(), Value::from(others));

		//Sent json data to ipfs via go api and sign a trnsaction in ethereum
		//It will return ipfsHash, gasFee, transactionHash
		Self::send_json_data_to_ipfs(final_ipfs_data)
		
	}

	/// fn send_json_data_to_ipfs: This function is responsible to serialize the json data in Vec<str> and than 
	/// send a http post request for setIpfs eth contract call.
	fn send_json_data_to_ipfs(json_data: Map<String, Value>) -> Result<(Vec<u8>, U256, Vec<u8>), Error<T>> {

		let req_obj_str = serde_json::to_string(&json_data).map_err(|e| {
            log::info!("UNable to parse: {:?}", e);
            <Error<T>>::NoneValue
        })?;
		let request_body = vec!(req_obj_str);
		match Self::fetch_n_parse_post_request(global_constants::_IPFS_REQUEST_API_ENDPOINT, request_body) {
			Ok(response) => {
				log::info!("fetch_from_remote info: {:?}", response);
				let response_body_code: u64 =  response["Code"].as_u64().ok_or(Error::<T>::NoneValue)?;
				if response_body_code != 200 {
					let _msg = response["Msg"].as_str().ok_or(Error::<T>::StringParsingError)?;
					Err(<Error<T>>::InvalidResponseCode)				
				}
				else {
					let ipfs_hash = response["hash"].as_str().ok_or(Error::<T>::StringParsingError)?;
					let trnx_price = U256::from(response["txnfee"].as_u64().ok_or(Error::<T>::NoneValue)?);
					log::info!("Ipfs hash: {:?}", ipfs_hash);
					let trnx_hash = response["txnHash"].as_str().ok_or(Error::<T>::StringParsingError)?;
					Ok((ipfs_hash.as_bytes().to_vec(), trnx_price, trnx_hash.as_bytes().to_vec()))
				}
			}
			Err(err) => {
				Err(err)
			}
		}	
	}
}