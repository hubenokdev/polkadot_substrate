//! Functions for the Assets pallet.

use super::*;
use frame_support::{dispatch::{ DispatchResult, DispatchError},  ensure};
// implementation of mudule
// utility and private functions
// if marked public, accessible by other modules
impl<T: Config> Pallet<T> {
    // the ERC20 standard transfer function
    // internal
    pub(super) fn _transfer(
        token_id: T::AssetId,
        from: T::AccountId,
        to: T::AccountId,
        value: U256,
    ) -> Result<(), DispatchError> {
        ensure!(<BalanceOf<T>>::contains_key(token_id, from.clone()), Error::<T>::AccountDoesNotOwnThisToken);
        let sender_balance = Self::balance_of(token_id, from.clone());
		ensure!(sender_balance >= value, Error::<T>::NotEnoughBalance);
        let updated_from_balance = sender_balance.checked_sub(value).ok_or(Error::<T>::OverflowInCalculatingBalance)?;
        let receiver_balance = Self::balance_of(token_id, to.clone());
        let updated_to_balance = receiver_balance.checked_add(value).ok_or(Error::<T>::OverflowInCalculatingBalance)?;
        
        // reduce sender's balance
        <BalanceOf<T>>::insert(token_id, from.clone(), updated_from_balance);

        // increase receiver's balance
        <BalanceOf<T>>::insert(token_id, to.clone(), updated_to_balance);

        Self::deposit_event(Event::Transferred{ asset_id: token_id, from, to, amount: value });
        Ok(())
    }

	/// Increases the asset `id` balance of `beneficiary` by `amount`.
	///
	/// This alters the registered supply of the asset and emits an event.
	///
	/// Will return an error or will increase the amount by exactly `amount`.
	pub(super) fn do_mint(
		id: T::AssetId,
		beneficiary: &T::AccountId,
		amount: U256,
		_maybe_check_issuer: Option<T::AccountId>,
	) -> DispatchResult {
		Self::increase_balance(id, beneficiary, amount, |details| -> DispatchResult {
			// if let Some(check_issuer) = maybe_check_issuer {
			// 	ensure!(&check_issuer == &details.issuer, Error::<T>::NoPermission);
			// }
			
			details.supply = details.supply.saturating_add(amount);
			Ok(())
		})?;
		Self::deposit_event(Event::Issued{asset_id: id, owner: beneficiary.clone(), balance: amount });
		Ok(())
	}


	/// Increases the asset `id` balance of `beneficiary` by `amount`.
	///
	/// LOW-LEVEL: Does not alter the supply of asset or emit an event. Use `do_mint` if you need
	/// that. This is not intended to be used alone.
	///
	/// Will return an error or will increase the amount by exactly `amount`.
	pub(super) fn increase_balance(
		id: T::AssetId,
		beneficiary: &T::AccountId,
		amount: U256,
		check: impl FnOnce(
			&mut AssetDetails<U256, T::AccountId>,
		) -> DispatchResult,
	) -> DispatchResult {
		if amount.is_zero() {
			return Ok(())
		}
		Asset::<T>::try_mutate(id, |maybe_details| -> DispatchResult {
			let details = maybe_details.as_mut().ok_or(Error::<T>::Unknown)?;

			check(details)?;

			<BalanceOf<T>>::try_mutate(id, beneficiary, |t| -> DispatchResult {
				
				let new_balance = t.saturating_add(amount);
				if t.is_zero() {
					Self::new_account(beneficiary, details)?;
				}
				*t = new_balance;
				Ok(())
			})?;
			Ok(())
		 })?;
		Ok(())
	}

	pub(super) fn new_account(
		_who: &T::AccountId,
		d: &mut AssetDetails<U256, T::AccountId>,
	) -> Result<(), DispatchError> {
		let accounts = d.accounts.checked_add(1).ok_or(ArithmeticError::Overflow)?;
		d.accounts = accounts;
		Ok(())
	}
	
	/// Reduces asset `id` balance of `target` by `amount`. Flags `f` can be given to alter whether
	/// it attempts a `best_effort` or makes sure to `keep_alive` the account.
	///
	/// This alters the registered supply of the asset and emits an event.
	///
	/// Will return an error and do nothing or will decrease the amount and return the amount
	/// reduced by.
	pub(super) fn do_burn(
		id: T::AssetId,
		target: &T::AccountId,
		amount: U256,
		_maybe_check_admin: Option<T::AccountId>,
	) -> Result<U256, DispatchError> {
		let actual = Self::decrease_balance(id, target, amount, |actual, details| {
			// Check admin rights.
			// if let Some(check_admin) = maybe_check_admin {
			// 	ensure!(&check_admin == &details.issuer, Error::<T>::NoPermission);
			// }

			debug_assert!(details.supply >= actual, "checked in prep; qed");
			details.supply = details.supply.saturating_sub(actual);

			Ok(())
		})?;
		Self::deposit_event(Event::Burned { asset_id: id, owner: target.clone(), balance: actual });
		Ok(actual)
	}

	/// Reduces asset `id` balance of `target` by `amount`. Flags `f` can be given to alter whether
	/// it attempts a `best_effort` or makes sure to `keep_alive` the account.
	///
	/// LOW-LEVEL: Does not alter the supply of asset or emit an event. Use `do_burn` if you need
	/// that. This is not intended to be used alone.
	///
	/// Will return an error and do nothing or will decrease the amount and return the amount
	/// reduced by.
	pub(super) fn decrease_balance(
		id: T::AssetId,
		target: &T::AccountId,
		amount: U256,
		check: impl FnOnce(
			U256,
			&mut AssetDetails<U256, T::AccountId>,
		) -> DispatchResult,
	) -> Result<U256, DispatchError> {
		if amount.is_zero() {
			return Ok(amount)
		}

		Asset::<T>::try_mutate(id, |maybe_details| -> DispatchResult {
			let details = maybe_details.as_mut().ok_or(Error::<T>::Unknown)?;

			check(amount, details)?;

			<BalanceOf<T>>::try_mutate_exists(id, target, |maybe_account| -> DispatchResult {
				let mut balance = maybe_account.take().unwrap_or_default();
				debug_assert!(balance >= amount, "checked in prep; qed");
				assert!(balance >= amount, "Insufficient to burn: {:?}=={:?}", balance, amount);
				ensure!(balance >= amount, "Insufficient to burn");
				// Make the debit.
				balance = balance.saturating_sub(amount);
				*maybe_account = if balance.is_zero() {
					debug_assert!(balance.is_zero(), "checked in prep; qed");
					details.accounts = details.accounts.saturating_sub(1);
					None
				} else {
					Some(balance)
				};
				Ok(())
			})?;

			Ok(())
		})?;

		Ok(amount)
	}

	pub fn initialize_vtbt_token (id: T::AssetId, addess: T::AccountId) {
		if !Self::is_init() {
			let token = Erc20Token {
				name:   "vtbt-token".as_bytes().to_vec(),
				symbol: "Vtbt".as_bytes().to_vec(),
				total_supply: U256::from(0_u64),
				decimals: 18
			};

			Asset::<T>::insert(
				id,
				AssetDetails {
					issuer: addess,
					supply: U256::from(0_u64),
					accounts: 0,
					asset_info: token,
				},
			);

			<InitVtbtErc20Token<T>>::put(true);
		};
	}

}
