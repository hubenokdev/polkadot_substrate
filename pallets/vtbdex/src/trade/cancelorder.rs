
//! Module Cancel Buy order
//! It contains trade features of vtb system.
//! CancelSell and CancelBuy order And Refund Crypto/Vtbc to buyer/Seller.
use sp_runtime::{DispatchError};
use codec::Encode;
use crate::{
    Vec, U256,
    Error, Pallet, Config, Event,
    trade::types::{TradeType, TradeRequest},
    {OrderBookNMap, UserWallet},
    TokenType,
    users::{WalletTrait, WalletUpdateReq},
};

pub struct CancelOrder<T>(T);

impl<T: Config> CancelOrder<T> {

    ///fn initiate_cancel_order -> This is public method inside the vtbdex crate.
    /// It will take three params ( AccountId, OrderId, TokenType)
    /// This method will only be initiated by cancel_buy_vtbc_order extrinsic defined in lib.rs 
    /// This contain the logic for cancel_buy_vtbc_order.
    /// If the orderId exist than it will refund the Token(ETH/EOS/VTBC) amount to user and remove the Order from OrderBook.
    pub(crate) fn initiate_cancel_order(address: &T::AccountId, 
                                        order_id: &Vec<u8>, 
                                        crypto_type: TokenType,
                                        trade_type: TradeType) -> Result<U256, DispatchError> {
        log::info!("===========Initiate cancel buy of account: {:?} with order id {:?}================", address, order_id);

        let account_key: Vec<u8> = address.encode();
        let user_data = UserWallet::<T>::get(&account_key);
        let user_crypto_data = user_data.crypto_addresses.get(&crypto_type).ok_or(Error::<T>::NoneValue)?;
        let orders = user_crypto_data.order.get(&trade_type).ok_or(Error::<T>::NoneValue)?;

      
        if let Some(index) = orders.get(order_id) {
            let order = OrderBookNMap::<T>::get(
                (&trade_type, 
                &crypto_type, 
                &index))
            .ok_or(Error::<T>::NoneValue)?;

            let req = WalletUpdateReq::new(Some(&crypto_type), &account_key, None, None );
            WalletTrait::<T, T::AccountId>::pay_trnx_fee_in_given_crypto(&req, address, "Cancel order")?;
            let buy_req = TradeRequest::new_from(crypto_type, trade_type, order, *index);
            let amount = Self::refund_order(&buy_req)?;

            Ok(amount)
        }
        else {
            Err(DispatchError::from(Error::<T>::CancelOrderFailed))
        }
    }

    ///fn refund_order -> This is public method inside the vtbdex crate.
    /// It will take five params (TradeRequest<T>)
    /// This contain the logic for refund ETH/EOS/ crypto amount to buyer 
    /// And if the order is Sell Type so refund VTBC to sellere
    /// And deposit an event BuyOrderRefunded/SellOrderRefunded to runtime.
    pub(crate) fn refund_order(order_req: &TradeRequest<T::AccountId>) ->  Result<U256, DispatchError>
    {
        let user_account_id = order_req.address.as_ref().ok_or(Error::<T>::NoneValue)?;
        let account_key: Vec<u8> = user_account_id.encode();

        match order_req.trade_type {
            TradeType::Buy => {
                //delete buy list
                if order_req.crypto_amt > U256::from(0_u64) {     
                    <Pallet<T>>::sub_update_balance(&order_req.crypto_type, &account_key, U256::from(0_u64), order_req.crypto_amt)?;
                    <Pallet<T>>::add_update_balance(&order_req.crypto_type, &account_key, order_req.crypto_amt, U256::from(0_u64))?;
                    <Pallet<T>>::deposit_event(Event::OrderRefunded {
                        trade_type: TradeType::Buy, 
                        order_id: order_req.id.clone(), 
                        user: user_account_id.clone(), 
                        token_type: order_req.crypto_type, 
                        amount: order_req.crypto_amt
                    });	
                }
                //UPDATE ORDER BOOK
                crate::trade::orderbook::OrderBook::<T, T::AccountId>::remove_indexed_old_order(order_req)?;  
                Ok(order_req.crypto_amt)
            },
            TradeType::Sell => {
                //delete sell list
                if order_req.vtbc_amt > U256::from(0_u64) {
                    <Pallet<T>>::add_update_balance(&TokenType::Vtbc, &account_key, order_req.vtbc_amt, U256::from(0_u64))?;  
                    <Pallet<T>>::sub_update_balance(&TokenType::Vtbc, &account_key, U256::from(0_u64), order_req.vtbc_amt)?; 
                    <Pallet<T>>::add_circulation_token_balance(&TokenType::Vtbc, order_req.vtbc_amt)?;   
                    <Pallet<T>>::deposit_event(Event::OrderRefunded {
                        trade_type: TradeType::Sell, 
                        order_id: order_req.id.clone(), 
                        user: user_account_id.clone(), 
                        token_type: order_req.crypto_type, 
                        amount: order_req.vtbc_amt 
                    });	                                   
                } 
                //crate::users::MigratedUsers::<T>::update_migrated_seller_balance(order_req, TokenType::Vtbc)?;
                //UPDATE ORDER BOOK
                crate::trade::orderbook::OrderBook::<T, T::AccountId>::remove_indexed_old_order(order_req)?;  
                Ok(order_req.vtbc_amt)
            },
        }
    }
}

#[cfg(test)]
mod test {
 // Next task to write test cases for trade.rs
}