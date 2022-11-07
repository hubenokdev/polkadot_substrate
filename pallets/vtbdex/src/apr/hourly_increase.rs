
use crate::*;

pub struct HourlyIncrease<T>(T);

impl<T: Config> HourlyIncrease<T> {

	///pub(crate) fn calc_hourly_vtbc_rate(time_stamp: T::Moment) -> Result<(), Error<T>>
	///  
	/// This function takes one argument and it is being invoked from offchain-worker context.
	/// It is reposnible to calculate and increase Vtbc price each hour as per the algorithm_1
	/// It first checks if the price set in not recent one, that means gap between the last sent and now 
	/// is atleast one hour than it invokes the function to calculate the increased price,
	/// otherwise it throws error saying "RECENTLY_SENT"
	/// 
	pub(crate) fn calc_hourly_vtbc_rate(time_stamp: T::Moment) -> Result<(), Error<T>> {

		const RECENTLY_SENT: () = ();
	
		let val = StorageValueRef::persistent(b"hourly_vtbc_rate::last_send_timestamp");
		let res = val.mutate(|last_send: Result<Option<T::Moment>, StorageRetrievalError>| {
			match last_send {
				// If we already have a value in storage and the block number is recent enough
				// we avoid sending another transaction at this time.
				Ok(Some(last_time_stamp)) if time_stamp < last_time_stamp + T::HourlyPeriod::get() =>
					Err(RECENTLY_SENT),
				// In everrust crate for logarithm of 10y other case we attempt to acquire the lock and send a transaction.
				_ => Ok(time_stamp),
			}
		});

		match res {
			// The value has been set correctly, which means we can safely send a transaction now.
			Ok(_block_number) => {
				let time_laps_for_1_hour = (T::HourlyPeriod::get() * T::SlotDuration::get()) + time_stamp;
				log::info!("Slot duration: {:?}", time_laps_for_1_hour);
			    let _res = Self::increase_hourly_vtbc_price(time_stamp);
				Ok(())
			},
			// We are in the grace period, we should not send a transaction this time.
			Err(MutateStorageError::ValueFunctionFailed(RECENTLY_SENT)) => Err(<Error<T>>::InSameaccuralPeriod),
			// We wanted to send a transaction, but failed to write the block number (acquire a
			// lock). This indicates that another offchain worker that was running concurrently
			// most likely executed the same logic and succeeded at writing to storage.
			// Thus we don't really want to send the transaction, knowing that the other run
			// already did.
			Err(MutateStorageError::ConcurrentModification(_)) => Err(<Error<T>>::LockAcquiredFailed),
		}
    }

	///pub(crate) fn calculate_ta_in_year_start() -> Result<(), Error<T>>
	/// 
	/// This function is used to calculate year constant for the apr which is named as "ta"
	/// This function is invoked via Ocw-context but once in a year at the start of year.
	/// 
	/// This function regulate two things:
	/// 1. Calculate ta(year constant)
	/// It will calculate the ta value and than it will save the calculated ta(year constant)
	/// in runtime storage.
	/// 
	/// For formula to calculate ta:
	/// ua = the target rate of increase for the year a (100% for the year 2021, 90% for the year
	/// 2022, 80% for the year 2023, … , 20% for the year 2050)
	/// 
	/// 𝐻𝑎 = the total number of hours in the year a (=8760 hours if it’s a non-leap year, 8784 hours
	/// if it’s a leap year)
	/// 
	/// ta = 10^((log(1+ua))/Ha) - 1
	/// [Note: ^ - symbol represents power]
	/// 
	///  2. Calculate current price if hours_elapsed missed at the beginning of the year.
	///  This case is possible if the node goes down near end of the current year..
	///  and restarted at the beginning of new year.
	///  So this case will makeup the price missed.
	/// 𝑽𝑻𝑩𝑪 𝒄𝒖𝒓𝒓𝒆𝒏𝒕_𝒑𝒓𝒊𝒄𝒆_a = 𝑽𝑻𝑩𝑪 year_𝒔𝒕𝒂𝒓𝒕_𝒑𝒓𝒊𝒄𝒆_a *(𝟏 + 𝒕𝒂 )^ 𝒉𝒂
	/// ℎ𝑎 = the number of hours elapsed since the beginning of the year a .
	/// 
	pub(crate) fn calculate_ta_in_year_start() -> Result<(), Error<T>> { 

		let globals = <Pallet<T>>::get_globals();	
		let ta = Self::calculate_ta_year_constant(&globals)?;
		let final_price = Self::calculate_missed_hours_price(&globals, ta)?;

        let call = Some(Call::initialize_value_for_year_ta { 
            ta: U256::from(ta as u128), 
            elpased_price: final_price 
        });

        Ok(Pallet::<T>::sign_and_submit_transaction(call)?)
	}

