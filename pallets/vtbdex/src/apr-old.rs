
use crate::*;
use crate::cryptos::utility::VtbcUsdRate;

impl<T: Config> Pallet<T> {

	pub fn calc_hourly_vtbc_rate(time_stamp: T::Moment) -> Result<(), Error<T>> {

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

	pub fn calculate_ta_in_year_start() -> Result<(), Error<T>> { 

		let globals = Self::get_globals();	
		let year_start_price = AprEstimate::<T>::get(globals.current_year).start_rate;
		log::info!("vtbc_start_price: {:?}", year_start_price);
		let hours_elapsed = <Globals<T>>::get().hours_elapsed as f64;
		log::info!("hours_elapsed: {:?}", hours_elapsed);
		let total_hours = <Globals<T>>::get().total_hours as f64;
		log::info!("total_hours: {:?}", total_hours);
		let target_rate = <Globals<T>>::get().target_rate_for_year as f64;
		log::info!("target_rate: {:?}", target_rate);
		let target_rate_decimals = target_rate/100_f64;
		log::info!("target_rate_decimals: {:?}", target_rate_decimals);
		let ua: f64 = 1_f64 + target_rate_decimals;
		log::info!("ua: {:?}", ua);
		// let log = ua.log10();
		let log = libm::log10(ua);
		log::info!("log: {:?}", log);

		let log_by_ha =  log/total_hours;
		log::info!("log_by_ha: {:?}", log_by_ha);

		let base: f64 = 10_f64;

		// let ta = base.powf(log_by_ha) - 1 as f64;
		let ta = libm::pow(base,log_by_ha) - 1_f64;
		log::info!("ta: {:?}", ta);
		let ta_18 = ta * 1000000000000000000.0_f64;
		log::info!("ta_18: {:?}", ta_18);

		let ta_1 = 1_f64 + ta;	
		// let ta_ha = ta_1.powf(hours_elapsed);
		let ta_ha = libm::pow(ta_1,hours_elapsed);

		let ta_ha_18 = ta_ha * 1000000000000000000.0_f64;
		log::info!("ta_ha_18: {:?}", ta_ha_18);
		let vtbc_current_price_h1 = year_start_price.checked_mul(U256::from(ta_ha_18 as u128)).unwrap();
		let final_price = vtbc_current_price_h1.checked_div(U256::from(100_0000_0000_0000_0000_i128)).unwrap();
		log::info!("Missed hourly rate calculation: {:?}", final_price);

		let signer = Signer::<T, <T as pallet::Config>::AuthorityId>::any_account();
		let result = signer.send_signed_transaction(|_acct|
			Call::initialize_value_for_year_ta{ ta: U256::from(ta_18 as u128), elpased_price: final_price });
		// Display error if the signed tx fails.
		if let Some((acc, res)) = result {
			if res.is_err() {
				log::info!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
				return Err(<Error<T>>::OffchainSignedTxError);
			}
			log::info!("Sent success: {:?}", res);
			// Transaction is sent successfully
			return Ok(());
		}
		log::info!("No local account available");
		Err(<Error<T>>::NoLocalAcctForSigning)
	}

	fn increase_hourly_vtbc_price(time_stamp: T::Moment) -> Result<(), Error<T>> {
		let signer = Signer::<T, <T as pallet::Config>::AuthorityId>::any_account();

		let last_apr_timestamp = Self::updated_time_list().apr_timestamp;
		let hours_laps = (time_stamp - last_apr_timestamp)/T::HourlyPeriod::get();
		let hours_laps_32: u32 = Self::moment_to_u32(hours_laps).unwrap();
		log::info!("last_apr_timestamp: {:?}=====time_stamp: {:?}======T::HourlyPeriod::get(): {:?}===hours_laps: {:?}=====hours_laps_32: {:?}",last_apr_timestamp, time_stamp, T::HourlyPeriod::get(), hours_laps, hours_laps_32);
		let globals = Self::get_globals();
		let mut remaining_hours = 0;
		let mut _hours_left_in_current_year = 0;
		let mut _hours_laps_increment_counter = 0;
		if globals.hours_elapsed + hours_laps_32 <= globals.total_hours {
			log::info!("globals.hours_elapsed  {:?}", globals.hours_elapsed );
			_hours_left_in_current_year = hours_laps_32;
			_hours_laps_increment_counter = hours_laps_32;
		}
		else {
			remaining_hours = (globals.hours_elapsed + hours_laps_32) - globals.total_hours;
			log::info!("Completed globals.hours_elapsed  {:?}", globals.hours_elapsed );
			_hours_left_in_current_year = hours_laps_32 - remaining_hours;
			_hours_laps_increment_counter = _hours_left_in_current_year;
		}
		log::info!("_hours_left_in_current_year: {:?}", _hours_left_in_current_year);
		let ta = AprEstimate::<T>::get(globals.current_year).ta;
		let vtbc_current_price_h = <UsdRate<T>>::get().vtbc_last_apr_rate;
		let mut total_hourly_increment = U256::from(0_u8);
		let mut vtbc_price_h = <UsdVtbcH<T>>::get();
		let mut i = 1;
		log::info!("After globals.hours_elapsed  {:?}", globals.hours_elapsed );
		while i <= _hours_left_in_current_year { // Loop to handle node stop case
			let bh_1 = ta.checked_mul(vtbc_price_h).unwrap();
			log::info!("bh_1: {:?}", bh_1);
			let bh_1_18 = bh_1.checked_div(U256::from(100_0000_0000_0000_0000_i128)).unwrap();
			log::info!("bh_1_18: {:?}", bh_1_18);
			total_hourly_increment = total_hourly_increment.checked_add(bh_1_18).unwrap();
			vtbc_price_h = vtbc_price_h.checked_add(bh_1_18).unwrap();
			log::info!("vtbc_price_h: {:?}", vtbc_price_h); 
			i+=1;
		}
		let final_new_price_h1 = vtbc_current_price_h.checked_add(total_hourly_increment).unwrap();
		let vtbc_price_h1 = <UsdVtbcH<T>>::get().checked_add(total_hourly_increment).unwrap();

		//Calculated transaction increment + current price
		let txn_calc_vtbc_price = Self::get_txn_affected_vtbc_price();
		log::info!("txn_calc_vtbc_price: {:?}", txn_calc_vtbc_price);
		let final_price_txn = vtbc_current_price_h.checked_add(txn_calc_vtbc_price).unwrap();
		log::info!("final_price_txn: {:?}", final_price_txn);

		let result = if final_price_txn >= final_new_price_h1 {
			signer.send_signed_transaction(|_acct|
				// This is the on-chain function
				Call::submit_vtbc_hourly_rate{ rate: final_price_txn, time_stamp, apr_calculated_rate: vtbc_price_h1, hours: _hours_laps_increment_counter, remaining_hours })
		}
		else {
			signer.send_signed_transaction(|_acct|
				// This is the on-chain function
				Call::submit_vtbc_hourly_rate{ rate: final_new_price_h1, time_stamp, apr_calculated_rate: vtbc_price_h1, hours: _hours_laps_increment_counter, remaining_hours })
		};

		// Display error if the signed tx fails.
		if let Some((acc, res)) = result {
			if res.is_err() {
				log::info!("failure: offchain_signed_tx: tx sent: {:?}", acc.id);
				return Err(<Error<T>>::OffchainSignedTxError);
			}
			log::info!("Sent success: {:?}", res);
			// Transaction is sent successfully
			return Ok(());
		}
		log::info!("No local account available");
		Err(<Error<T>>::NoLocalAcctForSigning)
    }
	
	pub fn _increase_transaction_count_vtbc_price(txn_amount: U256, r_sj_usd: U256, sale_from_reserve_value: U256, total_sales_journal: U256, total_reserve: U256) {

		let mut c = U256::from(0_u8);
		let mut r1_r = U256::from(0_u8);
		let total_reserve_in_usd = VtbcUsdRate::<T>::convert_vtbc_to_usd(total_reserve).unwrap();
		log::info!("r_sj_usd: {:?}", r_sj_usd);
		log::info!("Total sells journals: {:?}", total_sales_journal);
		log::info!("total_reserve: {:?}", total_reserve);
		if txn_amount >= total_sales_journal.checked_add(total_reserve_in_usd).unwrap() {
			log::info!("txn amount is greater than total sales journal+total_reserve_in_usd: {:?}", txn_amount);
			c = r_sj_usd.checked_add(total_reserve_in_usd).unwrap(); // c>rsj = r1r = c-rsj, 
			if total_reserve_in_usd == sale_from_reserve_value {	// rsj = 0, r1r = c
				r1_r = total_reserve_in_usd;						// rsj = 0, c > r1r , r1r = sale_from_reserve_value
			}
		}
		else if txn_amount > total_sales_journal {
			log::info!("txn amount is greater than total sales journal: {:?}", txn_amount);
			r1_r = sale_from_reserve_value;				// c>rsj = r1r = c-rsj Covered here
			c = r_sj_usd.checked_add(sale_from_reserve_value).unwrap();
		}
		else if txn_amount <= total_sales_journal {
			log::info!("txn amount is lesser than or equal total sales journal: {:?}", txn_amount);
			r1_r = U256::from(0_u8);
			c = txn_amount;
		}

		let vtbc_current_price= <UsdRate<T>>::get().vtbc_current_price;
        let vtbc_start_price = <UsdRate<T>>::get().vtbc_start_price;

		let vtbc_start_price_u128 = vtbc_start_price.as_u128();
		let vtbc_start_price_float = vtbc_start_price_u128 as f64;
		log::info!("vtbc_start_price_float: {:?}", vtbc_start_price_float);

		let vtbc_current_price_u128 = vtbc_current_price.as_u128();
		let vtbc_current_price_float = vtbc_current_price_u128 as f64;
		log::info!("vtbc_current_price_float: {:?}", vtbc_current_price_float);
		let vtbc_price_constant = 4_000_000_000_000_000_000.0;
		let alpha = vtbc_current_price_float/vtbc_price_constant;
		log::info!("alpha: {:?}", alpha);
    
		log::info!("txn_amount: {:?}", txn_amount);
		let tamountf64 = c.as_u128() as f64;
		log::info!("tamountf64: {:?}", tamountf64);

		let sales_and_reserve = total_sales_journal.checked_add(r1_r).unwrap();
		let sales_and_reserve_float = sales_and_reserve.as_u128() as f64;
		log::info!("sales_and_reserve_float: {:?}", sales_and_reserve_float); 
		
		let gama:f64 = if total_sales_journal.checked_add(total_reserve_in_usd).unwrap() == U256::from(0_u8) {
			1.0
		}
		else {
			tamountf64/sales_and_reserve_float
		};	
		log::info!("gama: {:?}", gama); 

		let t = U256::from(50_u8).checked_mul(U256::from(1_000_000_000_000_000_000_i128)).unwrap();
		let tf64 = t.as_u128() as f64;
		log::info!("tf64: {:?}", tf64);
		let nt = tamountf64/tf64;
		log::info!("nt: {:?}", nt);
		let dec = 0.0005;
        let vtbc_price_float = dec * gama * alpha * nt;
        log::info!("vtbc_price_float: {:?}", vtbc_price_float);
        let vtbc_price_final = vtbc_price_float * 1_000_000_000_000_000_000.0_f64;
        let vtbc_u256 = U256::from(vtbc_price_final as u128);
        log::info!("vtbc_price_final: {:?}", vtbc_price_final);
        log::info!("vtbc_u256: {:?}", vtbc_u256);
		let old_vtbc= Self::get_txn_affected_vtbc_price();
		<TxnAffectedVtbcPrice<T>>::put(vtbc_u256.checked_add(old_vtbc).unwrap());

		//Testing
		let mut old = U256::from(0_u8);
		let mut new = U256::from(0_u8);
		UsdRate::<T>::mutate(|usd_rate| {
			old = usd_rate.vtbc_current_price;
			new = usd_rate.vtbc_current_price.checked_add(vtbc_u256).unwrap();
			usd_rate.vtbc_current_price = new;
        });
		
		Globals::<T>::mutate(|get_globals| {
            get_globals.transaction_count += nt as u64;
        });
		Self::deposit_event(Event::IncreasedByTransaction {
			transaction_count: nt as u64, 
			amount: vtbc_u256
		});
		Self::deposit_event(Event::IncreasedVtbRateDueToTransaction {
			old_vtbc_rate: old, 
			new_vtbc_rate: new
		});
	}

	pub fn initiate_initialize_values_for_apr(vtbc_current: U256) -> Result<(), Error<T>>{

		let is_leap_year = Self::is_leap_year(2022);
		let apr_estimate_contants = AprEstimate::<T>::get(2022);

		if is_leap_year {
			Globals::<T>::mutate(|get_globals| {
            	get_globals.hours_elapsed = 623;
				get_globals.total_hours = 8784;
				get_globals.target_rate_for_year = apr_estimate_contants.target_rate;
				get_globals.start_year = 2022;
				get_globals.current_year = 2022;
        	});
		}
		else {
			Globals::<T>::mutate(|get_globals| {
            	get_globals.hours_elapsed = 623;
				get_globals.total_hours = 8760;
				get_globals.target_rate_for_year = apr_estimate_contants.target_rate;
				get_globals.start_year = 2022;
				get_globals.current_year = 2022;
        	});
		}

		UsdRate::<T>::mutate(|usd_rate| {
            usd_rate.vtbc_start_price = vtbc_current;
			usd_rate.vtbc_current_price = U256::from(5258116238184148478_u128);
			usd_rate.vtbc_last_apr_rate = U256::from(5258116238184148478_u128);
        });

		AprEstimate::<T>::mutate(2022, |obj|{
			obj.start_rate = vtbc_current;
		});
		
		<UsdVtbcH<T>>::put(U256::from(5257458721726318215_u128));
		
		UpdatedTimeList::<T>::mutate(|obj| {
			obj.apr_timestamp = Self::u32_to_moment(1646238479084_u64).unwrap();
		});
		
		<VtbcStartRate<T>>::put(true);
		
		Ok(())
	}

	pub fn hourly_hours_changes(elaps_hours: u32){

		let globals = Self::get_globals();
		if globals.total_hours == globals.hours_elapsed {
			let new_year = globals.current_year + 1;
			let is_leap_year = Self::is_leap_year(new_year as u64);
			let apr_estimate_contants = AprEstimate::<T>::get(new_year);
			let usd_rate = UsdRate::<T>::get();

			AprEstimate::<T>::mutate(globals.current_year, |obj|{
				obj.achieve_rate = usd_rate.vtbc_current_price;
			});

			if is_leap_year {
				Globals::<T>::mutate(|get_globals| {
					get_globals.hours_elapsed = 0;
					get_globals.total_hours = 8784;
					get_globals.target_rate_for_year = apr_estimate_contants.target_rate;
					get_globals.current_year = new_year;
				});
			}
			else {
				Globals::<T>::mutate(|get_globals| {
					get_globals.hours_elapsed = 0;
					get_globals.total_hours = 8760;
					get_globals.target_rate_for_year = apr_estimate_contants.target_rate;
					get_globals.current_year = new_year;
				});
			}
		
			AprEstimate::<T>::mutate(new_year, |obj|{
				obj.start_rate = usd_rate.vtbc_current_price;
			});
			<UsdVtbcH<T>>::put(usd_rate.vtbc_current_price);
		}

		Globals::<T>::mutate(|get_globals| {
			get_globals.hours_elapsed += elaps_hours;
		});
	}

	pub fn initialize_apr_contants() {
		
		for item in constants::TARGET_RATES {
			let year = item[0];
			let target = item[1];
			log::info!("{:?}", item);
			AprEstimate::<T>::mutate(year, |obj|{
				obj.year = year;
				obj.target_rate = target;
			});
		}
	}
}
