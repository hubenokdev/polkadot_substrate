//!
//! This mod is under experiment and research
//! Will implement soon
//! 
use crate::*;
use frame_system::pallet_prelude::OriginFor;

#[derive(Serialize, Deserialize, RuntimeDebug)]
pub struct GetVtbTBalanceRequestJson<'a> {
   pub address: &'a str
}

#[derive(Serialize, Deserialize, RuntimeDebug)]
pub struct VtbTRequestJson<'a> {
	pub address: &'a str,
	pub amount: &'a str
}

#[derive(Serialize, Deserialize, RuntimeDebug)]
pub struct TransferVtbTRequestJson<'a> {
	pub from: &'a str,
	pub to: &'a str,
	pub amount: &'a str
}


#[derive(Serialize, Deserialize, RuntimeDebug)]
pub struct GetPolkadotAddressRequestJson<'a> {
   pub eth_address: &'a str
}

// // ETHEREUM HODLT CONTRACT
// pub const VTBT_EVENT: &str = "0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"; // ETH HODLT transfer event
// pub const VTBTT_TOKEN_ADDRESS: &str = "0xBc4EB7280c63a760Cd669Df7e8D50BF4925b42Fc"; // ETH HODLT token address 
// pub const NULL_ETH_ADDRESS: &str = "0x0000000000000000000000000000000000000000"; // Null ETH address

