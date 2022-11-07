//!module User - This module have all the methods related to update of User's balance.
use sp_std::{ prelude::*, str};
use sp_runtime::{DispatchResult, DispatchError};
use codec::{Encode};
use scale_info::prelude::string::String;
use crate::{
    TokenType,
    VtbdexTransactionFee, UserWallet,
    Config, Error, Pallet, Event,
    U256, ensure, users, Vec,
    users::types::UserWalletDetail, UserCryptoBasedData
};

///WalletUpdateReq - This is custom struct type defined with 4 parameters.
/// account, tokenType, amount and controlled
pub struct WalletUpdateReq<'a> {
    pub account: &'a Vec<u8>,
    pub token_type: &'a TokenType, 
    pub amount: U256,
    pub controlled: U256,
}

impl<'a> WalletUpdateReq<'a> 
where {
    
    ///fn new(token_type: Option<&'a TokenType>, account: &'a T,  amount: Option<U256>, controlled: Option<U256>)
    /// This function is initializer for struct WalletUpdateReq<'a, T>.
    pub(crate) fn new(token_type: Option<&'a TokenType>, 
                    account: &'a Vec<u8>, 
                    amount: Option<U256>, 
                    controlled: Option<U256>) -> WalletUpdateReq<'a> {
        
        WalletUpdateReq {
            account,
            token_type: if let Some(token) = token_type {
                    token 
                } else {
                    &TokenType::None
                },
            amount: if let Some(amt) = amount {
                amt 
                } else {
                    U256::from(0_u8)
                },
            controlled: if let Some(amt) = controlled {
                amt 
                } else {
                    U256::from(0_u8)
                },
        }
    }
}

// Defining a Details trait by defining the functionality it should include
pub trait WalletTrait<'a, T, AccountId> {
    fn add_user_wallet_balance(&self, period: u64) -> DispatchResult;
    fn sub_user_wallet_balance(&self, period: u64) -> DispatchResult;
    fn check_and_add_new_period_in_user(&self, current_period: u64) -> DispatchResult; 
    fn check_to_pay_fee_for_given_crypto(&self) -> DispatchResult; 
    fn pay_trnx_fee_in_given_crypto(&self, user_address: &AccountId, fee_type: &str) -> DispatchResult;
    fn pay_trnx_fee_based_on_crpto_available_with_account(&self, user_address: &AccountId, fee_type: &str) -> DispatchResult;
    fn check_to_pay_trnx_fee_based_on_crpto_available_with_account(&self) -> Result<U256, Error<T>>;
}

