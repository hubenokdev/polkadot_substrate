
use crate::*;
use crate::cryptos::utility::VtbcUsdRate;
use sp_runtime::DispatchResult;
pub struct TransactionalIncrease<T>(T);

impl<T: Config> TransactionalIncrease<T> {
    
	///pub fn increase_transaction_count_vtbc_price(
	///	txn_amount: U256, 
	///	r_sj_usd: U256, 
	///	sale_from_reserve_value: U256, 
	///	total_sales_journal: U256, 
	///	total_reserve: U256
	///	) -> DispatchResult
	/// 
	/// This function is invoked when ever some Buy vtbc is done.
	/// This calculate and increase the Vtbc affected price due to Buy transaction.
	/// 
	/// Here the below if conditions handles the different cases of Buy..
	/// i.e BuyFromReserve(r1r), BuyFromSellOrder(rsj), Open new buyOrder(In this case txn_amount will decrease)
	/// all the above three scenarios affect the 'c' value.
	///
	/// C: is the invested capital (We assume that there is a transaction concluded)
	/// 𝑹 𝒔𝒋 : is the total value of the sales journal (but not yet sold) entered in the register.
	/// 𝑹 𝒓 : is the total value of the reserve.
	/// 𝑹 𝟏 𝒓 : is the total value sell from the reserve.
	/// 𝑹 𝒑𝒋 : is the total value of the purchase journal entered in the register.
	/// 
	/// a) If C > 𝑹 𝒔𝒋 + 𝑹 𝒓 then Max(𝟎, 𝑪 − 𝑹 𝒔𝒋 − 𝑹 𝒓 ) : the value recorded in the purchase
	/// journal.
	/// b) If C ≤ 𝑹 𝒔𝒋 + 𝑹 𝒓 then Min (𝑹 𝒓 , 𝐦𝐚𝐱 (𝟎, 𝑪 − 𝑹 𝒔𝒋 )) : is the total VTBC purchased from
	/// the reserve.
	/// If C > 𝑹 𝒔𝒋 + 𝑹 𝒓 then (C-𝑹 𝒔𝒋 - 𝑹 𝒓 ) must be registered in purchase journal and C := 𝑹 𝒔𝒋
	/// + 𝑹 𝒓 and 𝑹 𝟏 𝒓 : = 𝑹 𝒓
	/// Else If C > 𝑹 𝒔𝒋 then 𝑹 𝟏 𝒓 = (C-𝑹 𝒔𝒋 ) is the total value sell from the reserve and C := 𝑹 𝒔𝒋
	/// + 𝑹 𝟏 𝒓
	/// Else if C ≤ 𝑹 𝒔𝒋 then 𝑹 𝟏 𝒓 : = 0
    pub fn increase_transaction_count_vtbc_price(
        txn_amount: U256, 
        r_sj_usd: U256, 
        sale_from_reserve_value: U256, 
        total_sales_journal: U256, 
        total_reserve: U256
    ) -> DispatchResult {

		let mut c = U256::from(0_u8);
		let mut r1_r = U256::from(0_u8);
		let total_reserve_in_usd = VtbcUsdRate::<T>::convert_vtbc_to_usd(total_reserve)?;
		log::info!("r_sj_usd: {:?}", r_sj_usd);
		log::info!("Total sells journals: {:?}", total_sales_journal);
		log::info!("total_reserve: {:?}", total_reserve);
		if txn_amount >= total_sales_journal.checked_add(total_reserve_in_usd).ok_or(Error::<T>::NumberIsTooBigForU256)? {
			log::info!("txn amount is greater than total sales journal+total_reserve_in_usd: {:?}", txn_amount);
			// c>rsj = r1r = c-rsj, 
			c = r_sj_usd.checked_add(total_reserve_in_usd).ok_or(Error::<T>::NumberIsTooBigForU256)?; 
			// rsj = 0, r1r = c
			// rsj = 0, c > r1r , r1r = sale_from_reserve_value
			if total_reserve_in_usd == sale_from_reserve_value {	
				r1_r = total_reserve_in_usd;						
			}
		}
		else if txn_amount > total_sales_journal {
			log::info!("txn amount is greater than total sales journal: {:?}", txn_amount);
			// c>rsj = r1r = c-rsj Covered here
			r1_r = sale_from_reserve_value;				
			c = r_sj_usd.checked_add(sale_from_reserve_value).ok_or(Error::<T>::NumberIsTooBigForU256)?;
		}
		else if txn_amount <= total_sales_journal {
			log::info!("txn amount is lesser than or equal total sales journal: {:?}", txn_amount);
			r1_r = U256::from(0_u8);
			c = txn_amount;
		}

		log::info!("txn_amount: {:?}", txn_amount);
		let (increased_value, nt) = Self::calculate_increased_transaction_price(
            total_sales_journal,
            r1_r,
            total_reserve_in_usd,
            c
        )?;
        Self::update_increased_price_state(increased_value, nt)
	}

