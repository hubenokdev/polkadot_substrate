//! module MiscData: This module is defined to capture all other runtime state
//! Such as VTBc reserve balance, Crypto circulation balance
use crate::{
	Pallet, Error, Config, Get, Encode,
    AprEstimate, Globals, Circulation, UserWallet,
    TotalOrdersCountTillDate, TotalSellsJournal, TxnAffectedVtbcPrice,
    Distribution, VtbdexTransactionFee, UsdVtbcH, UsdRate,
    UpdatedTimeList, String, ToString,
};
use sp_runtime::SaturatedConversion;
use serde_json::{Value, Map};

pub(crate) struct MiscData<T>(T);

impl<T: Config> MiscData<T> {

    /// fn apr_estimate_data -> This function take mutable reference.
    /// It capture the AprEstimate runtime state and convert to Json Object
    /// At the end it insert the captured object into the reference parameters i.e FinalJson data to sent to ipfs.
    pub(crate) fn apr_estimate_data(res: &mut Map<String, Value>) -> Result<(), Error<T>> {
        let apr = AprEstimate::<T>::iter();
        let mut map = Map::new();

        for key in apr {
            let mut apr_map = Map::new();
            apr_map.insert("year".to_string(), Value::from(key.1.year));
            apr_map.insert("target_rate".to_string(), Value::from(key.1.target_rate));
            apr_map.insert("start_rate".to_string(), Value::from(key.1.start_rate.to_string()));
            apr_map.insert("achieved_rate".to_string(), Value::from(key.1.achieve_rate.to_string()));
            apr_map.insert("ta_year_constant".to_string(), Value::from(key.1.ta.to_string()));

            map.insert(key.0.to_string(), Value::from(apr_map));
        }

        res.insert("apr_estimate_data".to_string(),  Value::from(map));
        Ok(())
    }

    /// fn globals_data_to_map -> This function take mutable reference.
    /// It capture the Globals runtime state and convert to Json Object
    /// At the end it insert the captured object into the reference parameters i.e FinalJson data to sent to ipfs.
    pub(crate) fn globals_data_to_map(res: &mut Map<String, Value>) -> Result<(), Error<T>> {
        //Capture Globals state
        let mut map = Map::new();
        let global_state = Globals::<T>::get();
        //let data: Value = serde_json::to_value(global_state.clone()).ok_or(Error::<T>::NumberIsTooLowForU256)?;
        // log::info!("data: {:?}", data);
        map.insert("transaction_count".to_string(), Value::from(global_state.transaction_count));
        map.insert("hours_elapsed".to_string(), Value::from(global_state.hours_elapsed));
        map.insert("total_hours".to_string(), Value::from(global_state.total_hours));
        map.insert("target_rate_for_year".to_string(), Value::from(global_state.target_rate_for_year));
        map.insert("start_year".to_string(), Value::from(global_state.start_year));
        map.insert("current_year".to_string(), Value::from(global_state.current_year));
        map.insert("controlled".to_string(), Value::from(global_state.controlled.to_string()));
        map.insert("backing_reserve".to_string(), Value::from(global_state.backing_reserve.to_string()));

        res.insert("Globals".to_string(),  Value::from(map));
        Ok(())
    }

    /// fn circulation_data_to_map -> This function take mutable reference.
    /// It capture the Circulation runtime state and convert to Json Object
    /// At the end it insert the captured object into the reference parameters i.e FinalJson data to sent to ipfs.
    pub(crate) fn circulation_data_to_map(res: &mut Map<String, Value>) -> Result<(), Error<T>> {
        //Capture Circulation state
        let mut circulation_map = Map::new();
        let circulation_iter = Circulation::<T>::iter();
        for key in circulation_iter {
            let circulation_balance = key.1;   //::<T>::get();
            circulation_map.insert(key.0.to_string(), Value::from(circulation_balance.to_string()));
        }
        res.insert("Circulation".to_string(), Value::from(circulation_map));
        Ok(())
    }

    /// fn vtbt_supply_data -> This function take mutable reference.
    /// It capture the Asset runtime state defined in VTBT pallet and convert to Json Object
    /// At the end it insert the captured object into the reference parameters i.e FinalJson data to sent to ipfs.
    pub(crate) fn vtbt_supply_data(res: &mut Map<String, Value>) -> Result<(), Error<T>> {
        //Capture VTBT Supply state
        let mut vtbt_supply = Map::new();
        let assetid: T::AssetId = T::VtbErc20AssetId::get();
        let assets = <pallet_vtbt::Pallet<T>>::assets(assetid).ok_or(Error::<T>::NumberIsTooLowForU256)?; //Asset::<T>::get(assetid);
        vtbt_supply.insert("vtbt_supply".to_string(),  Value::from(assets.supply.to_string()));
        vtbt_supply.insert("accounts".to_string(),  Value::from(assets.accounts));

        res.insert("VTBT_Token".to_string(),  Value::from(vtbt_supply));
        Ok(())
    }

