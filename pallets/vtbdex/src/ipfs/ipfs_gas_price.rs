// pub(super) fn fetch_estimated_gas_price(block_number: T::BlockNumber) ->  Result<(), Error<T>> {
	// 	let data = r#"{
	// 		"ipfsHash": "QmeQpbPAwaxeTFuRhERqSBXEmfJBGnbvQbktf7ZAaWgfmN"
	// 	  }"#;

	// 	const RECENTLY_SENT: () = ();

	// 	let val = StorageValueRef::persistent(b"ipfs::last_fetch_estimated_price");
	// 	let res = val.mutate(|last_send: Result<Option<T::BlockNumber>, StorageRetrievalError>| {
	// 		match last_send {
	// 			Ok(Some(block)) if block_number < block + 20_u32.into() =>
	// 				Err(RECENTLY_SENT),
	// 			// In every other case we attempt to acquire the lock and send a transaction.
	// 			_ => Ok(block_number),
	// 		}
	// 	});

	// 	match res {
	// 		// The value has been set correctly, which means we can safely send a transaction now.
	// 		Ok(_block_number) => {
	// 			let request_body = vec!(data);
	// 			match Self::fetch_n_parse_post_request(global_constants::_IPFS_GAS_ESTIMATION_API_ENDPOINT, request_body) {
	// 				Ok(response) => {
	// 					log::info!("fetch_from_remote info: {:?}", response);
	// 					let response_body_code: u64 =  response["Code"].as_u64().unwrap();
	// 					if response_body_code == 200 {
	// 						let trnx_price = response["txnfee"].as_u64().unwrap();
	// 						log::info!("trnx_price: {:?}", trnx_price);	
	// 						Self::set_new_estimated_gas_price(trnx_price)		
	// 					}
	// 					else {
	// 						log::error!("Error in response code for _IPFS_GAS_ESTIMATION_API_ENDPOINT: {}", response_body_code);
	// 						Err(<Error<T>>::InvalidResponseCode)	
	// 					}
	// 				}
	// 				Err(err) => {
	// 					Err(err)
	// 				}
	// 			}	
	// 		},
	// 		// We are in the grace period, we should not send a transaction this time.
	// 		Err(MutateStorageError::ValueFunctionFailed(RECENTLY_SENT)) => Err(<Error<T>>::InSameaccuralPeriod),
	// 		Err(MutateStorageError::ConcurrentModification(_)) => Err(<Error<T>>::LockAcquiredFailed),
	// 	}
	// }

	// pub(super) fn send_withdraw_to_ipfs(token_type: &TokenType, block_number: T::BlockNumber, address: &T::AccountId) ->  Result<(), Error<T>> {
	// 	/// A friendlier name for the error that is going to be returned in case we are in the grace
	// 	/// period.
	// 	const RECENTLY_SENT: () = ();

	// 	let val = StorageValueRef::persistent(b"vtbdex::last_send_withdraw_ipfs");

	// 	let res = val.mutate(|last_send: Result<Option<T::BlockNumber>, StorageRetrievalError>| {
	// 		match last_send {
	// 			// If we already have a value in storage and the block number is recent enough
	// 			// we avoid sending another transaction at this time.
	// 			Ok(Some(block)) if block_number == block =>
	// 				Err(RECENTLY_SENT),
	// 			// In every other case we attempt to acquire the lock and send a transaction.
	// 			_ => Ok(block_number),
	// 		}
	// 	});

	// 	match res {
	// 		// The value has been set correctly, which means we can safely send a transaction now.
	// 		Ok(_block_number) => {
	// 			let time_stamp = <pallet_timestamp::Pallet<T>>::now();
	// 			let res = Self::merge_all_ipfs_data_and_send("Ipfs Due to Withdraw", time_stamp);
    //             match res {
	// 				Ok(obj) => {
	// 					match token_type {
	// 						TokenType::Eth => {
	// 							let eth_gas_fee = obj.1;
	// 							Self::charge_gas_trnx_price(address, eth_gas_fee, obj.0, token_type, obj.2)
	// 						},
	// 						TokenType::Eos => {
	// 							let eth_gas_fee = obj.1;
	// 							let eos_eq_fee = Self::convert_eth_to_eos(eth_gas_fee);
	// 							Self::charge_gas_trnx_price(address, eos_eq_fee, obj.0, token_type, obj.2)
	// 						},
	// 						_ => Err(<Error<T>>::ErrorInMatchingTokenType),
	// 					}
						
	// 				},
	// 				_ => Err(<Error<T>>::InChargeGasPrice),
	// 			}
	// 		},
	// 		// We are in the grace period, we should not send a transaction this time.
	// 		Err(MutateStorageError::ValueFunctionFailed(RECENTLY_SENT)) => Err(<Error<T>>::InSameGracePeriod),
	// 		Err(MutateStorageError::ConcurrentModification(_)) => Err(<Error<T>>::LockAcquiredFailed),
	// 	}
	// }

	// fn set_new_estimated_gas_price(eth_trnx_gas_price: u64) -> Result<(), Error<T>> {

	// 	log::info!(" params: {:?}", &eth_trnx_gas_price);
    //     let eth_trnx_gas_price_u256 = U256::from(eth_trnx_gas_price);
	// 	let signer = Signer::<T, <T as pallet::Config>::AuthorityId>::any_account();

	// 	let result = signer.send_signed_transaction(|_acct|
	// 		// This is the on-chain function
	// 		Call::submit_new_estimated_price(eth_trnx_gas_price_u256));
	// 	// Display error if the signed tx fails.
	// 	if let Some((acc, res)) = result {
	// 		if res.is_err() {
	// 			log::info!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
	// 			return Err(<Error<T>>::OffchainSignedTxError);
	// 		}
	// 		log::info!("Sent success: {:?}", res);
	// 		// Transaction is sent successfully
	// 		return Ok(());
	// 	}
	// 	log::info!("No local account available");
	// 	Err(<Error<T>>::NoLocalAcctForSigning)
	// }	

	// fn charge_gas_trnx_price(address: &T::AccountId, eth_trnx_gas_price: U256, ipfs_hash: Vec<u8>, token_type: &TokenType, txn_hash: Vec<u8>) -> Result<(), Error<T>> {

	// 	log::info!(" params: {:?}", &eth_trnx_gas_price);

	// 	let signer = Signer::<T, <T as pallet::Config>::AuthorityId>::any_account();

	// 	let result = signer.send_signed_transaction(|_acct|
	// 		// This is the on-chain function
	// 		Call::charge_set_ipfs_trnx_price(address.clone(), eth_trnx_gas_price, ipfs_hash.clone(), *token_type, txn_hash.clone()));
	// 	// Display error if the signed tx fails.
	// 	if let Some((acc, res)) = result {
	// 		if res.is_err() {
	// 			log::info!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
	// 			return Err(<Error<T>>::OffchainSignedTxError);
	// 		}
	// 		log::info!("Sent success: {:?}", res);
	// 		// Transaction is sent successfully
	// 		return Ok(());
	// 	}
	// 	log::info!("No local account available");
	// 	Err(<Error<T>>::NoLocalAcctForSigning)
	// }	