//! Module BuyVtbc
//! It contains trade features of vtb system.
//! Such as BuyVtbc from reserve, BuyVtbc from SellJournal, Open New BuyOrder
//! 
use sp_runtime::{DispatchResult, DispatchError};
use pallet_vtbc_token::{ReserveBalance};
use frame_support::{
	ensure,
    traits::Get
};
use crate::{
    Error, Pallet, Config, Event,
    Vec, TokenType, U256, Encode,
    InitializeDistributionReq,
    users::{WalletTrait, WalletUpdateReq},
    trade::types::{TradeType, OrderBookStruct, TradeRequest},
    TotalSellsJournal, OrderIndexedNMap, OrderBookNMap,
    trade::CancelOrder,
    cryptos::utility::VtbcUsdRate
};

pub struct BuyVtbc<T>(T);

impl<T: Config> BuyVtbc<T> {

    /// initiate_buy_vtbc -> This is public method inside the vtbdex crate.
    /// It will take three params ( AccountId, TokenType, U256)
    /// This method will only be initiated by buy_vtbc extrinsic defined in lib.rs 
    /// This contain the logiv for buy_vtbc.
    /// First it will check and fill sell orders.
    /// Second it will perform buy from reserve, if nothing is left in sell order
    /// Third and last create a new buy order, if nothing is left in sell order and reserve.
    /// This method is also responsible to charge substrate-fee from user And increase VTBC price.
    pub(crate) fn initiate_buy_vtbc(mut buy_req: TradeRequest<T::AccountId>) -> DispatchResult {
        let buyer_address = buy_req.address.clone().ok_or(Error::<T>::NoneValue)?;
        let account_key: Vec<u8> = buyer_address.encode();
       
        let (crypto_address, deposit_balance, active, _) = crate::WalletStorage::<T>::get_wallet_detail_balance(&account_key.encode(), &buy_req.crypto_type)?;
        buy_req.insert_crypto_address(crypto_address);
        let fee = <Pallet<T>>::check_and_calculate_fee(&buy_req.crypto_type)?;
    	let crypto_amt_usd = <Pallet<T>>::convert_crypto_to_usd(buy_req.crypto_amt, &buy_req.crypto_type)?;


		//debug_assert!(deposit_balance >= (buy_req.crypto_amt + fee), "{:?}, {:?}, {:?}", deposit_balance, buy_req.crypto_amt, fee);	
                log::info!("{:?}, {:?}, {:?}", deposit_balance, buy_req.crypto_amt, fee);
		// ensure!(deposit_balance >= (buy_req.crypto_amt + fee), Error::<T>::InsufficientFunds);	
		ensure!(active , Error::<T>::UserWalletIsLocked);	
        ensure!(crypto_amt_usd >= T::MinUsd::get(), Error::<T>::AmountIsLessThanMinUsd25);
        let res = WalletUpdateReq::new(Some(&buy_req.crypto_type), &account_key, None, None);
        WalletTrait::<T, T::AccountId>::pay_trnx_fee_in_given_crypto(&res, &buyer_address, "Buy vtbc")?;
        <Pallet<T>>::deposit_event(Event::BuyVtbcRequested {
            buyer: buyer_address.clone(), 
            token_type: buy_req.crypto_type, 
            crypto_amount: buy_req.crypto_amt
        });	
    
        log::info!("=======================Buy vtbc requested=======================================");
        let mut trasanction_count: u64 = 1;
        let mut r_sj: U256 = U256::from(0_u64);
        let mut r_r1: U256 = U256::from(0_u64);
        let total_sales_journal = <TotalSellsJournal<T>>::get();
        let total_reserve = <ReserveBalance<T>>::get();
        let res = match Self::buy_from_sell_orders(&mut buy_req, &mut trasanction_count) {
            Ok(amount) => {
                r_sj = amount.1;
                match Self::buy_from_reserve(&mut buy_req, &mut trasanction_count) {
                    Ok(amt) => {
                        r_r1 = amt.1;
                        Self::open_new_buy_order(&buy_req, &mut trasanction_count)?;
                        Ok(())
                    },
                    Err(err) => Err(err),
                }
            },
            _ => Err(DispatchError::from(Error::<T>::BuyVtbcFailed)),
        };

        if trasanction_count > 1 {
            let r_r1_usd =  VtbcUsdRate::<T>::convert_vtbc_to_usd(r_r1)?; //Current vtbc rate used
            <Pallet<T>>::calculate_increase_price(r_sj,
                buy_req.crypto_amt,
                r_r1_usd,
                &buy_req.crypto_type,
                total_sales_journal,
                total_reserve                        
            )?;
        }
        log::info!("===================================Buy vtbc Completed ==================================================");
       res
	}

