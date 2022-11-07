
use crate::*;
use sp_runtime::DispatchResult;

pub struct InitializeApr<T>(T);

impl<T: Config> InitializeApr<T> {
    
	///pub(crate) fn _initialize_apr_contants()
	/// This function is invoked once in a life cycle of node,
	/// When the node started that time it maintain the state for the target_rate for each upcoming year.
	/// Targetrate is contant and is declared in contants.rs file.
    pub(crate) fn _initialize_apr_contants() {
		
		for item in constants::_TARGET_RATES {
			let year = item[0];
			let target = item[1];
			log::info!("{:?}", item);
			AprEstimate::<T>::mutate(year, |obj|{
				obj.year = year;
				obj.target_rate = target;
			});
		}
	}

	/// pub(crate) fn initiate_initialize_values_for_apr(vtbc_current: U256) -> DispatchResult
	/// 
	/// This function is invoked only once at the start of the node/network.
	/// This sets the initial values for the Vtbc price.
	/// Every values used inside the function is constant taken from the old node and set during rebuild.
	/// 
    pub(crate) fn initiate_initialize_values_for_apr(vtbc_current: U256) -> DispatchResult {

        Self::mutate_globals_state_for_apr(2022, 623);
		let vtbc_price =  U256::from(5258116238184148478_u128);
		let timestamp = 1646238479084_u64;
		UsdRate::<T>::mutate(|usd_rate| {
            usd_rate.vtbc_start_price = vtbc_current;
			usd_rate.vtbc_current_price = vtbc_price;
			usd_rate.vtbc_last_apr_rate = vtbc_price;
        });

		AprEstimate::<T>::mutate(2022, |obj|{
			obj.start_rate = vtbc_current;
		});
		
		<UsdVtbcH<T>>::put(U256::from(5257458721726318215_u128));
		
		UpdatedTimeList::<T>::mutate(|obj| -> DispatchResult {
			obj.apr_timestamp = <Pallet<T>>::u32_to_moment(timestamp).ok_or(Error::<T>::NoneValue)?;

            Ok(())
		})?;
		
		<VtbcStartRate<T>>::put(true);
		
		Ok(())
	}

	///pub(crate) fn hourly_hours_changes(elaps_hours: u32) 
	/// 
	/// This function is invoked from extrinsic named as "submit_vtbc_hourly_rate",
	/// which is inoked via Ocw.
	/// It updates the states which affect the hourly prices change algorithm.
    pub(crate) fn hourly_hours_changes(elaps_hours: u32){

		let globals = <Pallet<T>>::get_globals();
		if globals.total_hours == globals.hours_elapsed {
			let new_year = globals.current_year + 1;
			let usd_rate = UsdRate::<T>::get();

			AprEstimate::<T>::mutate(globals.current_year, |obj|{
				obj.achieve_rate = usd_rate.vtbc_current_price;
			});

            Self::mutate_globals_state_for_apr(new_year, 0);
			
			AprEstimate::<T>::mutate(new_year, |obj|{
				obj.start_rate = usd_rate.vtbc_current_price;
			});
			<UsdVtbcH<T>>::put(usd_rate.vtbc_current_price);
		}

		Globals::<T>::mutate(|get_globals| {
			get_globals.hours_elapsed += elaps_hours;
		});
	}

	///fn mutate_globals_state_for_apr(current_year: u32, hours_elapsed: u32) 
	/// 
	/// This is helper function to initialize and update the state related to Apr in Globals state.
    fn mutate_globals_state_for_apr(current_year: u32, hours_elapsed: u32) {
		let apr_estimate_contants = AprEstimate::<T>::get(current_year);

		if <Pallet<T>>::is_leap_year(current_year.into()) {
			Globals::<T>::mutate(|get_globals| {
            	get_globals.hours_elapsed = hours_elapsed;
				get_globals.total_hours = 8784;
				get_globals.target_rate_for_year = apr_estimate_contants.target_rate;
				get_globals.start_year = 2022;
				get_globals.current_year = current_year;
        	});
		}
		else {
			Globals::<T>::mutate(|get_globals| {
            	get_globals.hours_elapsed = hours_elapsed;
				get_globals.total_hours = 8760;
				get_globals.target_rate_for_year = apr_estimate_contants.target_rate;
				get_globals.start_year = 2022;
				get_globals.current_year = current_year;
        	});
		}
    }
}