// Add below extrinsic in lib.rs
		// #[pallet::weight((<T as Config>::WeightInfo::initiate_convert_vtbc_to_vtbt(), Pays::No))]
		// pub fn initiate_convert_vtbc_to_vtbt(origin: OriginFor<T>, amount: U256) -> DispatchResult {
		// 	let who = ensure_signed(origin)?;

		// 	ensure!(<VtbSystemRunning<T>>::get(), Error::<T>::VtbdexSystemIsStopped);
		// 	ensure!(<Wallet<T>>::contains_key(&who), Error::<T>::PolkadotAddressNotLinked);
		
		// 	let res = Self::initiate_convert_vtbc_to_vtbt_token(who, amount);
		// 	match res {
		// 		Ok(_res) => Ok(().into()),
		// 		Err(err) => Err(frame_support::dispatch::DispatchError::from(err)), 
		// 	}
		// }

		// #[pallet::weight((<T as Config>::WeightInfo::initiate_convert_vtbt_to_vtbc(), Pays::No))]
		// pub fn initiate_convert_vtbt_to_vtbc(origin: OriginFor<T>, amount: U256) -> DispatchResult {
		// 	let who = ensure_signed(origin)?;

		// 	ensure!(<VtbSystemRunning<T>>::get(), Error::<T>::VtbdexSystemIsStopped);
		// 	ensure!(<Wallet<T>>::contains_key(&who), Error::<T>::PolkadotAddressNotLinked);
		
		// 	let res = Self::initiate_convert_vtbt_to_vtbc_token(who, amount);
		// 	match res {
		// 		Ok(_res) => Ok(().into()),
		// 		Err(err) => Err(frame_support::dispatch::DispatchError::from(err)), 
		// 	}
		// }

		// #[pallet::weight((25000000 + T::DbWeight::get().writes(0), Pays::No))]
		// pub fn initiate_transfer_of_vtbt(origin: OriginFor<T>, from_address: T::AccountId, to_address: T::AccountId, amount: U256) -> DispatchResult {
		// 	let _who = ensure_signed(origin)?;
		// 	ensure!(<VtbSystemRunning<T>>::get(), Error::<T>::VtbdexSystemIsStopped);

		// 	ensure!(<Wallet<T>>::contains_key(&from_address), Error::<T>::PolkadotAddressNotLinked);

		// 	ensure!(<Wallet<T>>::contains_key(&to_address), Error::<T>::PolkadotAddressNotLinked);
		// 	log::info!("Check transfer: &from_address != &to_address {:?}", &from_address != &to_address);
		// 	ensure!(&from_address != &to_address, Error::<T>::BothFromAndToCanNotBeSame);
		
		// 	let res = Self::initiate_transfer_of_vtbt_token(from_address, to_address, amount);
		// 	match res {
		// 		Ok(_) => Ok(().into()),
		// 		Err(err) => Err(frame_support::dispatch::DispatchError::from(err)), 
		// 	}
		// }

		// / Extrinsic signed by Ocw to submit mint in progress message for vtbt token on ethereum
		// / This extrinsic is designed to sign only via Ocw.
		// / This extrinsic is to notify about the InProgress state of mint Erc20 Vtbt token present on Ethereum.
		// / On success, it emit event as MintVtbTInProgress(Some(who), eth_address, transaction_hash).
		// #[pallet::weight(<T as Config>::WeightInfo::mint_vtbt_inprogress())]
		// pub fn mint_vtbt_inprogress(origin: OriginFor<T>, eth_address: Vec<u8>, transaction_hash: Vec<u8>) -> DispatchResult {
		// 	let who = ensure_signed(origin)?;

		// 	Self::deposit_event(Event::MintVtbTInProgress(Some(who), eth_address, transaction_hash));
		// 	Ok(())
		// }

		// /// Extrinsic signed by Ocw to submit mint in failed message for vtbt token on ethereum
		// /// This extrinsic is designed to sign only via Ocw.
		// /// This extrinsic is to notify about the failure of mint for Erc20 Vtbt token present on Ethereum.
		// /// On success, it emit event as MintVtbTFailed(Some(who), eth_address, msg).
		// #[pallet::weight(<T as Config>::WeightInfo::mint_vtbt_failed())]
		// pub fn mint_vtbt_failed(origin: OriginFor<T>, eth_address: Vec<u8>, msg: Vec<u8>) -> DispatchResult {
		// 	let who = ensure_signed(origin)?;
		
		// 	Self::deposit_event(Event::MintVtbTFailed(Some(who), eth_address, msg));
		// 	Ok(())
		// }

		// /// Extrinsic signed by Ocw to submit mint balance update for vtbt token on ethereum
		// /// Extrinsic signed by Ocw to submit mint 
		// /// i.e Vtbc to Vtbt in progress message for vtbt token on ethereum
		// /// This extrinsic is designed to sign only via Ocw.
		// /// This extrinsic is to notify about the InProgress state of conversion for Erc20 Vtbt token present on Ethereum.
		// /// On success, it emit event as MintVTBTSuccess(from_address, amount).
		// #[pallet::weight(<T as Config>::WeightInfo::signed_mint_of_vtbt_update_balance())]
		// pub fn signed_mint_of_vtbt_update_balance(origin: OriginFor<T>, from_address: T::AccountId, amount: U256) -> DispatchResult {
		// 	let _who = ensure_signed(origin)?;

		// 	ensure!(<Wallet<T>>::contains_key(&from_address), Error::<T>::PolkadotAddressNotLinked);

		// 	match Self::add_update_balance(&TokenType::Vtbt, &from_address, amount, U256::from(0_u64)) {
		// 		Ok(()) => {
		// 			let vtbc_amount: U256 = Self::convert_vtbt_to_vtbc(amount)?;
		// 			match Self::sub_update_balance(&TokenType::Vtbc, &from_address, vtbc_amount, U256::from(0_u8)) {
		// 				Ok(()) => {
		// 					Globals::<T>::mutate(|get_globals| {
		// 						get_globals.backing_reserve = get_globals.backing_reserve.checked_add(vtbc_amount).unwrap();
		// 					});
		// 					Wallet::<T>::mutate(&from_address, |user_record| {
		// 						//let user_record = user_record_opt.as_mut().unwrap();
		// 						user_record.active = true;
		// 					});
		// 					Self::deposit_event(Event::MintVTBTSuccess(from_address, amount));
		// 					Ok(())
		// 				},
		// 				Err(err) =>  Err(frame_support::dispatch::DispatchError::from(err)), 
		// 			}
		// 		},
		// 		Err(err) =>  Err(frame_support::dispatch::DispatchError::from(err)), 
		// 	}
		// }

		// /// Extrinsic signed by Ocw to submit burn in progress message for vtbt token on ethereum
		// /// This extrinsic is designed to sign only via Ocw.
		// /// This extrinsic is to notify about the InProgress state of burn Erc20 Vtbt token present on Ethereum.
		// /// On success, it emit event as BurnVtbTInProgress(Some(who), eth_address, transaction_hash).
		// #[pallet::weight(<T as Config>::WeightInfo::burn_vtbt_inprogress())]
		// pub fn burn_vtbt_inprogress(origin: OriginFor<T>, eth_address: Vec<u8>, transaction_hash: Vec<u8>) -> DispatchResult {
		// 	let who = ensure_signed(origin)?;

		// 	Self::deposit_event(Event::BurnVtbTInProgress(Some(who), eth_address, transaction_hash));
		// 	Ok(())
		// }

		// /// Extrinsic signed by Ocw to submit mint in failed message for vtbt token on ethereum
		// /// This extrinsic is designed to sign only via Ocw.
		// /// This extrinsic is to notify about the failure of burn for Erc20 Vtbt token present on Ethereum.
		// /// On success, it emit event as BurnVtbTFailed(Some(who), eth_address, msg).
		// #[pallet::weight(<T as Config>::WeightInfo::burn_vtbt_failed())]
		// pub fn burn_vtbt_failed(origin: OriginFor<T>, eth_address: Vec<u8>, msg: Vec<u8>) -> DispatchResult {
		// 	let who = ensure_signed(origin)?;

		// 	Self::deposit_event(Event::BurnVtbTFailed(Some(who), eth_address, msg));
		// 	Ok(())
		// }

		// /// Extrinsic signed by Ocw to submit burn balance update for vtbt token on ethereum
		// /// Extrinsic signed by Ocw to submit burn 
		// /// i.e Vtbt to Vtbc in progress message for vtbt token on ethereum
		// /// This extrinsic is designed to sign only via Ocw.
		// /// This extrinsic is to notify about the InProgress state of back-conversion for Erc20 Vtbt token present on Ethereum.
		// /// On success, it emit event as BurnVTBTSuccess(from_address, amount).
		// #[pallet::weight(<T as Config>::WeightInfo::signed_burn_of_vtbt_update_balance())]
		// pub fn signed_burn_of_vtbt_update_balance(origin: OriginFor<T>, from_address: T::AccountId, amount: U256) -> DispatchResult {
		// 	let _who = ensure_signed(origin)?;

		// 	ensure!(<Wallet<T>>::contains_key(&from_address), Error::<T>::PolkadotAddressNotLinked);

		// 	match Self::sub_update_balance(&TokenType::Vtbt, &from_address, amount, U256::from(0_u64)) {
		// 		Ok(()) => {
		// 			let vtbc_amount: U256 = Self::convert_vtbt_to_vtbc(amount)?;
		// 			match Self::add_update_balance(&TokenType::Vtbc, &from_address, vtbc_amount, U256::from(0_u8)) {
		// 				Ok(()) => {
		// 					Globals::<T>::mutate(|get_globals| {
		// 						get_globals.backing_reserve = get_globals.backing_reserve.checked_sub(vtbc_amount).unwrap();
		// 					});
		// 					Wallet::<T>::mutate(&from_address, |user_record| {
		// 						//let user_record = user_record_opt.as_mut().unwrap();
		// 						user_record.active = true;
		// 					});
		// 					Self::deposit_event(Event::BurnVTBTSuccess(from_address, amount));
		// 					Ok(())
		// 				},
		// 				Err(err) =>  Err(frame_support::dispatch::DispatchError::from(err)), 
		// 			}
		// 		},
		// 		Err(err) =>  Err(frame_support::dispatch::DispatchError::from(err)), 
		// 	}
		// }

		// /// Extrinsic signed by Ocw to submit transfer in progress message for vtbt token on ethereum
		// /// This extrinsic is designed to sign only via Ocw.
		// /// This extrinsic is to notify about the InProgress state of transfer for Erc20 Vtbt token present on Ethereum.
		// /// On success, it emit event as TransferVtbTInProgress(Some(who), eth_address, transaction_hash).
		// #[pallet::weight(<T as Config>::WeightInfo::transfer_vtbt_inprogress())]
		// pub fn transfer_vtbt_inprogress(origin: OriginFor<T>, eth_address: Vec<u8>, transaction_hash: Vec<u8>) -> DispatchResult {
		// 	let who = ensure_signed(origin)?;

		// 	Self::deposit_event(Event::TransferVtbTInProgress(Some(who), eth_address, transaction_hash));
		// 	Ok(())
		// }

		// /// Extrinsic signed by Ocw to submit transfer failed message for vtbt token on ethereum
		// /// This extrinsic is designed to sign only via Ocw.
		// /// This extrinsic is to notify about the failure of transfer for Erc20 Vtbt token present on Ethereum.
		// /// On success, it emit event as TransferVtbTFailed(Some(who), eth_address, msg).
		// #[pallet::weight(<T as Config>::WeightInfo::transfer_vtbt_failed())]
		// pub fn transfer_vtbt_failed(origin: OriginFor<T>, eth_address: Vec<u8>, msg: Vec<u8>) -> DispatchResult {
		// 	let who = ensure_signed(origin)?;
		// 	Self::deposit_event(Event::TransferVtbTFailed(Some(who), eth_address, msg));
		// 	Ok(())
		// }

		// /// Extrinsic signed by Ocw to submit transfer event balance update for vtbt token on ethereum
		// /// This extrinsic is designed to sign only via Ocw.
		// /// This extrinsic is submit and update balance for transfer for Erc20 Vtbt token present on Ethereum.
		// /// On success, it emit event as TransferVTBTSuccess(from_address, to_address, amount).
		// #[pallet::weight(<T as Config>::WeightInfo::signed_transfer_of_vtbt_update_balance())]
		// pub fn signed_transfer_of_vtbt_update_balance(origin: OriginFor<T>, from_address: T::AccountId, to_address: T::AccountId, amount: U256) -> DispatchResult {
		// 	let _who = ensure_signed(origin)?;

		// 	ensure!(<Wallet<T>>::contains_key(&from_address), Error::<T>::PolkadotAddressNotLinked);
		// 	ensure!(<Wallet<T>>::contains_key(&to_address), Error::<T>::PolkadotAddressNotLinked);
			
		// 	match Self::sub_update_balance(&TokenType::Vtbt, &from_address, amount, U256::from(0_u64)) {
		// 		Ok(()) => {
		// 			let _ = Self::add_update_balance(&TokenType::Vtbt, &to_address, amount, U256::from(0_u64));
		// 			Self::deposit_event(Event::TransferVTBTSuccess(from_address, to_address, amount));
		// 			Ok(())
		// 		},
		// 		Err(err) =>  Err(frame_support::dispatch::DispatchError::from(err)), 
		// 	}
		// }

	