impl<'a, T> WalletTrait<'a, T, T::AccountId> for WalletUpdateReq<'a> 
where 
T: Config {

    ///fn sub_user_wallet_balance -> This function takes the argument of self reference.
    ///Based on match condition, it subtract and update the balance of user in Wallet<T> runtime storage defined in pallet vtbdex.
    fn add_user_wallet_balance(&self, period: u64) -> DispatchResult {
        // let account_key: Vec<u8> = self.account.encode();
        match self.token_type {
			TokenType::Vtbc => {
                UserWallet::<T>::mutate(&self.account, |user_record| -> DispatchResult {
                    user_record.vtbc_balance = user_record.vtbc_balance.checked_add(self.amount).ok_or(Error::<T>::NumberIsTooLowForU256)?;
                    let user_vtbc_period_bal = user_record.vtbc_period_balance.get_mut(&period).ok_or(Error::<T>::UserPeriodBalDoesNotExist)?;
                    user_vtbc_period_bal.balance = user_vtbc_period_bal.balance.checked_add(self.amount).ok_or(Error::<T>::NumberIsTooBigForU256)?;
                    if self.controlled > U256::from(0_u8) {
                        user_record.sells_journal_balance = user_record.sells_journal_balance.checked_add(self.controlled).ok_or(Error::<T>::NumberIsTooLowForU256)?;
                        user_vtbc_period_bal.controlled = user_vtbc_period_bal.controlled.checked_add(self.controlled).ok_or(Error::<T>::NumberIsTooBigForU256)?;
                    }
                    
                    Ok(())
                })?;
			},
			TokenType::Vtbt => {
                UserWallet::<T>::mutate(&self.account, |user_record| -> DispatchResult {
                    user_record.vtbt_balance = user_record.vtbt_balance.checked_add(self.amount).ok_or(Error::<T>::NumberIsTooLowForU256)?;
                    Ok(())
                })?;
			},
            TokenType::None => log::error!("None type"),
            _ => {
                UserWallet::<T>::mutate(&self.account, |user| -> DispatchResult{
                    let each_crypto = user.crypto_addresses.get_mut(self.token_type).ok_or(Error::<T>::UserDoesNotHaveLinkedCryptoAddress)?;
                    log::info!("each_crypto: {:?}", each_crypto);

                    each_crypto.deposit_balance = each_crypto.deposit_balance.checked_add(self.amount).ok_or(Error::<T>::NumberIsTooLowForU256)?;
                    log::info!("each_crypto: {:?}", each_crypto.deposit_balance);
    // assert!(1 == 2 , "user: {:?}", self.amount);


                    if self.controlled > U256::from(0_u8) {
                        each_crypto.buy_journal_balance = each_crypto.buy_journal_balance.checked_add(self.controlled).ok_or(Error::<T>::NumberIsTooLowForU256)?;
                    }
                    Ok(())
                })?;
			},
		}

        Ok(())
	}

    ///fn sub_user_wallet_balance -> This function takes the argument of self reference.
    ///Based on match condition, it subtract and update the balance of user in Wallet<T> runtime storage defined in pallet vtbdex.
    fn sub_user_wallet_balance(&self, period: u64) -> DispatchResult {
        // let account_key: Vec<u8> = self.account.encode();
        match self.token_type {
			TokenType::Vtbc => {
                UserWallet::<T>::mutate(&self.account, |user_record| -> DispatchResult {
                    user_record.vtbc_balance = user_record.vtbc_balance.checked_sub(self.amount).ok_or(Error::<T>::NumberIsTooLowForU256)?;
                    let user_vtbc_period_bal = user_record.vtbc_period_balance.get_mut(&period).ok_or(Error::<T>::UserPeriodBalDoesNotExist)?;
                    user_vtbc_period_bal.balance = user_vtbc_period_bal.balance.checked_add(self.amount).ok_or(Error::<T>::NumberIsTooBigForU256)?;
                    if self.controlled > U256::from(0_u8) {
                        user_record.sells_journal_balance = user_record.sells_journal_balance.checked_sub(self.controlled).ok_or(Error::<T>::NumberIsTooLowForU256)?;
                        user_vtbc_period_bal.controlled = user_vtbc_period_bal.controlled.checked_add(self.controlled).ok_or(Error::<T>::NumberIsTooBigForU256)?;
                    }
                    
                    Ok(())
                })?;
			},
			TokenType::Vtbt => {
                UserWallet::<T>::mutate(&self.account, |user_record| -> DispatchResult {
                    user_record.vtbt_balance = user_record.vtbt_balance.checked_sub(self.amount).ok_or(Error::<T>::NumberIsTooLowForU256)?;
                    Ok(())
                })?;
			},
            TokenType::None => log::error!("None type"),
            _ => {
                UserWallet::<T>::mutate(&self.account, |user| -> DispatchResult{
                    let each_crypto = user.crypto_addresses.get_mut(self.token_type).ok_or(Error::<T>::UserDoesNotHaveLinkedCryptoAddress)?;
                    each_crypto.deposit_balance = each_crypto.deposit_balance.checked_sub(self.amount).ok_or(Error::<T>::NumberIsTooLowForU256)?;
                    if self.controlled > U256::from(0_u8) {
                        each_crypto.buy_journal_balance = each_crypto.buy_journal_balance.checked_sub(self.controlled).ok_or(Error::<T>::NumberIsTooLowForU256)?;
                    }
                    Ok(())
                })?;
			},
		}

        Ok(())
	}

    ///fn check_to_pay_fee_for_given_crypto -> This function takes the argument of self reference.
    ///Calculate the substrate fee based on provided crypto_type 
    ///And than validate in user wallet has sufficient balance to pay the fee or not.
    ///If not than throw error. Error leads to fail the transaction.
    fn check_to_pay_fee_for_given_crypto(&self) -> DispatchResult {
        log::info!("======================= Check to pay eth as transaction fee =======================================");
        // let from_user = <UserWallet<T>>::get(self.account);
        // let crypto_address =  from_user.crypto_addresses.get(self.token_type).ok_or(Error::<T>::InvalidTokenType)?;

        let fee_amt = <Pallet<T>>::check_and_calculate_fee(self.token_type)?;	
        let (_, deposit_balance, _, _) = WalletStorage::<T>::get_wallet_detail_balance(self.account, self.token_type)?;

		ensure!(deposit_balance >= fee_amt, Error::<T>::InsufficientFundsToPayFee);
        
        Ok(())
    }

    ///fn pay_trnx_fee_in_given_crypto -> This function takes the argument of self reference.
    ///Calculate the substrate fee based on provided crypto_type 
    ///And than validate in user wallet has sufficient balance to pay the fee or not.
    ///If not than throw error. Error leads to fail the transaction.
    ///If ensure passes than substract the fee from user account and credit the fee in the fee-collector account.
    fn pay_trnx_fee_in_given_crypto(&self, user_address: &T::AccountId, fee_type: &str) -> DispatchResult {

        log::info!("======================= Pay eth as transaction fee =======================================");
        let (_, deposit_balance, _, _) = WalletStorage::<T>::get_wallet_detail_balance(self.account, self.token_type)?;
        // let from_user = <UserWallet<T>>::get(self.account);
        // let crypto_address =  from_user.crypto_addresses.get(self.token_type).ok_or(Error::<T>::InvalidTokenType)?;
        let fee_amt = <Pallet<T>>::check_and_calculate_fee(self.token_type)?;
		ensure!(deposit_balance >= fee_amt, Error::<T>::InsufficientFundsToPayFee);	
        //self.amount = fee_amt;
        let fee_collector = <VtbdexTransactionFee<T>>::get().ok_or(Error::<T>::NoneValue)?;
        Pallet::<T>::sub_update_balance(self.token_type, self.account, fee_amt, U256::from(0_u8))?;
        Pallet::<T>::add_update_balance(self.token_type, &fee_collector.fee_collector_address.encode(), fee_amt, U256::from(0_u8))?;
		Pallet::<T>::deposit_event(Event::TransactionSuccessFee { 
			user: user_address.clone(), 
			reason: fee_type.as_bytes().to_vec(), 
			token_type: *self.token_type, 
			amount: fee_amt 
		});

        Ok(())
    }

    ///fn check_and_add_new_period_in_user -> This function takes the argument of self reference & u64.
    ///This function is responsible to create a storage space for the user for as specific period.
    ///In the function if condition restrict the execution to only once in a distribution period(a month).
    fn check_and_add_new_period_in_user(&self, current_period: u64) -> DispatchResult {
       // let account_key: Vec<u8> = self.account.encode();

        let user_wallet = UserWallet::<T>::get(&self.account);
        if user_wallet.latest_period == current_period {
            return Ok(());
        }
        for period in user_wallet.latest_period+1..current_period+1 {
            UserWallet::<T>::mutate(&self.account, |user| -> DispatchResult {
                let user_balance_last_period = user.vtbc_period_balance.get(&(period-1)).ok_or(Error::<T>::UserPeriodBalDoesNotExist)?;
                user.vtbc_period_balance.insert(period, *user_balance_last_period);
                user.latest_period = period;
                Ok(())
            })?;
        }
        Ok(())
    }

    ///fn pay_trnx_fee_based_on_crpto_available_with_account -> This function takes the argument of self reference & &str .
    ///Calculate the substrate fee with linked cryptos with sufficient balance to pay fee. 
    ///And than validate in user wallet has sufficient balance to pay the fee or not.
    ///If ensure passes than substract the fee from user account and credit the fee in the fee-collector account.
    fn pay_trnx_fee_based_on_crpto_available_with_account(&self, user_address: &T::AccountId, fee_type: &str) -> DispatchResult {

        log::info!("======================= Check & pay transaction fee =======================================");
        let from_user = <UserWallet<T>>::get(&self.account);

        let fee_collector = <VtbdexTransactionFee<T>>::get().ok_or(Error::<T>::NoneValue)?;
        let mut _flag = false;
        for asset in TokenType::_crypto_iterator() {
            match from_user.crypto_addresses.get(asset) {
                Some(crypto_address) => {
                    log::info!("======================= Check to pay eth as transaction fee =======================================");
                    let fee_amt = Pallet::<T>::convert_usd_to_crypto(fee_collector.fee, asset)?;
                    if crypto_address.deposit_balance >= fee_amt {
                        Pallet::<T>::sub_update_balance(asset, self.account, fee_amt, U256::from(0_u8))?;
                        Pallet::<T>::add_update_balance(asset, &fee_collector.fee_collector_address.encode(), fee_amt, U256::from(0_u8))?;
                        Pallet::<T>::deposit_event(Event::TransactionSuccessFee { 
                            user: user_address.clone(), 
                            reason: fee_type.as_bytes().to_vec(), 
                            token_type: *asset, 
                            amount: fee_amt 
                        });

                        _flag = true;
                        return Ok(());
                    }   
                },
                None => {
                  log::debug!("None for {:?} crypto: InsufficientFundsToPayFee", asset);
                  continue;
                }
            };
        };

        // This ensure to fail extrinsic if no crypto balance is pay to fee
        ensure!(_flag, Error::<T>::InsufficientFundsToPayFee);
        Err(DispatchError::from(Error::<T>::InsufficientFundsToPayFee))
    }

    ///fn check_to_pay_trnx_fee_based_on_crpto_available_with_account -> This function takes the argument of self reference.
    ///Calculate the substrate fee with linked cryptos with sufficient balance to pay fee. 
    ///And than validate in user wallet has sufficient balance to pay the fee or not.
    ///If in user wallet any of the linked crypto does not have balance to pay fee, than it will throw error which leads to fail
    ///the transaction due to ensure!(_) condition.
    fn check_to_pay_trnx_fee_based_on_crpto_available_with_account(&self) -> Result<U256, Error<T>> {

        log::info!("======================= Check transaction fee =======================================");
        let from_user = <UserWallet<T>>::get(&self.account);

        let fee_collector = <VtbdexTransactionFee<T>>::get().ok_or(Error::<T>::NoneValue)?;
        let mut _flag = false;
        for asset in TokenType::_crypto_iterator() {
            match from_user.crypto_addresses.get(asset) {
                Some(crypto_address) => {
                    log::info!("======================= Check And pay transaction fee =======================================");
                    let fee_amt = Pallet::<T>::convert_usd_to_crypto(fee_collector.fee, asset)?;
                    if crypto_address.deposit_balance >= fee_amt {
                        _flag = true;
                        return Ok(fee_amt);
                    }   
                },
                None => {
                  log::debug!("None for {:?} crypto: InsufficientFundsToPayFee", asset);
                  continue;
                }
            };
        };
        // This ensure to fail extrinsic if no crypto balance is pay to fee
        ensure!(_flag, Error::<T>::InsufficientFundsToPayFee);
        Err(Error::<T>::InsufficientFundsToPayFee)
    }
}

