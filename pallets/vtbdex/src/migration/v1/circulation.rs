use crate::*;
use frame_support::pallet_prelude::Weight;

pub mod v1 {
    use super::*;
    use frame_support::storage_alias;
    use frame_support::pallet_prelude::*;
            	
    #[storage_alias]
    pub type Circulation<T: Config> = StorageValue<Pallet<T>, migration::types::v1::Circulation, ValueQuery>;

} // only contains V1 storage format
    
pub fn migrate_to_v2<T: Config>() -> frame_support::weights::Weight { 
    sp_runtime::runtime_logger::RuntimeLogger::init();

    // Storage migrations should use storage versions for safety.
    if <PalletStorageVersion<T>>::get(&migration::types::MigrationType::Circulation) == migration::types::StorageVersion::V1_0_0 {

        let _ = v1::Circulation::<T>::translate::<migration::types::v1::Circulation, _> (

            |circulation_data: Option<migration::types::v1::Circulation>| {
                frame_support::log::info!(" Migrate circulation_data for {:?}", circulation_data);
                if let Some(circulation) = circulation_data {
                    Circulation::<T>::insert(&TokenType::Eth, circulation.eth_deposit_amount);
                    Circulation::<T>::insert(&TokenType::Eos, circulation.eos_deposit_amount);
                    Circulation::<T>::insert(&TokenType::Vtbc, circulation.vtbc_amount);
                };

                None
            }

        );

        // Update storage version.
        PalletStorageVersion::<T>::insert(migration::types::MigrationType::Circulation, migration::types::StorageVersion::V2_0_0);
        // Very inefficient, mostly here for illustration purposes.
        let count = Circulation::<T>::iter().count();
        frame_support::log::info!(" <<< After Circulation storage updated. Migrated data for {} ✅", count);

        // Return the weight consumed by the migration.
        T::DbWeight::get().reads_writes(count as Weight + 1, count as Weight + 1)
    } else {
        frame_support::log::info!(" >>> Unused migration!");
        0
    }
} // contains checks and transforms storage to V2 format