    /// fn other_state_to_map -> This function take mutable reference.
    /// It capture the TotalOrdersCountTillDate, TxnAffectedVtbcPrice , TotalSellsJournal, runtime state defined 
    /// in vtbdex pallet and convert to Json Object
    /// At the end it insert the captured object into the reference parameters i.e FinalJson data to sent to ipfs.
    pub(crate) fn other_state_to_map(res: &mut Map<String, Value>) -> Result<(), Error<T>> {
        //captured other state
        let mut other_state_map = Map::new();
        let total_orders_count_till_date_state = TotalOrdersCountTillDate::<T>::get();
        other_state_map.insert("TotalOrdersCountTillDate".to_string(), Value::from(total_orders_count_till_date_state.to_string()));
        let txn_affected_price = TxnAffectedVtbcPrice::<T>::get();
        other_state_map.insert("TxnAffectedVtbcPrice".to_string(), Value::from(txn_affected_price.to_string()));
        let total_sells_journals= TotalSellsJournal::<T>::get();
        other_state_map.insert("TotalSellsJournal".to_string(), Value::from(total_sells_journals.to_string()));
        let reserve_balance = <pallet_vtbc_token::Pallet<T>>::get_reserve_balance();
        other_state_map.insert("ReserveBalance".to_string(), Value::from(reserve_balance.to_string()));

        res.insert("OtherState".to_string(),  Value::from(other_state_map));
        Ok(())
    }

    /// fn usd_rate_data_to_map -> This function take mutable reference.
    /// It capture the UsdRate runtime state defined in vtbdex pallet and convert to Json Object.
    /// At the end it insert the captured object into the reference parameters i.e FinalJson data to sent to ipfs.
    pub(crate) fn usd_rate_data_to_map(res: &mut Map<String, Value>) -> Result<(), Error<T>> {
        //Capture USD RATES state
        let mut usd_rate_map = Map::new();
        let usd_rate_state = UsdRate::<T>::get();
        usd_rate_map.insert("eth".to_string(), Value::from(usd_rate_state.eth.to_string()));
        usd_rate_map.insert("eos".to_string(), Value::from(usd_rate_state.eos.to_string()));
        usd_rate_map.insert("vtbc_current_price".to_string(), Value::from(usd_rate_state.vtbc_current_price.to_string()));
        usd_rate_map.insert("vtbc_start_price".to_string(), Value::from(usd_rate_state.vtbc_start_price.to_string()));
        usd_rate_map.insert("vtbc_last_apr_rate".to_string(), Value::from(usd_rate_state.vtbc_last_apr_rate.to_string()));
        let usd_vtbc_h_state = UsdVtbcH::<T>::get();
        usd_rate_map.insert("usd_vtbc_h_state".to_string(), Value::from(usd_vtbc_h_state.to_string()));

        res.insert("Usd_rate".to_string(),  Value::from(usd_rate_map));
        Ok(())
    }

    /// fn fee_collector_data_to_map -> This function take mutable reference.
    /// It capture the VtbdexTransactionFee runtime state defined in vtbdex pallet and convert to Json Object.
    /// At the end it insert the captured object into the reference parameters i.e FinalJson data to sent to ipfs.
    pub(crate) fn fee_collector_data_to_map(res: &mut Map<String, Value>) -> Result<(), Error<T>> {
        //Capture Feecollector state
        let mut vtb_trnx_fee = Map::new();
        let trnx_fee_state = VtbdexTransactionFee::<T>::get().ok_or(Error::<T>::NumberIsTooLowForU256)?;
        let val = <UserWallet<T>>::get(trnx_fee_state.fee_collector_address.encode());
        let address = Value::String(<Pallet<T>>::bytes_to_string(&val.polkadot_address)?);
        vtb_trnx_fee.insert("fee_collector_address".to_string(), address);
        vtb_trnx_fee.insert("trnx_fee".to_string(), Value::from(trnx_fee_state.fee.to_string()));

        res.insert("Vtb_trnx_fee".to_string(),  Value::from(vtb_trnx_fee));
        Ok(())
    }

    /// fn distribution_data_to_map -> This function take mutable reference.
    /// It capture the Distribution & UpdatedTimeList runtime state defined in vtbdex pallet and convert to Json Object.
    /// At the end it insert the captured object into the reference parameters i.e FinalJson data to sent to ipfs.
    pub(crate) fn distribution_data_to_map(res: &mut Map<String, Value>) -> Result<(), Error<T>> {
		//Capture Distribution state
		let mut dist_table = Map::new();
		let keys = Distribution::<T>::iter();
		for key in keys {
			let mut each_dist = Map::new();
			let mut bala = Map::new();
			let dist_data =  key.1;
			for (key1, value) in dist_data.balances.iter() {
				let mut crypto = Map::new();
				crypto.insert("total_balance".to_string(), Value::from(value.total_balance.to_string()));
				crypto.insert("current_balance".to_string(), Value::from(value.current_balance.to_string()));
				bala.insert(key1.to_string(), Value::from(crypto));
			}
			each_dist.insert("balances".to_string(), Value::from(bala));
			each_dist.insert("inittimestamp".to_string(), Value::from(dist_data.inittimestamp.saturated_into::<u64>()));
			each_dist.insert("closed".to_string(), Value::from(dist_data.closed));
			each_dist.insert("denominator".to_string(), Value::from(dist_data.denominator.to_string()));
			dist_table.insert(key.0.to_string(), Value::from(each_dist));
		}

		// Updated time list
		let time = UpdatedTimeList::<T>::get();
		let mut updated_time = Map::new();
		updated_time.insert("distribution_timestamp".to_string(), Value::from(time.distribution_timestamp.saturated_into::<u64>()));
		updated_time.insert("apr_timestamp".to_string(), Value::from(time.apr_timestamp.saturated_into::<u64>()));
		dist_table.insert("UpdatedTimeList".to_string(), Value::from(updated_time));

        res.insert("Distribution_table".to_string(),  Value::from(dist_table));
		Ok(())
	}
}