	///fn calculate_ta_year_constant(globals: &custom_types::Globals) -> Result<f64, Error<T>>
	/// It takes one argument of reference of runtime-storage Globals
	/// 
	/// Calculate ta(year constant):
	/// It will calculate the ta value and than it will save the calculated ta(year constant)
	/// in runtime storage.
	/// 
	/// For formula to calculate ta:
	/// ua = the target rate of increase for the year a (100% for the year 2021, 90% for the year
	/// 2022, 80% for the year 2023, … , 20% for the year 2050)
	/// 
	/// 𝐻𝑎 = the total number of hours in the year a (=8760 hours if it’s a non-leap year, 8784 hours
	/// if it’s a leap year)
	/// 
	/// ta = 10^((log(1+ua))/Ha) - 1
	/// [Note: ^ - symbol represents power]
	/// 
	fn calculate_ta_year_constant(globals: &custom_types::Globals) -> Result<f64, Error<T>> {
		//Ha
		let total_hours = globals.total_hours as f64;
		log::debug!("total_hours: {:?}", total_hours);
		//ua
		let target_rate = globals.target_rate_for_year as f64;
		log::debug!("target_rate: {:?}", target_rate);
		let target_rate_decimals = target_rate/100_f64;
		log::debug!("target_rate_decimals: {:?}", target_rate_decimals);
		//(1+ua)
		let ua: f64 = 1_f64 + target_rate_decimals;
		log::debug!("ua: {:?}", ua);
		//(log(1+ua))
		let log = libm::log10(ua);
		log::debug!("log: {:?}", log);
		//(log(1+ua))/Ha
		let log_by_ha =  log/total_hours;
		log::debug!("log_by_ha: {:?}", log_by_ha);
		let base: f64 = 10_f64;
		//ta = 10^((log(1+ua))/Ha) -1 
		let ta = libm::pow(base,log_by_ha) - 1_f64;
		log::debug!("ta: {:?}", ta);

		Ok(ta)
	}

	/// fn calculate_missed_hours_price(globals: &custom_types::Globals, ta: f64) -> Result<U256, Error<T>>
	///  It takes two argument of reference of runtime-storage Globals and ta(year_contant) value.
	/// 
	///  Calculate current price if hours_elapsed missed at the beginning of the year:
	///  This case is possible if the node goes down near end of the current year..
	///  and restarted at the beginning of new year. So this case will makeup the price missed.
	/// 𝑽𝑻𝑩𝑪 𝒄𝒖𝒓𝒓𝒆𝒏𝒕_𝒑𝒓𝒊𝒄𝒆_a = 𝑽𝑻𝑩𝑪 year_𝒔𝒕𝒂𝒓𝒕_𝒑𝒓𝒊𝒄𝒆_a *(𝟏 + 𝒕𝒂 )^ 𝒉𝒂
	/// ℎ𝑎 = the number of hours elapsed since the beginning of the year a .
	/// 
	fn calculate_missed_hours_price(globals: &custom_types::Globals, ta: f64) -> Result<U256, Error<T>> {
		let year_start_price = AprEstimate::<T>::get(globals.current_year).start_rate;
		log::debug!("vtbc_start_price: {:?}", year_start_price);
		let hours_elapsed = globals.hours_elapsed as f64;
		let ta_18 = ta * 1000000000000000000.0_f64;
		log::debug!("ta_18: {:?}", ta_18);

		let ta_1 = 1_f64 + ta;	
		let ta_ha = libm::pow(ta_1,hours_elapsed);

		let ta_ha_18 = ta_ha * 1000000000000000000.0_f64;
		log::debug!("ta_ha_18: {:?}", ta_ha_18);
		let vtbc_current_price_h1 = year_start_price.checked_mul(U256::from(ta_ha_18 as u128)).ok_or(Error::<T>::NumberIsTooBigForU256)?;
		let final_price = vtbc_current_price_h1.checked_div(U256::from(100_0000_0000_0000_0000_i128)).ok_or(Error::<T>::NumberIsTooLowForU256)?;
		log::info!("Missed hourly rate calculation new vtbc_price: {:?}", final_price);

		Ok(final_price)
	}

