use crate::*;
use frame_support::pallet_prelude::Weight;

pub mod v1 {
	use super::*;

	use frame_support::pallet_prelude::*;
	use frame_support::storage_alias;

	/// Store of OrderBookList when new order request is accepted.
	#[storage_alias]
	pub type BuyOrderIdList<T: Config> =
		StorageValue<Pallet<T>, VecDeque<migration::types::v1::GlobalOrders>, ValueQuery>;

	/// Store of OrderBookList when new order request is accepted.
	#[storage_alias]
	pub type SellOrderIdList<T: Config> =
		StorageValue<Pallet<T>, VecDeque<migration::types::v1::GlobalOrders>, ValueQuery>;

	/// Store of OrderBookList when new order request is accepted.
	#[storage_alias]
	pub type OrderBookList<T: Config> = StorageMap<
		Pallet<T>,
		Blake2_128Concat,
		Vec<u8>,
		migration::types::v1::OrderBook<<T as frame_system::Config>::AccountId>,
		OptionQuery,
	>;

	#[storage_alias]
	pub type UserOrderIdList<T: Config> = StorageMap<
		Pallet<T>,
		Blake2_128Concat,
		<T as frame_system::Config>::AccountId,
		VecDeque<Vec<u8>>,
		ValueQuery,
	>;

	/// Store of OrderBookList when new order request is accepted.
	#[storage_alias]
	pub type TotalOrdersCountTillDate<T: Config> = StorageValue<Pallet<T>, i128, ValueQuery>;
} // only contains V1 storage format

pub fn migrate_to_v2<T: Config>() -> frame_support::weights::Weight {
	sp_runtime::runtime_logger::RuntimeLogger::init();

	let count = v1::SellOrderIdList::<T>::get().len();
	frame_support::log::info!(
		" <<<Before migration, SellOrderIdList::<T>::iter(), Migrate orders data for {} ✅",
		count
	);

	let count = v1::BuyOrderIdList::<T>::get().len();
	frame_support::log::info!(
		" <<< Before migration, BuyOrderIdList::<T>::iter() Migrated orders data for {} ✅",
		count
	);

	// Storage migrations should use storage versions for safety.
	if <PalletStorageVersion<T>>::get(&migration::types::MigrationType::TradeOrders)
		== migration::types::StorageVersion::V2_0_0
	{
		let default_pdot_address = "5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM";
    	let default_address = <crate::Pallet<T>>::convert_str_to_valid_account_id(default_pdot_address).unwrap();
		let _ = v1::SellOrderIdList::<T>::translate::<
			VecDeque<migration::types::v1::GlobalOrders>,
			_,
		>(|sellorders_data: Option<VecDeque<migration::types::v1::GlobalOrders>>| {
			//frame_support::log::info!(" Migrated sellorders_data for {:?}", sellorders_data);

			if let Some(seller_order_ids) = sellorders_data {
				for id in seller_order_ids {
					let orders = v1::OrderBookList::<T>::get(&id.order_id).unwrap();

					let mut req_data = if orders.address == default_address {
						trade::types::TradeRequest {
							crypto_type: orders.crypto_type, 
							trade_type: TradeType::Sell,
							address: None, 
							crypto_address: Some(orders.crypto_address.clone()), 
							crypto_amt: U256::from(0_u8), 
							vtbc_amt: orders.amount,
							controlled_amt: U256::from(0_u8),
							id: Vec::new(), 
							usd_rate: U256::from(0_u8),
							index: 0,
						}

					} else {
						trade::types::TradeRequest {
							crypto_type: orders.crypto_type, 
							trade_type: TradeType::Sell,
							address: Some(orders.address), 
							crypto_address: Some(orders.crypto_address.clone()), 
							crypto_amt: U256::from(0_u8), 
							vtbc_amt: orders.amount,
							controlled_amt: U256::from(0_u8),
							id: Vec::new(), 
							usd_rate: U256::from(0_u8),
							index: 0,
						}
					};

					// let mut req_data = trade::types::TradeRequest::new(
					// 	orders.crypto_type,
					// 	TradeType::Sell,
					// 	orders.address,
					// 	orders.amount,
					// );
					//req_data.insert_crypto_address(Some(orders.crypto_address.clone()));
					// crypto_address
					frame_support::log::info!(" new_sell_order_entry req_data {:?} ==== {:?}", &req_data.crypto_address, req_data.address);

					let mut new_sell_order_entry =
						OrderBookStruct::new(&req_data, 0_u64);
					new_sell_order_entry.update_amount(orders.amount);
					new_sell_order_entry.update_usd_rate(orders.usd_rate);
					new_sell_order_entry.order_id = orders.order_id;
					frame_support::log::info!(" new_sell_order_entry {:?}", new_sell_order_entry);
					let _ = crate::trade::orderbook::OrderBook::<T, T::AccountId>::push_last_new_order(
						&req_data,
						new_sell_order_entry,
					);
				}
			}

			None
		});

		let _ = v1::BuyOrderIdList::<T>::translate::<VecDeque<migration::types::v1::GlobalOrders>, _>(
			|buyorders_data: Option<VecDeque<migration::types::v1::GlobalOrders>>| {
				//frame_support::log::info!(" Migrated buyorders_data for {:?}", buyorders_data);

				if let Some(seller_order_ids) = buyorders_data {
					for id in seller_order_ids {
						let orders = v1::OrderBookList::<T>::get(&id.order_id).unwrap();
						let mut req_data = trade::types::TradeRequest::new(
							orders.crypto_type,
							TradeType::Buy,
							orders.address,
							orders.amount,
						);
						req_data.insert_crypto_address(Some(orders.crypto_address));
						let mut new_buy_order_entry =
							OrderBookStruct::new(&req_data, 0_u64);
						new_buy_order_entry.update_amount(orders.amount);
						new_buy_order_entry.order_id = orders.order_id;
						let _ = crate::trade::orderbook::OrderBook::<T, T::AccountId>::push_last_new_order(
							&req_data,
							new_buy_order_entry,
						);
					}
				}

				None
			},
		);

		let _ = v1::TotalOrdersCountTillDate::<T>::translate::<i128, _>(
			|total_orders_count: Option<i128>| {
				frame_support::log::info!(
					" Migrated TotalOrdersCountTillDate for {:?}",
					total_orders_count
				);

				if let Some(orders_count) = total_orders_count {
					let order_u256 = U256::from(orders_count);
					frame_support::log::info!(
						" Migrated TotalOrdersCountTillDate order_u256: for {:?}",
						order_u256
					);
					TotalOrdersCountTillDate::<T>::put(order_u256)
				}

				None
			},
		);

		// Update storage version.
		PalletStorageVersion::<T>::insert(
			migration::types::MigrationType::TradeOrders,
			migration::types::StorageVersion::V2_0_0,
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
