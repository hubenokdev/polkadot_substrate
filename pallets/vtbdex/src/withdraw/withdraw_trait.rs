//! Module withdraw_trait: This module have features related to withdraw crypto.
//!
//! #Example
//! use withdraw::withdraw_trait::WithdrawCrypto;
//! let mut req_data = WithdrawCryptoReq::new(who, crypto_type, amount);
//! let res = WithdrawCrypto::<T>::initiate_withdraw(&mut req_data);
use frame_support::{
	dispatch::{ DispatchResult},  
	ensure
};
use frame_support::traits::Len;
use crate::{
	withdraw::types::WithdrawClaim,
	users::{WalletTrait, WalletUpdateReq},
	*,
};

/// Trait WithdrawCrypto<T: Config> 
/// This trait implement the method for withdraw crypto.
pub trait WithdrawCrypto<T: Config> {
	fn initiate_withdraw(&mut self) -> DispatchResult;
	fn crypto_withdraw_call(&self) -> DispatchResult;
	fn emit_withdraw_failed_due_to_time_out(&self, msg: Vec<u8>) -> DispatchResult;
	fn emit_withdraw_failed(&self, msg: Vec<u8>) -> DispatchResult;
	fn emit_withdraw_inprogress(&self, transaction_hash: Vec<u8>) -> DispatchResult;
	fn pay_trnx_fee_for_withdraw(&self, fee_amount: U256, fee_type: &str) -> DispatchResult;
}


