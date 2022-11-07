/** Module name: eos_block
 *  Description: It has code implementation to read each block of eth & perform as per the logs type. 
 *  Method: fetch_eos_block_info - To get log of last process blocks with filter as contract address.
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
use sp_std::{ prelude::*, str};
use serde::__private::ToString;
use scale_info::prelude::string::String;
use crate::*;
 impl<T: Config> Pallet<T> {
 
    /// fn fetch_eos_block_info(eos_curr_block_number: u64) -> Result<(), Error<T>>
    ///This function send the Http request to fetch block info in Ethereum network.
    pub fn fetch_eos_block_info(eos_curr_block_number: u64) -> Result<(), Error<T>> {

		let mut lock = StorageLock::<BlockAndTime<frame_system::Pallet<T>>>::with_block_and_time_deadline(
			b"eos-vtb-contract::eos-block-processing-lock",
			constants::LOCK_BLOCK_EXPIRATION,
			rt_offchain::Duration::from_millis(constants::LOCK_TIMEOUT_EXPIRATION),
		);

		let mut from_block: u64 = <pallet_cross_chain::Pallet<T>>::get_last_index_data_from_processed_range_record(constants::storage_keys::PROCESS_RANGE_STORAGE_KEY).map_err(|e| {
            log::info!("UNable to parse: {:?}", e);
            <Error<T>>::InvalidValueEos
        })?;
		let mut to_block = from_block;
		log::info!("To_block: {:?}", to_block);
		loop {
			to_block = from_block + 100;
			if to_block > eos_curr_block_number {
				break;
			}
			let body = serde_json::json!({
                "fromBlock": &from_block.to_string(),
                "toBlock": &to_block.to_string()
            });
            let body_str = serde_json::to_string(&body).map_err(|e| {
				log::info!("UNable to parse: {:?}", e);
				<Error<T>>::InvalidValueEos
			})?;
			let request_body = vec!(body_str);
			   if let Ok(_guard) = lock.try_lock() {
				match <pallet_cross_chain::Pallet<T>>::fetch_n_parse_post_request(global_constants::_EOS_BLOCK_INFO_API_ENDPOINT, request_body) {

					Ok(block_info) => {	
						log::info!("block info eos {:?}",block_info.clone());
						match &block_info["Result"] {
							serde_json::Value::Object(block) => {
								let results = block["searchTransactionsForward"].as_object().ok_or(Error::<T>::InvalidValueEos)?;
								let result_arr = results["results"].clone();
								match result_arr {
									serde_json::Value::Array(ref traces)  => {
										let _res = Self::process_each_traces(traces, from_block);
									},
									serde_json::Value::Null => log::info!("Traces is Null" ),
									_ => {
										log::info!("Error in match arms of, traces should be null/Array");
									}
								}
								<pallet_cross_chain::Pallet<T>>::append_or_replace_processed_range(to_block, constants::storage_keys::PROCESS_RANGE_STORAGE_KEY);
							}
							serde_json::Value::Null => log::info!("Traces is Null" ),
							_ => {
								log::info!("Error in match arms of, traces should be null/Array");
							}
						}
					}
					Err(err) => {
						log::info!("Error in eos-vtb-contract pallet: {:?}", err);
						return Err(Error::<T>::HttpFetchingError);
					}
				}
			}	
			from_block = to_block;
		}	
		Ok(())
	}

	/// fn process_each_traces(log_info: &Vec<serde_json::Value>, from_block: u64) 
    ///This function iterate over all Event from the list one by one and execute the
	///matched actions.
	pub fn process_each_traces(log_info: &[serde_json::Value], from_block: u64) -> Result<(), Error<T>>{
		log::info!("from_block: {:?}", from_block);
		// for index in 0..log_info.len() {
		for item in log_info {
			let trace = item["trace"].as_object().ok_or(Error::<T>::InvalidValueEos)?;
			let block = trace["block"].as_object().ok_or(Error::<T>::InvalidValueEos)?;
			log::info!("block eos {:?}", &block);
			let _each_block_number: u64 = block["num"].as_u64().ok_or(Error::<T>::InvalidValueEos)?;
			match Self::vtb_contract_process(trace) {
				Ok(_status) => {
					log::info!("Transaction suceed!");
				},
				Err(err) => {
					log::info!("Error: {:?}", err);
				}	
			};	
		}
		
		Ok(())
	}

	/// fn vtb_contract_process(each_log: &serde_json::Map<String, serde_json::Value>) -> Result<bool, Error<T>>  
    ///This function matches the Event/Topic hash.
    ///Based on the matched event_name, it execute the further operation.
    ///Match condition is to match the event_name for Onboarding/Deposit/Withdraw.
    pub fn vtb_contract_process(each_log: &serde_json::Map<String, serde_json::Value>) -> Result<bool, Error<T>> {
		log::info!("!vtb_contract_process eos==================================");

		let transactions = each_log["matchingActions"].as_array().ok_or(Error::<T>::InvalidValueEos)?;
		let transacion_id = each_log["id"].as_str().ok_or(Error::<T>::InvalidValueEos)?;
		let transaction_hash_lc_1 = transacion_id.to_lowercase();
		let transaction_hash_lc = transaction_hash_lc_1.as_bytes().to_vec();

		// for index in 0..transactions.len() {
		for item in transactions {
			// let action_name_array = transactions[index].clone();
			let action_params =  item["json"].as_object().ok_or(Error::<T>::InvalidValueEos)?;
			let action_name: &str = item["name"].as_str().ok_or(Error::<T>::InvalidValueEos)?;
	
			log::info!("actions params transactions {:?}",action_name);
			match action_name {
				constants::EOS_VTB_ACTION_ONBOARD_USER => {
					log::info!("Listen event for user Onboard");
					let _ = Self::vtb_onboard_user(action_params, &transaction_hash_lc);

					//To Buy RAM for contract account
				   // let _ = <pallet_vtbdex::Pallet<T>>::buy_ram_in_eos_account(global_constants::_EOS_CONTRACT_ACCOUNT); //Ram needed to store sate in contract
				},
				constants::EOS_VTB_ACTION_EOSIO_TRANSFER => {
					log::info!("Listen event for EOS Deposits");
					let _ = Self::vtb_eos_deposits(action_params, &transaction_hash_lc);
				},
				constants::EOS_VTB_ACTION_WITHDRAW_EOS => {
					log::info!("Listen event for Withdraw Eos");
					let _ = Self::vtb_eos_withdrawn(action_params, &transaction_hash_lc);
				},
				&_ => {
					log::info!("Error in match arms");
				}
			}
		}
		Ok(true)
	}

    /// fn fetch_eos_current_block_number() -> Result<u64, Error<T>>
	///Fetch current block number from Eos mainnet
	///And store the block-number in ocw local storage.
    pub fn fetch_eos_current_block_number() -> Result<u64, Error<T>> {
     
		let ocw_store_val_eos_pros_range = <pallet_cross_chain::Pallet<T>>::get_processed_range_obj(b"eos-vtb-contract::eos-processed-block-range-record");
         match <pallet_cross_chain::Pallet<T>>::fetch_n_parse_get_request(global_constants::_EOS_CURRENT_BLOCK_NUMBER_API_ENDPOINT) {
             Ok(block_info) => {
                 if block_info["LastIrreversibleBlockNum"].is_number() {
                    let block_number =  block_info["LastIrreversibleBlockNum"].as_u64().ok_or(Error::<T>::InvalidValueEos)?;
                    log::info!("current block number: {:?}", &block_number);
                    <pallet_cross_chain::Pallet<T>>::append_or_replace_current_crypto_block_number(block_number, constants::storage_keys::CURRENT_BLOCK_RANGE_STORAGE_KEY);

                    if ocw_store_val_eos_pros_range.range_req.is_empty() {
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