// bench marking code 
 // Vtbt token in Ethereum
//  mint_vtbt_inprogress {
// 	let caller = whitelisted_caller();
// 	let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
// 	let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
// 	let user_address_lc_1 = user_address.to_lowercase();
// 	let user_address_lc = user_address_lc_1.to_string().as_bytes().to_vec();
// 	let txn_hash = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
// 	let txn_hash_lc = txn_hash.to_lowercase();
// 	let txn_hash_vec = txn_hash_lc.to_string().as_bytes().to_vec();

// }:_(RawOrigin::Signed(caller), user_address_lc.clone(), txn_hash_vec)

// mint_vtbt_failed {
// 	let caller = whitelisted_caller();
// 	let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
// 	let user_address_lc_1 = user_address.to_lowercase();
// 	let user_address_lc = user_address_lc_1.to_string().as_bytes().to_vec();
// 	let msg = "failed due to time out or http error";
// 	let msg = msg.to_lowercase();
// 	let msg_vec = msg_lc.to_string().as_bytes().to_vec();
// }:_(RawOrigin::Signed(caller), user_address_lc.clone(), msg_vec)


impl<T: Config> Pallet<T> {

    pub fn initiate_convert_vtbc_to_vtbt_token(from_address: T::AccountId, amount: U256) -> Result<(), Error<T>> {

		let user = Self::user_record(&from_address);
		let crypto_details =  user.crypto_addresses.get(&TokenType::Eth).clone();
		match crypto_details {
            Some(val) => {val},
            None => {return Err(Error::<T>::UserDoesNotHaveLinkedEthAddress);}
        };

		let crypto_address =  crypto_details.unwrap();
		
		let vtbc_amount = Self::convert_vtbt_to_vtbc(amount);
		
        ensure!(user.vtbc_balance >= vtbc_amount , Error::<T>::InsufficientFunds);
		ensure!(user.active , Error::<T>::UserWalletIsLocked);	
		//ensure!(crypto_address.deposit_balance >= T::MinDepositBalanceToPayFee::get(), Error::<T>::InsufficientFundsToPayFees);

		let key = Self::derived_key(frame_system::Pallet::<T>::block_number(), "mint_initiate", 1); //TODO key change
		let data = custom_types::MintIndexingData(b"mint_vtbt_initiate".to_vec(), crypto_address.crypto_addresses.clone(), amount);

		offchain_index::set(&key, &data.encode());
		Wallet::<T>::mutate(&from_address, |user_record| {
			user_record.active = false;
		});
		Self::deposit_event(Event::MintVTBtInitiated(from_address, amount, key));	
		
		Ok(())
	}

