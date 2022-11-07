/** Module name: vtb_offchain
 *Description: It has code implementation with offchain worker to get and post http request
 *Method: fetch_n_parse_get_request, fetch_from_remote_get_request, fetch_n_parse_post_request, fetch_from_remote_post_request
 *Method: fetch_crypto_current_block_number - to get current block number from crypto network
 *Method: append_or_replace_current_crypto_block_number - It is to maintian last 10 current crypto_blocknumber record in offchain storage
 **/
 use sp_runtime::{
    offchain as rt_offchain,
    offchain::{
        storage::{StorageValueRef}
    },
};
use sp_std::{collections::vec_deque::VecDeque, prelude::*, str};
use crate::*;

impl<T: Config> Pallet<T> {

    /// Fetch from remote and deserialize the JSON to a struct
    pub fn fetch_n_parse_get_request(req_url: &str) -> Result<serde_json::Value, Error<T>> {

        let resp_bytes = Self::fetch_from_remote_get_request(req_url).map_err(|e| {
            log::info!("fetch_from_remote error: {:?}", e);
            <Error<T>>::HttpFetchingError
        })?;

        let resp_str = str::from_utf8(&resp_bytes).map_err(|_| <Error<T>>::ByteToStringConversionError)?;
        // Print out our fetched JSON string
        log::info!("{}", resp_str);

        let http_resp: serde_json::Value =
        serde_json::from_str(resp_str).map_err(|_| <Error<T>>::StringParsingError)?;

        Ok(http_resp)
    }

    /// This function uses the `offchain::http` API to query the remote github information,
    ///   and returns the JSON response as vector of bytes.
    pub fn fetch_from_remote_get_request(req_url: &str) -> Result<Vec<u8>, Error<T>> {
        // Initiate an external HTTP GET request. This is using high-level wrappers from `sp_runtime`.
        let request = rt_offchain::http::Request::get(req_url);
        
        // Keeping the offchain worker execution time reasonable, so limiting the call to be within 3s.
        let timeout = sp_io::offchain::timestamp()
            .add(rt_offchain::Duration::from_millis(constants::FETCH_TIMEOUT_PERIOD));

        let pending = request
            .deadline(timeout) // Setting the timeout time
            .send() // Sending the request out by the host
            .map_err(|_| <Error<T>>::HttpFetchingError)?;

        // By default, the http request is async from the runtime perspective. So we are asking the
        //   runtime to wait here.
        // The returning value here is a `Result` of `Result`, so we are unwrapping it twice by two `?`
        //   ref: https://substrate.dev/rustdocs/v3.0.0/sp_runtime/offchain/http/struct.PendingRequest.html#mcryptood.try_wait
        let response = pending
            .try_wait(timeout)
            .map_err(|_| <Error<T>>::HttpFetchingError)?
            .map_err(|_| <Error<T>>::HttpFetchingError)?;

        if response.code != 200 {
            log::info!("Unexpected http request status code: {}", response.code);
            return Err(<Error<T>>::HttpFetchingError);
        }
        // Next we fully read the response body and collect it to a vector of bytes.
        Ok(response.body().collect::<Vec<u8>>())
    }

    /// Fetch from remote and deserialize the JSON to a struct
    pub fn fetch_n_parse_post_request<I, U>(req_url: &str, request_body: U) -> Result<serde_json::Value, Error<T>> 
    where 
    U: sp_std::clone::Clone + sp_std::default::Default,
    I: AsRef<[u8]>, U: IntoIterator<Item = I>
    {
        let resp_bytes = Self::fetch_from_remote_post_request(req_url, request_body).map_err(|e| {
            log::info!("fetch_from_remote error: {:?}", e);
            <Error<T>>::HttpFetchingError
        })?;

        let resp_str = str::from_utf8(&resp_bytes).map_err(|_| <Error<T>>::ByteToStringConversionError)?;
        // Print out our fetched JSON string
        log::info!("{}", resp_str);
    
        let http_resp: serde_json::Value =
        serde_json::from_str(resp_str).map_err(|_| <Error<T>>::StringParsingError)?;

        Ok(http_resp)
    }

