//! Module name: distribute
//!   Description: It has code implementation for distribute VTBC/ETH/EOS after completion of distribution period
use frame_support::pallet_prelude::DispatchResult;
use crate::{
    Pallet, Error, Config, Event,
    U256, distribution::types::Balances,
    Period, Distribution, UpdatedTimeList, Globals,
    TokenType, Get, Circulation,
};

pub(crate) struct InitializeDistributionReq<T>(T);

impl<T: Config> InitializeDistributionReq<T> {

    /// fn init_distribution: It take one argument for Moment type
    /// This function will be invoked only once when the node will be started for the first time
    /// It creates a open Distribution storage with default values
    pub fn _init_distribution(time_stamp: T::Moment) -> DispatchResult {

        Period::<T>::put(0);
        let current_period = <Period<T>>::get();
        Self::initialize_distribution(time_stamp, current_period)?;

        Ok(())
    }

    /// fn initialize_distribution: It take two argument of Moment & u64
    /// It creates a open Distribution storage with default values for the given period in runtime storage.
    fn initialize_distribution(time_stamp: T::Moment, current_period: u64) -> DispatchResult {

        let balances = Balances {
            total_balance: U256::from(0_u8), 
            current_balance : U256::from(0_u8), 
        };
        Distribution::<T>::mutate(current_period, |distribute| -> DispatchResult{
            distribute.denominator =  U256::from(0_u8);
            distribute.inittimestamp = time_stamp;
            distribute.closed = false;
            for assets in TokenType::_distributable_iterator() {
                distribute.balances.insert(*assets, balances.clone());
            }
            Ok(())
        })?;

        Ok(())
    }

    /// fn check_and_initialize_distribution: It take one argument of Moment type
    /// It first closes the ongoing distribution period 
    /// after setting the denominator values, it means it capture the total vtbc + controlled before end of the period
    /// and make this value as a denominator for claim calculation
    /// Also the amount of vtbc available for distribution
    /// At the end creates a open new Distribution storage with default values for the new upcoming period in runtime storage.
    pub(crate) fn check_and_initialize_distribution(time_stamp: T::Moment) -> DispatchResult {

        let current_period = <Period<T>>::get();
        let circulation_vtbc = <Circulation<T>>::get(TokenType::Vtbc);
        let globals_state = <Globals<T>>::get();
        Distribution::<T>::mutate(current_period, |distribute| -> DispatchResult{
            distribute.denominator = circulation_vtbc.checked_add(globals_state.controlled).ok_or(Error::<T>::NumberIsTooBigForU256)?;
            distribute.closed = true;
            Ok(())
        })?;
        let assetid: T::AssetId = T::VtbErc20AssetId::get();
        let vtbt_circulation_balance = <pallet_vtbt::Pallet<T>>::assets(assetid).ok_or(Error::<T>::NoneValue)?;
        let vtbc_equivalent_amount = if vtbt_circulation_balance.supply > U256::from(0_u8) { 
            <Pallet<T>>::convert_vtbt_to_vtbc(vtbt_circulation_balance.supply)?
        } else {U256::from(0_u8)};
        if vtbc_equivalent_amount < globals_state.backing_reserve {
            let vtbc_distributable_amount = globals_state.backing_reserve.checked_sub(vtbc_equivalent_amount).ok_or(Error::<T>::NumberIsTooLowForU256)?;
            Distribution::<T>::mutate(current_period, |distribute| -> DispatchResult {
                let dist_obj = distribute.balances.get_mut(&TokenType::Vtbc).ok_or(Error::<T>::NoneValue)?;
                dist_obj.total_balance = vtbc_distributable_amount;
                dist_obj.current_balance = vtbc_distributable_amount;
                Ok(())
            })?;
          
            Globals::<T>::mutate(|get_globals| -> DispatchResult{
                get_globals.backing_reserve = get_globals.backing_reserve.checked_sub(vtbc_distributable_amount).ok_or(Error::<T>::NumberIsTooLowForU256)?;
                Ok(())
            })?;
        }
        let new_period = current_period + 1;
        Self::initialize_distribution(time_stamp, new_period)?;
        UpdatedTimeList::<T>::mutate(|obj| {
            obj.distribution_timestamp = time_stamp + T::DistributionTimestamp::get();
        });
        <Period<T>>::put(new_period);
        <Pallet<T>>::deposit_event(Event::NewDistributionPeriodAdded(new_period, time_stamp));
       // let _res = <Pallet<T>>::add_and_update_migrated_each_period_balance_for_sellers(current_period);

        Ok(())
    }

    /// fn increase_distribution: It take two argument of TokenType enum & U256
    /// It increase the amount for distribution of the given token token type.
    /// This function will be invoked whenever user buy vtbc from reserve via using crypto.
    /// So that crypto amount will be available for distribution
    pub(crate) fn increase_distribution(amount: U256, token_type: &TokenType) -> DispatchResult {
        let current_period = <Period<T>>::get();
        Distribution::<T>::mutate(current_period, |distribute| -> DispatchResult {
            let dist_obj = distribute.balances.get_mut(token_type).ok_or(Error::<T>::NoneValue)?;
            let updated_distributable_balance = dist_obj.total_balance.checked_add(amount).ok_or(Error::<T>::NumberIsTooBigForU256)?;
            dist_obj.total_balance = updated_distributable_balance;
            dist_obj.current_balance = updated_distributable_balance;
            Ok(())
        })?;
        Ok(())
    }
}


