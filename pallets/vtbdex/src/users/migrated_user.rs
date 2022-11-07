//!module migrated_user -> This module have all the methods which will maintain the migrated-user balance of hodldex.
//!Hodldex migrated user/seller have balance of eth/eos/vtbc/controlled.
use crate::{Config, Error, Pallet,
    U256, Vec, Encode,
    ClaimTokenReq, ClaimTokenTrait,
    TokenType, TradeType, trade::types::TradeRequest, 
    OrderBookNMap,  Distribution, UserWallet, Period,
};
use sp_runtime::{DispatchResult, DispatchError};

pub struct MigratedUsers<T>(T);

impl<T: Config> MigratedUsers<T> {

    // /// fn update_migrated_user_balance ->  This function is public to the crate.
    // /// It take six argument (TokenType, AccountId, CryptoAddress, U256, U256, U256)
    // /// If the fulfilluse crate::Wallet; if seller is default address than this sell order is migrated one.
    // /// So linked the crypto_amount/VTBC with the crypto address for the migrated user.
    // pub(crate) fn update_migrated_seller_balance(req: &TradeRequest<T::AccountId>, token_type: TokenType) -> DispatchResult {

    //     let default_pdot_address = "5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM";
    //     let account_id = <Pallet<T>>::convert_str_to_valid_account_id(default_pdot_address).map_err(|e| {
    //         log::info!("UNable to parse: {:?}", e);
    //         <Error<T>>::NoneValue
    //     })?;

    //     if req.address == account_id {
    //         log::info!("migrated user");
    //         let address = req.crypto_address.clone().ok_or(Error::<T>::NoneValue)?;
    //         <Pallet<T>>::sub_update_balance(&TokenType::Vtbc, &address, U256::from(0_u64), req.controlled_amt)?;
    //         <Pallet<T>>::add_update_balance(&token_type, &address, req.crypto_amt, U256::from(0_u64))?;
    //     }

    //     Ok(())
    // }

    /// fn add_and_update_migrated_balance: It takes 4 parameters.
    /// This function is reponsible to update the migrated balance of the hodldex users.
    /// This function is invoked once in a user life cycle and check if the given cryoto_address is migrated 
    /// than it migrate the sell orders in user's account and migrate and update all token balance.
    pub(crate) fn add_and_update_migrated_balance(account: &T::AccountId, 
        token_type: TokenType, 
        crypto_address: &Vec<u8>, 
        current_period: u64) 
    -> DispatchResult {

        if UserWallet::<T>::contains_key(&crypto_address) {

            Self::calculate_and_process_automatic_claim_balance(crypto_address, account, current_period)?;
            Self::check_and_add_migrated_sell_order(account.clone(), crypto_address, &token_type)?;
            let user_val = UserWallet::<T>::get(crypto_address);

            <Pallet<T>>::add_update_balance(&TokenType::Vtbc, &account.encode(), user_val.vtbc_balance, U256::from(0_u8))?;
            <Pallet<T>>::add_update_balance(&TokenType::Vtbt, &account.encode(), user_val.vtbt_balance, U256::from(0_u8))?;
            // To handle claim for all non linked cryptos

            for (key, value) in user_val.crypto_addresses {
                log::info!("key 57::: = {:?}=====value=={:?}", key, value);

                <Pallet<T>>::add_update_balance(&key, &account.encode(), value.deposit_balance, U256::from(0_u8))?;
            }
            <UserWallet<T>>::remove(&crypto_address);
        }

        Ok(())
    }

    /// fn check_and_add_migrated_sell_order -> This fuction is public.
    /// This method will be call by eth-vtb-pallet and eos-vtb-pallet, so it is kept as public.
    /// This method is responsible to mutate existing OrderBook for migrated user during their onboarding process.
    /// It takes three parameters (AccountId, crypto_address, TokenType)
    /// This function will update the ss58 address in the orderbook via address linked with crypto.
    /// Intially the migrated OrderBook will have default address 5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM as seller address
    fn check_and_add_migrated_sell_order(account: T::AccountId, 
        crypto_address: &Vec<u8>, 
        crypto_type: &TokenType) -> DispatchResult {

        if <UserWallet<T>>::contains_key(&crypto_address) {
            let user_data = &<UserWallet<T>>::get(&crypto_address);  
            // let user_crypto_data = user_data.crypto_addresses.get(crypto_type).ok_or(Error::<T>::NoneValue)?; 

            if let Some(user_crypto_data) = user_data.crypto_addresses.get(crypto_type) {
                let default_address = <Pallet<T>>::convert_str_to_valid_account_id("5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM")?;
                if let Some(migrated_sell_orders) = user_crypto_data.order.get(&TradeType::Sell) {
                    let account_key: Vec<u8> = account.encode();
                    let _ = <UserWallet<T>>::mutate(account_key, |user_data| -> DispatchResult {
                        let user_crypto_data = user_data.crypto_addresses.get_mut(crypto_type).ok_or(Error::<T>::UserDoesNotHaveLinkedCryptoAddress)?;
                        user_crypto_data.order.insert(TradeType::Sell, migrated_sell_orders.clone());
        
                        Ok(())
                    })?;
                    let mut vtbc_amount: U256 =  U256::from(0_u64);
                    for (order_id, index) in migrated_sell_orders {
                        log::debug!("Orderid: {:?}, index: {:?}", order_id, index);
                        OrderBookNMap::<T>::mutate((&TradeType::Sell, 
                                                    &crypto_type, 
                                                    &index), |order_opt| -> DispatchResult {
                            let order = order_opt.as_mut().ok_or(Error::<T>::NoneValue)?;
                            order.address = Some(account.clone());
                            vtbc_amount = vtbc_amount.checked_add(order.amount).unwrap_or_default();
        
                            Ok(())
                        })?;
                    } 
                    let _ = <Pallet<T>>::add_update_balance(&TokenType::Vtbc, &account.encode(), U256::from(0_u64), vtbc_amount);
                    let _ = <Pallet<T>>::sub_update_balance(&TokenType::Vtbc, &default_address.encode(), U256::from(0_u64), vtbc_amount);      
                }  
            }
        }

        Ok(())
	}