	///fn calculate_increased_transaction_price(
	///	total_sales_journal: U256, 
	///	r1_r: U256, 
	///	total_reserve_in_usd: U256, 
	///	c: U256, 
	/// ) -> Result<(U256, u64), Error<T>> 
	/// 
	/// This function calculate the actual increased value due to Buy Transaction.
	/// Formula = 0.0005 *alpha * gamma* NT
	/// where,
	/// NT: number of current transactions (C/M=1000/50=20 transactions)
	/// M: is the price of a transaction at the beginning of the system and this value is $50.
	/// alpha = 𝑽𝑻𝑩𝑪𝒄𝒖𝒓𝒓𝒆𝒏𝒕_𝒑𝒓𝒊𝒄𝒆/𝑽𝑻𝑩𝑪𝒔𝒕𝒂𝒓𝒕_𝒑𝒓𝒊𝒄𝒆
	/// 𝑽𝑻𝑩𝑪𝒔𝒕𝒂𝒓𝒕_𝒑𝒓𝒊𝒄𝒆 = $4 (i.e 4_000_000_000_000_000_000.0)
	/// gamma = C/(Rsj + rr1) (0<= gamma <= 1)
	/// Rsj = Total sells journal value.
	/// Rr1 = Total reserve value
	/// rr1 = Amount bought from reserve.
	/// if Rsj + Rr1 == 0 (gamma = 1.0)
	/// 
    fn calculate_increased_transaction_price(
        total_sales_journal: U256, 
        r1_r: U256, 
        total_reserve_in_usd: U256, 
        c: U256, 
    ) -> Result<(U256, u64), Error<T>> {
        let usd_rates = <UsdRate<T>>::get();
        let vtbc_current_price= usd_rates.vtbc_current_price;
		let vtbc_current_price_u128 = vtbc_current_price.as_u128();
		let vtbc_current_price_float = vtbc_current_price_u128 as f64;
		log::info!("vtbc_current_price_float: {:?}", vtbc_current_price_float);
		// 𝑽𝑻𝑩𝑪𝒔𝒕𝒂𝒓𝒕_𝒑𝒓𝒊𝒄𝒆 = $4
		let vtbc_price_constant = 4_000_000_000_000_000_000.0;
		// alpha = 𝑽𝑻𝑩𝑪𝒄𝒖𝒓𝒓𝒆𝒏𝒕_𝒑𝒓𝒊𝒄𝒆/𝑽𝑻𝑩𝑪𝒔𝒕𝒂𝒓𝒕_𝒑𝒓𝒊𝒄𝒆
		let alpha = vtbc_current_price_float/vtbc_price_constant;
		log::info!("alpha: {:?}", alpha);
    
		//C
		let tamountf64 = c.as_u128() as f64;
		log::info!("tamountf64: {:?}", tamountf64);

		let sales_and_reserve = total_sales_journal.checked_add(r1_r).ok_or(Error::<T>::NumberIsTooBigForU256)?;
		let sales_and_reserve_float = sales_and_reserve.as_u128() as f64;
		log::info!("sales_and_reserve_float: {:?}", sales_and_reserve_float); 
		
		//gamma = C/(Rsj + rr1) (0<= gamma <= 1)
		let gama:f64 = if total_sales_journal.checked_add(total_reserve_in_usd).ok_or(Error::<T>::NumberIsTooBigForU256)? == U256::from(0_u8) {
			1.0
		}
		else {
			tamountf64/sales_and_reserve_float
		};	
		log::info!("gama: {:?}", gama); 
		
		//M: is the price of a transaction at the beginning of the system and this value is $50.
		let m = U256::from(50_u8).checked_mul(U256::from(1_000_000_000_000_000_000_i128)).ok_or(Error::<T>::NumberIsTooBigForU256)?;
		let mf64 = m.as_u128() as f64;
		log::info!("tf64: {:?}", mf64);
		//NT: number of current transactions (C/M)
		let nt = tamountf64/mf64;
		log::info!("nt: {:?}", nt);
		let dec = 0.0005;
        let vtbc_price_float = dec * gama * alpha * nt;
        log::info!("vtbc_price_float: {:?}", vtbc_price_float);
        let vtbc_price_final = vtbc_price_float * 1_000_000_000_000_000_000.0_f64;
        let vtbc_u256 = U256::from(vtbc_price_final as u128);
        log::info!("vtbc_price_final: {:?}", vtbc_price_final);
        log::info!("vtbc_u256: {:?}", vtbc_u256);

        Ok((vtbc_u256, nt as u64))
    }

	///fn update_increased_price_state(increased_price: U256, transaction_count: u64) -> DispatchResult
	/// 
	/// This function update the state related to transactional increased price.
	/// After uodating the state successfuly it emit event named as 
	/// IncreasedByTransaction and IncreasedVtbRateDueToTransaction.
    fn update_increased_price_state(increased_price: U256, transaction_count: u64) -> DispatchResult {
        let old_vtbc= <Pallet<T>>::get_txn_affected_vtbc_price();
		<TxnAffectedVtbcPrice<T>>::put(increased_price.checked_add(old_vtbc).ok_or(Error::<T>::NumberIsTooBigForU256)?);

		let mut old = U256::from(0_u8);
		let mut new = U256::from(0_u8);
		UsdRate::<T>::mutate(|usd_rate| -> DispatchResult {
			old = usd_rate.vtbc_current_price;
			new = usd_rate.vtbc_current_price.checked_add(increased_price).ok_or(Error::<T>::NumberIsTooBigForU256)?;
			usd_rate.vtbc_current_price = new;

            Ok(())
        })?;
		
		Globals::<T>::mutate(|get_globals| {
            get_globals.transaction_count += transaction_count;
        });
		<Pallet<T>>::deposit_event(Event::IncreasedByTransaction {
			transaction_count, 
			amount: increased_price
		});
		<Pallet<T>>::deposit_event(Event::IncreasedVtbRateDueToTransaction {
			old_vtbc_rate: old, 
			new_vtbc_rate: new
		});

        Ok(())
    }

}
