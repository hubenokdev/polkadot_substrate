use crate::*;
use frame_support::pallet_prelude::Weight;
use sp_std::{
    collections::{btree_map::BTreeMap}
};
pub mod v1 {
    use super::*;
    use frame_support::storage_alias;
    use frame_support::pallet_prelude::*;
     
    #[storage_alias]
    pub type MigratedUser<T: Config> = StorageMap<Pallet<T>, Blake2_128Concat, Vec<u8>, migration::types::v1::MigratedUser, ValueQuery>;
     
    #[storage_alias]
    pub type MigratedUserEachPeriodBal<T: Config> = StorageDoubleMap<Pallet<T>, Blake2_128Concat, Vec<u8>, Blake2_128Concat, u64, migration::types::v1::MigratedUser, OptionQuery>;
 
    #[storage_alias]
    pub type UserHodldexOrderIdList<T: Config> = StorageMap<Pallet<T>, Blake2_128Concat, Vec<u8>, VecDeque<Vec<u8>>, ValueQuery>;
} // only contains V1 storage format
    
pub fn migrate_to_v2<T: Config>() -> frame_support::weights::Weight { 
    sp_runtime::runtime_logger::RuntimeLogger::init();
    let default_pdot_address = "5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM";
    let default_address = <crate::Pallet<T>>::convert_str_to_valid_account_id(default_pdot_address).unwrap();
    frame_support::log::info!("default addrss: {:?}", default_address);
    let default_user = UserWallet::<T>::get(default_address.encode());
    let crytos_detail_of_default_eth_type = default_user.crypto_addresses.get(&TokenType::Eth).unwrap();
    let default_sell_orders_eth_type = crytos_detail_of_default_eth_type.order.get(&TradeType::Sell).unwrap();

    let crytos_detail_of_default_eos_type = default_user.crypto_addresses.get(&TokenType::Eos).unwrap();
    let default_sell_orders_eos_type = crytos_detail_of_default_eos_type.order.get(&TradeType::Sell).unwrap();

    let count2 =  v1::MigratedUser::<T>::iter().count();
    frame_support::log::info!(" <<< Before Hodldex storage updated! MigratedUser::<T>::iter().count(); Migrated {} User data for ✅", count2);
    let count = UserWallet::<T>::iter().count();
    frame_support::log::info!(" <<< Before Hodldex storage updated! Migrated {} User data for ✅", count);
    frame_support::log::info!(" <<< After migration Total user must be {} ✅", count + count2);

    // Storage migrations should use storage versions for safety.
    if <PalletStorageVersion<T>>::get(&migration::types::MigrationType::HodldexData) == migration::types::StorageVersion::V1_0_0 {

        v1::MigratedUser::<T>::translate::<migration::types::v1::MigratedUser, _> (

            |k1: Vec<u8>, wallet_data: migration::types::v1::MigratedUser| {
                // frame_support::log::info!(" Migrated wallet for {:?}...{:?}", k1, wallet_data);

                let mut mapdata = BTreeMap::new();
                let zero_index = 0_u64;
                
                let old_holdex_orders = v1::UserHodldexOrderIdList::<T>::get(&k1);
                if &k1[0..2] == "0x".as_bytes() {
                    log::info!("Eth!"); 
                    let mut sell_orders = BTreeMap::new();
                     // This is because of holdldex orders removed manualy earlier due to testing
                    for order_id in old_holdex_orders.iter() {
                        let order_index = default_sell_orders_eth_type.get(order_id).unwrap_or(&zero_index);
                        sell_orders.insert(order_id.clone(), *order_index);
                    }
                    let mut orders = BTreeMap::new();
                    orders.insert(TradeType::Sell, sell_orders);
                    orders.insert(TradeType::Buy, BTreeMap::new());
                    // crypto detail
                    let crypto_detail = crate::users::types::UserCryptoBasedData {
                        crypto_network: TokenType::Eth.to_string().as_bytes().to_vec(),
                        crypto_address: Some(k1.clone()),
                        deposit_balance: wallet_data.eth_balance,
                        buy_journal_balance: U256::from(0_u8),
                        order: orders.clone(),
                    };
                    mapdata.insert(TokenType::Eth, crypto_detail);
                }
                else {
                    log::info!("Eos!"); 
                    let mut sell_orders = BTreeMap::new();

                    // This is because of holdldex orders removed manualy earlier due to testing
                    for order_id in old_holdex_orders.iter() {
                        let order_index = default_sell_orders_eos_type.get(order_id).unwrap_or(&zero_index);
                        sell_orders.insert(order_id.clone(), *order_index);
                    }
                    let mut orders = BTreeMap::new();
                    orders.insert(TradeType::Sell, sell_orders);
                    orders.insert(TradeType::Buy, BTreeMap::new());
                    // crypto detail
                    let crypto_detail = crate::users::types::UserCryptoBasedData {
                        crypto_network: TokenType::Eos.to_string().as_bytes().to_vec(),
                        crypto_address: Some(k1.clone()),
                        deposit_balance: wallet_data.eos_balance,
                        buy_journal_balance: U256::from(0_u8),
                        order: orders.clone(),
                    };
                    mapdata.insert(TokenType::Eos, crypto_detail);
                }
              
                let mut distribution_index = BTreeMap::new();
                let mut latest_period = 0;
                for assets in TokenType::_distributable_iterator() {
                    distribution_index.insert(*assets, 0);
                    latest_period = 0;
                }

                let mut new_vtbc_period_balance = BTreeMap::new();
                let current_period = Period::<T>::get();
                for period in 0..current_period {
                    if let Some(old_vtbc_period_balance) = v1::MigratedUserEachPeriodBal::<T>::take(&k1, &period) {
                        let balances = crate::users::types::Balances {
                            balance:  old_vtbc_period_balance.vtbc_balance,
                            controlled: old_vtbc_period_balance.controlled_vtbc,   
                            counter: 0,
                        };
                        new_vtbc_period_balance.insert(period, balances);
                    }  
                }
               
                if new_vtbc_period_balance.get(&latest_period).is_none() {
                    let balances = crate::users::types::Balances {
                        balance:  wallet_data.vtbc_balance,
                        controlled: wallet_data.controlled_vtbc,  
                        counter: 0,
                    };
                    new_vtbc_period_balance.insert(latest_period, balances);
                }
                let new_user_obj = crate::users::types::User {
                    polkadot_address: None,
                    vtbc_balance: wallet_data.vtbc_balance,
                    vtbt_balance: U256::from(0_u64),
                    sells_journal_balance: wallet_data.controlled_vtbc,
                    crypto_addresses: mapdata,
                    active: true,
                    latest_period,
                    processing_distribution_index: distribution_index,
                    vtbc_period_balance: new_vtbc_period_balance,
                };

                UserWallet::<T>::insert(k1, new_user_obj);
                
                None
            }
        );
        // Update storage version.
        PalletStorageVersion::<T>::insert(migration::types::MigrationType::HodldexData, migration::types::StorageVersion::V2_0_0);
        // Very inefficient, mostly here for illustration purposes.
        let count = UserWallet::<T>::iter().count();
        frame_support::log::info!(" <<< After Hodldex storage updated! Migrated {} User data for ✅", count);

        // Return the weight consumed by the migration.
        T::DbWeight::get().reads_writes(count as Weight + 1, count as Weight + 1)
    } else {
        frame_support::log::info!(" >>> Unused migration!");
        0
    }
} // contains checks and transforms storage to V2 format