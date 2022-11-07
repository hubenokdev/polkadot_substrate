use serde::{Deserialize, Serialize};
use crate::*;
use sp_std::{
    str,
	collections::{btree_map::BTreeMap}
};

#[derive( Encode, Decode, Debug, Clone, PartialEq,  Serialize, Deserialize, Eq, Ord, PartialOrd, scale_info::TypeInfo)]
pub enum MigrationType {
	User,
	HodldexData,
	TradeOrders,
	Claim,
	Circulation,
	WithdrawCount,
	Others,
}

/// Utility type for managing upgrades/migrations.
#[derive(codec::Encode, codec::Decode, Clone, frame_support::RuntimeDebug, PartialEq, scale_info::TypeInfo)]
pub enum StorageVersion {
	//Old version
	V1_0_0,
	//New version
	V2_0_0,
	//Upcoming version
	V3_0_0,
}

pub mod v1 {
	use super::*;
	#[derive( Encode, Decode, Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default, scale_info::TypeInfo)]
	pub struct CryptoDetail{
		pub crypto_network: Vec<u8>,
		// pub crypto_addresses: Vec<u8>,
		pub crypto_addresses: Option<Vec<u8>>,
		pub deposit_balance: U256,
	}

	#[derive( Encode, Decode, Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default, scale_info::TypeInfo)]
	pub struct DeprecatedWallet {
		pub polkadot_address: Vec<u8>,
		pub vtbc_balance: U256,
		pub vtbt_balance: U256,
		pub controlled: U256,
		pub crypto_addresses: BTreeMap<TokenType, CryptoDetail>,
		pub active: bool,
	}

	#[derive( Encode, Decode, Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, scale_info::TypeInfo)]
	pub struct UserBalance {
		pub balance: U256,    // Last observed user balance in an accounting period
		pub controlled: U256,  // Additional funds controlled the the user
	}

	#[derive( Encode, Decode, Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, scale_info::TypeInfo)]
	pub struct User {
		pub user_balance: BTreeMap<u64, UserBalance>, //Last observed user balance in an accounting period 
		pub processing_distribution_index: u64,  // The next distribution of *this asset* to process for the user.
		pub processing_balance_index: u64,       // The *distributableAsset* balance record to use to compute user shares for the next distribution.
	}

	#[derive( Encode, Decode, Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, scale_info::TypeInfo)]
	pub struct MigratedUser {
		pub eth_balance: U256,
		pub eos_balance: U256,
		pub vtbc_balance: U256,
		pub controlled_vtbc: U256,
	}

	//sell orders
	#[derive( Encode, Decode, Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, scale_info::TypeInfo)]
	pub struct GlobalOrders {
		pub order_id: Vec<u8>,
		pub crypto_type: TokenType,
		pub usd_rate: U256, 
	}

	#[derive( Encode, Decode, Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, scale_info::TypeInfo)]
	pub struct OrderBook<T> {
		pub order_id: Vec<u8>,
		pub order_type: Vec<u8>,
		pub address: T,
		pub crypto_type: TokenType,
		pub crypto_address: Vec<u8>,
		pub amount: U256,
		pub usd_rate: U256, 
	}

	#[derive( Encode, Decode, Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, scale_info::TypeInfo)]
	pub struct HodldexOrderBook {
		pub order_id: Vec<u8>,
		pub order_type: Vec<u8>,
		pub crypto_type: Vec<u8>,
		pub crypto_address: Vec<u8>,
		pub amount: U256,
		pub usd_rate: U256, 
	}

	#[derive( Encode, Decode, Debug, Clone, PartialEq, Serialize, Deserialize, Eq, Default, scale_info::TypeInfo)]
	pub struct ToClaimUserBalance {
		pub eth: U256,
		pub eos: U256,
		pub vtbc: U256,
		pub to_update_period: u64
	}

	#[derive( Encode, Decode, Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
	pub struct Circulation {
		pub eth_deposit_amount: U256,
		pub eos_deposit_amount: U256,
		pub vtbc_amount: U256,
	}
}