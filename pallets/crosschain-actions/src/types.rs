/** Module name: custom_type
 *  Description: Here all userdefined type is declared which will be used within this crypto_vtb_contract pallet
 **/

 use codec::{Decode, Encode};
 use serde::{Deserialize};
 use sp_std::{ prelude::*, collections::vec_deque::VecDeque, str};
 use sp_runtime::{
	RuntimeDebug,
 };
  
/// Payload used by this crate to hold price
/// data required to submit a transaction.
#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
pub struct VtbContractPayload<Public, BlockNumber> {
	pub block_number: BlockNumber,
	pub public: Public,
}
 
/// Crypto network(e.g Eth, Eos)
/// Nth current block record
#[derive(Deserialize, Encode, Decode, Default, RuntimeDebug, scale_info::TypeInfo)]
pub struct CryptoCurrentBlockRecord {
	pub blocks: VecDeque<u64>,
}
 
/// Crypto network(e.g Eth, Eos)
/// Nth processed block record
#[derive(Deserialize, Encode, Decode, Default, RuntimeDebug, scale_info::TypeInfo)]
pub struct CryptoProcessedRange {
	pub range_req: VecDeque<u64>,
}
 
/// Crypto network(e.g Eth, Eos)
/// Nth block record which have events
#[derive(Deserialize, Encode, Decode, Default, RuntimeDebug, scale_info::TypeInfo)]
pub struct CryptoProcessedBlocksWithLogs {
	pub blocks: Vec<u64>,
}
