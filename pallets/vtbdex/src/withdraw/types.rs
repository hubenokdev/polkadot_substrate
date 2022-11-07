use codec::{Decode, Encode};
use serde::__private::ToString;
use scale_info::TypeInfo;
use crate::{ 
	Pallet, Config,
	U256, TokenType,
	WithdrawCountRecord, WithdrawCrypto,
	StorageValueRef, Vec
};

///WithdrawCryptoReq<T>: This is custom struct designed for transaction related to VTBT token request
#[derive( Encode, Decode, Debug, Default, TypeInfo)]
pub(crate) struct WithdrawCryptoReq<T, U> {
	pub id: Vec<u8>,
    pub account: T,
	pub crypto_type: TokenType,
    pub amount: U256,
	pub crypto_address: Vec<u8>,
	pub block_number: Option<U>,
}

/// WithDrawCryptoReq<T> Struct implementation
/// It has only one method to create new data.
impl<T, U> WithdrawCryptoReq<T, U> 
{
    pub(crate) fn new(account: T, crypto_type: TokenType, amount: U256) -> WithdrawCryptoReq<T, U> 
	{
        WithdrawCryptoReq {
			account,
			crypto_type,
			amount,
			crypto_address: Vec::new(),
			block_number: None,
			id: Vec::new(),
		}
    }

	/// pub(crate) fn check_for_withdraw_indexed_crypto_data<I: Config>(block_number: I::BlockNumber)
	/// This function is invoked via ```fn offchain_worker(block_number: T::BlockNumber)``` from lib.rs
	/// Fetch withdraw_request data from offchain indexed storage data 
	/// and send request for withdraw based on match data.
	pub(crate) fn check_for_withdraw_indexed_crypto_data<I: Config>(block_number: I::BlockNumber) 
	{
		let req_block_num = block_number - 2u32.into();
		for asset in TokenType::_crypto_iterator() {
			let total_request = WithdrawCountRecord::<I>::get((&asset, req_block_num));
			let mut start:u64 = 1;
			while start <= total_request {
				let mut index_key =  b"withdraw_initiate_".to_vec();
				index_key.extend(asset.to_string().as_bytes());
				let key = <Pallet<I>>::derive_index_key(block_number - 2u32.into(), &index_key, start);
				let mut oci_mem = StorageValueRef::persistent(&key);

				let data = oci_mem.get::<WithdrawCryptoReq<I::AccountId, I::BlockNumber>>();
				match data {
					Ok(Some(data)) => {
						log::info!("{:?}", data);
						//Call withdarw task
						let _ = WithdrawCrypto::<I>::crypto_withdraw_call(&data);
						oci_mem.clear();
					},
					Err(_err) => {
						log::info!("Error: Offchain indexed data failed: {:?}", req_block_num);
					},
					_ => log::info!("Info: No off-chain-indexing data present for  withdraw in the block number: {:?}", req_block_num),
				} 
				start += 1;
			}
		}
	}
	
}

/// This is to store all the essential detail about withdraw_request
#[derive( Encode, Decode, Debug, Clone, Default, TypeInfo)]
pub struct WithdrawClaim<AccountId, Moment, Blocknumber> {
	pub polkadot_address: AccountId,
	pub id: Vec<u8>,
	pub token_type: TokenType,
	pub withdraw_amount: U256,
	pub fee: U256,
	pub transaction_hash: Vec<u8>,
	pub timestamp: Moment,
	pub node_block_number: Blocknumber,
	pub requested: bool,
}

///Implement methods for struct.
impl<T, U, I> WithdrawClaim<T, U, I> 
where 
T: sp_std::clone::Clone
{
	pub(crate) fn new(req_data: &WithdrawCryptoReq<T, I>, len: usize, fee: U256, time_stamp: U, blocknumber: I ) ->  WithdrawClaim<T, U, I>  {

		WithdrawClaim {
			polkadot_address: req_data.account.clone(),
			id: len.to_string().as_bytes().to_vec(),
			token_type: req_data.crypto_type,
			withdraw_amount: req_data.amount,
			fee,
			transaction_hash: Vec::new(),
			timestamp: time_stamp,
			node_block_number: blocknumber,
			requested: false
		}
	}
}