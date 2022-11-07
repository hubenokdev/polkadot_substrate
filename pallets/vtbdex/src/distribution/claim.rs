//! Module name: claim
//! Description: It has code implementation to claim distributable assets such as VTBC/ETH/EOS after completion of distribution period
use sp_runtime::{DispatchResult, DispatchError};
use crate::{
    Pallet, Error, Config, Event,
    Encode, Decode, U256, Vec,
    Period, Distribution, ClaimToken, UserWallet,
    TokenType, Get,
    users::{WalletTrait, WalletUpdateReq},
};
/// ClaimTokenReq<T>: It take one generic argument for account field.
/// account will store ss58format account address
/// token_Type is the value for the enum TokenType declared in cryptos mod
#[derive( Encode, Decode, Debug, Default)]
pub(crate) struct ClaimTokenReq<T> {
    account: T,
	token_type: TokenType,
}

/// ClaimTokenReq implements one method 
impl<T> ClaimTokenReq<T> {
    ///fn new: It take two parameters as account and token_type
    /// And return value of Self type
    pub(crate) fn new(account: T, token_type: Option<TokenType>) -> ClaimTokenReq<T> {
        ClaimTokenReq {
            account,
            token_type: if let Some(token) = token_type {
                token 
            } else {
                TokenType::None
            },
        }
    }
}

/// ClaimTokenTrait<T: Config>: This is trait defined with all the methods for claim functionality.
pub trait ClaimTokenTrait<T: Config> {
    fn check_claim_distribution_initiate(&mut self) -> DispatchResult;
    fn initiate_claim_distribution(self) -> DispatchResult;
    fn initiate_claim_all_distribution(&mut self) -> DispatchResult;
    fn update_to_claim_storage(&self, amount: U256) -> DispatchResult;
    fn process_claim_for_all_available_closed_period(&self) -> Result<U256, Error<T>>;
    fn get_user_and_calculate_claim_amount(&self, distribution_index: u64 ) -> Result<U256, Error<T>>; 
    fn calculate_claim_amount(&self, distribution_index: u64, user_share: U256) -> Result<U256, Error<T>>;
}

impl<T: Config> ClaimTokenTrait<T> for ClaimTokenReq<T::AccountId> {

    /// fn check_claim_distribution_initiate: This function declared in ClaimTokenTrait.
    /// It takes one parameters of self object 
    /// It will calculate and process the claim for the given token_type
    /// calculate claim amount for all distributable token 
    /// Update the runtime state as per the claim process completed
    fn check_claim_distribution_initiate(&mut self) -> DispatchResult {

        let mut current_period = <Period<T>>::get();
        let account_key: Vec<u8> = self.account.encode();
        let res = WalletUpdateReq::new(None, &account_key, None, None);
        WalletTrait::<T, T::AccountId>::check_and_add_new_period_in_user(&res, current_period)?;

        if current_period == 0 {
            return Ok(())
        }
        else {
            current_period -= 1;
        }
        let account_key: Vec<u8> = self.account.encode();
        let user_sharable_assets = <UserWallet<T>>::get(account_key); //Always sharable assets will be vtbc
        // let dist_index = user_sharable_assets.processing_distribution_index.get(&TokenType::Vtbc).ok_or(Error::<T>::NumberIsTooLowForU256)?;

        // if user_data.to_update_period <= current_period && user_data.to_update_period >= dist_index {
        //if user_data.to_update_period <= current_period && user_data.to_update_period >= dist_index {
        for assets in TokenType::_distributable_iterator() {
            let dist_index = user_sharable_assets.processing_distribution_index.get(assets).ok_or(Error::<T>::NumberIsTooLowForU256)?;
            if dist_index <= &current_period {
            // for assets in TokenType::_distributable_iterator() {
                self.token_type = *assets;
                let mut user_data = <ClaimToken<T>>::get(&self.account, assets);
                if &user_data.to_update_period < dist_index {
                    user_data.to_update_period = *dist_index;
                }
                if &user_data.to_update_period >= dist_index {
                    let res = ClaimTokenTrait::<T>::get_user_and_calculate_claim_amount(self, user_data.to_update_period)?;
                    ClaimTokenTrait::<T>::update_to_claim_storage(self, res)?;
                    ClaimToken::<T>::mutate(&self.account, assets, |user| {
                        user.to_update_period = user_data.to_update_period + 1;
                    });
                }
            }
        }

        Ok(())
    }