    /// This function uses the `offchain::http` API to query the remote github information,
    ///   and returns the JSON response as vector of bytes.
    pub fn fetch_from_remote_post_request<I, U>(req_url: &str, request_body: U) -> Result<Vec<u8>, Error<T>> 
    where 
    U: sp_std::clone::Clone + sp_std::default::Default,
    I: AsRef<[u8]>, U: IntoIterator<Item = I>
    {
        // Initiate an external HTTP GET request. This is using high-level wrappers from `sp_runtime`.
        let request = rt_offchain::http::Request::new(req_url);
        
        // Keeping the offchain worker execution time reasonable, so limiting the call to be within 3s.
        let timeout = sp_io::offchain::timestamp()
            .add(rt_offchain::Duration::from_millis(constants::FETCH_TIMEOUT_PERIOD));

        let pending = request
            .method(sp_runtime::offchain::http::Method::Post)
            .body(request_body)
            .deadline(timeout) // Setting the timeout time
            .send() // Sending the request out by the host
            .map_err(|_| <Error<T>>::HttpFetchingError)?;

        // By default, the http request is async from the runtime perspective. So we are asking the
        //   runtime to wait here.
        // The returning value here is a `Result` of `Result`, so we are unwrapping it twice by two `?`
        //   ref: https://substrate.dev/rustdocs/v3.0.0/sp_runtime/offchain/http/struct.PendingRequest.html#mcryptood.try_wait
        let response = pending
            .try_wait(timeout)
            .map_err(|_| <Error<T>>::HttpFetchingError)?
            .map_err(|_| <Error<T>>::HttpFetchingError)?;

        if response.code != 200 {
            log::info!("Unexpected http request status code: {}", response.code);
            return Err(<Error<T>>::HttpFetchingError);
        }
        // Next we fully read the response body and collect it to a vector of bytes.
        Ok(response.body().collect::<Vec<u8>>())
    }

    pub fn get_processed_range_obj(key: &[u8]) -> types::CryptoProcessedRange {
        let ocw_store_key_crypto_pros_range = StorageValueRef::persistent(key);
 
        let data = ocw_store_key_crypto_pros_range.get::<types::CryptoProcessedRange>();

        log::info!("data from fetcheoscurrent block ");
        match data {
         	Ok(Some(data)) => {
         		data
         	},
         	_ => {
                log::info!("Error: offchain data reteieved failed eos!");
                types::CryptoProcessedRange{ range_req: VecDeque::new()}
            }
         
        }
    }

    pub fn append_or_replace_processed_range(block_number: u64, key: &[u8]) {
        let ocw_store_key_crypto_pros_range  = StorageValueRef::persistent(key);
        
        let mut ocw_store_val_crypto_pros_range = match ocw_store_key_crypto_pros_range.get::<types::CryptoProcessedRange>() {
            Ok(Some(data)) => {
                data
            },
            _ => {
               log::info!("Error: offchain data reteieved failed!");
               types::CryptoProcessedRange{ range_req: VecDeque::new()}
           }
        };

        if ocw_store_val_crypto_pros_range.range_req.contains(&block_number) { 
            log::info!("This block already exist");
            return
        } 
        else if ocw_store_val_crypto_pros_range.range_req.len() == 200 {
            let _ = ocw_store_val_crypto_pros_range.range_req.pop_front();
        }
        if constants::BIRTHDAY_BLOCK_NUMBER != 0 && ocw_store_val_crypto_pros_range.range_req.is_empty() {
            ocw_store_val_crypto_pros_range.range_req.push_back(constants::BIRTHDAY_BLOCK_NUMBER);
        }
        else {
            ocw_store_val_crypto_pros_range.range_req.push_back(block_number);
        }
        ocw_store_key_crypto_pros_range.set(&ocw_store_val_crypto_pros_range);	

     }
     
     pub fn get_last_index_data_from_processed_range_record(key: &[u8]) -> Result<u64, Error<T>> {
        let ocw_store_key_crypto_pros_range  = StorageValueRef::persistent(key);

        let ocw_store_val_crypto_pros_range = match ocw_store_key_crypto_pros_range.get::<types::CryptoProcessedRange>() {
            Ok(Some(data)) => {
                data
            },
            _ => {
               log::info!("Error: offchain data reteieved failed!");
               types::CryptoProcessedRange{ range_req: VecDeque::new()}
           }
        };

        let length = ocw_store_val_crypto_pros_range.range_req.len();
     
        if length == 0 {
            return Err(<Error<T>>::OcwStoreCurrentBlockFetching0thPositionError)
        }
 
        let block: u64 = ocw_store_val_crypto_pros_range.range_req[length-1];
         
        Ok(block)
    }

    pub fn append_or_replace_current_crypto_block_number(block_number: u64, key: &[u8]) {
       let ocw_store_key_crypto_current_blocks = StorageValueRef::persistent(key);
  
       let data = ocw_store_key_crypto_current_blocks.get::<types::CryptoCurrentBlockRecord>();

       let mut ocw_store_val_crypto_current_blocks = match data {
            Ok(Some(data)) => {
                data
            },
            _ => {
                log::info!("Error: offchain data reteieved failed!");
                types::CryptoCurrentBlockRecord{ blocks: VecDeque::new()}
            }
        
       } ;

       if ocw_store_val_crypto_current_blocks.blocks.contains(&block_number) { 
           log::info!("This block already exist");
           return
       } 

       else if ocw_store_val_crypto_current_blocks.blocks.len() == constants::NUM_VEC_LEN {
           let _ = ocw_store_val_crypto_current_blocks.blocks.pop_front();
       }
          
       ocw_store_val_crypto_current_blocks.blocks.push_back(block_number);
       ocw_store_key_crypto_current_blocks.set(&ocw_store_val_crypto_current_blocks);
     }
} 