	///fn check_hours_elapsed_value(time_stamp: T::Moment) -> (u32, u32, u32, custom_types::Globals)
	/// 
	/// This function checks the gap between last apr calculated with current timestamp..
	/// Based on the difference it calculate how many hours passed.
	/// Also it check and calculate how many hours are remaining in current year..
	/// i.e Total_hours(8760) - globals.hours_elapsed value.
    fn check_hours_elapsed_value(time_stamp: T::Moment) -> (u32, u32, custom_types::Globals) {
        let last_apr_timestamp = <Pallet<T>>::updated_time_list().apr_timestamp;
		let hours_laps = (time_stamp - last_apr_timestamp)/T::HourlyPeriod::get();
		let hours_laps_32: u32 = <Pallet<T>>::moment_to_u32(hours_laps).unwrap_or(0);
		log::info!("last_apr_timestamp: {:?}=====time_stamp: {:?}======T::HourlyPeriod::get(): {:?}===hours_laps: {:?}=====hours_laps_32: {:?}",last_apr_timestamp, time_stamp, T::HourlyPeriod::get(), hours_laps, hours_laps_32);
		
        let globals = <Pallet<T>>::get_globals();
		let mut remaining_hours = 0;
		let mut _hours_left_in_current_year = 0;
		let mut _hours_laps_increment_counter = 0;
		if globals.hours_elapsed + hours_laps_32 <= globals.total_hours {
			log::info!("globals.hours_elapsed  {:?}", globals.hours_elapsed );
			_hours_laps_increment_counter = hours_laps_32;
		}
		else {
			remaining_hours = (globals.hours_elapsed + hours_laps_32) - globals.total_hours;
			log::info!("Completed globals.hours_elapsed  {:?}", globals.hours_elapsed );
			_hours_laps_increment_counter = hours_laps_32 - remaining_hours;
		}
		log::info!("_hours_laps_increment_counter: {:?}", _hours_laps_increment_counter);

        (   
            _hours_laps_increment_counter, 
            remaining_hours, 
            globals
        )
    }

	///fn calculate_hourly_increased(globals: custom_types::Globals, hours_laps_increment_counter: u32) -> Result<U256, Error<T>>
	/// 
	/// This calculated the elapsed hours increased value.
	/// For calculating increased price
	/// Where,
	/// 𝑽𝑻𝑩𝑪𝒑𝒓𝒊𝒄𝒆𝒉 = last hour vtbc price
	/// 𝐵ℎ+1 = t𝑎 × 𝑉𝑇𝐵𝐶𝑝𝑟𝑖𝑐𝑒ℎ
	/// 
	/// And final price (𝑽𝑻𝑩𝑪𝒑𝒓𝒊𝒄𝒆𝒉+𝟏) is set via signing a transaction submit_vtbc_hourly_rate to the runtime. 
	/// 
    fn calculate_hourly_increased(
		globals: custom_types::Globals, 
		hours_laps_increment_counter: u32
	) -> Result<U256, Error<T>> {

		let mut total_hourly_increment = U256::from(0_u8);
		let ta = AprEstimate::<T>::get(globals.current_year).ta;
		let mut vtbc_price_h = <UsdVtbcH<T>>::get();
        log::info!("After globals.hours_elapsed  {:?}", globals.hours_elapsed );
        let mut i = 1;
		//Loop is used to repeat the same formula to fix the gap between the last hour price and the current price.
		while i <= hours_laps_increment_counter { // Loop to handle node stop case
			//𝐵ℎ+1 = t𝑎 × 𝑉𝑇𝐵𝐶𝑝𝑟𝑖𝑐𝑒ℎ
			let bh_1 = ta.checked_mul(vtbc_price_h).ok_or(Error::<T>::NumberIsTooBigForU256)?;
			log::info!("bh_1: {:?}", bh_1);
			//This converting to 18 decimals
			let bh_1_18 = bh_1.checked_div(U256::from(1_000_000_000_000_000_000_u128)).ok_or(Error::<T>::NumberIsTooLowForU256)?;
			log::info!("bh_1_18: {:?}", bh_1_18);
			//Here taking the sum of each hour increased price
			total_hourly_increment = total_hourly_increment.checked_add(bh_1_18).ok_or(Error::<T>::NumberIsTooBigForU256)?;
			//Updating 𝑉𝑇𝐵𝐶𝑝𝑟𝑖𝑐𝑒ℎ value so that it can be used at the next hour calculation.
			vtbc_price_h = vtbc_price_h.checked_add(bh_1_18).ok_or(Error::<T>::NumberIsTooBigForU256)?;
			log::info!("vtbc_price_h: {:?}", vtbc_price_h); 
			i+=1;
		}

        Ok(total_hourly_increment)
    }

