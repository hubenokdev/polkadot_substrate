//!module migrated_data: This module is responsible to capture all the state related to migrated sellorder or 
//! user balances from hodldex.
//! This module has two methods to expose
//! 1. Hodldexsellorders -> The sellers of hodldex who has not onboarded to Vtbdex system yet.
//! 2. Migrateduser balance -> The user of hodldex who has not onboarded to vtbdex system yet.
use crate::{
	Pallet, Error, Config,
    Period, 
    String, Vec, ToString
};
use serde_json::{Value, Map};
pub(crate) struct MigratedData<T>(T);

impl<T: Config> MigratedData<T> {

    /// fn user_hodldex_selllist_data - The visibility of this function is set to with module ipfs.
    /// This function is responsible to take migrated hodldex sellorders and convert to Object.
    /// At the end append this converted object in the final json data object. 
    pub(super) fn user_hodldex_selllist_data(res: &mut Map<String, Value>) -> Result<(), Error<T>> {
        let hodldex_order_id = UserHodldexOrderIdList::<T>::iter();
        let mut map = Map::new();

        for key in hodldex_order_id {
            let mut apr_map = Vec::new();
            for id in 0..key.1.len() {
                let id_str = <Pallet<T>>::bytes_to_string(&key.1[id])?;
                apr_map.push(id_str);
            }
            let map_key = <Pallet<T>>::bytes_to_string(&key.0)?;
            map.insert(map_key, Value::from(apr_map));
        }

        res.insert("Users_distr_list".to_string(), Value::from(map));
        Ok(())
    }

    /// fn migrated_users_ipfs_call - This function is responsible to grab all balances details of migrated user state
    /// And convert in object.
    /// At the end append this converted object in the final json data object. 
    pub(super) fn migrated_users_ipfs_call(res: &mut Map<String, Value>) -> Result<(), Error<T>> {

        let keys = MigratedUser::<T>::iter();
        let mut map = Map::new();
        for key in keys {
            let val = key.1;
            let mut each_migrated_users = Map::new();
            let address = <Pallet<T>>::bytes_to_string(&key.0)?;
            each_migrated_users.insert("crypto_address".to_string(), Value::String(address));
            Self::migrated_data(&mut each_migrated_users, val)?;
            // Upload Users list
            let mut users = Map::new();
            for key2 in 0..Period::<T>::get() {
                let user_assets = MigratedUserEachPeriodBal::<T>::get(&key.0, key2);
                let mut user_assets_val = Map::new();
                Self::migrated_data(&mut user_assets_val, user_assets)?;
                users.insert(key2.to_string(), serde_json::to_value(user_assets_val).ok_or(Error::<T>::NoneValue)?);
            }
            each_migrated_users.insert("MigratedUserEachPeriodBal".to_string(), serde_json::to_value(users).ok_or(Error::<T>::NoneValue)?);
            let map_key = <Pallet<T>>::bytes_to_string(&key.0)?;
            map.insert(map_key, Value::Object(each_migrated_users));

        }

        res.insert("Users_distr_list".to_string(), Value::from(map));
        Ok(())
    }

    /// fn migrated_data: This function is responsible to grab each key of the MigratedUser types 
    /// and insert in mutable reference object
    fn migrated_data(user_assets_val: &mut Map<String, Value>, user_assets: MigratedUserStruct) -> Result<(), Error<T>> {
        user_assets_val.insert("eth_balance".to_string(), Value::String(user_assets.eth_balance.to_string()));
        user_assets_val.insert("eos_balance".to_string(), Value::String(user_assets.eos_balance.to_string()));
        user_assets_val.insert("vtbc_balance".to_string(), Value::String(user_assets.vtbc_balance.to_string()));
        user_assets_val.insert("controlled_vtbc".to_string(), Value::String(user_assets.controlled_vtbc.to_string()));

        Ok(())
    }
}