pub struct WalletStorage<T>(T);

impl<T: Config> WalletStorage<T> {
    pub fn is_user_exist(account: &T::AccountId) -> bool {
        <UserWallet<T>>::contains_key(&account.encode())
    }

    pub fn insert_new_wallet_record(account: &Vec<u8>, crypto_address: &Vec<u8>, mut info: crate::users::types::User) {
        if <UserWallet<T>>::contains_key(crypto_address) {
            let wallet = <UserWallet<T>>::get(crypto_address);
            for (key, value) in wallet.crypto_addresses {
               if !info.crypto_addresses.contains_key(&key) {
                let mut crypto_detail = UserCryptoBasedData::new_for_none_crypto_address(key);
                crypto_detail.crypto_address = value.crypto_address;
                info.crypto_addresses.insert(key, crypto_detail);
               }
            }
        }
        <UserWallet<T>>::insert(account, info);
    }

    ///fn append_or_replace_user_wallet_crypto(record: &CryptoDetail, account: &T::AccountId, token_type: &TokenType)
    ///This function insert the detail of new linked cryptos.
    ///UserCryptoBasedData is struct type defined in custom_types.rs as below
    /// pub struct UserCryptoBasedData {
    /// //Name of the crypto network Eth/Eos/Usdc
	/// pub crypto_network: Vec<u8>,
	/// //linked crypto_address/crypto_Account_name, if it is not linked yet than None
	/// pub crypto_address: Option<Vec<u8>>,
	/// //Crypto deposit balance
	/// pub deposit_balance: U256,
	/// //Deposit Amount present in buy journal
	/// pub buy_journal_balance: U256,
	/// //List of sell/buy journal with orderid -> index
	/// pub order: BTreeMap<TradeType, BTreeMap<Vec<u8>, U256>>,   
    /// }
    pub fn append_or_replace_user_wallet_crypto(record: &UserCryptoBasedData, account: &T::AccountId, token_type: TokenType) -> DispatchResult {
		UserWallet::<T>::mutate(account.encode(), |user_record| -> DispatchResult {
            if let Some(address_detail) = user_record.crypto_addresses.get_mut(&token_type) {
                if address_detail.crypto_address.is_none() {
                    address_detail.crypto_address = record.crypto_address.clone();
                }
                else {
                    return Err(frame_support::dispatch::DispatchError::from("CryptoAddressAlreadyLinked"));  
                }
            }
            else {
                user_record.crypto_addresses.insert(token_type, record.clone());
            }
            
            Ok(())
		})
	}