	///fn increase_hourly_vtbc_price(time_stamp: T::Moment) -> Result<(), Error<T>>
	/// This function takes one argument of temistamp.
	/// 
	/// This calculated the elapsed hours increased value.
	/// And than calculate the new vtbc price.
	/// For calculating new price it does the comparison of
	/// Where, 
	/// 𝑽𝑻𝑩𝑪𝒑𝒓𝒊𝒄𝒆𝒉 = last hour vtbc price
	/// 𝑩𝒉+𝟏 = Hourly increased price
	/// 𝑨𝒉+𝟏 = Transactional increased price
	/// 𝑽𝑻𝑩𝑪𝒑𝒓𝒊𝒄𝒆𝒉+𝟏 = 𝐦𝐚𝐱 (𝑽𝑻𝑩𝑪𝒑𝒓𝒊𝒄𝒆𝒉 + 𝑩𝒉+𝟏 , 𝑽𝑻𝑩𝑪𝒑𝒓𝒊𝒄𝒆𝒉 + 𝑨𝒉+𝟏 )
	/// 
	/// And final price (𝑽𝑻𝑩𝑪𝒑𝒓𝒊𝒄𝒆𝒉+𝟏) is set via signing a transaction submit_vtbc_hourly_rate to the runtime. 
	fn increase_hourly_vtbc_price(time_stamp: T::Moment) -> Result<(), Error<T>> {

        let (
            hours_laps_increment_counter,
            remaining_hours,
            globals
        ) = Self::check_hours_elapsed_value(time_stamp);
        // 𝑩𝒉+𝟏
		let total_hourly_increment = Self::calculate_hourly_increased(
            globals,
            hours_laps_increment_counter
        )?;

		let vtbc_price_h1 = <UsdVtbcH<T>>::get().checked_add(total_hourly_increment).ok_or(Error::<T>::NumberIsTooBigForU256)?;

		let vtbc_current_price_h = <UsdRate<T>>::get().vtbc_last_apr_rate;
		//𝑽𝑻𝑩𝑪𝒑𝒓𝒊𝒄𝒆𝒉 + 𝑩𝒉+𝟏
		let final_new_price_h1 = vtbc_current_price_h.checked_add(total_hourly_increment).ok_or(Error::<T>::NumberIsTooBigForU256)?;
		
		//𝑨𝒉+𝟏
		let txn_calc_vtbc_price = <Pallet<T>>::get_txn_affected_vtbc_price();
		log::info!("txn_calc_vtbc_price: {:?}", txn_calc_vtbc_price);
		// 𝑽𝑻𝑩𝑪𝒑𝒓𝒊𝒄𝒆𝒉 + 𝑨𝒉+𝟏
		let final_price_txn = vtbc_current_price_h.checked_add(txn_calc_vtbc_price).ok_or(Error::<T>::NumberIsTooBigForU256)?;
		log::info!("final_price_txn: {:?}", final_price_txn);
       
		//𝑽𝑻𝑩𝑪𝒑𝒓𝒊𝒄𝒆𝒉+𝟏 = 𝐦𝐚𝐱 (𝑽𝑻𝑩𝑪𝒑𝒓𝒊𝒄𝒆𝒉 + 𝑩𝒉+𝟏 , 𝑽𝑻𝑩𝑪𝒑𝒓𝒊𝒄𝒆𝒉 + 𝑨𝒉+𝟏 )
        let call = if final_price_txn >= final_new_price_h1 {
			Some(Call::submit_vtbc_hourly_rate{ 
                rate: final_price_txn,
                time_stamp, 
                apr_calculated_rate: vtbc_price_h1, 
                hours: hours_laps_increment_counter, 
                remaining_hours 
            })
		}
		else {
			Some(Call::submit_vtbc_hourly_rate { 
                rate: final_new_price_h1, 
                time_stamp, 
                apr_calculated_rate: vtbc_price_h1, 
                hours: hours_laps_increment_counter, 
                remaining_hours 
            })
		};
		
        Ok(Pallet::<T>::sign_and_submit_transaction(call)?)
    }
}