    /// fn calculate_and_process_automatic_claim_balance - It take 3 parameter.
    /// This function is responsible to iterate iterate and claim balances for all periods from 0th to current closed period.
    /// It do the same caculation for all the distributable tokens.
    fn calculate_and_process_automatic_claim_balance(crypto_address: &Vec<u8>, 
        account: &T::AccountId, 
        current_period: u64, 
    ) -> Result<(), DispatchError> {

        if UserWallet::<T>::contains_key(&crypto_address) {
            let user_balance = UserWallet::<T>::get(&crypto_address);
            <Pallet<T>>::add_update_balance(&TokenType::Vtbc, &crypto_address, U256::from(0_u8), U256::from(0_u8))?;
                for assets in TokenType::_distributable_iterator() {
                    let mut distribution_index = *user_balance.processing_distribution_index.get(&assets).unwrap_or(&0);
                    let mut total_claim_amount = U256::from(0_u8);
                    while distribution_index < current_period {
                        let balances = user_balance.vtbc_period_balance.get(&distribution_index).ok_or(Error::<T>::NoneValue)?;
                        let user_shares = balances.balance.checked_add(balances.controlled).ok_or(Error::<T>::NumberIsTooBigForU256)?;
                        let claim_req = ClaimTokenReq::new(account.clone(), Some(*assets));
                        total_claim_amount += if let Ok(amt) = ClaimTokenTrait::<T>::calculate_claim_amount(&claim_req, distribution_index, user_shares) {
                            amt
                        } else {
                            U256::from(0_u8)
                        };

                        distribution_index += 1;
                    }
                    Self::migrated_user_automatic_distribution(distribution_index, current_period, total_claim_amount, assets, crypto_address)?;
                }
            Ok(())
        }
        else {
            Ok(())
        }
    }

    /// fn migrated_user_automatic_distribution(distribution_index: u64, claim_amount: U256, token_type: &TokenType, crypto_address: &Vec<u8>) -> DispatchResult;
    /// This function is responsible to update claim balance for the migrated-user of hodldex.
    /// It will update MigratedUser<T> runtime state which will have updated balance which includes claim amount.
    fn migrated_user_automatic_distribution(distribution_index: u64, 
        current_period: u64,
        claim_amount: U256, 
        token_type: &TokenType, 
        crypto_address: &Vec<u8>) -> DispatchResult {
        log::info!("claim_amount: {}=== distribution_index: {}========token_type: {}", claim_amount, distribution_index, token_type);

        if claim_amount > U256::from(0_u8) {
            Distribution::<T>::mutate(distribution_index, |distribute| -> DispatchResult{
                let bal = distribute.balances.get_mut(token_type).ok_or(Error::<T>::NoneValue)?;
                let updated_distributable_balance = bal.current_balance.checked_sub(claim_amount).ok_or(Error::<T>::NumberIsTooLowForU256)?;
                bal.current_balance = updated_distributable_balance;
                Ok(())
            })?;
        //ToDo!("Need t recheck automatic claim for crypotos");

            if token_type == &TokenType::Vtbc { 
                UserWallet::<T>::mutate(&crypto_address, |user_data| -> DispatchResult {
                    let processing_period_balance = *user_data.vtbc_period_balance.get_mut(&distribution_index).ok_or(Error::<T>::NoneValue)?;
                    processing_period_balance.balance.checked_add(claim_amount).ok_or(Error::<T>::NumberIsTooLowForU256)?;
                    let next_distributable_period = distribution_index;
                    *user_data.processing_distribution_index.get_mut(token_type).ok_or(Error::<T>::NoneValue)? = next_distributable_period;
                    Ok(())
                })?;
            }
            else { 
                UserWallet::<T>::mutate(&crypto_address, |user_data| -> DispatchResult {
                    if user_data.crypto_addresses.get(&token_type).is_none() && token_type != &TokenType::Vtbc && token_type != &TokenType::Vtbt {
                        user_data.update_crypto_details(*token_type, None);
                    };
                    let each_crypto = user_data.crypto_addresses.get_mut(token_type).ok_or(Error::<T>::UserDoesNotHaveLinkedCryptoAddress)?;
                    log::info!("each_crypto: {:?}", each_crypto);

                    each_crypto.deposit_balance = each_crypto.deposit_balance.checked_add(claim_amount).ok_or(Error::<T>::NumberIsTooLowForU256)?;
                    log::info!("each_crypto: {:?}", each_crypto.deposit_balance);
                    let next_distributable_period = distribution_index;
                    *user_data.processing_distribution_index.get_mut(token_type).ok_or(Error::<T>::NoneValue)? = next_distributable_period;
                    Ok(())
                })?;
            }
            <Pallet<T>>::add_circulation_token_balance(token_type, claim_amount)?;
        }
        Ok(())
    }
}