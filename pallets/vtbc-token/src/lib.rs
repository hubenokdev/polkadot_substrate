// This file is part of Substrate.
#![cfg_attr(not(feature = "std"), no_std)]

use primitive_types::U256;
use sp_std::{ str};
use sp_std::convert::TryInto;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	
	/// This pallet's configuration trait
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Attempted to transfer more funds than available
		InsufficientFunds,
	}

	/// Events for the pallet.
	#[pallet::event]
	#[pallet::generate_deposit(pub fn deposit_event)]
	pub enum Event<T: Config>{
		/// Token was initialized to user
		IssuedVtbcToken { user: T::AccountId, amount: U256 }, // user address, amount
	}

	// This storage entry defines when new transaction is going to be accepted.
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::type_value]
	pub fn DefaultTotalSupply<T: Config>() -> U256 { U256::from(40_0000_0000_0000_0000_0000_0000_i128) } //40_0000_0000_0000_0000_0000_0000_i128

	#[pallet::storage]
	#[pallet::getter( fn total_supply)]
	pub(super) type TotalSupply<T> = StorageValue<_, U256, ValueQuery, DefaultTotalSupply<T>>; //totalsupply balance of vtbc

	#[pallet::storage]
	#[pallet::getter( fn get_reserve_balance)]
	pub type ReserveBalance<T> = StorageValue<_, U256, ValueQuery, DefaultTotalSupply<T>>; //reserve balance of vtbc

	impl<T: Config> Pallet<T> {
		pub fn issue_vtbc_token(to: T::AccountId, value: U256) -> Result<(), Error<T>>  {

			let reserve_balance = <ReserveBalance<T>>::get();
			let updated_reserve_balance = reserve_balance.checked_sub(value).ok_or(<Error<T>>::InsufficientFunds)?;
	
			<ReserveBalance<T>>::put(updated_reserve_balance);
	
			Self::deposit_event(Event::IssuedVtbcToken {user: to, amount: value });
	
			Ok(())
		}
	}
}