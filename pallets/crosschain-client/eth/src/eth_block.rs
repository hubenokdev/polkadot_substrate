/** Module name: eth_block
 *  Description: It has code implementation to read each block of eth & perform as per the logs type. 
 *  Method: fetch_eth_block_info - To get log of last process blocks with filter as contract address.
 *  Method: process_each_log - It parse and read each traces/logs present in the block, as log parameter is of array type, so it will process each log one via one in loop.
 *  Method: vtb_contract_process - It match the logs action name, based on the match found it does the relative work such as onboarding, deposit, withdraw.
 *  Method: append_or_replace_processed_range - To keep record of the eth blocknumber in which the logs related to vtb_contract is present. Keeps record in ocw storage.
 *  Method: get_last_index_data_from_processed_range_record - Helper function to get the index of last processed eth_blocknumber.
 **/
 use sp_runtime::{
     offchain as rt_offchain,
     offchain::{
        storage_lock::{BlockAndTime, StorageLock},
     },
 };
 use sp_std::{prelude::*, str};
 use scale_info::prelude::string::String;
 use crate::*;

 impl<T: Config> Pallet<T> {
 
    /// fn fetch_eth_block_info(eth_curr_block_number: u64) -> Result<(), Error<T>> 
    ///This function send the Http request to fetch block info in Ethereum network.
    pub fn fetch_eth_block_info(eth_curr_block_number: u64) -> Result<(), Error<T>> {

        let mut lock = StorageLock::<BlockAndTime<frame_system::Pallet<T>>>::with_block_and_time_deadline(
            b"eth-vtb-contract::eth-block-processing-lock",
            constants::LOCK_BLOCK_EXPIRATION,
            rt_offchain::Duration::from_millis(constants::LOCK_TIMEOUT_EXPIRATION),
        );
        
        let mut from_block: u64 = <pallet_cross_chain::Pallet<T>>::get_last_index_data_from_processed_range_record(constants::storage_keys::PROCESS_RANGE_STORAGE_KEY)
        .map_err(|e| {
            log::info!("UNable to parse: {:?}", e);
            <Error<T>>::InvalidValueEth
        })?;
        let mut to_block: u64 = from_block;
        log::info!("To block: {:?}", to_block);
        loop {
            to_block = from_block + 1;
            if to_block > (eth_curr_block_number - 10) {
                 break;
            }

            log::info!("Read blocks, to-block: {}====from-block:{}=====eth_current_block: {}====", to_block, from_block, eth_curr_block_number);
            let body = serde_json::json!({
                "address": [global_constants::_ETHEREUM_CONTRACT_ADDRESS],
                "fromBlock": from_block,
                "toBlock": to_block
            });
            let body_str = serde_json::to_string(&body).map_err(|e| {
				log::info!("UNable to parse: {:?}", e);
				<Error<T>>::InvalidValueEth
			})?;	
            let request_body = vec!(&body_str);
            if let Ok(_guard) = lock.try_lock() {
                match <pallet_cross_chain::Pallet<T>>::fetch_n_parse_post_request(global_constants::_ETH_BLOCK_INFO_RANGE_API_ENDPOINT, request_body) {
                    Ok(log_info) => {
                        let _res = Self::process_each_log(log_info.clone(), from_block);
                        <pallet_cross_chain::Pallet<T>>::append_or_replace_processed_range(to_block, constants::storage_keys::PROCESS_RANGE_STORAGE_KEY);
                        //let vtbt_params = log_info["vtbt_params"].as_array().ok_or(Error::<T>::InvalidValueEth)?;

                        // if vtbt_params.len() > 0 {
                        //     <pallet_vtbdex::Pallet<T>>::process_each_vtbt_log(vtbt_params);
                        // }
                    }
                    Err(_err) => {
                        return Err(Error::<T>::HttpFetchingError);
                    }
                }
                from_block = to_block;
            }
            else {
               log::info!("Lock is acquired via another OCW instance");
               return Err(<Error<T>>::LockAcquiredFailed)
            }
        }
        Ok(())
    }
 
    /// fn process_each_log(log_info: serde_json::Value, from_block: u64) 
    ///This function iterate over all Event from the list one by one and execute the
	///matched actions.
    pub fn process_each_log(log_info: serde_json::Value, from_block: u64) -> Result<(), Error<T>> {
        log::info!("from_block: {:?}", from_block);
        for index in 0..log_info["params"].as_array().ok_or(Error::<T>::InvalidValueEth)?.len() {
            let each_log = log_info["params"][index].as_object().ok_or(Error::<T>::InvalidValueEth)?;
            match Self::vtb_contract_process(each_log) {
                Ok(_status) => {
                    log::info!("Transaction suceed!");
                    Ok(())
                },
                Err(err) => {
                    log::error!("process_each_log Error: {:?}", err);
                    Err(err)
                }	
            }?;	
        }

        Ok(())
    }

    /// fn vtb_contract_process(each_log: &serde_json::Map<String, serde_json::Value>) -> Result<(), Error<T>> 
    ///This function matches the Event/Topic hash.
    ///Based on the matched event_name, it execute the further operation.
    ///Match condition is to match the event_name for Onboarding/Deposit/Withdraw.
    pub fn vtb_contract_process(each_log: &serde_json::Map<String, serde_json::Value>) -> Result<(), Error<T>> {
         
        let event_name: &str =  each_log["topicHash"].as_str().ok_or(Error::<T>::InvalidValueEth)?;
        match event_name {
            constants::ETH_VTB_TOPIC_ONBOARD => {
                log::info!("Listen event for Onboard user");
                let _ = Self::vtb_onboard_user(each_log);		
            },
            constants::ETH_VTB_TOPIC_DEPOSITETH => {
                log::info!("Listen event for Deposit ETH");
                let _ = Self::vtb_eth_deposits(each_log);
            },
            constants::ETH_VTB_TOPIC_WITHDRAWETH => {
                log::info!("Listen event for Withdraw ETH");
                let _ = Self::vtb_eth_withdrawn(each_log);
            },
            &_ => {
                log::info!("Error in match arms");
            }
        }
 
        Ok(())
     }
 

    /// fn fetch_eth_current_block_number() -> Result<u64, Error<T>>
	///Fetch current block number from Eth network
	///And store the block-number in ocw local storage.
    pub fn fetch_eth_current_block_number() -> Result<u64, Error<T>> {

        let ocw_store_val_eth_pros_range = <pallet_cross_chain::Pallet<T>>::get_processed_range_obj(constants::storage_keys::PROCESS_RANGE_STORAGE_KEY);
        match <pallet_cross_chain::Pallet<T>>::fetch_n_parse_get_request(global_constants::_ETH_CURRENT_BLOCK_NUMBER_API_ENDPOINT) {
            Ok(block_info) => {
                if block_info["HeadBlockNum"].is_string() {
                    let result: &str =  block_info["HeadBlockNum"].as_str().ok_or(Error::<T>::InvalidValueEth)?;
                    let block_number = u64::from_str_radix(result.trim_start_matches("0x"),16).map_err(|e| {
                        log::info!("UNable to parse: {:?}", e);
                        <Error<T>>::InvalidValueEth
                    })?;	
                    log::info!("current block number: {:?}", &block_number);
                    <pallet_cross_chain::Pallet<T>>::append_or_replace_current_crypto_block_number(block_number, constants::storage_keys::CURRENT_BLOCK_RANGE_STORAGE_KEY);

                    if ocw_store_val_eth_pros_range.range_req.is_empty() {
                        <pallet_cross_chain::Pallet<T>>::append_or_replace_processed_range(block_number, constants::storage_keys::PROCESS_RANGE_STORAGE_KEY); 
                    }
                    Ok(block_number)
                }
                else {
                    Err(<Error<T>>::ResultNotAStringError)
                }
            }
             Err(_err) => {
                Err(Error::<T>::HttpFetchingError)
            }
        }
    }
}