	pub fn mint_call(key: &str, eth_address: &str, amount: U256) -> Result<(), Error<T>> {
					
		log::info!("eth_address for mint {:?}", eth_address);
		log::info!("amount for mint {:?}", amount);
		log::info!("keys: {:?}", key);

		let amount_value = amount.to_string();
		let req_obj = custom_types::VtbTRequestJson {
			address: &eth_address,
			amount: &amount_value
		};
	
		let req_obj_str: &str = &serde_json::to_string(&req_obj).unwrap();
		let request_body = vec!(req_obj_str);

        let mut lock = StorageLock::<BlockAndTime<frame_system::Pallet<T>>>::with_block_and_time_deadline(
            b"vtbdex::mint-vtbt-erc20-lock",
            constants::LOCK_BLOCK_EXPIRATION,
            rt_offchain::Duration::from_millis(constants::LOCK_TIMEOUT_EXPIRATION),
        );
		
		if let Ok(_guard) = lock.try_lock() {
			match Self::fetch_n_parse_post_request(global_constants::_MINT_REQUEST_API_ENDPOINT, request_body) {
				Ok(response) => {
					log::info!("fetch_from_remote info: {:?}", response);
					let response_body_code: u64 =  response["Code"].as_u64().unwrap();
					if response_body_code != 200 {
						let msg = response["Msg"].as_str().unwrap();
						let _ = Self::emit_mint_vtbt_failed(eth_address.as_bytes().to_vec(), msg.as_bytes().to_vec());
						
					}
					else {
						let transaction_hash = response["txnHash"].as_str().unwrap();
						let _ = Self::emit_mint_vtbt_inprogress(eth_address.as_bytes().to_vec(), transaction_hash.as_bytes().to_vec());
					}
				}
				Err(err) => {
					return Err(err);
				}
			}	
		};

		Ok(())
	}

	fn emit_mint_vtbt_inprogress(eth_address: Vec<u8>, transaction_hash: Vec<u8>) -> Result<(), Error<T>> {

		let signer = Signer::<T, <T as pallet::Config>::AuthorityId>::any_account();

		let result = signer.send_signed_transaction(|_acct|
			Call::mint_vtbt_inprogress(eth_address.clone(), transaction_hash.clone()));
		// Display error if the signed tx fails.
		if let Some((acc, res)) = result {
			if res.is_err() {
				log::info!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
				return Err(<Error<T>>::OffchainSignedTxError);
			}
	
			log::info!("Sent success: {:?}", res);
			// Transaction is sent successfully
			return Ok(());
		}
		log::info!("No local account available");
		Err(<Error<T>>::NoLocalAcctForSigning)

	}

	fn emit_mint_vtbt_failed(eth_address: Vec<u8>, msg: Vec<u8>) -> Result<(), Error<T>> {

		let signer = Signer::<T, <T as pallet::Config>::AuthorityId>::any_account();

		let result = signer.send_signed_transaction(|_acct|
			Call::mint_vtbt_failed(eth_address.clone(), msg.clone()));
		// Display error if the signed tx fails.
		if let Some((acc, res)) = result {
			if res.is_err() {
				log::info!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
				return Err(<Error<T>>::OffchainSignedTxError);
			}
	
			log::info!("Sent success: {:?}", res);
			// Transaction is sent successfully
			return Ok(());
		}
		log::info!("No local account available");
		Err(<Error<T>>::NoLocalAcctForSigning)

	}

