use crate::*;
use frame_support::pallet_prelude::Weight;
pub mod v1 {
    use super::*;
    use frame_support::storage_alias;
    use frame_support::pallet_prelude::*;
            	
    #[storage_alias]
    pub(super) type WithdrawRecord<T: Config> = StorageMap<Pallet<T>, Blake2_128Concat, <T as frame_system::Config>::BlockNumber, u64, ValueQuery>; 
} // only contains V1 storage format
    
pub fn migrate_to_v2<T: Config>() -> frame_support::weights::Weight { 
    sp_runtime::runtime_logger::RuntimeLogger::init();

    // Very inefficient, mostly here for illustration purposes.
    let count = v1::WithdrawRecord::<T>::iter().count();
    frame_support::log::info!(" <<< Before v1::WithdrawRecord storage updated. Migrated data for {} ✅", count);
    let blocknumber = frame_system::Pallet::<T>::block_number();
    frame_support::log::info!(" Blocknumber : {:?}", blocknumber);

    // Storage migrations should use storage versions for safety.
    if <PalletStorageVersion<T>>::get(&migration::types::MigrationType::WithdrawCount) == migration::types::StorageVersion::V1_0_0 {

        let _ = v1::WithdrawRecord::<T>::translate::< u64, _> (

            |k1: <T as frame_system::Config>::BlockNumber, record_data: u64| {
                //frame_support::log::info!(" Migrate record_data for {:?} k1 : {:?}", record_data, k1);

                if k1 > blocknumber - 50_u32.into() {
                    WithdrawCountRecord::<T>::insert((TokenType::Eth, k1), record_data);
                    WithdrawCountRecord::<T>::insert((TokenType::Eos, k1), record_data);
                }

                None
            }
        );

        // Update storage version.
        PalletStorageVersion::<T>::insert(migration::types::MigrationType::WithdrawCount, migration::types::StorageVersion::V2_0_0);
        // Very inefficient, mostly here for illustration purposes.
        let count = WithdrawCountRecord::<T>::iter().count();
        frame_support::log::info!(" <<< After WithdrawCount storage updated. Migrated data for {} ✅", count);

        // Return the weight consumed by the migration.
        T::DbWeight::get().reads_writes(count as Weight + 1, count as Weight + 1)
    } else {
        frame_support::log::info!(" >>> Unused migration!");
        0
    }
} // contains checks and transforms storage to V2 format    