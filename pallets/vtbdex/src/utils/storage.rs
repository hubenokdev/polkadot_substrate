
//!module storage - This module will have methods of storage update balance
use crate::{
    Config, Pallet, Error,
    Circulation, U256, TokenType, BlockedUserWallet,
    Vec,
};
use sp_runtime::DispatchResult;

impl<T: Config> Pallet<T> {

    ///fn add_circulation_token_balance(token_type: &TokenType, token_amt: U256)
    ///This function add the circulation balance of given cryptos.
    ///It add and update the balance in Circulation<T> runtime storage.
    pub fn add_circulation_token_balance(token_type: &TokenType, token_amt: U256) -> DispatchResult{
        Circulation::<T>::mutate(token_type, |get_circulation_value| -> DispatchResult {
            *get_circulation_value = get_circulation_value.checked_add(token_amt).ok_or(Error::<T>::NoneValue)?;

            Ok(())
        })?;

        Ok(())
    }

    ///fn sub_circulation_token_balance(token_type: &TokenType, token_amt: U256)
    ///This function subtract the circulation balance of given cryptos.
    ///It subtract and update the balance in Circulation<T> runtime storage.
    pub fn sub_circulation_token_balance(token_type: &TokenType, token_amt: U256) -> DispatchResult {
        Circulation::<T>::mutate(token_type, |get_circulation_value| -> DispatchResult {
            // this amount will go for dostribution
            *get_circulation_value = get_circulation_value.checked_sub(token_amt).ok_or(Error::<T>::NoneValue)?; 

            Ok(())
        })?;

        Ok(())
    }

    /// fn update_blocked_user_state_balance(user_pdot_address: T::AccountId, transaction_hash: Vec<u8>) -> DispatchResult
    ///Check and if exist user with the given transact_hash in BlockedUser list, 
    ///Remove the list
    pub fn update_blocked_user_state_balance(user_pdot_address: &T::AccountId, transaction_hash: &Vec<u8>) -> DispatchResult {
        BlockedUserWallet::<T>::mutate(&user_pdot_address, |list| -> DispatchResult {
            let list_mut = list.as_mut().ok_or(crate::Error::<T>::NoneValue)?;
            match list_mut.iter().position(|u| &u.transaction_hash == transaction_hash ) {
                Some(index) => {
                    list_mut.remove(index);
                    Ok(())
                },
                None =>  Err(frame_support::dispatch::DispatchError::from("User does not exist in blocked user list"))
            }
        })
    }
}