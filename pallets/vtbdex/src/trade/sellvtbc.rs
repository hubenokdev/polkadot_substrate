//! Module trade
//! It contains trade features of vtb system.
//! Such as SellVtbc, Fulfill BuyOrder, Open NewSellOrder.
//! 
use sp_runtime::{DispatchError, DispatchResult};
use pallet_usd_rate::UsdRate;
use pallet_vtbc_token::{ReserveBalance};
use frame_support::{
	ensure, traits::Get
};
use crate::{
    Vec, U256, Encode,
    TokenType,
    Error, Pallet, Config, Event,
    trade::types::{TradeType, OrderBookStruct, TradeRequest},
    {TotalSellsJournal, OrderIndexedNMap, OrderBookNMap, },
    users::{WalletTrait, WalletUpdateReq},
    cryptos::utility::VtbcUsdRate,
    trade::CancelOrder,
};

pub struct SellVtbc<T>(T);

impl<T: Config> SellVtbc<T> {

    /// initiate_sell_vtbc -> This is public method inside the vtbdex crate.
    /// It will take three params ( AccountId, TokenType, U256)
    /// This method will only be initiated by sell_vtbc extrinsic defined in lib.rs 
    /// This contain the logic for sell_vtbc.
    /// First it will check and fill buy orders.
    /// Second last create a new sell order, if nothing is left in buy order.
    /// This method is also responsible to charge substrate-fee from user And increase VTBC price.
    pub(crate) fn initiate_sell_vtbc(mut sell_req: TradeRequest<T::AccountId>) -> Result<(), DispatchError> {
        //log::info!("=======================Initiate sell vtbc=======================================");
        let seller_id = sell_req.address.clone().ok_or(Error::<T>::NoneValue)?;
        let account_key = seller_id.encode();
        
        let (crypto_address, _, active, user_vtbc_balance) = 
        crate::WalletStorage::<T>::get_wallet_detail_balance(&account_key, &sell_req.crypto_type)?;
        //crate::trade::orderbook::OrderBook::<T, T::AccountId>::get_crypto_address(&sell_req)?;
        sell_req.insert_crypto_address(crypto_address);
        let crypto_amt_usd = match  VtbcUsdRate::<T>::convert_vtbc_to_usd(sell_req.vtbc_amt) {
            Ok(amt) => amt,
            Err(_err) => U256::from(0_u64)
        };
        
        ensure!(crypto_amt_usd >= T::MinUsd::get(), Error::<T>::AmountIsLessThanMinUsd25);

		ensure!(user_vtbc_balance >= sell_req.vtbc_amt, Error::<T>::InsufficientFunds);	
		ensure!(active , Error::<T>::UserWalletIsLocked);	
        let req = WalletUpdateReq::new(Some(&sell_req.crypto_type), &account_key, None, None );	
        WalletTrait::<T, T::AccountId>::pay_trnx_fee_in_given_crypto(&req, &seller_id, "Sell vtbc")?;
		<Pallet<T>>::deposit_event(Event::SellVtbcRequested {
            seller: seller_id.clone(), 
            vtbc_amount: sell_req.vtbc_amt
        });	

        let mut trasanction_count: u64 = 1;
        let res = match Self::fill_buy_order(&mut sell_req, &mut trasanction_count) {
            Ok(_amount) => {
                Self::open_new_sell_order(&sell_req, &mut trasanction_count)?;
                Ok(())
            },
            _ => return Err(DispatchError::from(Error::<T>::SellVtbcFailed)),
        };

        log::info!("===================================Sell vtbc Completed ==================================================");
        res
	}

