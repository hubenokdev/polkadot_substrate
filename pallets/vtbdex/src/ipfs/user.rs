
use crate::{
	Pallet, Error, Config,
    BlockedUserWallet, UserWallet, 
    String, Vec, ToString
};
use codec::Encode;
use sp_runtime::SaturatedConversion;
use serde_json::{Value, Map};
use rustc_hex::ToHex;
use sp_std::collections::btree_map::BTreeMap;
use crate::ipfs::OrderData;

pub(super) struct UserData<T>(T);

impl<T: Config> UserData<T> { 

    ///fn wallet_data_ipfs_call -> This function take mutable reference of map.
    ///It iterate over all user address in Wallet runtime state and deserailize wallet to object
    pub(super) fn wallet_data_ipfs_call(res: &mut Map<String, Value>) -> Result<(), Error<T>> { 

        let keys = UserWallet::<T>::iter();
        let mut wallet = Map::new();
        for key in keys {
            let val = key.1;	
            let map_key = <Pallet<T>>::bytes_to_string(&val.polkadot_address)?;
            wallet.insert(map_key.clone(), Value::from(Self::wallet_to_map(&val)?));
        }
        res.insert("Users_Wallet".to_string(),  Value::from(wallet));

        Ok(())
    }

    ///fn blocked_users_wallet -> This function take mutable reference of map.
    ///It iterate over all user address in Blockedlist runtime state and deserailize req to object
    pub(super) fn blocked_users_wallet(res: &mut Map<String, Value>) -> Result<(), Error<T>> {
        let keys = BlockedUserWallet::<T>::iter();
        let mut map = Map::new();
        for key in keys {
            let each_map_iter = key.1;
            let val = <UserWallet<T>>::get(key.0.encode());	
            let address = <Pallet<T>>::bytes_to_string(&val.polkadot_address)?;
            let mut each_user_list = Vec::new();
            for key2 in each_map_iter.iter() {
                let mut withdraw_req = Map::new();
                withdraw_req.insert("polkadot_address".to_string(), Value::String(address.clone()));
                withdraw_req.insert("transaction_hash".to_string(), Value::String(<Pallet<T>>::bytes_to_string(&Some(key2.transaction_hash.clone()))?));
                withdraw_req.insert("id".to_string(), Value::String((&key2.id).to_hex()));
                withdraw_req.insert("requested".to_string(), Value::Bool(key2.requested));
                withdraw_req.insert("withdraw_amount".to_string(), Value::String(key2.withdraw_amount.to_string()));
                withdraw_req.insert("fee".to_string(), Value::String(key2.fee.to_string()));
                withdraw_req.insert("token_type".to_string(), Value::String(key2.token_type.to_string()));
                withdraw_req.insert("timestamp".to_string(), Value::from(key2.timestamp.saturated_into::<u64>()));
                withdraw_req.insert("node_block_number".to_string(), Value::from(key2.node_block_number.saturated_into::<u64>()));

                each_user_list.push(withdraw_req);
            }

            map.insert(address, Value::from(each_user_list));
        }

        res.insert("Withdraw_blocked_users_wallet".to_string(),  Value::from(map));
        Ok(())
    }

    ///fn wallet_to_map -> This function take reference of Wallet type.
    ///It iterate over all field of Wallet type and deserailize wallet to object
    fn wallet_to_map(val: &crate::users::types::User) -> Result<Map<String, Value>, Error<T>> {
        let mut each_wallet = Map::new();
    
        each_wallet.insert("polkadot_address".to_string(), Value::String(<Pallet<T>>::bytes_to_string(&val.polkadot_address)?));
        each_wallet.insert("active".to_string(), Value::Bool(val.active));
        each_wallet.insert("sells_journal_balance".to_string(), Value::String(val.sells_journal_balance.to_string()));
        each_wallet.insert("vtbc_balance".to_string(), Value::String(val.vtbc_balance.to_string()));
        each_wallet.insert("vtbt_balance".to_string(), Value::String(val.vtbt_balance.to_string()));
        each_wallet.insert("latest_period".to_string(), Value::from(val.latest_period));
        each_wallet.insert("crypto_detail".to_string(), Self::user_crypto_detail(&val.crypto_addresses));
        each_wallet.insert("vtbc_period_balance".to_string(), Self::user_vtbc_period_balance(&val.vtbc_period_balance));
        each_wallet.insert("processing_distribution_index".to_string(), Self::user_processing_distribution_index(&val.processing_distribution_index));
        
        Ok(each_wallet)
    }

    fn user_crypto_detail(crypto_addresses: &BTreeMap<crate::TokenType, crate::UserCryptoBasedData>) -> Value {
        let mut each_wallet_cryptos = Map::new();
        for (key, value) in crypto_addresses.iter() {
            let mut each_crypto = Map::new();
            each_crypto.insert("crypto_address".to_string(), Value::String(<Pallet<T>>::bytes_to_string(&value.crypto_address).unwrap()));
            each_crypto.insert("crypto_network".to_string(), Value::String(<Pallet<T>>::bytes_to_string(&Some(value.crypto_network.clone())).unwrap()));
            each_crypto.insert("deposit_balance".to_string(), Value::from(value.deposit_balance.to_string()));
            each_crypto.insert("buy_journal_balance".to_string(), Value::from(value.buy_journal_balance.to_string()));
            each_crypto.insert("orders".to_string(), Self::user_orders_list(&value.order));
            each_wallet_cryptos.insert(key.to_string(), Value::Object(each_crypto));
        }

        Value::from(each_wallet_cryptos)
    }

    fn user_vtbc_period_balance(balances: &BTreeMap<u64, crate::Balances>) -> Value {
        let mut user_balance = Map::new();
        for (key, value) in balances.iter() {
            let mut key_balance = Map::new();
            key_balance.insert("balance".to_string(), Value::from(value.balance.to_string()));
            key_balance.insert("controlled".to_string(), Value::from(value.balance.to_string()));
            user_balance.insert(key.to_string(), serde_json::to_value(key_balance).unwrap());
        }

        Value::from(user_balance)
    }

    fn user_processing_distribution_index(index: &BTreeMap<crate::TokenType, u64>) -> Value {
        let mut processing_distribution_index = Map::new();
        for (key, value) in index.iter() {
            processing_distribution_index.insert(key.to_string(), Value::from(*value));
        }
        Value::from(processing_distribution_index)
    }

    fn user_orders_list(orders: &BTreeMap<crate::TradeType, BTreeMap<Vec<u8>, u64>>) -> Value {
        let mut orders_json_data = Map::new();
        for (trade_type, orders_data) in orders.iter() {
            let mut list = Map::new();
            for (key, value) in orders_data.iter() {
                let map_key = OrderData::<T>::convert_order_id_to_string(key.clone()).unwrap();
                list.insert(map_key, Value::from(value.to_string()));
            }
            orders_json_data.insert(trade_type.to_string(), Value::from(list));
        };  

        Value::from(orders_json_data)
    }
}