    pub fn initiate_convert_vtbt_to_vtbc_token(from_address: T::AccountId, amount: U256) -> Result<(), Error<T>> {

		let user = Self::user_record(&from_address);
		let crypto_details =  user.crypto_addresses.get(&TokenType::Eth).clone();
		match crypto_details {
            Some(val) => {val},
            None => {return Err(Error::<T>::UserDoesNotHaveLinkedEthAddress);}
        };
		let crypto_address =  crypto_details.unwrap();

		let eth_address = str::from_utf8(&crypto_address.crypto_addresses).unwrap_or("error");

        ensure!(user.vtbt_balance >= amount , Error::<T>::InsufficientFunds); 
		ensure!(user.active , Error::<T>::UserWalletIsLocked);
		ensure!(crypto_address.deposit_balance >= T::MinDepositBalanceToPayFee::get(), Error::<T>::InsufficientFundsToPayFees);
     	#[cfg(feature = "std")]
		{
			let vtbt_balance = Self::get_vtbt_balance_from_erc20_token(eth_address).unwrap();
			ensure!(vtbt_balance >= amount , Error::<T>::InsufficientFundsInToken);
		}
		
		let key = Self::derived_key(frame_system::Pallet::<T>::block_number(), "burn_initiate", 1); //TODO key change
		let data = custom_types::BurnIndexingData(b"burn_vtbt_initiate".to_vec(), crypto_address.crypto_addresses.clone(), amount);

		offchain_index::set(&key, &data.encode());

		Self::deposit_event(Event::BurnVTBtInitiated(from_address, amount, key));	
		//TODO
		Ok(())
	}

	pub fn burn_call_substrate(origin: OriginFor<T>, from_address: T::AccountId, amount: U256) -> Result<(), Error<T>>  {

		let assetid: T::AssetId = T::VtbErc20AssetId::get();
		let _ = pallet_vtbt::Pallet::<T>::burn(origin, assetid, from_address,amount);

		Ok(())
	}
   
	pub fn burn_call(key: &str, eth_address: &str, amount: U256) -> Result<(), Error<T>> {
					
		log::info!("eth_address for burn {:?}", eth_address);
		log::info!("amount for burn {:?}", amount);
		log::info!("keys: {:?}", key);

		let amount_value = amount.to_string();
		let req_obj = custom_types::VtbTRequestJson {
			address: &eth_address,
			amount: &amount_value
		};
	
		let req_obj_str: &str = &serde_json::to_string(&req_obj).unwrap();
		let request_body = vec!(req_obj_str);

        let mut lock = StorageLock::<BlockAndTime<frame_system::Pallet<T>>>::with_block_and_time_deadline(
            b"vtbdex::burn-vtbt-erc20-lock",
            constants::LOCK_BLOCK_EXPIRATION,
            rt_offchain::Duration::from_millis(constants::LOCK_TIMEOUT_EXPIRATION),
        );
		
		if let Ok(_guard) = lock.try_lock() {
			match Self::fetch_n_parse_post_request(global_constants::_MINT_REQUEST_API_ENDPOINT, request_body) {
				Ok(response) => {
					log::info!("fetch_from_remote info: {:?}", response);
					let response_body_code: u64 =  response["Code"].as_u64().unwrap();
					if response_body_code != 200 {
						let msg = response["Msg"].as_str().unwrap();
						let _ = Self::emit_burn_vtbt_failed(eth_address.as_bytes().to_vec(), msg.as_bytes().to_vec());
						
					}
					else {
						let transaction_hash = response["txnHash"].as_str().unwrap();
						let _ = Self::emit_burn_vtbt_inprogress(eth_address.as_bytes().to_vec(), transaction_hash.as_bytes().to_vec());
					}
				}
				Err(err) => {
					return Err(err);
				}
			}	
		};

		Ok(())
	}

	fn emit_burn_vtbt_inprogress(eth_address: Vec<u8>, transaction_hash: Vec<u8>) -> Result<(), Error<T>> {

		let signer = Signer::<T, <T as pallet::Config>::AuthorityId>::any_account();

		let result = signer.send_signed_transaction(|_acct|
			Call::burn_vtbt_inprogress(eth_address.clone(), transaction_hash.clone()));
		// Display error if the signed tx fails.
		if let Some((acc, res)) = result {
			if res.is_err() {
				log::info!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
				return Err(<Error<T>>::OffchainSignedTxError);
			}
	
			log::info!("Sent success: {:?}", res);
			// Transaction is sent successfully
			return Ok(());
		}
		log::info!("No local account available");
		Err(<Error<T>>::NoLocalAcctForSigning)

	}