    /// fn initiate_claim_distribution: This function declared in ClaimTokenTrait.
    /// It takes one parameters of self object 
    /// It will calculate and process the claim for the given token_type
    /// calculate claim amount for all distributable token 
    /// Update the runtime state as per the claim process completed
    fn initiate_claim_distribution(self) -> DispatchResult {
        let account_key: Vec<u8> = self.account.encode();
        let req = WalletUpdateReq::new(None, &account_key, None, None );
        WalletTrait::<T, T::AccountId>::check_to_pay_trnx_fee_based_on_crpto_available_with_account(&req)?;

        let current_period = <Period<T>>::get();
        let _ = WalletTrait::<T, T::AccountId>::check_and_add_new_period_in_user(&req, current_period);
        let amount = ClaimTokenTrait::<T>::process_claim_for_all_available_closed_period(&self)?;

        match <Pallet<T>>::add_update_balance(&self.token_type, &self.account.encode(), amount, U256::from(0_u8)) {
            Ok(()) => {  
                WalletTrait::<T, T::AccountId>::pay_trnx_fee_based_on_crpto_available_with_account(&req, &self.account, "Claim crypto")?;
                <Pallet<T>>::add_circulation_token_balance(&self.token_type, amount)?;
                <Pallet<T>>::deposit_event(Event::ClaimedSuccess {
                    user: self.account, 
                    token_type: self.token_type, 
                    claimed_amount: amount 
                });
                
                Ok(())
            }
            Err(err) => Err(DispatchError::from(err))
        }
    }

    /// fn initiate_claim_all_distribution: This function declared in ClaimTokenTrait.
    /// It takes one parameters of self object 
    /// It iterate over all distributable token iterator defined in TokenType enum
    /// calculate claim amount for all distributable token 
    /// Update the runtime state as per the claim process completed
    fn initiate_claim_all_distribution(&mut self) -> DispatchResult {
        let account_key: Vec<u8> = self.account.encode();
        let req = WalletUpdateReq::new(None, &account_key, None, None);
        let current_period = <Period<T>>::get();
        let _ = WalletTrait::<T, T::AccountId>::check_and_add_new_period_in_user(&req, current_period);
        for assets in TokenType::_distributable_iterator() {
            self.token_type = *assets;
            let amount = ClaimTokenTrait::<T>::process_claim_for_all_available_closed_period(self)?;
            if amount > U256::from(0_u8) {
                WalletTrait::<T, T::AccountId>::pay_trnx_fee_based_on_crpto_available_with_account(&req, &self.account, "Claim crypto")?;
                match <Pallet<T>>::add_update_balance(assets, &self.account.encode(), amount, U256::from(0_u8)) {
                    Ok(()) => {
                        <Pallet<T>>::add_circulation_token_balance(assets, amount)?;
                        <Pallet<T>>::deposit_event(Event::ClaimedSuccess {
                            user: self.account.clone(), 
                            token_type: *assets, 
                            claimed_amount: amount 
                        });
                    }
                    Err(err) => {
                        log::error!("initiate_claim_all_distribution ERROR: {:?}", err);
                        return  Err(DispatchError::from(err));
                    }
                }
           }
            ClaimTokenTrait::<T>::update_to_claim_storage(self, U256::from(0_u8))?;
        }
	
       Ok(())
    }

    /// fn update_to_claim_storage: This function declared in ClaimTokenTrait.
    /// It takes two parameters of self object & amount
    /// It update the Tobe Claim state of the storage
    /// This storage is used via unsigned extrinsic to show to user that how much amount is available for claim.
    fn update_to_claim_storage(&self, amount: U256) ->  DispatchResult {

        let account_key: Vec<u8> = self.account.encode();
        let user_sharable_assets = <UserWallet<T>>::get(&account_key);
        let user_distribution_index = user_sharable_assets.processing_distribution_index.get(&self.token_type).ok_or(Error::<T>::NumberIsTooLowForU256)?;
        ClaimToken::<T>::mutate(&self.account, &self.token_type, |user| {
            user.token_amount = amount;
            user.to_update_period = *user_distribution_index;
        });	

        Ok(())
    }

