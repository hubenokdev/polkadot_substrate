
use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use sp_std::{ prelude::*, str};
use sp_runtime::{
	RuntimeDebug,
};
use primitive_types::U256;
use scale_info::TypeInfo;
use frame_support::pallet_prelude::MaxEncodedLen;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
pub struct VtbdexPayload<Public, BlockNumber> {
	pub block_number: BlockNumber,
	pub price: u32,
	pub public: Public,
}

#[derive( Encode, Decode, Copy, Clone, Debug, Default, TypeInfo, MaxEncodedLen)]
pub struct ToClaimUserBalance {
	pub token_amount: U256,
	pub to_update_period: u64
}

#[derive( Encode, Decode, Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen)]
pub struct Globals {
	pub transaction_count: u64,
	pub controlled: U256,
	pub backing_reserve: U256,
	pub hours_elapsed: u32,
	pub total_hours: u32,
	pub target_rate_for_year: u32,
	pub start_year: u32,
	pub current_year: u32
}

#[derive( Encode, Decode, Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, TypeInfo)]
pub struct FeeCollectorAccount<AccountId> {
	pub fee_collector_address: AccountId,
	pub fee: U256,
}

#[derive( Encode, Decode, Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen)]
pub struct FutureEstimatedTimeList<T> {
  pub distribution_timestamp: T,
  pub apr_timestamp: T,
}

#[derive( Encode, Decode, Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, TypeInfo, MaxEncodedLen)]
pub struct AprEstimation {
  pub year: u32,
  pub target_rate: u32,
  pub start_rate: U256,
  pub achieve_rate: U256,
  pub ta: U256,
}

pub enum cryptoFunctionality {
	Deposit,
	Withdraw,
	Onboarding,
	All, 
	None,
}

pub enum StopFeatues {
	Eth,
	Eos,
	Vtbdex
}


#[derive( Encode, Decode, Debug, Serialize, Deserialize, Clone, PartialEq, Eq, TypeInfo, MaxEncodedLen)]
pub enum StopData {
	Eth {deposit: bool, onboarding: bool, withdraw: bool},
	Eos {deposit: bool, onboarding: bool, withdraw: bool},
	Vtbdex {buy: bool, sell: bool, issue_vtbt: bool, burn_vtbt: bool, transfer_vtbt: bool},
	All
}