	fn emit_burn_vtbt_failed(eth_address: Vec<u8>, msg: Vec<u8>) -> Result<(), Error<T>> {

		let signer = Signer::<T, <T as pallet::Config>::AuthorityId>::any_account();

		let result = signer.send_signed_transaction(|_acct|
			Call::burn_vtbt_failed(eth_address.clone(), msg.clone()));
		// Display error if the signed tx fails.
		if let Some((acc, res)) = result {
			if res.is_err() {
				log::info!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
				return Err(<Error<T>>::OffchainSignedTxError);
			}
	
			log::info!("Sent success: {:?}", res);
			// Transaction is sent successfully
			return Ok(());
		}
		log::info!("No local account available");
		Err(<Error<T>>::NoLocalAcctForSigning)

	}

    pub fn initiate_transfer_of_vtbt_token(from: T::AccountId, to: T::AccountId, amount: U256) -> Result<(), Error<T>> {

        let from_user = Self::user_record(&from);
		let from_crypto_details =  from_user.crypto_addresses.get(&TokenType::Eth).clone();
		match from_crypto_details {
            Some(val) => {val},
            None => {return Err(Error::<T>::UserDoesNotHaveLinkedEthAddress);}
        };
		let from_crypto_address =  from_crypto_details.unwrap();

        let to_user = Self::user_record(&to);
		let to_crypto_details =  to_user.crypto_addresses.get(&TokenType::Eth).clone();
		match to_crypto_details {
            Some(val) => {val},
            None => {return Err(Error::<T>::UserDoesNotHaveLinkedEthAddress);}
        };
		
        ensure!(from_user.vtbt_balance >= amount , Error::<T>::InsufficientFunds);
		ensure!(from_user.active , Error::<T>::UserWalletIsLocked);	
        ensure!(to_user.active , Error::<T>::UserWalletIsLocked);	

		let key = Self::derived_key(frame_system::Pallet::<T>::block_number(), "transfer_initiate", 1); //TODO key change
		let data = custom_types::BurnIndexingData(b"burn_vtbt_initiate".to_vec(), from_crypto_address.crypto_addresses.clone(), amount); //todo

		offchain_index::set(&key, &data.encode());

		Self::deposit_event(Event::TransferVTBtInitiated(from, to, amount, key));
		
		Ok(())
	}

	#[cfg(feature = "std")]
    pub fn match_vtbt_erc20_token_transfer_event_type(action_params: &serde_json::Map<std::string::String, serde_json::Value>) -> Result<(), Error<T>> {
        let from_address = action_params["from"].as_str().unwrap();
		let to_address = action_params["to"].as_str().unwrap();
        if from_address == constants::NULL_ETH_ADDRESS {
            //mint
            let _ = Self::vtbt_mint_event(action_params);
        }
        else if to_address == constants::NULL_ETH_ADDRESS {
            //burn 
            let _ = Self::vtbt_burn_event(action_params);
        }
        else {
            //transfer
            let _ = Self::vtbt_transfer_event(action_params);
        }
        
		Ok(())
    }

	#[cfg(feature = "std")]
    fn vtbt_mint_event(action_params: &serde_json::Map<std::string::String, serde_json::Value>) -> Result<(), Error<T>> {

		let mut lock = StorageLock::<BlockAndTime<frame_system::Pallet<T>>>::with_block_and_time_deadline(
			b"vtbt::vtbt-mint-lock",
			constants::LOCK_BLOCK_EXPIRATION,
			rt_offchain::Duration::from_millis(constants::LOCK_TIMEOUT_EXPIRATION),
		);

		let to_address = action_params["to"].as_str().unwrap();
		let amount_of_vtbt = U256::from_dec_str(action_params["amount"].as_str().unwrap()).unwrap_or_default();

	    let account_id = match Self::get_pdot_address(to_address) {
			Ok(address) => {
				log::info!("address id {:?}", address);
				address
			},
			Err(err) => {
				log::info!("err id {:?}", err);
				return Err(err)
			}
		};
		
		if <Wallet<T>>::contains_key(&account_id) {
			if let Ok(_guard) = lock.try_lock() {
				let _ = Self::signed_mint_of_vtbt_update(account_id, amount_of_vtbt);
			};
		}
		else {
		    log::info!("User Polkadot address not registered");
		}

		Ok(())
	}

	fn signed_mint_of_vtbt_update(from: T::AccountId, amount: U256) -> Result<(), Error<T>> {

		let signer = Signer::<T, <T as pallet::Config>::AuthorityId>::any_account();

		let result = signer.send_signed_transaction(|_acct|
			Call::signed_mint_of_vtbt_update_balance(from.clone(), amount));
		// Display error if the signed tx fails.
		if let Some((acc, res)) = result {
			if res.is_err() {
				log::info!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
				return Err(<Error<T>>::OffchainSignedTxError);
			}
	
			log::info!("Sent success: {:?}", res);
			// Transaction is sent successfully
			return Ok(());
		}
		log::info!("No local account available");
		Err(<Error<T>>::NoLocalAcctForSigning)

	}

