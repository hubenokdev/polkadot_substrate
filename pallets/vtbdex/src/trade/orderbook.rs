//! Module OrderBook
//! It defines some helper method to mutate OrderBook runtime storage
use codec::Encode;
use sp_std::collections::btree_map::BTreeMap;
use crate::{
    Vec, Config, Error,
    trade::types::{ OrderBookStruct, TradeRequest},
    {OrderIndexedNMap, UserWallet, OrderBookNMap},
};
use sp_runtime::DispatchResult;
// Defining a Details trait by defining the functionality it should include
pub trait OrderBook<T, AccountId> {
    fn push_last_new_order(&self, new_order_entry: OrderBookStruct<AccountId>) -> DispatchResult; 
    fn pop_first_old_order(&self) -> DispatchResult;
    fn remove_indexed_old_order(&self) -> DispatchResult;
}

impl<T: Config> OrderBook<T, T::AccountId> for TradeRequest<T::AccountId> 
where 
T: Config {

    /// fn push_last_new_order -> This fuction is public inside the crate vtbdex.
    /// It takes four parameters (TradeType, TokenType, AccountId, OrderBookStruct)
    /// This function will insert new Order in the OrderBook at the last index
    fn push_last_new_order(&self,
        new_order_entry: OrderBookStruct<T::AccountId>) -> DispatchResult {
        let mut push_index = 0_u64;

        OrderIndexedNMap::<T>::mutate((&self.trade_type, &self.crypto_type), |vec_obj|{
            push_index = if let Some(last_index) = vec_obj.last() {
                last_index + 1_u64
            }
            else {
                1_u64
            };
            vec_obj.push(push_index);
        });

        let account_key = self.address.as_ref().ok_or(Error::<T>::NoneValue)?;
        UserWallet::<T>::mutate(account_key.encode(), |user_data| -> DispatchResult {
            let user_crypto_data = user_data.crypto_addresses.get_mut(&self.crypto_type).ok_or(Error::<T>::NoneValue)?;
            match user_crypto_data.order.get_mut(&self.trade_type) {
                Some(orders) => {
                    orders.insert(new_order_entry.order_id.clone(), push_index);
                },
                None => {
                    let mut new_map = BTreeMap::new();
                    new_map.insert(new_order_entry.order_id.clone(), push_index);
                    user_crypto_data.order.insert(self.trade_type, new_map);
                }
            }
            
            Ok(())
        })?;

        OrderBookNMap::<T>::insert((&self.trade_type, 
                                    &self.crypto_type, 
                                    &push_index), 
                                    &new_order_entry); 

        Ok(())
    }

    /// fn pop_first_old_order -> This fuction is public inside the crate vtbdex.
    /// It takes four parameters (TradeType, TokenType, AccountId, OrderId)
    /// This function will remove old Order from the OrderBook from the first index
    fn pop_first_old_order(&self) -> DispatchResult {
        let mut pop_index = 0_u64;

        OrderIndexedNMap::<T>::mutate((&self.trade_type, &self.crypto_type), |vec_obj|{
            pop_index = if let Some(first_index) = vec_obj.first() {
                *first_index
            }
            else {
                return 
            };
            vec_obj.remove(0);
        });

        let account_key = self.address.as_ref().ok_or(Error::<T>::NoneValue)?;
        UserWallet::<T>::mutate(account_key.encode(), |user_data| -> DispatchResult {
            let user_crypto_data = user_data.crypto_addresses.get_mut(&self.crypto_type).ok_or(Error::<T>::NoneValue)?;
            let orders = user_crypto_data.order.get_mut(&self.trade_type).ok_or(Error::<T>::NoneValue)?;
            orders.remove(&self.id); 

            Ok(())
        })?;

        OrderBookNMap::<T>::remove((&self.trade_type, 
            &self.crypto_type, 
            &pop_index)
        ); 

        Ok(())
    }

    /// fn remove_indexed_old_order -> This fuction is public inside the crate vtbdex.
    /// It takes five parameters (TradeType, TokenType, AccountId, Index, OrderId)
    /// This function will remove old Order from the OrderBook from the given index in parameter
    fn remove_indexed_old_order(&self) -> DispatchResult {
        
        let account_key = self.address.as_ref().ok_or(Error::<T>::NoneValue)?;
        UserWallet::<T>::mutate(account_key.encode(), |user_data| -> DispatchResult {
            let user_crypto_data = user_data.crypto_addresses.get_mut(&self.crypto_type).ok_or(Error::<T>::NoneValue)?;
            let orders = user_crypto_data.order.get_mut(&self.trade_type).ok_or(Error::<T>::NoneValue)?;
            orders.remove(&self.id);
            
            Ok(())
        })?;
        OrderBookNMap::<T>::remove((&self.trade_type, 
                                    &self.crypto_type, 
                                    &self.index)
        ); 

        OrderIndexedNMap::<T>::mutate((&self.trade_type, &self.crypto_type), |vec_obj|{
            if let Ok(vec_index) = vec_obj.binary_search(&self.index){
                vec_obj.remove(vec_index);
            }    
        });
        
        Ok(())
    }
}

#[cfg(test)]
mod test {
 // Next task to write test cases for trade.rs
}