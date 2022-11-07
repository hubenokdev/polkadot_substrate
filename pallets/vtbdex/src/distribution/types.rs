use crate::{TokenType, U256};
use codec::{Decode, Encode};
use serde::{Deserialize, Serialize};
use sp_std::collections::btree_map::BTreeMap;
use scale_info::TypeInfo;

#[derive( Encode, Decode, Debug, Clone, PartialEq, Eq, Default, TypeInfo)]
  pub struct Distribution<Moment> {
  // Vtbc in Circulation at the end of the specific period
  pub denominator: U256, 
  pub balances: BTreeMap<TokenType, Balances>,
  pub inittimestamp: Moment,
  pub closed: bool
}
    
#[derive( Encode, Decode, Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default, TypeInfo)]
pub struct Balances {
    // balance of asset at the end of the period, used for calcs
    pub total_balance: U256, 
    // running balance of the asset, subtract for claim
    pub current_balance : U256, 
}