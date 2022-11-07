//! Benchmarking setup for pallet-template


use super::*;
use frame_benchmarking::{
	account, whitelisted_caller, impl_benchmark_test_suite, benchmarks,
};
use frame_system::RawOrigin as SystemOrigin;
use sp_std::prelude::*;
use sp_runtime::traits::StaticLookup;

use crate::Pallet as Assets;

const SEED: u32 = 0;

fn create_default_asset<T: Config>(
	_is_sufficient: bool,
) -> (T::AccountId, T::AccountId) {
	let caller: T::AccountId = whitelisted_caller();

    Assets::<T>::initialize_vtbt_token(Default::default(), caller.clone());
	(caller.clone(), caller)
}

fn create_default_minted_asset<T: Config>(
	is_sufficient: bool,
	amount: U256,
) -> (T::AccountId, T::AccountId) {
	let (caller, caller_lookup) = create_default_asset::<T>(is_sufficient);
	
	assert!(Assets::<T>::mint(
		SystemOrigin::Signed(caller.clone()).into(),
		Default::default(),
		caller_lookup.clone(),
		amount,
	)
	.is_ok());

	(caller, caller_lookup)
}

fn create_default_allowance_asset<T: Config>(
	is_sufficient: bool,
	spender: T::AccountId,
	amount: U256,
) -> (T::AccountId, T::AccountId) {
	let (caller, caller_lookup) = create_default_asset::<T>(is_sufficient);

	assert!(Assets::<T>::approve(
		SystemOrigin::Signed(caller.clone()).into(),
		Default::default(),
		spender.clone(),
		amount,
	)
	.is_ok());

	(caller, caller_lookup)
}

fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

benchmarks! {
	mint {
		let (caller, caller_lookup) = create_default_asset::<T>(true);
		let amount = U256::from(100u32);
	}: _(SystemOrigin::Signed(caller.clone()), Default::default(), caller_lookup, amount)
	verify {
		assert_last_event::<T>(Event::Issued { asset_id: Default::default(), owner: caller, balance: amount }.into());
	}

	burn {
		// let amount = T::Balance::from(100u32);
		let amount = U256::from(100u32);

		let (caller, caller_lookup) = create_default_minted_asset::<T>(true, U256::from(1000u32));
	}: _(SystemOrigin::Signed(caller.clone()), Default::default(), caller_lookup, amount)
	verify {
		assert_last_event::<T>(Event::Burned { asset_id: Default::default(), owner: caller, balance: amount }.into());
	}

	transfer {
		let amount = U256::from(100u32);
		let (caller, caller_lookup) = create_default_minted_asset::<T>(true, U256::from(1000u32));
		let target: T::AccountId = account("target", 0, SEED);
		let target_lookup = T::Lookup::unlookup(target.clone());
	}: _(SystemOrigin::Signed(caller.clone()), Default::default(), target.clone(), amount)
	verify {
		assert_last_event::<T>(Event::Transferred { asset_id: Default::default(), from: caller, to: target, amount }.into());
	}

    transfer_from {
		let amount = U256::from(100u32);
		let (from, caller_lookup) = create_default_minted_asset::<T>(true, amount);
		let caller: T::AccountId = account("from", 1, SEED);
		let target: T::AccountId = account("target", 0, SEED);
		let target_lookup = T::Lookup::unlookup(target.clone());
		create_default_allowance_asset::<T>(true, caller.clone(), amount);
	}: _(SystemOrigin::Signed(caller.clone()), Default::default(), from.clone(), target.clone(), amount)
	verify {
		assert_last_event::<T>(Event::Transferred { asset_id: Default::default(), from: from, to: target, amount }.into());
	}

	approve {
		let amount = U256::from(100u32);
		let (caller, caller_lookup) = create_default_minted_asset::<T>(true, U256::from(1000u32));
		let target: T::AccountId = account("target", 0, SEED);
		let target_lookup = T::Lookup::unlookup(target.clone());
	}: _(SystemOrigin::Signed(caller.clone()), Default::default(), target.clone(), amount)
	verify {
		assert_last_event::<T>(Event::Approval {asset_id: Default::default(), owner: caller.clone(), spender: target.clone(), amount: amount }.into());
	}
}

impl_benchmark_test_suite!(Assets, crate::mock::new_test_ext(), crate::mock::Test);

// ./target/release/vtb-node benchmark pallet --pallet pallet_vtbt --extrinsic "*"  --output=pallets/vtbt/src/weights1.rs 
  