	#[cfg(feature = "std")]
    fn vtbt_burn_event(action_params: &serde_json::Map<std::string::String, serde_json::Value>) -> Result<(), Error<T>> {

		let mut lock = StorageLock::<BlockAndTime<frame_system::Pallet<T>>>::with_block_and_time_deadline(
			b"vtbt::vtbt-burn-lock",
			constants::LOCK_BLOCK_EXPIRATION,
			rt_offchain::Duration::from_millis(constants::LOCK_TIMEOUT_EXPIRATION),
		);

		let from_address = action_params["from"].as_str().unwrap();
		let amount_of_vtbt = U256::from_dec_str(action_params["amount"].as_str().unwrap()).unwrap_or_default();

	    let account_id = match Self::get_pdot_address(from_address) {
			Ok(address) => {
				log::info!("address id {:?}", address);
				address
			},
			Err(err) => {
				log::info!("err id {:?}", err);
				return Err(err)
			}
		};
		
		if <Wallet<T>>::contains_key(&account_id) {
			if let Ok(_guard) = lock.try_lock() {
				let _ = Self::signed_burn_of_vtbt_update(account_id, amount_of_vtbt );
			};
		}
		else {
		    log::info!("User Polkadot address not registered");
		}

		Ok(())
	}

	fn signed_burn_of_vtbt_update(from: T::AccountId, amount: U256) -> Result<(), Error<T>> {

		let signer = Signer::<T, <T as pallet::Config>::AuthorityId>::any_account();

		let result = signer.send_signed_transaction(|_acct|
			Call::signed_burn_of_vtbt_update_balance(from.clone(), amount));
		// Display error if the signed tx fails.
		if let Some((acc, res)) = result {
			if res.is_err() {
				log::info!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
				return Err(<Error<T>>::OffchainSignedTxError);
			}
	
			log::info!("Sent success: {:?}", res);
			// Transaction is sent successfully
			return Ok(());
		}
		log::info!("No local account available");
		Err(<Error<T>>::NoLocalAcctForSigning)

	}

	#[cfg(feature = "std")]
    fn vtbt_transfer_event(action_params: &serde_json::Map<std::string::String, serde_json::Value>) -> Result<(), Error<T>> {

		let mut lock = StorageLock::<BlockAndTime<frame_system::Pallet<T>>>::with_block_and_time_deadline(
			b"vtbt::vtbt-transfer-lock",
			constants::LOCK_BLOCK_EXPIRATION,
			rt_offchain::Duration::from_millis(constants::LOCK_TIMEOUT_EXPIRATION),
		);

        let to_address = action_params["to"].as_str().unwrap();
		let from_address = action_params["from"].as_str().unwrap();
		let amount_of_vtbt = U256::from_dec_str(action_params["amount"].as_str().unwrap()).unwrap_or_default();

	    let from_account_id = match Self::get_pdot_address(from_address) {
			Ok(address) => {
				log::info!("address id {:?}", address);
				address
			},
			Err(err) => {
				log::info!("err id {:?}", err);
				return Err(err)
			}
		};
		
        let to_account_id = match Self::get_pdot_address(to_address) {
			Ok(address) => {
				log::info!("address id {:?}", address);
				address
			},
			Err(err) => {
				log::info!("err id {:?}", err);
				return Err(err)
			}
		};

		if <Wallet<T>>::contains_key(&from_account_id) &&  <Wallet<T>>::contains_key(&to_account_id) {
			if let Ok(_guard) = lock.try_lock() {
				let _ = Self::signed_transfer_of_vtbt_update(from_account_id, to_account_id, amount_of_vtbt );
			};
		}
		else {
		    log::info!("User Polkadot address not registered");
		}

		Ok(())
	}

	fn signed_transfer_of_vtbt_update(from: T::AccountId, to: T::AccountId, amount: U256) -> Result<(), Error<T>> {

		let signer = Signer::<T, <T as pallet::Config>::AuthorityId>::any_account();

		let result = signer.send_signed_transaction(|_acct|
			Call::signed_transfer_of_vtbt_update_balance(from.clone(), to.clone(), amount));
		// Display error if the signed tx fails.
		if let Some((acc, res)) = result {
			if res.is_err() {
				log::info!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
				return Err(<Error<T>>::OffchainSignedTxError);
			}
	
			log::info!("Sent success: {:?}", res);
			// Transaction is sent successfully
			return Ok(());
		}
		log::info!("No local account available");
		Err(<Error<T>>::NoLocalAcctForSigning)

	}

	#[cfg(feature = "std")]
	fn get_pdot_address(eth_address: &str) -> Result<T::AccountId, Error<T>> {

		let account_id: T::AccountId;
		let params = custom_types::GetPolkadotAddressRequestJson {
			eth_address: eth_address
		};
	
		let req_obj_str: &str = &serde_json::to_string(&params).unwrap();		
		let request_body = vec!(req_obj_str);
        match Self::fetch_n_parse_post_request(global_constants::_ETH_VTB_GET_POLKADOT_ADDRESS_API, request_body) {
			Ok(log_info) => {
				let pdot_address = log_info["pdot_address"].as_str().unwrap();

				if pdot_address != "" {

					let account: AccountId32 = match pdot_address.parse::<AccountId32>() {
						Ok(acc) => acc,
						Err(_) => return Err(<Error<T>>::InvalidSs58Address),
					};
			
					account_id = T::AccountId::decode(& mut AccountId32::as_ref(&account)).unwrap_or_default();
				}
				else {
					return Err(<Error<T>>::InvalidSs58Address);
				}
			}
			Err(err) => {
				return Err(err)
			}
		}
		Ok(account_id)
	}

