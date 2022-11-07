use codec::{Decode, Encode};
use primitive_types::U256;
use scale_info::TypeInfo;
use sp_std::{
	prelude::*, str,
	collections::{btree_map::BTreeMap}
};
use serde::__private::ToString;
use crate::{Vec, trade::types::TradeType, TokenType} ;

#[derive( Encode, Decode, Debug, Clone, PartialEq, Eq, Default, TypeInfo)]
pub struct User {
	pub polkadot_address: Option<Vec<u8>>,
	pub vtbc_balance: U256,
	pub vtbt_balance: U256,
	pub sells_journal_balance: U256,
	pub active: bool,
	pub latest_period: u64,
	//Here TokenType must be only CryptosType such as Eth/Eos/Usdc
	pub crypto_addresses: BTreeMap<TokenType, UserCryptoBasedData>,
	//Last observed user balance in an accounting period 
	pub vtbc_period_balance: BTreeMap<u64, Balances>, 
	//The next distribution of *this asset* to process for the user.
	//Here TokenType must be only Distributable tokens such as Eth/Eos/Usdc/Vtbc 
	pub processing_distribution_index: BTreeMap<TokenType, u64>,   
}

#[derive( Encode, Decode, Debug, Clone, Copy, PartialEq, Eq, Default, TypeInfo)]
pub struct Balances {
	// Last observed user balance in an accounting period
	pub balance: U256,    
	// Additional funds controlled by the user in sells journal
	pub controlled: U256,  
	// To maintain counter for claim of cryptos
	// It will be used to remove balances for old claimed period
	pub counter: u32,
}

pub type UserWalletDetail = (Option<Vec<u8>>, U256, bool, U256);

#[derive( Encode, Decode, Debug, Clone, PartialEq, Eq, Default, TypeInfo)]
pub struct UserCryptoBasedData {
    //Name of the crypto network Eth/Eos/Usdc
	pub crypto_network: Vec<u8>,
	//linked crypto_address/crypto_Account_name, if it is not linked yet than None
	pub crypto_address: Option<Vec<u8>>,
	//Crypto deposit balance
	pub deposit_balance: U256,
	//Deposit Amount present in buy journal
	pub buy_journal_balance: U256,
	//List of sell/buy journal with orderid -> index
	pub order: BTreeMap<TradeType, BTreeMap<Vec<u8>, u64>>,   
}

impl User {
	pub fn new(pdot_address: Option<&str>, current_period: u64) -> User {
		
		User {
			polkadot_address: if let Some(address) = pdot_address {
				Some(address.to_string().into_bytes())
			}else {
				None
			},
			vtbc_balance: U256::from(0_u8),
			vtbt_balance: U256::from(0_u8),
			sells_journal_balance: U256::from(0_u8),
			crypto_addresses: BTreeMap::new(),
			active: true,
			latest_period: current_period,
			processing_distribution_index: Self::new_distribution_index(current_period),
			vtbc_period_balance: Self::insert_vtbc_period_balance(current_period),
		}
	}

	pub fn new_distribution_index(current_period: u64) -> BTreeMap<TokenType, u64> {
		let mut distribution_index = BTreeMap::new();
	
		for assets in TokenType::_distributable_iterator() {
			distribution_index.insert(*assets, current_period);
		}
		
		distribution_index
	}

	pub fn insert_vtbc_period_balance(current_period: u64) -> BTreeMap<u64, Balances> {

		let mut vtbc_period_index = BTreeMap::new();
		vtbc_period_index.insert(current_period, Balances::new());

		vtbc_period_index
	}
	
	pub fn update_crypto_details(&mut self, token_type: TokenType, crypto_address: Option<&str>) {
		let crypto_detail = if let Some(address) = crypto_address { UserCryptoBasedData::new(token_type, address)}
		else {
			UserCryptoBasedData::new_for_none_crypto_address(token_type)
		};
		
		self.crypto_addresses.insert(token_type, crypto_detail);
	}
}

impl Balances {
	pub fn new() -> Balances {
		Balances {
            balance:  U256::from(0_u8),
            controlled:  U256::from(0_u8),   
			counter: 0,
        }
	}
}

impl UserCryptoBasedData {
	pub fn new(token_type: TokenType, user_crypto_addr: &str) -> UserCryptoBasedData {
		let crypto_bytes = token_type.to_string().into_bytes();
		let user_crypto_addr_lc_1 = user_crypto_addr.to_lowercase();
		let user_crypto_addr_lc = user_crypto_addr_lc_1.as_bytes().to_vec();

		let mut orders = BTreeMap::new();
		for trade_type in TradeType::_iterator() {
			orders.insert(*trade_type, BTreeMap::new());
		}
		
		UserCryptoBasedData {
			crypto_network: crypto_bytes,
			crypto_address: Some(user_crypto_addr_lc),
			deposit_balance: U256::from(0_u8),
			buy_journal_balance: U256::from(0_u8),
			order: orders,
		}
	}

	pub fn new_data(token_type: TokenType, user_crypto_addr: Option<Vec<u8>>) -> UserCryptoBasedData {
		
		let mut orders = BTreeMap::new();
		for trade_type in TradeType::_iterator() {
			orders.insert(*trade_type, BTreeMap::new());
		}
		let crypto_bytes = token_type.to_string().into_bytes();
		UserCryptoBasedData {
			crypto_network: crypto_bytes,
			crypto_address: user_crypto_addr,
			deposit_balance: U256::from(0_u8),
			buy_journal_balance: U256::from(0_u8),
			order: orders,
		}
	}


	pub fn new_for_none_crypto_address(token_type: TokenType) -> UserCryptoBasedData {
		let crypto_bytes = token_type.to_string().into_bytes();
		let mut orders = BTreeMap::new();
		for trade_type in TradeType::_iterator() {
			orders.insert(*trade_type, BTreeMap::new());
		}
		
		UserCryptoBasedData {
			crypto_network: crypto_bytes,
			crypto_address: None,
			deposit_balance: U256::from(0_u8),
			buy_journal_balance: U256::from(0_u8),
			order: orders,
		}
	}
}