//!module order: This module is responsible to capture the BuyOrder & SellOrder book for each crypto and append it to the 
//! final json object, which will be sent to ipfs
use crate::{
	Pallet, Error, Config, Encode,
    OrderIndexedNMap, OrderBookNMap, UserWallet,
    TradeType, TokenType, String, Vec, ToString
};
use serde_json::{Value, Map};
use rustc_hex::ToHex;

pub(crate) struct OrderData<T>(T);

impl<T: Config> OrderData<T> {

    /// fn orders_data -> This function take mutable reference.
    /// The main aim of the funtion to iterate over TradeType(Buy/Sell) enum and capture the order state.
    /// At the end append the order state to mutable reference object
    pub(super) fn orders_data(res: &mut Map<String, Value>) -> Result<(), Error<T>> {
        let mut global_orders = Map::new();

        for trade_type in TradeType::_iterator() {
            let list = Self::orders_data_to_map(trade_type)?;
            global_orders.insert(trade_type.to_string(), Value::from(list));   
        };

        res.insert("Orders".to_string(), Value::from(global_orders));

        Ok(())
    }

    /// fn orders_data_to_map -> This function take reference of TradeType enum.
    /// The main aim of the funtion to iterate over all orders of the given TradeType 
    /// And searialize the order_detail to Json Object.
    fn orders_data_to_map(trade_type: &TradeType) -> Result<Map<String, Value>, Error<T>> {
        let mut orders_list = Map::new();
        for assets in TokenType::_crypto_iterator() {
            let order_index = <OrderIndexedNMap<T>>::get((trade_type, assets));
            let mut each_crypto_sell_list = Map::new();
            each_crypto_sell_list.insert("OrderIndexedNMap".to_string(), Value::from(order_index.clone()));
            for index in order_index.iter() {
                let key = <OrderBookNMap<T>>::get((&trade_type, assets, index)).ok_or(Error::<T>::NumberIsTooLowForU256)?;
                let order_id_str = Self::convert_order_id_to_string(key.order_id)?;
                let val = <UserWallet<T>>::get(key.address.encode());
                let mut each_order = Map::new();
                each_order.insert("order_id".to_string(), Value::String(order_id_str));
                each_order.insert("address".to_string(),  Value::String(<Pallet<T>>::bytes_to_string(&val.polkadot_address)?));
                each_order.insert("crypto_address".to_string(), Value::String(<Pallet<T>>::bytes_to_string(&key.crypto_address)?));
                each_order.insert("amount".to_string(), Value::String(key.amount.to_string()));
                each_order.insert("usd_rate".to_string(), Value::String(key.usd_rate.to_string()));
                each_crypto_sell_list.insert(index.to_string(), Value::from(each_order));
            }
            orders_list.insert(assets.to_string(), Value::from(each_crypto_sell_list));
        }

        Ok(orders_list)
    }

    /// fn convert_order_id_to_string -> This function take argument of Vec<u8>.
    /// It tries to convert the given bytes to str using sp_std::str::from_utf8( This is for order migrated from hodldex), 
    /// if it fails than it convert into Hex(This is for order generated in vtbdex system)
    /// And return the String of the id.
    pub(crate) fn convert_order_id_to_string(id: Vec<u8>) -> Result<String, Error<T>> {
        match sp_std::str::from_utf8(&id) {
            Ok(val) =>{
                Ok(val.to_string())
            },
            Err(_err) => {
                Ok((&id).to_hex())
            }
        }
    }
}