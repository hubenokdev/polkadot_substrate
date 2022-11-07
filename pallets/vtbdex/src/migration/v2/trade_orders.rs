use crate::*;
use frame_support::pallet_prelude::Weight;

pub mod v2 {
	use super::*;

	use frame_support::pallet_prelude::*;
	use frame_support::storage_alias;

	/// Store of OrderBookList when new order request is accepted.
	// #[storage_alias]
	// pub(super) type OrderIndexedNMap <T: Config> = StorageNMap<
    //     Pallet<T>,
	// 	(NMapKey<Blake2_128Concat, TradeType>,
	// 	NMapKey<Blake2_128Concat, TokenType>),
	// 	Indexed,
	// 	ValueQuery,
	// >;

    #[storage_alias]
    pub type OrderBookNMap <T: Config> = StorageNMap<
    Pallet<T>,
    (NMapKey<Blake2_128Concat, TradeType>,
    NMapKey<Blake2_128Concat, TokenType>,
    NMapKey<Twox64Concat, U256>),
    OrderBookStruct<<T as frame_system::Config>::AccountId>,
    OptionQuery,
>;
} // only contains V1 storage format

pub fn migrate_to_v3<T: Config>() -> frame_support::weights::Weight {
	sp_runtime::runtime_logger::RuntimeLogger::init();

    // Storage migrations should use storage versions for safety.
	if <PalletStorageVersion<T>>::get(&migration::types::MigrationType::TradeOrders)
		== migration::types::StorageVersion::V2_0_0
	{
		let _ = v2::OrderBookNMap::<T>::translate::<
        OrderBookStruct<T::AccountId>,
			_,
		>(|(k1, k2, _k3), sellorders_data: OrderBookStruct<T::AccountId>| {
			frame_support::log::info!(" Migrated sellorders_data for {:?}", sellorders_data);

			if let seller_orders = sellorders_data {
					let mut req_data = trade::types::TradeRequest::new(
						k2,
						k1,
						seller_orders.address,
						seller_orders.amount,
					);
					req_data.insert_crypto_address(seller_orders.crypto_address.clone());
					// crypto_address
					frame_support::log::info!(" new_sell_order_entry req_data {:?}", &req_data.crypto_address);

					let mut new_sell_order_entry =
						OrderBookStruct::new(&req_data, 0_u64);
					new_sell_order_entry.update_amount(seller_orders.amount);
					new_sell_order_entry.update_usd_rate(seller_orders.usd_rate);
					new_sell_order_entry.order_id = seller_orders.order_id;
					frame_support::log::info!(" new_sell_order_entry {:?}", new_sell_order_entry);
					let _ = crate::trade::orderbook::OrderBook::<T, T::AccountId>::push_last_new_order(
						&req_data,
						new_sell_order_entry,
					);
			}

			None
		});

		// Update storage version.
		PalletStorageVersion::<T>::insert(
			migration::types::MigrationType::TradeOrders,
			migration::types::StorageVersion::V3_0_0,
		);
		// Very inefficient, mostly here for illustration purposes.
		let count = OrderBookNMap::<T>::iter().count();
		frame_support::log::info!(
			" <<< Orderbook storage updated! Migrated orders data for {} ✅",
			count
		);

		// Return the weight consumed by the migration.
		T::DbWeight::get().reads_writes(count as Weight + 1, count as Weight + 1)
	} else {
		frame_support::log::info!(" >>> Unused migration!");
		0
	}
} // contains checks and transforms storage to V2 format