    /// fn buy_from_sell_orders -> This is private method.
    /// It will take four params ( AccountId, TokenType, U256, u64)
    /// This method will only be initiated by initiate_buy_vtbc method 
    /// This contain the logic for buy_vtbc from sell order.
    /// It will check if any sell order exist with a given TokenType, than it will fulfill the sell order 
    /// and will return tuple have remaining crypto_amount and amount bought from sell order in usd
    fn buy_from_sell_orders(buy_req: &mut TradeRequest<T::AccountId>,
        trasanction_count: &mut u64) -> Result<(U256, U256), DispatchError>  {

        let order_index = OrderIndexedNMap::<T>::get((&TradeType::Sell, &buy_req.crypto_type));
        let mut r_sj: U256 = U256::from(0_u64);
        if order_index.is_empty() {
            return Ok((buy_req.crypto_amt, r_sj));
        }

        for index in order_index.iter() {
            if buy_req.crypto_amt <= U256::from(0_u64) {
                break;
            }
            let order_detail = OrderBookNMap::<T>::get((&TradeType::Sell, &buy_req.crypto_type, index)); 
            if let Some(order_info) = order_detail {
                let sell_req = TradeRequest::new_from(buy_req.crypto_type, TradeType::Sell, order_info, *index);
                match Self::fill_crypto_sell_order(buy_req, &sell_req, trasanction_count)
                {
                    Ok(res) => {
                        r_sj = r_sj.checked_add(res.1).ok_or(Error::<T>::NoneValue)?;
                        log::info!("buy fill order: {:?}, rsj: {:?}", res.0, r_sj);
                        buy_req.crypto_amt = buy_req.crypto_amt.checked_sub(res.0).ok_or(Error::<T>::NoneValue)?;
                    },
                    Err(err) => log::info!("Error: {:?}", err),
                }
            }
        }
        Ok((buy_req.crypto_amt, r_sj))
    }

    ///fn fill_crypto_sell_order -> This is private method.
    /// It will take six params (index, TokenType, AccountId, U256, OrderBookStruct, mut ref u64)
    /// This method will only be initiated by buy_from_sell_orders method 
    /// This contain the logic for buy_vtbc from sell order based on given TokenType.
    /// It will fulfill the sell order, give crypto amount to seller and equivalent VTBC to buyer account.
    /// and will return tuple have transaction crypto_amount and amount bought from sell order in usd
    fn fill_crypto_sell_order( buy_req: &TradeRequest<T::AccountId>, 
        sell_req: &TradeRequest<T::AccountId>,
        trasanction_count: &mut u64) -> Result<(U256, U256), DispatchError> {

        log::info!("=======================Buy vtbc requested from eth sell order=======================================");
        let mut txn_crypto: U256 = U256::from(0_u64);
        let mut txn_vtbc: U256 = U256::from(0_u64);

        //convert order vtbc to usd
        let order_crypto = Self::convert_sell_vtbc_to_crypto(sell_req)?;

        if order_crypto == U256::from(0_u64) {
            <CancelOrder<T>>::refund_order(sell_req)?;
        }
        else {
            txn_crypto = buy_req.crypto_amt;
            txn_vtbc = <Pallet<T>>::convert_crypto_to_given_usd_to_vtbc(txn_crypto, sell_req.usd_rate, &sell_req.crypto_type)?;
            if order_crypto <= txn_crypto {
                txn_crypto = order_crypto;
                txn_vtbc = sell_req.vtbc_amt;
            }

            let token_type: TokenType = buy_req.crypto_type;
            let seller_address = if let Some(account_id)  = sell_req.address.as_ref() {
                account_id.encode()
            } else {
                sell_req.crypto_address.clone().ok_or(Error::<T>::NoneValue)?
            };
            let buyer_address = buy_req.address.as_ref().ok_or(Error::<T>::NoneValue)?;

            <Pallet<T>>::sub_update_balance(&token_type, &buyer_address.encode(), txn_crypto, U256::from(0_u64))?;
            <Pallet<T>>::add_update_balance(&token_type, &seller_address, txn_crypto, U256::from(0_u64))?; 
            <Pallet<T>>::add_update_balance(&TokenType::Vtbc, &buyer_address.encode(), txn_vtbc, U256::from(0_u64))?;
            <Pallet<T>>::sub_update_balance(&TokenType::Vtbc, &seller_address, U256::from(0_u64), txn_vtbc)?;
            <Pallet<T>>::add_circulation_token_balance(&TokenType::Vtbc, txn_vtbc)?;                                            

            *trasanction_count += 1;

            //crate::users::MigratedUsers::<T>::update_migrated_seller_balance(sell_req, TokenType::Vtbc)?;
            if sell_req.vtbc_amt == txn_vtbc {
                //UPDATE ORDER BOOK
                crate::trade::orderbook::OrderBook::<T, T::AccountId>::pop_first_old_order(sell_req)?; 
            }
            else {
                OrderBookNMap::<T>::mutate((&TradeType::Sell, 
                    &buy_req.crypto_type, 
                    &sell_req.index), |order_opt| -> DispatchResult {
                        let order = order_opt.as_mut().ok_or(Error::<T>::NoneValue)?;
                        order.amount = order.amount.checked_sub(txn_vtbc).ok_or(Error::<T>::NoneValue)?;

                        Ok(())
                    }
                )?; 
            }
            <Pallet<T>>::deposit_event(Event::BuyVtbcFromSellOrder {
                buyer: buyer_address.clone(), 
                token_type: buy_req.crypto_type, 
                seller: sell_req.address.clone(), 
                order_id: sell_req.id.clone(), 
                crypto_amount: txn_crypto, 
                vtbc_amount: txn_vtbc
        });	
        }
        let r_sj_usd = Self::sub_total_sells_journal(txn_vtbc, sell_req.usd_rate)?;

        Ok((txn_crypto, r_sj_usd))
    }

