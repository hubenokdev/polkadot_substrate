/** Module name: ocw_offchain
 *Description: It has code implementation with offchain worker to get and post http request and sign transaction to runtime
 **/
 use sp_runtime::{
    offchain as rt_offchain
 };
 use sp_std::{ str};
 use crate::*;

 impl<T: Config> Pallet<T> {
   
    pub fn sign_and_submit_transaction(call: Option<pallet::Call<T>>) -> Result<(), Error<T>> {
        let signer = Signer::<T, <T as pallet::Config>::AuthorityId>::any_account();
        if let Some(c) = call {
            let result = signer.send_signed_transaction(|_acct| c.clone());
            // Display error if the signed tx fails.
            if let Some((acc, res)) = result {
                if res.is_err() {
                    log::error!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
                    return Err(<Error<T>>::OffchainSignedTxError);
                }
                // Transaction is sent successfully
                log::info!("Sent success: {:?}", res);
			    // Transaction is sent successfully
			    Ok(())
            } else {
                log::info!("transaction sent but no result");
                Err(<Error<T>>::NoLocalAcctForSigning)
            }
        }
        else {
            log::info!("transaction sent but no result");
            Err(<Error<T>>::CallIsEmpty)
        }
    }

    /// Fetch from remote and deserialize the JSON to a struct
    pub(crate) fn fetch_n_parse_post_request<I, U>(req_url: &str, request_body: U)-> Result<serde_json::Value, Error<T>> 
    where 
    U: sp_std::clone::Clone + sp_std::default::Default,
    I: AsRef<[u8]>, U: IntoIterator<Item = I>
    {
        let resp_bytes = Self::fetch_from_remote_post_request(req_url, request_body).map_err(|err| {
            log::info!("fetch_from_remote_post_request error: {:?}", err);
            err
        })?;
     
        let resp_str = str::from_utf8(&resp_bytes).map_err(|_| <Error<T>>::ByteToStringConversionError)?;
        // Print out our fetched JSON string
        log::info!("{}", resp_str);
         
        let http_resp: serde_json::Value =
        serde_json::from_str(resp_str).map_err(|_| <Error<T>>::StringParsingError)?;
     
        Ok(http_resp)
    }
       
    /// This function uses the `offchain::http` API to query the remote api information,
    /// and returns the JSON response as vector of bytes.
    fn fetch_from_remote_post_request<I, U>(req_url: &str, request_body: U) -> Result<Vec<u8>, Error<T>> 
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
             .add_header("Authorization", global_constants::_BASIC_AUTHORIZATION_KEY)
             .body(request_body)
             .deadline(timeout) // Setting the timeout time
             .send() // Sending the request out by the host
             .map_err(|_| <Error<T>>::HttpFetchingError)?;
 
         // By default, the http request is async from the runtime perspective. So we are asking the
         //   runtime to wait here.
         // The returning value here is a `Result` of `Result`, so we are unwrapping it twice by two `?`
         //   ref: https://substrate.dev/rustdocs/v3.0.0/sp_runtime/offchain/http/struct.PendingRequest.html#method.try_wait
         let response = pending
             .try_wait(timeout)
             .map_err(|_| <Error<T>>::HttpFetchingError)?
             .map_err(|_| <Error<T>>::HttpFetchingError)?;
 
         log::info!("Response: {:?}", response);
         if response.code != 200 {
             log::info!("Unexpected http request status code: {}", response.code);
             if response.code == 502 {
                return Err(<Error<T>>::HttpFetchingErrorBadGateWay);
             }
             else if response.code == 504 || response.code == 503 {
                return Err(<Error<T>>::HttpFetchingErrorTimeoutError);
             }
             else {
                return Err(<Error<T>>::HttpFetchingErrorOther);
             }
         }
         // Next we fully read the response body and collect it to a vector of bytes.
         Ok(response.body().collect::<Vec<u8>>())
    }  
} 