	#[cfg(feature = "std")]
	fn get_vtbt_balance_from_erc20_token(eth_address: &str) -> Result<U256, Error<T>> {

		let balance: U256;
		let params = custom_types::GetVtbTBalanceRequestJson {
			address: eth_address
		};
	
		let req_obj_str: &str = &serde_json::to_string(&params).unwrap();		
		let request_body = vec!(req_obj_str);
        match Self::fetch_n_parse_post_request(global_constants::_GET_VTBT_TOKEN_BALANCE_API, request_body) {
			Ok(log_info) => {
				balance = U256::from_dec_str(log_info["balanceOf"].as_str().unwrap()).unwrap();
			}
			Err(err) => {
				return Err(err)
			}
		}
		Ok(balance)
	}

	pub fn process_each_vtbt_log(log_info: &Vec<serde_json::Value>) {

		let ocw_store_key_eth_processed_blocks = StorageValueRef::persistent(b"vtbt::vtbt-processed-blocks-having-log");
 
		let mut ocw_store_val_eth_processed_blocks = match ocw_store_key_eth_processed_blocks.get::<custom_types::EthProcessedBlocksWithLogs>() {
			 Ok(Some(data)) => {
				 data
			 },
			 _ => {
				log::info!("Error: offchain data reteieved failed!");
				custom_types::EthProcessedBlocksWithLogs{ blocks: Vec::new()}
			}
		};
 
		for index in 0..log_info.len() {
			let each_log = log_info[index].as_object().unwrap();
			log::info!("logs: {:?}", &each_log);
			let each_block_number =  each_log["blocknumber"].as_u64().unwrap(); //get block number for each transaction from log
			
			if !ocw_store_val_eth_processed_blocks.blocks.contains(&each_block_number) { 
 
				let event_name: &str =  each_log["topicHash"].as_str().unwrap();
				match event_name {
					constants::VTBT_EVENT => {
					   log::info!("Listen event for vtbt erc20 Transfer");	
 
						 #[cfg(feature = "std")]
						 let _ = Self::match_vtbt_erc20_token_transfer_event_type(each_log);	
 
						ocw_store_val_eth_processed_blocks.blocks.push(each_block_number); //add blocks in process blocks records	
					},
					&_ => {
					   log::info!("Error in match arms");
					}
				} 
 
			}
		}
		ocw_store_key_eth_processed_blocks.set(&ocw_store_val_eth_processed_blocks);
	}

	fn _check_for_mint_indexed_data(block_number: T::BlockNumber) {
		let key = Self::derived_key(block_number, "mint_initiate", 1); // Recheck when eth vtbt will be in place
		let oci_mem = StorageValueRef::persistent(&key);

		let data = oci_mem.get::<custom_types::MintIndexingData>();
		match data {
			Ok(Some(data)) => {
				log::info!("off-chain indexing data: {:?}, {:?} , {:?}, {:?}",
					str::from_utf8(&data.0).unwrap_or("error"),&data.1, str::from_utf8(&data.1).unwrap_or("error"), data.2, );
		
				match str::from_utf8(&data.0) {
					Ok("mint_vtbt_initiate") => {
						let eth_address = str::from_utf8(&data.1).unwrap_or("error");
						let amount = data.2;
						let key_in_str = str::from_utf8(&key).unwrap_or("error");
						let _ = Self::mint_call(key_in_str, &eth_address, amount);
					},
					_ => log::info!("Error"),
				}
				
			},
			Err(_err) => {
				log::info!("Error: Offchain indexed data failed: {:?}", block_number);
			},
			_ => log::info!("Info: No off-chain indexing data present for mint in the block number: {:?}", block_number - 2u32.into()),
		
		} 
	}

	fn _check_for_burn_indexed_data(block_number: T::BlockNumber) {
		let key = Self::derived_key(block_number, "burn_initiate", 1);
		let mut oci_mem = StorageValueRef::persistent(&key);

		let data = oci_mem.get::<custom_types::BurnIndexingData>();
		match data {
			Ok(Some(data)) => {
				log::info!("off-chain indexing data: {:?}, {:?} , {:?}, {:?}",
					str::from_utf8(&data.0).unwrap_or("error"),&data.1, str::from_utf8(&data.1).unwrap_or("error"), data.2, );
		
				match str::from_utf8(&data.0) {
					Ok("burn_vtbt_initiate") => {
						let eth_address = str::from_utf8(&data.1).unwrap_or("error");
						let amount = data.2;
						let key_in_str = str::from_utf8(&key).unwrap_or("error");
						let _ = Self::mint_call(key_in_str, &eth_address, amount);
						oci_mem.clear();
					},
					_ => log::info!("Error"),	
				}
			},
			Err(_err) => {
				log::info!("Error: Offchain indexed data failed: {:?}", block_number);
			},
			_ => log::info!("Info: No off-chain indexing data present for burn in the block number: {:?}", block_number - 2u32.into()),
		
		} 
	}
}