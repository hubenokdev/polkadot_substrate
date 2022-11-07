pub mod eth;
pub mod eos;
mod vtbc;
use eth::EthUsdRate;
use eos::EosUsdRate;
pub use vtbc::VtbcUsdRate;

use crate::*;

impl TokenType {
	pub fn withdraw_params<'a>(&self, address: &str, amt_value: U256) -> Option<(&'a str, Vec<String>)> {
		match self {
			TokenType::Eth => {
				crate::cryptos::utility::eth::withdraw::crypto_req_data(address, amt_value)
			},		
			TokenType::Eos => {
				crate::cryptos::utility::eos::withdraw::crypto_req_data(address, amt_value)
			}
			_ => None
		}
	}
}

impl<T: Config> Pallet<T> {

    /// fn convert_crypto_to_vtbc: This function take two argument of U256 type & TokenType enum.
    /// It have the logic to convert provided Crypto value to equivalent Vtbc based on match condition.
    pub fn convert_crypto_to_vtbc(crypto_amt: U256, crypto_type: &TokenType) -> Result<U256, Error<T>> {

        match crypto_type {
            TokenType::Eth => EthUsdRate::<T>::convert_eth_to_vtbc(crypto_amt),
            TokenType::Eos => EosUsdRate::<T>::convert_eos_to_vtbc(crypto_amt),
            _ =>  Err(Error::<T>::NoneValue),
        }
    }
    /// fn convert_vtbc_to_crypto: This function take two argument of U256 type & TokenType enum.
    /// It have the logic to convert provided Vtbc value to equivalent Crypto amount based on match condition.
    pub fn convert_vtbc_to_crypto(vtbc_amt: U256, crypto_type: &TokenType) -> Result<U256, Error<T>> {

        match crypto_type {
            TokenType::Eth => EthUsdRate::<T>::convert_vtbc_to_eth(vtbc_amt),
            TokenType::Eos => EosUsdRate::<T>::convert_vtbc_to_eos(vtbc_amt),
            _ => Err(Error::<T>::NoneValue),
        }
    }

    /// fn convert_vtbt_to_vtbc: This function take one argument of U256 type.
    /// It have the logic to convert provided Vtbt value to equivalent Vtbc.
    /// Here, T::CryptoPrecision::get() - represent 18decimals such as 1_000_000_000_000_000_000
    pub fn convert_vtbt_to_vtbc(amountvtbt:  U256) -> Result<U256, Error<T>> {

        let vtbusd = <UsdRate<T>>::get().vtbc_current_price;      
		let vtbtamt = amountvtbt.checked_mul(T::CryptoPrecision::get()).ok_or(Error::<T>::NumberIsTooBigForU256)?;
        
        vtbtamt.checked_div(vtbusd).ok_or(Error::<T>::NumberIsTooBigForU256)
    }

    /// fn convert_crypto_to_usd: This function take two argument of U256 type & TokenType enum.
    /// It have the logic to convert provided Crypto value to equivalent Usd based on match condition.
    pub fn convert_crypto_to_usd(crypto_amt: U256, crypto_type: &TokenType) -> Result<U256, Error<T>> {

        match crypto_type {
            TokenType::Eth => Ok(EthUsdRate::<T>::convert_eth_to_usd(crypto_amt)?),
            TokenType::Eos => Ok(EosUsdRate::<T>::convert_eos_to_usd(crypto_amt)?),
            _ => Err(Error::<T>::NoneValue),
        }
    }

    /// fn convert_usd_to_crypto: This function take two argument of U256 type & TokenType enum.
    /// It have the logic to convert provided Usd value to equivalent Crypto based on match condition.
    pub fn convert_usd_to_crypto(usd_amt: U256, crypto_type: &TokenType) -> Result<U256, Error<T>> {

        match crypto_type {
            TokenType::Eth => Ok(EthUsdRate::<T>::convert_usd_to_eth(usd_amt)?),
            TokenType::Eos => Ok(EosUsdRate::<T>::convert_usd_to_eos(usd_amt)?),
            _ => Err(Error::<T>::NoneValue),
        }
    }

    /// fn convert_vtbc_to_given_usd_to_crypto: This function take three argument of U256, U256 & TokenType enum.
    /// It have the logic to convert provided Vtbc value to equivalent Crypto amount based on given usd-rate.
    pub fn convert_vtbc_to_given_usd_to_crypto(vtbc_amt: U256, order_usd: U256, crypto_type: &TokenType) -> Result<U256, Error<T>> {
        let order_vtbc_usd = match vtbc_amt.checked_mul(order_usd).ok_or(Error::<T>::NumberIsTooBigForU256)?.checked_div(T::CryptoPrecision::get()){
            Some(res) => res,
            _ => return Err(Error::<T>::EthUsdConversionFailed),
        };

        Self::convert_usd_to_crypto(order_vtbc_usd, crypto_type)
    }

    /// fn convert_crypto_to_given_usd_to_vtbc: This function take three argument of U256, U256 & TokenType enum.
    /// It have the logic to convert provided Crypto amount to equivalent Vtbc amount based on given usd-rate.
    pub fn convert_crypto_to_given_usd_to_vtbc(crypto_amt: U256, order_usd: U256, crypto_type: &TokenType) -> Result<U256, Error<T>> {
        let txn_usd = Self::convert_crypto_to_usd(crypto_amt, crypto_type)?;
        //convert usd to vtbc
        let txn_vtbc = match txn_usd.checked_mul(T::CryptoPrecision::get()).ok_or(Error::<T>::NumberIsTooBigForU256)?.checked_div(order_usd){
            Some(res) => res,
            _ => return Err(Error::<T>::EthUsdConversionFailed),
        };
           
        Ok(txn_vtbc)
    }

    /// fn check_and_calculate_fee: This function take one argument of reference of TokenType enum.
    /// It have the logic to convert provided Substrate Usd fee value to equivalent Crypto amount based on match condition.
    /// This function is used to convert the fee amount to equivalent provided cryptos.
    pub fn check_and_calculate_fee(crypto_type: &TokenType) -> Result<U256, Error<T>> {

        let fee_collector = <VtbdexTransactionFee<T>>::get().ok_or(Error::<T>::NumberIsTooLowForU256)?;
        match crypto_type {
            TokenType::Eth => EthUsdRate::<T>::convert_usd_to_eth(fee_collector.fee),
            TokenType::Eos => EosUsdRate::<T>::convert_usd_to_eos(fee_collector.fee),
            _ => Err(Error::<T>::NoneValue),
        } 
    }

}