impl<T: Config> WithdrawCrypto<T> for WithdrawCryptoReq<T::AccountId, T::BlockNumber> 
where 
{
	///fn initiate_withdraw(&mut self) -> DispatchResult.
	///This take object for struct WithdrawCryptoReq<T, U>.
	///This function is invoked by withdraw_initiate Extrinsic from runtime-context.
	///
	///When user initiate withdraw of Cryptos via signing extrinsic, this function is being called.
	///This function verify the user balance if the deposit-balance is sufficient for the user 
	///to pay fee as well as amount for withdraw
	///such as deposit-balance >= fee + withdraw_amount
	///If the above check passes, than 
	///it substract the (fee + withdraw_amount) from user account,
	///transfer the fee to fee-collector account and keep withdraw_amount in the BlockedUserWallet list.
	///Save the withdrawReq details in offchain-indexing data list, 
	///so that it will be checked and processed from the Ocw context.
	///Once done, emit the event to notify user as below
	///WithdrawInitiated(T::AccountId, Vec<u8>, U256, Vec<u8>)
	///For e.g
	///WithdrawInitiated(user-address, crypto-address, withdraw-amount, offchain-index-id).
	fn initiate_withdraw(&mut self) -> DispatchResult {
		let user = <UserWallet<T>>::get(&self.account.encode());
		let crypto_address =  user.crypto_addresses.get(&self.crypto_type).ok_or(Error::<T>::UserDoesNotHaveLinkedCryptoAddress)?;
		ensure!(crypto_address.crypto_address != None, Error::<T>::CryptoAddressNotLinked); 

		let blocknumber = frame_system::Pallet::<T>::block_number();
		let time_stamp = <pallet_timestamp::Pallet<T>>::now();
		let fee = <Pallet<T>>::check_and_calculate_fee(&self.crypto_type)?;	
		let total_amount = self.amount + fee;
		ensure!(crypto_address.deposit_balance >= total_amount, Error::<T>::InsufficientFunds);	
		ensure!(user.active , Error::<T>::UserWalletIsLocked);
		//let estimated_gas_fee = <EstimatedGasPrice<T>>::get();
		//ensure!(crypto_address.deposit_balance >= total_amount + estimated_gas_fee*5, Error::<T>::InsufficientFundToPayIpfsGasFee);	
		//let pdot_address = user.polkadot_address;
		<Pallet<T>>::sub_update_balance(&self.crypto_type, &self.account.encode(), total_amount, U256::from(0_u8))?;
		<Pallet<T>>::sub_circulation_token_balance(&self.crypto_type, self.amount)?;
		let account_key = self.account.encode();
		let res = WalletUpdateReq::new(Some(&self.crypto_type), &account_key, None, None);
		WalletTrait::<T, T::AccountId>::pay_trnx_fee_in_given_crypto(&res, &self.account, "Withdraw Crypto")?;
	
		let old_len = BlockedUserWallet::<T>::get(&self.account).len();
		let mut withdraw_claim_data = WithdrawClaim::new(self, old_len, fee, time_stamp, blocknumber);
		let bytes: &[u8] = &withdraw_claim_data.encode();
		let order_keccak_hash = sp_io::hashing::keccak_256(bytes);
		let id: Vec<u8> = order_keccak_hash.to_vec();
		withdraw_claim_data.id = id.clone();

		if <BlockedUserWallet<T>>::contains_key(&self.account) {
			BlockedUserWallet::<T>::mutate(&self.account, |list_opt| -> DispatchResult {
				let list = list_opt.as_mut().ok_or(Error::<T>::NoneValue)?;
				list.push_back(withdraw_claim_data);

				Ok(())
			})?;
		}
		else {
			let mut data = VecDeque::new();
			data.push_back(withdraw_claim_data);
			<BlockedUserWallet<T>>::insert(&self.account, data);
		}
		let new_counter = WithdrawCountRecord::<T>::get((self.crypto_type, blocknumber)) + 1;
		WithdrawCountRecord::<T>::mutate((self.crypto_type, blocknumber), |counter| {
			*counter = new_counter;
		});
		let mut index_key =  b"withdraw_initiate_".to_vec();
		index_key.extend(self.crypto_type.to_string().as_bytes());
		let key = <Pallet<T>>::derive_index_key(blocknumber, &index_key, new_counter);
		self.crypto_address = crypto_address.crypto_address.clone().ok_or(Error::<T>::NoneValue)?;
		self.id = id.clone();
		self.block_number = Some(blocknumber);
		//let data = custom_types::WithdrawIndexingData(index_key, crypto_address.crypto_addresses.clone(), self.amount, pdot_address, id);
		//offchain_index::set(&key, &data.encode());
		offchain_index::set(&key, &self.encode());

		<Pallet<T>>::deposit_event(Event::WithdrawInitiated { 
			user: self.account.clone(), 
			crypto_address: self.crypto_address.clone(), 
			amount: self.amount, 
			id 
		});
		
		Ok(())
	}
	
	///fn crypto_withdraw_call(&self) -> DispatchResult.
	///This take object for struct WithdrawCryptoReq<T, U>.
	///This function is invoked by check_for_withdraw_indexed_crypto_data from offchain-context.
	///
	///Make a http call to sign a transaction via go api, which create, sign and send the transaction 
	///on crypto-network.
	///
	///Filter the response, 
	///if the response gives the transaction hash, sign extrinsic Withdraw_in_progress
	///else check the reason for failure.
	///and sign extrinsic withdraw_failed.
	/// 
	///Error
	///If the http call does not give the response, than search for the reason of failure.
	///Based on stattus code, it can be BadGateWay or TimeoutError.
	///If it is of BadGateWay, than sign withdraw_failed, 
	///else if it due to TimeoutError, than sign withdraw_failed_due_to_time_out.
	fn crypto_withdraw_call(&self) -> DispatchResult {
		log::info!("crypto_address for withdraw {:?}", self.crypto_address);
		log::info!("amount for withdraw {:?}", self.amount);
		let crypto_address_str = str::from_utf8(&self.crypto_address).unwrap_or("error");
		let req_payload = self.crypto_type.withdraw_params(crypto_address_str, self.amount).ok_or(Error::<T>::NoneValue)?;
        let mut lock = StorageLock::<BlockAndTime<frame_system::Pallet<T>>>::with_block_and_time_deadline(
            b"vtbdex::withdraw-crypto-lock",
            constants::LOCK_BLOCK_EXPIRATION,
            rt_offchain::Duration::from_millis(constants::LOCK_TIMEOUT_EXPIRATION),
        );
		
		if let Ok(_guard) = lock.try_lock() {
			match <Pallet<T>>::fetch_n_parse_post_request(req_payload.0, req_payload.1.to_vec()) {
				Ok(response) => {
					log::info!("fetch_from_remote info: {:?}", response);
					let response_body_code: u64 =  response["Code"].as_u64().ok_or(Error::<T>::NoneValue)?;
					if response_body_code != 200 {
						let msg = response["Msg"].as_str().ok_or(Error::<T>::NoneValue)?;
						let txn_fee = response["txnfee"].as_u64().unwrap_or(0);
						if txn_fee > 0 {
							let final_msg = msg.to_owned() + ", gas fee: " + &txn_fee.to_string() + " wei";
							let _ = WithdrawCrypto::<T>::emit_withdraw_failed(self, final_msg.as_bytes().to_vec());
						}
						else {
							let _ = WithdrawCrypto::<T>::emit_withdraw_failed(self, msg.as_bytes().to_vec());
						}
						
					}
					else {
						let transaction_hash = response["txnHash"].as_str().ok_or(Error::<T>::NoneValue)?; // eos api txn_hash
						let transaction_hash_lc_1 = transaction_hash.to_lowercase();
						let transaction_hash_lc = transaction_hash_lc_1.as_bytes().to_vec();
						let _txn_fee = response["txnfee"].as_u64().ok_or(Error::<T>::NoneValue)?;
						WithdrawCrypto::<T>::emit_withdraw_inprogress(self, transaction_hash_lc)?;
						//Self::send_withdraw_to_ipfs(&crypto_type, block_number, &account_id)?;
					}
					
				}
				Err(err) => {
					match err {
						<Error<T>>::HttpFetchingErrorBadGateWay => {
							WithdrawCrypto::<T>::emit_withdraw_failed(self, "Withdraw service unavailable".as_bytes().to_vec())?;
							return Err(frame_support::dispatch::DispatchError::from(err))
						},
						<Error<T>>::HttpFetchingErrorTimeoutError => {
							WithdrawCrypto::<T>::emit_withdraw_failed_due_to_time_out(self, "Your balance will be updated within 7 working days".as_bytes().to_vec())?;
							return Err(frame_support::dispatch::DispatchError::from(err))
						},
						_ => {
							//Case where Api gives timeout error, may or may not be case for transaction signed
							return Err(frame_support::dispatch::DispatchError::from(err))
						}
					}
				}
			}	
		};

		Ok(())

	}

	///fn emit_withdraw_failed(&self, msg: Vec<u8>) -> DispatchResult;
	///It take object of self & reason of failure.
	///self is object of struct WithdrawCryptoReq<T, U>.
	///This makes a sign transaction from the ocw-context to notify user that withdraw_crypto is failed.
	fn emit_withdraw_failed(&self, msg: Vec<u8>) -> DispatchResult {
		let call = Some(Call::withdraw_failed{ 
			pdot_address: self.account.clone(), 
			cryto_address: self.crypto_address.clone(), 
			msg: msg.clone(), 
			amount: self.amount, 
			id: self.id.clone() 
		});

        Ok(Pallet::<T>::sign_and_submit_transaction(call)?)
	}

	///fn emit_withdraw_failed_due_to_time_out(&self, msg: Vec<u8>) -> DispatchResult;
	///It take object of self & reason of failure.
	///self is object of struct WithdrawCryptoReq<T, U>.
	///This makes a sign transaction from the ocw-context to notify user that withdraw_crypto is failed due to timeout error of http call.
	fn emit_withdraw_failed_due_to_time_out(&self, msg: Vec<u8>) -> DispatchResult {
		let call = Some(Call::withdraw_failed_due_to_time_out { 
			pdot_address: self.account.clone(), 
			cryto_address: self.crypto_address.clone(), 
			msg: msg.clone(), 
			id: self.id.clone() 
		});

        Ok(Pallet::<T>::sign_and_submit_transaction(call)?)
	}


	///fn emit_withdraw_inprogress(&self, transaction_hash: Vec<u8>) -> DispatchResult;
	///It take object of self & transaction_hash.
	///self is object of struct WithdrawCryptoReq<T, U>.
	///This makes a sign transaction from the ocw-context to notify user that withdraw_crypto is in progress
	///And also provide transaction-hash of the crypto_network.
	fn emit_withdraw_inprogress(&self, transaction_hash: Vec<u8>) -> DispatchResult {
		let call = Some(Call::withdraw_inprogress{ 
			pdot_address: self.account.clone(), 
			crypto_address: self.crypto_address.clone(), 
			transaction_hash: transaction_hash.clone(), 
			id: self.id.clone() 
		});

        Ok(Pallet::<T>::sign_and_submit_transaction(call)?)
	}

	///fn pay_trnx_fee_for_withdraw(&self, fee_amount: U256, fee_type: &str) -> DispatchResult;
	///self is object of struct WithdrawCryptoReq<T, U>.
	///This is helper method to pay fee.
	///This fee is only specific to withdraw during ipfs transaction.
	/// 
	///Todo: This is under observation
	fn pay_trnx_fee_for_withdraw(&self, fee_amount: U256, fee_type: &str) -> DispatchResult {

        log::info!("======================= Pay eth as transaction fee =======================================");

		let fee_collector = <VtbdexTransactionFee<T>>::get().ok_or(Error::<T>::NoneValue)?;
        Pallet::<T>::add_update_balance(&self.crypto_type, &fee_collector.fee_collector_address.encode(), fee_amount, U256::from(0_u8))?;
		Pallet::<T>::deposit_event(Event::TransactionSuccessFee { 
			user: self.account.clone(), 
			reason: fee_type.as_bytes().to_vec(), 
			token_type: self.crypto_type, 
			amount: fee_amount 
		});

        Ok(())
    }
}