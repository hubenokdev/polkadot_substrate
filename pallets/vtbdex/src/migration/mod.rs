
pub mod types;
mod v1;
pub use v1::*;
use crate::*;
use frame_support::traits::OnRuntimeUpgrade;
pub struct MigrateToV2<T>(sp_std::marker::PhantomData<T>);
impl<T: Config> OnRuntimeUpgrade for MigrateToV2<T> {
	fn on_runtime_upgrade() -> frame_support::weights::Weight {
        let w1 = v1::user_wallet::migrate_to_v2::<T>();
		let w2 = v1::trade_orders::migrate_to_v2::<T>();
		let w3 = v1::hodldex_users::migrate_to_v2::<T>();
		let w4 = v1::circulation::migrate_to_v2::<T>();
		let w5 = v1::withdraw_count::migrate_to_v2::<T>();
		let w6 = v1::claim_data::migrate_to_v2::<T>();

		w1 + w2 + w3 + w4 + w5 + w6
    }
}

// mod v2;
// pub use v2::*;
// pub struct MigrateToV3<T>(sp_std::marker::PhantomData<T>);
// impl<T: Config> OnRuntimeUpgrade for MigrateToV3<T> {
// 	fn on_runtime_upgrade() -> frame_support::weights::Weight {
// 		let w2 = v2::trade_orders::migrate_to_v3::<T>();

// 		w2 
//     }
// }