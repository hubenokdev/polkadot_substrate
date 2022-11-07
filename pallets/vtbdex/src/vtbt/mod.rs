//! mod vtbt is decalared to handle all different types of Vtbt ERC20 token supported via vtbdex
//! ERC20 token feature include
//! Transfer/ Transfer from / Mint / Burn

// pub mod erc20_eth;
pub mod erc20_substrate;
use frame_support::{
	dispatch::{ DispatchResult}
};
use crate::{Config, U256};

///Trait VTBTErc20<T> is design for all the available features for VtbtErc20 token
/// This trait 7 methods implemenatation to support main ERC20 feature
/// such as Mint, Burn, Transfer, TransferFrom
pub trait VTBTErc20<T: Config> {
	fn initiate_convert_vtbc_to_vtbt_erc20substrate_token(&mut self) -> DispatchResult;
	fn mint_vtbt_substrate_state_update(&self) -> DispatchResult;
    fn initiate_convert_vtbt_to_vtbc_erc20substrate_token(&self) -> DispatchResult;
	fn burn_vtbt_substrate_state_update(&self) -> DispatchResult;
	fn initiate_transfer_of_vtbt_erc20substrate_token(&self) -> DispatchResult;
	fn initiate_transfer_from_of_vtbt_erc20substrate_token(&self, signer: T::AccountId) -> DispatchResult;
	fn transfer_vtbt_substrate_state_update(&self) -> DispatchResult;
}

///VtbtErc20Req<T, U>: This is custom struct designed for transaction related to VTBT token request
pub(crate) struct VtbtErc20Req<T, U> {
	origin: U, 
    account: T,
    vtbtamount: U256,
    vtbcamount: U256,
	account2: Option<T>,
}

/// VtbtErc20Req<T, U> Struct implementation
/// It has only one method to create new data.
impl<T, U> VtbtErc20Req<T, U> 
where {
    pub(crate) fn new(origin: U, account: T, vtbtamount: U256, account2: Option<T>) -> VtbtErc20Req<T, U> 
	{
        VtbtErc20Req {
			origin,
			account,
			vtbtamount,
			vtbcamount: U256::from(0_u8),
			account2
		}
    }
}