    pub fn get_wallet_record(account: &Vec<u8>) -> crate::users::types::User {
        <UserWallet<T>>::get(&account)
    }

    pub fn get_registered_crypto_address(account: &Option<T::AccountId>, token_type: &TokenType) -> Result<String, Error<T>> {

        if let Some(id) = account {
            let user_wallet = <UserWallet<T>>::get(&id.encode());
            let linked_crypto_address = match user_wallet.crypto_addresses.get(token_type) {
                Some(val) => {
                    if let Some(address) = val.crypto_address.as_ref() {
                        str::from_utf8(address).unwrap_or("Error").to_owned()
                    }
                    else {
                        "".to_owned() 
                    }
                },
                None => {"".to_owned()}
            };	

            Ok(linked_crypto_address)	
        }
        else {
            Ok("".to_owned())
        }		
	}

    pub fn get_wallet_detail_balance(account_key: &Vec<u8>, token_type: &TokenType) -> Result<UserWalletDetail, Error<T>> {
        let user = UserWallet::<T>::get(account_key);
        let crypto_details =  user.crypto_addresses.get(token_type);
        match crypto_details {
            Some(val) => Ok((val.crypto_address.clone(), val.deposit_balance, user.active, user.vtbc_balance)),
            None => {
                Ok((None, U256::from(0), user.active, user.vtbc_balance))
            }
        } 
    }
}


