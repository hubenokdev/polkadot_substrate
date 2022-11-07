
//! Module trade
//! It contains trade features of vtb system.
//! Such as BuyVTbc, SellVtbc, CancelSell order, CancelBuy order.
pub mod buyvtbc;
pub mod sellvtbc;
pub mod types;
pub mod orderbook;
pub mod cancelorder;
pub use cancelorder::CancelOrder;

use sp_runtime::DispatchResult;
use crate::{
    TokenType, U256,
    Pallet, Config, Error,
    cryptos::utility::VtbcUsdRate
};

impl<T: Config> Pallet<T> {

    /// fn calculate_increase_price ->  This function is public to the crate.
    /// It take six argument (U256, U256, U256, TokenType, U256, U256)
    /// Return -> Ok()
    /// This method will calculate the increase VTBC price based on the Equation2.
    /// Update the Increase price in VTBC field of UsdRate runtime state.
    pub(crate) fn calculate_increase_price(r_sj: U256, 
        buy_order_amt: U256, 
        r_r1_usd: U256, 
        crypto_type: &TokenType, 
        total_sales_journal: U256, 
        total_reserve: U256) -> DispatchResult 
    {
        if r_sj.checked_add(r_r1_usd).ok_or(Error::<T>::NoneValue)? > U256::from(0_u64) {
                
            let crypto_to_vtbc = Self::convert_crypto_to_vtbc(buy_order_amt, crypto_type)?;
            let c =  VtbcUsdRate::<T>::convert_vtbc_to_usd(crypto_to_vtbc)?;
            let r_sj_usd =  VtbcUsdRate::<T>::convert_vtbc_to_usd(r_sj)?;
            crate::apr::TransactionalIncrease::<T>::increase_transaction_count_vtbc_price(c, r_sj_usd, r_r1_usd, total_sales_journal, total_reserve)?;
        }
        Ok(())
    }
}