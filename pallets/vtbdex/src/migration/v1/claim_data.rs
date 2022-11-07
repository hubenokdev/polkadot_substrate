use crate::*;
use frame_support::pallet_prelude::Weight;
use frame_support::traits::Get;

pub mod v1 {
    use super::*;
    use frame_support::storage_alias;
    use frame_support::pallet_prelude::*;
     
    #[storage_alias]
    pub type Claim<T: Config> = StorageMap<Pallet<T>, Blake2_128Concat, <T as frame_system::Config>::AccountId, migration::types::v1::ToClaimUserBalance, ValueQuery>;
} // only contains V1 storage format

pub fn migrate_to_v2<T: Config>() -> frame_support::weights::Weight { 
    sp_runtime::runtime_logger::RuntimeLogger::init();

    frame_support::log::info!(" Migrated Claim for {:?}", v1::Claim::<T>::iter().count());

    // Storage migrations should use storage versions for safety.
    if <PalletStorageVersion<T>>::get(&migration::types::MigrationType::Claim) == migration::types::StorageVersion::V1_0_0 {

        frame_support::log::info!(" Migrated Claim for {:?}", v1::Claim::<T>::iter().count());
        v1::Claim::<T>::translate::<migration::types::v1::ToClaimUserBalance, _> (

            |k1: T::AccountId, user_data: migration::types::v1::ToClaimUserBalance| {
                //frame_support::log::info!(" Migrated Claim for {:?}...{:?}", k1, user_data);

                let data_eth = custom_types::ToClaimUserBalance {
                    token_amount: user_data.eth,
                    to_update_period: user_data.to_update_period,
                };
                ClaimToken::<T>::insert(&k1, &TokenType::Eth, data_eth);
              
                let data_eos = custom_types::ToClaimUserBalance {
                    token_amount: user_data.eos,
                    to_update_period: user_data.to_update_period,
                };
                ClaimToken::<T>::insert(&k1, &TokenType::Eos, data_eos);

                let data_vtbc = custom_types::ToClaimUserBalance {
                    token_amount: user_data.vtbc,
                    to_update_period: user_data.to_update_period,
                };
                ClaimToken::<T>::insert(&k1, &TokenType::Vtbc, data_vtbc);

                None
            }
        );

        // Update storage version.
        PalletStorageVersion::<T>::insert(migration::types::MigrationType::Claim, migration::types::StorageVersion::V2_0_0);
        // Very inefficient, mostly here for illustration purposes.
        let count = ClaimToken::<T>::iter().count();
        frame_support::log::info!(" <<< Claim storage updated. Migrated User data for {} ✅", count);

        // Return the weight consumed by the migration.
        T::DbWeight::get().reads_writes(count as Weight + 1, count as Weight + 1)
    } else {
        frame_support::log::info!(" >>> Unused migration!");
        0
    }
} // contains checks and transforms storage to V2 format