    ///fn fill_buy_order -> This is private method.
    /// It will take four params ( AccountId, TokenType, U256, u64)
    /// This method will only be initiated by initiate_sell_vtbc method 
    /// This contain the logic to fulfill opened buy order.
    /// It will check if any buy order exist with a given TokenType, than it will fulfill the buy order 
    /// and will return remaining VTBC
    fn fill_buy_order(sell_req: &mut TradeRequest<T::AccountId>, trasanction_count: &mut u64) -> Result<U256, DispatchError> {
        log::info!("=======================fill buy order initiate=======================================");
        let order_index = OrderIndexedNMap::<T>::get((&TradeType::Buy, &sell_req.crypto_type));

        if order_index.is_empty() {
            return Ok(sell_req.vtbc_amt);
        }
  
        for index in order_index.iter() {
            if sell_req.vtbc_amt <= U256::from(0_u64) {
                break;
            }
            let mut r_sj = U256::from(0_u64);
            let order_detail = OrderBookNMap::<T>::get((&TradeType::Buy, &sell_req.crypto_type, &index)); 
            if let Some(order_info) = order_detail {
                let buy_req = TradeRequest::new_from(sell_req.crypto_type, TradeType::Buy, order_info, *index);
                match Self::fill_crypto_buy_order(&buy_req, sell_req, trasanction_count)
                {
                    Ok(txn_vtbc) => {
                        r_sj = txn_vtbc;
                        sell_req.vtbc_amt = sell_req.vtbc_amt.checked_sub(txn_vtbc).ok_or(Error::<T>::NoneValue)?;
                    },
                    Err(err) => log::info!("Error: {:?}", err),
                }
                let total_sales_journal = <TotalSellsJournal<T>>::get();
                let total_reserve = <ReserveBalance<T>>::get(); 
                <Pallet<T>>::calculate_increase_price(
                    r_sj,
                    buy_req.crypto_amt,
                    U256::from(0_u64),
                    &sell_req.crypto_type,
                    total_sales_journal,
                    total_reserve
                )?; 
            }
        }
        
        Ok(sell_req.vtbc_amt)
    }

    ///fn fill_crypto_buy_order -> This is private method.
    /// It will take six params (index, TokenType, AccountId, U256, OrderBookStruct, mut ref u64)
    /// This method will only be initiated by fill_buy_order method 
    /// This contain the logic to fulfill opened buy order.
    /// It will fulfill the buy order, and give VTBC to buyer and equivalent crypto amount to seller
    /// and will return transaction VTBC
    fn fill_crypto_buy_order(buy_order: &TradeRequest<T::AccountId>,
        sell_req: &TradeRequest<T::AccountId>,
        trasanction_count: &mut u64) -> Result<U256, DispatchError> {

        log::info!("=======================Fill eth buy order=======================================");
        let mut txn_vtbc: U256 = U256::from(0_u64);
       // let (order_id, buyer_address, _crypto_address, buy_order_amt, _usd_rate) =  Details::<T::AccountId>::get_order_detail_tuple(buy_order_data);

        let order_vtbc = <Pallet<T>>::convert_crypto_to_vtbc(buy_order.crypto_amt, &buy_order.crypto_type)?;
        if order_vtbc == U256::from(0_u64) {
            <CancelOrder<T>>::refund_order(buy_order)?;
        }
        else {
            let mut txn_crypto = <Pallet<T>>::convert_vtbc_to_crypto(sell_req.vtbc_amt, &sell_req.crypto_type)?;
            
            txn_vtbc = sell_req.vtbc_amt;

            if buy_order.crypto_amt <= txn_crypto {
                txn_crypto = buy_order.crypto_amt;
                txn_vtbc = order_vtbc;
            }
            let seller_id = sell_req.address.as_ref().ok_or(Error::<T>::NoneValue)?;
            let buyer_id = buy_order.address.as_ref().ok_or(Error::<T>::NoneValue)?;
            <Pallet<T>>::deposit_event(Event::SellVtbcToFillBuyOrder {
                seller: seller_id.clone(), 
                token_type: sell_req.crypto_type, 
                buyer: buyer_id.clone(), 
                order_id: buy_order.id.clone(),
                vtbc_amount: txn_vtbc, 
                crypto_amount: txn_crypto 
            });	

            <Pallet<T>>::sub_update_balance(&buy_order.crypto_type, &buyer_id.encode(), U256::from(0_u64), txn_crypto)?;
            <Pallet<T>>::add_update_balance(&sell_req.crypto_type, &seller_id.encode(), txn_crypto, U256::from(0_u64))?;
            <Pallet<T>>::sub_update_balance(&TokenType::Vtbc, &seller_id.encode(), txn_vtbc, U256::from(0_u64))?;
            <Pallet<T>>::add_update_balance(&TokenType::Vtbc, &buyer_id.encode(), txn_vtbc, U256::from(0_u64))?;
        
            if buy_order.crypto_amt == txn_crypto {
                //UPDATE ORDER BOOK
                crate::trade::orderbook::OrderBook::<T, T::AccountId>::pop_first_old_order(buy_order)?;
            }
            else {
                OrderBookNMap::<T>::mutate(
                    (&TradeType::Buy, 
                    &buy_order.crypto_type, 
                    &buy_order.index
                ), |order_opt| -> DispatchResult {
                        let order = order_opt.as_mut().ok_or(Error::<T>::NoneValue)?;
                        order.amount = order.amount.checked_sub(txn_crypto).ok_or(Error::<T>::NoneValue)?;

                        Ok(())
                    })?; 
            }
            *trasanction_count += 1;
        }

        Ok(txn_vtbc)
    }