    /// fn process_claim_for_all_available_closed_period: This function declared in ClaimTokenTrait.
    /// It takes one parameters of self object 
    /// It iterate over last unclaimed period to latest closed period.
    /// It calculate the claim amount for all closed unclaimed period for the given user
    /// And update the Distribution runtime storage balance of the token
    /// and return the total_claim amount.
    fn process_claim_for_all_available_closed_period(&self) -> Result<U256, Error<T>> {
        let mut amount = U256::from(0_u8);
        let current_period = <Period<T>>::get();
        let account_key: Vec<u8> = self.account.encode();
        let user = <UserWallet<T>>::get(&account_key);

       // let mut distribution_index = *user.processing_distribution_index.get(&self.token_type).ok_or(Error::<T>::NumberIsTooLowForU256)?;
        let mut distribution_index = if let Some(x) = user.processing_distribution_index.get(&self.token_type) {
            *x
        } else {
            0
        };

        while distribution_index < current_period {
            if !<Distribution<T>>::get(distribution_index).closed { break; }

            let claim_amount = ClaimTokenTrait::<T>::get_user_and_calculate_claim_amount(self, distribution_index)?;
            log::info!("fn process_claim_for_all_available_closed_period: claim_amount: {}", claim_amount);
           
            if claim_amount > U256::from(0_u8) {
                Distribution::<T>::mutate(distribution_index, |distribute| -> Result<(), Error<T>>{
                    let bal = distribute.balances.get_mut(&self.token_type).ok_or(Error::<T>::NoneValue)?;
                    bal.current_balance = bal.current_balance.checked_sub(claim_amount).ok_or(Error::<T>::NoneValue)?;
                    Ok(())
                })?;
                amount = amount.checked_add(claim_amount).ok_or(Error::<T>::NoneValue)?;
            }

          
            UserWallet::<T>::mutate(&account_key, |user_data| {
                if let Some(balance) = user_data.vtbc_period_balance.get_mut(&distribution_index) {
                    balance.counter += 1;
                }  
                if let Some(balance) = user_data.vtbc_period_balance.get_mut(&distribution_index) {
                    if balance.counter == TokenType::_distributable_iterator().len() as u32 {
                        user_data.vtbc_period_balance.remove(&distribution_index);
                    }
                }
                distribution_index += 1;
                if let Some(x) = user_data.processing_distribution_index.get_mut(&self.token_type) {
                    *x = distribution_index;
                }
                else {
                    //This to handle new upcoming cryptos such as Usdc/Usdt/Dot
                    user_data.processing_distribution_index.insert(self.token_type, distribution_index); 
                }
                if user_data.crypto_addresses.get(&self.token_type).is_none() && self.token_type != TokenType::Vtbc && self.token_type != TokenType::Vtbt {
                    user_data.update_crypto_details(self.token_type, None);
                } 
                 
            });
    
        }
       
        let _ = ClaimTokenTrait::<T>::update_to_claim_storage(self, U256::from(0_u8));

        Ok(amount)
    }

    /// fn get_user_and_calculate_claim_amount: This function declared in ClaimTokenTrait.
    /// It takes two parameters, one as self object and second one u64 value 
    /// It calculate the available claim for the user in the provided distribution index
    /// For calculate claim the basic formula is:
    /// For a specific period
    /// user_vtbc_balance(controlled+vtbc hold) * (distributable_token / total_circulating_vtbc(which includes(total_vtbc + total_controlled)))
    /// The method return the calculated claim amount
    fn get_user_and_calculate_claim_amount(&self, distribution_index: u64 ) -> Result<U256, Error<T>> {
        let account_key: Vec<u8> = self.account.encode();
        let user_sharable_assets = <UserWallet<T>>::get(&account_key); //Always sharable assets will be vtbc
        //let ub = user_sharable_assets.vtbc_period_balance.get(&distribution_index).ok_or(Error::<T>::NoneValue)?;
        let ub = if let Some(x) = user_sharable_assets.vtbc_period_balance.get(&distribution_index) {
            *x
        } else {
            crate::users::types::Balances::new()
        };

        let shares = ub.balance.checked_add(ub.controlled).ok_or(Error::<T>::NumberIsTooBigForU256)?;

        ClaimTokenTrait::<T>::calculate_claim_amount(self, distribution_index, shares)
    }

    /// fn calculate_claim_amount: This function declared in ClaimTokenTrait.
    /// It takes two parameters, one as self object and second one u64 value 
    /// It calculate the available claim for the user in the provided distribution index
    /// For calculate claim the basic formula is:
    /// For a specific period
    /// user_vtbc_balance(controlled+vtbc hold) * (distributable_token / total_circulating_vtbc(which includes(total_vtbc + total_controlled)))
    /// The method return the calculated claim amount
    fn calculate_claim_amount(&self, distribution_index: u64 ,user_share: U256) -> Result<U256, Error<T>> {
        let mut amount = U256::from(0_u8);

        let distribute = <Distribution<T>>::get(&distribution_index);

        if distribute.closed {
            let denominator = distribute.denominator;
            let distro_amt = distribute.balances.get(&self.token_type).ok_or(Error::<T>::NoneValue)?.total_balance;
    
            //Reason for none value
            if distro_amt > U256::from(0_u8) && denominator > U256::from(0_u8) {
                let global_ratio = distro_amt.checked_mul(T::CryptoPrecision::get()).ok_or(Error::<T>::NoneValue)?.checked_div(denominator).ok_or(Error::<T>::NoneValue)?;
                amount = user_share.checked_mul(global_ratio).ok_or(Error::<T>::NoneValue)?.checked_div(T::CryptoPrecision::get()).ok_or(Error::<T>::NoneValue)?;
            } 
            log::debug!("denominator: {}", denominator);
            log::debug!("distro_amt: {}, shares: {}", distro_amt, user_share); 
            log::debug!(" amount: {}", amount);       
        }
       
        Ok(amount)
    }
}