    ///fn buy_from_reserve -> This is private method.
    /// It will take four params (AccountId, TokenType, U256, mut ref u64)
    /// This method will only be initiated by initiate_buy_vtbc method 
    /// This contain the logic for buy_vtbc from reserve.
    /// It will perform buy from reserve and add crypto for distribution and add equivalent VTBC in buyer account
    /// Method will return tuple have remaining crypto_amount and amount bought from reserve.
    fn buy_from_reserve(buy_req: &mut TradeRequest<T::AccountId>,
        trasanction_count: &mut u64) -> Result<(U256, U256), DispatchError>  {
        log::info!("=======================Buy vtbc requested from reserve=======================================");

        let mut _txn_crypto: U256 = U256::from(0_u64);
        let mut txn_vtbc = U256::from(0_u64);
        let token_type: TokenType = buy_req.crypto_type;
        if buy_req.crypto_amt > U256::from(0_u64) {
            let reserve_balance = <ReserveBalance<T>>::get();
            let amount_vtbc = <Pallet<T>>::convert_crypto_to_vtbc(buy_req.crypto_amt, &buy_req.crypto_type)?;
            let buyer_address = buy_req.address.as_ref().ok_or(Error::<T>::NoneValue)?;

            txn_vtbc = if amount_vtbc <= reserve_balance {amount_vtbc} else {reserve_balance};
            if txn_vtbc > U256::from(0_u64) {
                _txn_crypto = <Pallet<T>>::convert_vtbc_to_crypto(txn_vtbc, &buy_req.crypto_type)?;

                let _ = <pallet_vtbc_token::Pallet<T>>::issue_vtbc_token(buyer_address.clone(), txn_vtbc); //reserve vtbc update
                <Pallet<T>>::sub_update_balance(&token_type, &buyer_address.encode(), _txn_crypto, U256::from(0_u64))?;
                <Pallet<T>>::add_update_balance(&TokenType::Vtbc, &buyer_address.encode(), txn_vtbc, U256::from(0_u64))?;
                <Pallet<T>>::add_circulation_token_balance(&TokenType::Vtbc, txn_vtbc)?; 
                <Pallet<T>>::sub_circulation_token_balance(&token_type, _txn_crypto)?; 
                InitializeDistributionReq::<T>::increase_distribution(_txn_crypto, &token_type)?;

                <Pallet<T>>::deposit_event(Event::BuyVtbcFromReserve {
                    buyer: buyer_address.clone(), 
                    token_type: buy_req.crypto_type,
                    crypto_amount: _txn_crypto, 
                    vtbc_amount: txn_vtbc
                });	
                buy_req.crypto_amt = buy_req.crypto_amt.checked_sub(_txn_crypto).ok_or(Error::<T>::NoneValue)?;
                *trasanction_count += 1;
            }
            else if reserve_balance == U256::from(0_u64) {
                <Pallet<T>>::deposit_event(Event::ReserveBalanceIsZero { amount: txn_vtbc });
            }
            else {
                return Err(DispatchError::from(Error::<T>::CryptoamountIsInsufficientToBuyVtbc))
            }
        }
        Ok((buy_req.crypto_amt, txn_vtbc)) 
    }
    
