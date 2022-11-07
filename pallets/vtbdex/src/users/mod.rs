mod user;
mod migrated_user;
pub mod types;
pub use migrated_user::MigratedUsers;
pub use user::{WalletUpdateReq, WalletTrait, WalletStorage};
use sp_runtime::DispatchResult;
use crate::{
    Pallet, Error, Config, Encode,
    TokenType, U256, Period,
    Vec
};

impl<T: Config> Pallet<T> {

    ///fn add_update_balance(token_type: &TokenType, account: &T::AccountId, balance: U256, controlled: U256) -> Result<(), Error<T>>;
    ///This function is used to update Wallet<T> and User<T> runtime state to maintain user balance.
    pub fn add_update_balance(token_type: &TokenType, 
        account: &Vec<u8>, 
        balance: U256, 
        controlled: U256) -> Result<(), Error<T>> {

        let current_period = <Period<T>>::get();
        let res = WalletUpdateReq::new( Some(token_type), account, Some(balance), Some(controlled) );
        let _ = WalletTrait::<T, T::AccountId>::check_and_add_new_period_in_user(&res, current_period);
        log::info!("token: {:?}====={:?}", res.account, res.token_type);
        let _ = WalletTrait::<T, T::AccountId>::add_user_wallet_balance(&res, current_period);
        Ok(())
    }

    ///fn sub_update_balance(token_type: &TokenType, account: &T::AccountId, balance: U256, controlled: U256) -> Result<(), Error<T>>;
    ///This function is used to update Wallet<T> and User<T> runtime state to maintain user balance.
    pub fn sub_update_balance(token_type: &TokenType,
        account: &Vec<u8>,
        balance: U256, 
        controlled: U256) -> Result<(), Error<T>> {

        let current_period = <Period<T>>::get();
        let res = WalletUpdateReq::new( Some(token_type), account, Some(balance), Some(controlled) );
        let _ = WalletTrait::<T, T::AccountId>::check_and_add_new_period_in_user(&res, current_period);
        let _ = WalletTrait::<T, T::AccountId>::sub_user_wallet_balance(&res, current_period);
        Ok(())
    }

    ///fn check_and_add_new_period_in_user(current_period: u64, account: &T::AccountId);
    ///This function is used to create space for User<T> for the given period runtime state to maintain user balance.
    pub fn check_and_add_new_period_in_user(current_period: u64, account: &T::AccountId) {
        let account_key = account.encode();
        let res = WalletUpdateReq::new(None, &account_key, None, None );
        let _ = WalletTrait::<T, T::AccountId>::check_and_add_new_period_in_user(&res, current_period);
    }

    pub fn add_and_update_migrated_balance(account: &T::AccountId, 
        crypto_address: &Vec<u8>, 
        token_type: TokenType,
        current_period: u64) 
    -> DispatchResult {

        <MigratedUsers<T>>::add_and_update_migrated_balance(account, token_type, crypto_address, current_period)?;

        Ok(())
    }
}