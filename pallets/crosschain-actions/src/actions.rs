//!Module actions: This module have all the helper methods related to Vtb contract, such as Onboarding, Deposit, Withdraw.

use crate::*;
use sp_runtime::{DispatchResult};
use sp_runtime::DispatchError;
use frame_support::ensure;
use codec::Encode;

impl<T: Config> Pallet<T> {

	/// fn initialize_user_for_distribution(account: &T::AccountId, 
	///crypto_address: &Vec<u8> )
	/// 
	///This function check for the current period and if the user is new,
	///it create space in the runtime for the user,
	///so that user each period balance can be maintained
	pub(super) fn initialize_user_for_distribution(account: &T::AccountId, 
		crypto_address: &Vec<u8>,
		token_type: TokenType
	) -> DispatchResult {

		let current_period = <pallet_vtbdex::Pallet<T>>::get_current_period();
		//<pallet_vtbdex::Pallet<T>>::initialize_user_for_distribution(account, current_period, token_type);
		<pallet_vtbdex::Pallet<T>>::add_and_update_migrated_balance(account, crypto_address, token_type, current_period)?;
		
		Ok(())
	}

	///fn check_and_create_wallet_storage(
	/// 	mut account_key: Vec<u8>,
	/// 	token_type: TokenType, 
	/// 	pdot_address: Option<T::AccountId>, 
	/// 	crypto_detail: Option<pallet_vtbdex::UserCryptoBasedData>, 
	/// 	new_wallet: Option<pallet_vtbdex::UserType>, 
	/// ) -> DispatchResult;
	/// 
	/// This function is used to handle deposit for different scenarios of Usdt/Usdc.. Erc20 existing token contract.
	pub fn check_and_create_wallet_storage<'a>(
		account_key: &'a mut Vec<u8>,
		token_type: TokenType, 
		pdot_address: &Option<T::AccountId>, 
		crypto_detail: Option<pallet_vtbdex::UserCryptoBasedData>, 
		new_wallet: Option<pallet_vtbdex::UserType>, 
	) -> DispatchResult {
		if let Some(address) = pdot_address.clone() {
			ensure!(<pallet_vtbdex::WalletStorage<T>>::is_user_exist(&address), Error::<T>::PolkadotAddressNotLinked);
			if let Some(info) = crypto_detail {
				<pallet_vtbdex::WalletStorage<T>>::append_or_replace_user_wallet_crypto(&info, &address, token_type)?;
			}
			*account_key = address.encode();
			Ok(())
		}
		else {
			if let Some(info) = new_wallet {
				let crypto_details = info.crypto_addresses.get(&token_type).ok_or(Error::<T>::InvalidTokenType)?;
				let crypto_address = crypto_details.crypto_address.as_ref().ok_or(Error::<T>::CrytoAddressNotLinked)?;
				<pallet_vtbdex::WalletStorage<T>>::insert_new_wallet_record(&crypto_address, &crypto_address, info.clone());
				*account_key = crypto_address.clone();
			}

			Ok(())
		}
	}

	/// fn vtb_onboard_user(user_pdot_address: &str, 
	///user_crypto_addr: &str, 
	///transaction_hash: Vec<u8>,
	///token_type: TokenType) -> Result<(), Error<T>> 
	/// 
	///This function verify that user_ss58 address and crypto_address is linked or not.
	///If anyof the address is not linked, 
	///than it creates a wallet for the user with a given ss58 address, 
	///Or if the user wallet exist but the given crypto_address is not linked
	///than it link the crypto_address to the matched user wallet.
	///One wallet can have multiple cryptos address.
	pub fn vtb_onboard_user(user_pdot_address: &str, 
		user_crypto_addr: &str, 
		transaction_hash: Vec<u8>,
		token_type: TokenType) -> DispatchResult
	{
		let account_id = <pallet_vtbdex::Pallet<T>>::convert_str_to_valid_account_id(user_pdot_address)?;

		if <pallet_vtbdex::WalletStorage<T>>::is_user_exist(&account_id) {
			let existing_crypto_address = <pallet_vtbdex::WalletStorage<T>>::get_registered_crypto_address(
				&Some(account_id.clone()), &token_type
			)?;
			if existing_crypto_address.is_empty() || existing_crypto_address == "error" {
				let _ = Self::link_crypto_address_with_wallet(&account_id, user_crypto_addr, transaction_hash, token_type);
			} 
			else {
				log::debug!(
					"This polkadot_address {:?} has been already linked with the mentioned crypto address : {:?}", 
					user_pdot_address, existing_crypto_address
				);
				return Err(DispatchError::from(<Error<T>>::OnBoardFailed))
			}
		}
		else {
			log::debug!(
				"Is polkadot address already registered: {:?}", 
				<pallet_vtbdex::WalletStorage<T>>::is_user_exist(&account_id)
			);
			let _ = Self::create_new_wallet(user_pdot_address, user_crypto_addr, transaction_hash, token_type);	
		}

		Ok(())
	}

	/// fn create_new_wallet(pdot_address: &str, 
	///user_crypto_addr: &str, 
	///tx_hash: Vec<u8>,
	///token_type: TokenType,
	///) -> Result<(), Error<T>>
	/// 
	///This function make a signed transaction from ocw-context to create a new user wallet in runtime.
	pub fn create_new_wallet(pdot_address: &str, 
		user_crypto_addr: &str, 
		tx_hash: Vec<u8>,
		token_type: TokenType,
	) -> DispatchResult {
	
		let current_period = <pallet_vtbdex::Pallet<T>>::get_current_period();
		let mut wallet = pallet_vtbdex::UserType::new(Some(pdot_address), current_period);
		wallet.update_crypto_details(token_type, Some(user_crypto_addr));

		let account_id = <pallet_vtbdex::Pallet<T>>::convert_str_to_valid_account_id(pdot_address)?;
		log::info!("create_new_wallet pdot_address: {:?} = {:?}", pdot_address, account_id);

		let signer = Signer::<T, <T as pallet::Config>::AuthorityId>::any_account();
	
		let result = signer.send_signed_transaction(|_acct|
			// This is the on-chain function
			Call::submit_signed_user_new_wallet_record { 
				token_type, 
				info: wallet.clone(), 
				pdot_address: account_id.clone(), 
				transaction_hash: tx_hash.clone() 
			}
		);
		// Display error if the signed tx fails.
		if let Some((acc, res)) = result {
			if res.is_err() {
				log::info!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
				return Err(DispatchError::from(<Error<T>>::OffchainSignedTxError));
			}
			log::info!("Sent success: {:?}", res);
			// Transaction is sent successfully
			return Ok(());
		}
		log::info!("No local account available");
		Err(DispatchError::from(<Error<T>>::NoLocalAcctForSigning))
	}

	/// fn link_crypto_address_with_wallet(account: &T::AccountId, 
	///user_crypto_addr: &str, 
	///tx_hash: Vec<u8>,
	///token_type: TokenType
	///) -> Result<(), Error<T>>
	/// 
	///This function make a signed transaction from ocw-context to link new cryptos to the wallet.
	pub fn link_crypto_address_with_wallet(account: &T::AccountId, 
		user_crypto_addr: &str, 
		tx_hash: Vec<u8>,
		token_type: TokenType,
		//app_crypto: dyn AppCrypto<V, U>
		//signer: frame_system::offchain::Signer<T, <T as pallet::Config>::AuthorityId>
	) -> DispatchResult {
	
		let crypto_detail = pallet_vtbdex::UserCryptoBasedData::new(token_type, user_crypto_addr);
		let signer = Signer::<T, <T as pallet::Config>::AuthorityId>::any_account();
		let result = signer.send_signed_transaction(|_acct|
			// This is the on-chain function
			Call::submit_signed_user_wallet_add_new_crypto { 
				token_type, 
				pdot_address: account.clone(), 
				info: crypto_detail.clone(), 
				transaction_hash: tx_hash.clone() 
			}
		);
		// Display error if the signed tx fails.
		if let Some((acc, res)) = result {
			if res.is_err() {
				log::info!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
				return Err(DispatchError::from(<Error<T>>::OffchainSignedTxError));
			}
			log::info!("Sent success: {:?}", res);
			// Transaction is sent successfully
			return Ok(());
		}
		log::info!("No local account available");
		Err(DispatchError::from(<Error<T>>::NoLocalAcctForSigning))
	}

	/// fn vtb_crypto_deposits_sign_trnx(user_pdot_address: &str, 
	///amount: U256,
	///transaction_hash: Vec<u8>,
	///token_type: TokenType
	///) -> Result<(), Error<T>> 
	/// 
	///This function make a signed transaction from ocw-context to update user deposit balance in wallet.
	pub fn vtb_crypto_deposits_sign_trnx(user_pdot_address: Option<&str>, 
		user_crypto_addr: &str, 
		amount: U256,
		transaction_hash: Vec<u8>,
		token_type: TokenType
	) -> DispatchResult {

		let current_period = <pallet_vtbdex::Pallet<T>>::get_current_period();
		let mut wallet = pallet_vtbdex::UserType::new(None, current_period);

		let account_id =  if let Some(address) = user_pdot_address {
			Some(<pallet_vtbdex::Pallet<T>>::convert_str_to_valid_account_id(address)?)
		} else {
			
			wallet.update_crypto_details(token_type, Some(user_crypto_addr));
			None
		};
		let existing_crypto_address = <pallet_vtbdex::WalletStorage<T>>::get_registered_crypto_address(
			&account_id, &token_type
		)?;
		

		let signer = Signer::<T, <T as pallet::Config>::AuthorityId>::any_account();
		let result = signer.send_signed_transaction(|_acct|
			if existing_crypto_address.is_empty() || existing_crypto_address == "error" {
				let crypto_detail = pallet_vtbdex::UserCryptoBasedData::new(token_type, user_crypto_addr);

				Call::crypto_deposit_success { 
					token_type, 
					pdot_address: account_id.clone(), 
					deposit_amount: amount, 
					transaction_hash: transaction_hash.clone(), 
					crypto_detail: Some(crypto_detail.clone()),
					new_wallet: Some(wallet.clone())
				}
			} 
			else {
				Call::crypto_deposit_success { 
					token_type, 
					pdot_address: account_id.clone(), 
					deposit_amount: amount, 
					transaction_hash: transaction_hash.clone(), 
					crypto_detail: None,
					new_wallet: None
				}
			}	
		);
		// Display error if the signed tx fails.
		if let Some((acc, res)) = result {
			if res.is_err() {
				log::info!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
				return Err(DispatchError::from(<Error<T>>::OffchainSignedTxError));
			}
	
			log::info!("Sent success: {:?}", res);
			// Transaction is sent successfully
			return Ok(());
		}
		log::info!("No local account available");
		Err(DispatchError::from(<Error<T>>::NoLocalAcctForSigning))
	}

	/// fn vtb_crypto_withdraw_sign_trnx(user_pdot_address: &str, 
	///amount: U256,
	///transaction_hash: Vec<u8>,
	///token_type: TokenType
	///) -> Result<(), Error<T>>
	/// 
	/// This function make a signed transaction from ocw-context to update user withdraw balance in wallet.
	pub fn vtb_crypto_withdraw_sign_trnx(user_pdot_address: &str, 
		amount: U256,
		transaction_hash: Vec<u8>,
		token_type: TokenType
	) -> DispatchResult {

		let account_id = <pallet_vtbdex::Pallet<T>>::convert_str_to_valid_account_id(user_pdot_address)?;

		let signer = Signer::<T, <T as pallet::Config>::AuthorityId>::any_account();

		let result = signer.send_signed_transaction(|_acct|
			Call::crypto_withdraw_success {
				token_type, 
				pdot_address: account_id.clone(), 
				withdrawn_amount: amount, 
				transaction_hash: transaction_hash.clone() 
			} 
		);
		// Display error if the signed tx fails.
		if let Some((acc, res)) = result {
			if res.is_err() {
				log::info!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
				return Err(DispatchError::from(<Error<T>>::OffchainSignedTxError));
			}
	
			log::info!("Sent success: {:?}", res);
			// Transaction is sent successfully
			return Ok(());
		}
		log::info!("No local account available");
		Err(DispatchError::from(<Error<T>>::NoLocalAcctForSigning))
	}

}
