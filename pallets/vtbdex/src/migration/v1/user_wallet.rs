use crate::*;
use frame_support::pallet_prelude::Weight;
use sp_std::{
    collections::{btree_map::BTreeMap}
};
use crate::migration::types::v1::{DeprecatedWallet };
pub mod v1 {
    use super::*;
    use frame_support::storage_alias;
    use frame_support::pallet_prelude::*;

    #[storage_alias]
    pub type Wallet<T: Config> = StorageMap<Pallet<T>, Blake2_128Concat, <T as frame_system::Config>::AccountId, DeprecatedWallet, ValueQuery>;

    #[storage_alias]
    pub type User<T: Config> = StorageDoubleMap<Pallet<T>, Blake2_128Concat, TokenType, Blake2_128Concat, <T as frame_system::Config>::AccountId, migration::types::v1::User, ValueQuery>;

} // only contains V1 storage format

pub fn migrate_to_v2<T: Config>() -> frame_support::weights::Weight { 
    sp_runtime::runtime_logger::RuntimeLogger::init();

    let count1 = v1::Wallet::<T>::iter().count();
    frame_support::log::info!(" <<< Before migration, wallet migration v1::Wallet::<T>::iter() older data! Migrate wallet data for {} ✅", count1);
    let count2 = v1::User::<T>::iter().count();
    frame_support::log::info!(" <<< Before migration, wallet migration v1::User::<T>::iter() older data! Migrate user data for {} ✅", count2);

    // Storage migrations should use storage versions for safety.
    if <PalletStorageVersion<T>>::get(&migration::types::MigrationType::User) == migration::types::StorageVersion::V1_0_0 {
        v1::Wallet::<T>::translate::<DeprecatedWallet, _> (

            |k1: T::AccountId, wallet_data: DeprecatedWallet| {

                let mut orders = BTreeMap::new();
                for trade_type in TradeType::_iterator() {
                    orders.insert(*trade_type, BTreeMap::new());
                }

                let mut mapdata = BTreeMap::new();
                for (key, each_cryptos) in wallet_data.crypto_addresses {
                    let crypto_detail = crate::users::types::UserCryptoBasedData {
                        crypto_network: each_cryptos.crypto_network,
                        // crypto_address: Some(each_cryptos.crypto_addresses), // This will uncomment in old code
                        crypto_address: each_cryptos.crypto_addresses,
                        deposit_balance: each_cryptos.deposit_balance,
                        buy_journal_balance: U256::from(0_u8),
                        order: orders.clone(),
                    };
                    mapdata.insert(key, crypto_detail);
                }

                let mut distribution_index = BTreeMap::new();
                let mut new_vtbc_period_balance = BTreeMap::new();
                let mut latest_period = 0;
                for assets in TokenType::_distributable_iterator() {
                    let old_assets_period_balance = v1::User::<T>::take(assets, &k1);
                    distribution_index.insert(*assets, old_assets_period_balance.processing_distribution_index);
                    latest_period = old_assets_period_balance.processing_balance_index;

                    if assets == &TokenType::Vtbc {
                        for (key, each_balance) in old_assets_period_balance.user_balance {
                            let mut counter = 0;
                            for token in distribution_index.iter() {
                                if token.1 >= &key {
                                    counter += 1;
                                }
                            }
                            let balances = crate::users::types::Balances {
                                balance:  each_balance.balance,
                                controlled: each_balance.controlled,   
                                counter,
                            };
    
                            if counter != TokenType::_distributable_iterator().count() as u32 {
                                new_vtbc_period_balance.insert(key, balances);
                            }
                            
                        } 
                    }
                }
                v1::User::<T>::remove(&TokenType::Vtbt, &k1);
                let new_user_obj = crate::users::types::User {
                    polkadot_address: Some(wallet_data.polkadot_address),
                    vtbc_balance: wallet_data.vtbc_balance,
                    vtbt_balance: wallet_data.vtbt_balance,
                    sells_journal_balance: wallet_data.controlled,
                    crypto_addresses: mapdata,
                    active: wallet_data.active,
                    latest_period,
                    processing_distribution_index: distribution_index,
                    vtbc_period_balance: new_vtbc_period_balance,
                };
               
                UserWallet::<T>::insert(k1.encode(), new_user_obj);
                
                None
            }
        );

        // Update storage version.
        PalletStorageVersion::<T>::insert(migration::types::MigrationType::User, migration::types::StorageVersion::V2_0_0);
        // Very inefficient, mostly here for illustration purposes.
        let count = UserWallet::<T>::iter().count();
        frame_support::log::info!(" <<< After New Wallet storage updated! UserWallet::<T>::iter() Migrated wallet data for {} ✅", count);

        let count1 = v1::Wallet::<T>::iter().count();
        frame_support::log::info!(" <<< After Wallet older data! v1::Wallet::<T>::iter(): {} ✅", count1);

        let count2 = v1::User::<T>::iter().count();
        frame_support::log::info!(" <<< After  Wallet older data! v1::User::<T>::iter(): {} ✅", count2);

        // Return the weight consumed by the migration.
        T::DbWeight::get().reads_writes(count as Weight + 1, count as Weight + 1)
    } else {
        frame_support::log::info!(" >>> Unused migration!");
        0
    }
} // contains checks and transforms storage to V2 format