    ///fn open_new_buy_order -> This is private method.
    /// It will take four params (AccountId, Vec<u8>, TokenType, U256, mut ref u64)
    /// This method will only be initiated by initiate_buy_vtbc method 
    /// This contain the logic to open new buy-order.
    /// This method will execute when SellsJournal and Reserve both are empty.
    /// It will create new buyOrder with the given buyer detail and append at the end of OrderBook.
    fn open_new_buy_order(buy_req: &TradeRequest<T::AccountId>,
        trasanction_count: &mut u64) -> DispatchResult {

        log::info!("=======================Open new buy order=======================================");
        let amount = buy_req.vtbc_amt;
        let amt_crypto_usd = <Pallet<T>>::convert_crypto_to_usd(amount, &buy_req.crypto_type)?;

        if amount <= U256::from(0_u64) { Ok(()) }

        else if amount > U256::from(0_u64) && amt_crypto_usd >= T::MinUsd::get() {
            let sell_index = OrderIndexedNMap::<T>::get((&TradeType::Buy, &buy_req.crypto_type));
            let len = if let Some(last_data) = sell_index.last() {
                last_data + 1_u64
            } else {
                1_u64
            };

            let mut new_buy_order_entry = OrderBookStruct::new(buy_req, len); //.update_amount(amount);
            new_buy_order_entry.update_amount(amount);
            
            let bytes: &[u8] = &new_buy_order_entry.encode();
            let order_keccak_hash = sp_io::hashing::keccak_256(bytes);
            let order_id: Vec<u8> = order_keccak_hash.to_vec();
            new_buy_order_entry.order_id = order_id.clone();

            let buyer_address = buy_req.address.as_ref().ok_or(Error::<T>::NoneValue)?;
            crate::trade::orderbook::OrderBook::<T, T::AccountId>::push_last_new_order(buy_req, new_buy_order_entry)?;
            <Pallet<T>>::sub_update_balance(&buy_req.crypto_type, &buyer_address.encode(), amount, U256::from(0_u64))?;
            <Pallet<T>>::add_update_balance(&buy_req.crypto_type, &buyer_address.encode(), U256::from(0_u64), amount)?;
            <Pallet<T>>::deposit_event(Event::OpenedNewBuyVtbcOrder {
                buyer: buyer_address.clone(), 
                token_type: buy_req.crypto_type, 
                order_id, 
                crypto_amount: amount
            });
            *trasanction_count += 1;
            Ok(())
        }
        else {
            if <ReserveBalance<T>>::get() == U256::from(0_u64) {
                return Err(DispatchError::from(Error::<T>::OpenNewBuyOrderFailedDueToAmountLessThenMinUsd))
            }
            Ok(())  
        }
    }

    pub(crate) fn convert_sell_vtbc_to_crypto(sell_req: &TradeRequest<T::AccountId>) -> Result<U256, Error<T>> {
        let order_vtbc_usd = match sell_req.vtbc_amt.checked_mul(sell_req.usd_rate).ok_or(Error::<T>::NoneValue)?.checked_div(T::CryptoPrecision::get()){
            Some(res) => res,
            _ => return Err(Error::<T>::EthUsdConversionFailed),
        };

        <Pallet<T>>::convert_usd_to_crypto(order_vtbc_usd, &sell_req.crypto_type)
    }

    /// fn sub_total_sells_journal ->  This function is public to the crate.
    /// It take two argument (U256, U256)
    /// Return -> U256 Increased SellJournal Usd value
    /// This method will calculate the VTBC in usd and decrease the runtime state via the calculated usd value. 
    pub(crate) fn sub_total_sells_journal(txn_vtbc: U256, 
        order_usd: U256) -> Result<U256, DispatchError> 
    {
        log::info!("Txn_vtbc: {:?}", txn_vtbc);
        let r_sj_usd = match txn_vtbc.checked_mul(order_usd).ok_or(Error::<T>::NoneValue)?.checked_div(T::CryptoPrecision::get()){
            Some(res) => res,
            _ => return Err(DispatchError::from(Error::<T>::VtbcUsdConversionFailed)),
        };

        log::info!("r_sj_usd: {:?}", r_sj_usd);
        <TotalSellsJournal<T>>::mutate(|total| -> DispatchResult {
            *total = total.checked_sub(r_sj_usd).ok_or(Error::<T>::NoneValue)?;

            Ok(())
        })?;

        Ok(r_sj_usd)
    }
}

#[cfg(test)]
mod test {
 // Next task to write test cases for trade.rs
}