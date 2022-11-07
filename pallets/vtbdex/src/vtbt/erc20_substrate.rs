

///mod erc20_substrate is responsible for all transaction related to VTBT ERC20 token in substrate.
use frame_support::{
	dispatch::{ DispatchResult},  
	ensure, traits::Get
};
use crate::{
	U256, Pallet, Config, Error, Encode,
	Event, Globals, UserWallet,
	TokenType, VTBTErc20, VtbtErc20Req,
	users::{WalletTrait, WalletUpdateReq}
};

impl<'a, T: Config, U> VTBTErc20<T> for VtbtErc20Req<T::AccountId, U> 
where 
T: frame_system::Config<Origin = U>, 
U: sp_std::clone::Clone {

	/// initiate_convert_vtbc_to_vtbt_erc20substrate_token: It take one parameter of self.
	/// This will mint given amount of VTBT token,
	/// And will add the VTBT amount and will substratct the equivalent amount of VTBC in user's account.
	/// Also the user will be charge to pay substrate trasnaction fee from the ETH/EOS crypto token balance.
	fn initiate_convert_vtbc_to_vtbt_erc20substrate_token(&mut self) -> DispatchResult {

		let user = <UserWallet<T>>::get(&self.account.encode());
		self.vtbcamount = <Pallet<T>>::convert_vtbt_to_vtbc(self.vtbtamount)?;
        ensure!(user.vtbc_balance >= self.vtbcamount , Error::<T>::InsufficientFunds);
		ensure!(user.active , Error::<T>::UserWalletIsLocked);	
		let account_key = self.account.encode();		
		let req = WalletUpdateReq::new(None, &account_key, None, None );
		WalletTrait::<T, T::AccountId>::pay_trnx_fee_based_on_crpto_available_with_account(&req, &self.account, "Convert vtbc to vtbt")?;	
		<Pallet<T>>::deposit_event(Event::MintSubstrateErc20VTBtInitiated {
			user: self.account.clone(), 
			vtbt_amount: self.vtbtamount
		});
		
		let assetid: T::AssetId = T::VtbErc20AssetId::get();
		let _ = <pallet_vtbt::Pallet<T>>::mint(self.origin.clone(), assetid, self.account.clone(), self.vtbtamount)?;
	    //self.mint_vtbt_substrate_state_update()?;
		VTBTErc20::<T>::mint_vtbt_substrate_state_update(self)?;
		Ok(())
	}

	/// fn mint_vtbt_substrate_state_update: This function is for vtbt/vtbc balance update in the user account.
	/// This will be internaly called by method initiate_convert_vtbc_to_vtbt_erc20substrate_token 
	fn mint_vtbt_substrate_state_update(&self) -> DispatchResult {
		match <Pallet<T>>::add_update_balance(&TokenType::Vtbt, &self.account.encode(), self.vtbtamount, U256::from(0_u8)) {
			Ok(()) => {
				match <Pallet<T>>::sub_update_balance(&TokenType::Vtbc, &self.account.encode(), self.vtbcamount, U256::from(0_u8)) {
					Ok(()) => {
						Globals::<T>::mutate(|get_globals| -> DispatchResult{
							get_globals.backing_reserve = get_globals.backing_reserve.checked_add(self.vtbcamount).ok_or(Error::<T>::NumberIsTooBigForU256)?;
							Ok(())
						})?;
						<Pallet<T>>::sub_circulation_token_balance(&TokenType::Vtbc, self.vtbcamount)?;  
						<Pallet<T>>::deposit_event(Event::ConvertVtbcToVtbtSuccess {
							user: self.account.clone(), 
							vtbc_amount: self.vtbcamount, 
							vtbt_amount: self.vtbtamount 
						});
						Ok(())
					},
					Err(err) =>  Err(frame_support::dispatch::DispatchError::from(err)), 
				}
			},
			Err(err) =>  Err(frame_support::dispatch::DispatchError::from(err)), 
		}
	}

	///initiate_convert_vtbt_to_vtbc_erc20substrate_token: It take one parameter of self.
	/// This will burn given amount of VTBT token,
	/// And will subtract the VTBT amount and will add the equivalent amount of VTBC in user's account.
	/// Also the user will be charge to pay substrate trasnaction fee from the ETH/EOS crypto token balance.
    fn initiate_convert_vtbt_to_vtbc_erc20substrate_token(&self) -> DispatchResult {
		let user = <UserWallet<T>>::get(&self.account.encode());
        ensure!(user.vtbt_balance >= self.vtbtamount , Error::<T>::InsufficientFunds); 
		ensure!(user.active , Error::<T>::UserWalletIsLocked);
		let account_key = self.account.encode();	
		let req = WalletUpdateReq::new(None, &account_key, None, None );
		WalletTrait::<T, T::AccountId>::pay_trnx_fee_based_on_crpto_available_with_account(&req, &self.account, "Convert vtbt to vtbc")?;
		<Pallet<T>>::deposit_event(Event::BurnSubstrateErc20VTBtInitiated {
			user: self.account.clone(), 
			vtbt_amount: self.vtbtamount
		});	

		let assetid: T::AssetId = T::VtbErc20AssetId::get();
		let _ = <pallet_vtbt::Pallet<T>>::burn(self.origin.clone(), assetid, self.account.clone(), self.vtbtamount);
		VTBTErc20::<T>::burn_vtbt_substrate_state_update(self)?;
		Ok(())
	}

	/// fn burn_vtbt_substrate_state_update: This function is for vtbt/vtbc balance update in the user account.
	/// This will be internaly called by method initiate_convert_vtbt_to_vtbc_erc20substrate_token 
	fn burn_vtbt_substrate_state_update(&self) -> DispatchResult {
		match <Pallet<T>>::sub_update_balance(&TokenType::Vtbt, &self.account.encode(), self.vtbtamount, U256::from(0_u8)) {
			Ok(()) => {
				let vtbc_amount: U256 = <Pallet<T>>::convert_vtbt_to_vtbc(self.vtbtamount)?;
				match <Pallet<T>>::add_update_balance(&TokenType::Vtbc, &self.account.encode(), vtbc_amount, U256::from(0_u8)) {
					Ok(()) => {
						Globals::<T>::mutate(|get_globals| -> DispatchResult {
							get_globals.backing_reserve = get_globals.backing_reserve.checked_sub(vtbc_amount).ok_or(Error::<T>::NumberIsTooLowForU256)?;
							Ok(())
						})?;
						<Pallet<T>>::add_circulation_token_balance(&TokenType::Vtbc, vtbc_amount)?;  
						<Pallet<T>>::deposit_event(Event::ConvertVtbtToVtbcSuccess {
							user: self.account.clone(), 
							vtbt_amount: self.vtbtamount, 
							vtbc_amount
						});
						Ok(())
					},
					Err(err) =>  Err(frame_support::dispatch::DispatchError::from(err)), 
				}
			},
			Err(err) =>  Err(frame_support::dispatch::DispatchError::from(err)), 
		}
	}

	///initiate_transfer_of_vtbt_erc20substrate_token: It take two parameter (self).
	/// This will transfer given amount of VTBT from sender account to receiver account,
	/// Also the sender will be charge to pay substrate transaction fee from the ETH/EOS crypto token balance.
	fn initiate_transfer_of_vtbt_erc20substrate_token(&self) -> DispatchResult {
		let to_address = self.account2.as_ref().ok_or(Error::<T>::NoneValue)?;
		let from_user = <UserWallet<T>>::get(&self.account.encode());
        let to_user = <UserWallet<T>>::get(&to_address.encode());
        ensure!(from_user.vtbt_balance >= self.vtbtamount , Error::<T>::InsufficientFunds);
		ensure!(from_user.active , Error::<T>::UserWalletIsLocked);	
        ensure!(to_user.active , Error::<T>::UserWalletIsLocked);
		let account_key = self.account.encode();	
		let req = WalletUpdateReq::new(None, &account_key, None, None );
        WalletTrait::<T, T::AccountId>::pay_trnx_fee_based_on_crpto_available_with_account(&req, &self.account, "Transfer vtbt")?;	
		<Pallet<T>>::deposit_event(Event::TransferSubstrateErc20VTBtInitiated {
			sender_address: self.account.clone(), 
			receiver_address: to_address.clone(),
			vtbt_amount: self.vtbtamount
		});

		let assetid: T::AssetId = T::VtbErc20AssetId::get();
		let _ = <pallet_vtbt::Pallet<T>>::transfer(self.origin.clone(), assetid, to_address.clone(), self.vtbtamount);
		VTBTErc20::<T>::transfer_vtbt_substrate_state_update(self)?;
		Ok(())
	}

	///initiate_transfer_from_of_vtbt_erc20substrate_token: It take two parameter (self, signer_address: T::AccountId).
	/// This will transfer given amount of VTBT from sender account to receiver account,
	/// Also the signer will be charge to pay substrate trasnaction fee from the ETH/EOS crypto token balance in place of sender.
	fn initiate_transfer_from_of_vtbt_erc20substrate_token(&self, signer: T::AccountId) -> DispatchResult {
		let to_address = self.account2.as_ref().ok_or(Error::<T>::NoneValue)?;
        let from_user = <UserWallet<T>>::get(&self.account.encode());
        let to_user = <UserWallet<T>>::get(&to_address.encode());
		
        ensure!(from_user.vtbt_balance >= self.vtbtamount , Error::<T>::InsufficientFunds);
		ensure!(from_user.active , Error::<T>::UserWalletIsLocked);	
        ensure!(to_user.active , Error::<T>::UserWalletIsLocked);
		let account_key = signer.encode();
		let req = WalletUpdateReq::new(None, &account_key, None, None );
        WalletTrait::<T, T::AccountId>::pay_trnx_fee_based_on_crpto_available_with_account(&req, &self.account, "Transfer vtbt")?;	
		<Pallet<T>>::deposit_event(Event::TransferSubstrateErc20VTBtInitiated {
			sender_address: self.account.clone(), 
			receiver_address: to_address.clone(), 
			vtbt_amount: self.vtbtamount
		});		
		
		let assetid: T::AssetId = T::VtbErc20AssetId::get();
		let _ = <pallet_vtbt::Pallet<T>>::transfer_from(self.origin.clone(), assetid, self.account.clone(), to_address.clone(), self.vtbtamount);
		VTBTErc20::<T>::transfer_vtbt_substrate_state_update(self)?;
		Ok(())
	}

	/// fn transfer_vtbt_substrate_state_update: This function is for vtbt/vtbc balance update in the user account.
	/// This will be internaly called by method initiate_transfer_from_of_vtbt_erc20substrate_token 
	fn transfer_vtbt_substrate_state_update(&self) -> DispatchResult {
		let to_address = self.account2.as_ref().ok_or(Error::<T>::NoneValue)?;
		match <Pallet<T>>::sub_update_balance(&TokenType::Vtbt, &self.account.encode(), self.vtbtamount, U256::from(0_u8)) {
			Ok(()) => {
				match <Pallet<T>>::add_update_balance(&TokenType::Vtbt, &to_address.encode(), self.vtbtamount, U256::from(0_u8)) {
					Ok(()) => {
						<Pallet<T>>::deposit_event(Event::TransferVtbtErc20Success {
							sender_address: self.account.clone(), 
							receiver_address: to_address.clone(), 
							vtbt_amount: self.vtbtamount
						});
						Ok(())
					},
					Err(err) =>  Err(frame_support::dispatch::DispatchError::from(err)), 
				}
			},
			Err(err) =>  Err(frame_support::dispatch::DispatchError::from(err)), 
		}
	}
}