    /// fn open_new_sell_order -> This is private method.
    /// It will take four params (AccountId, Vec<u8>, TokenType, U256, mut ref u64)
    /// This method will only be initiated by initiate_sell_vtbc method 
    /// This contain the logic to open new sell-order.
    /// This method will execute when BuyJournal is empty.
    /// It will create new sellOrder with the given seller detail, current VTBC rate and append at the end of OrderBook.
    fn open_new_sell_order(sell_req: &TradeRequest<T::AccountId>, trasanction_count: &mut u64) -> DispatchResult  {
        
        log::info!("=======================Open new sell order=======================================");
        let volume_vtbc = sell_req.vtbc_amt;
        let order_usd = VtbcUsdRate::<T>::convert_vtbc_to_usd(volume_vtbc)?;

        if volume_vtbc <= U256::from(0_u64) { Ok(()) }

        else if volume_vtbc > U256::from(0_u64) && order_usd >= T::MinUsd::get() {
            let sell_index_vec = OrderIndexedNMap::<T>::get((&TradeType::Sell, &sell_req.crypto_type));
            let len = if let Some(index_data) = sell_index_vec.last()  {
                index_data + 1_u64
            } 
            else {
                1_u64 
            };
            let usd_rate =  <UsdRate<T>>::get().vtbc_current_price;

            let mut new_sell_order_entry = OrderBookStruct::new(sell_req, len);
            new_sell_order_entry.update_amount(volume_vtbc);
            new_sell_order_entry.update_usd_rate(usd_rate);
    
            let bytes: &[u8] = &new_sell_order_entry.encode();
            let order_keccak_hash = sp_io::hashing::keccak_256(bytes);
            let order_id: Vec<u8> = order_keccak_hash.to_vec();
            new_sell_order_entry.order_id = order_id.clone();

            crate::trade::orderbook::OrderBook::<T, T::AccountId>::push_last_new_order(sell_req, new_sell_order_entry)?;
            <TotalSellsJournal<T>>::mutate(|total| -> DispatchResult {
                *total = total.checked_add(order_usd).ok_or(Error::<T>::NoneValue)?;

                Ok(())
            })?;
            let seller_id = sell_req.address.as_ref().ok_or(Error::<T>::NoneValue)?;

            <Pallet<T>>::sub_update_balance(&TokenType::Vtbc, &seller_id.encode(), volume_vtbc, U256::from(0_u64))?;
            <Pallet<T>>::add_update_balance(&TokenType::Vtbc, &seller_id.encode(), U256::from(0_u64), volume_vtbc)?;
            <Pallet<T>>::sub_circulation_token_balance(&TokenType::Vtbc, volume_vtbc)?;  
            *trasanction_count += 1;

            <Pallet<T>>::deposit_event(Event::OpenSellOrder {
                seller: seller_id.clone(), 
                order_id, 
                token_type: sell_req.crypto_type, 
                vtbc_amount: volume_vtbc 
            });
            
            Ok(())
        }
        else {
            Err(DispatchError::from(Error::<T>::OpenNewSellOrderFailedDueToAmountLessThenMinUsd))
        }
    }
}

#[cfg(test)]
mod test {
 // Next task to write test cases for trade.rs
}