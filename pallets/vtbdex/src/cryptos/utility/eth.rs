
pub(crate) mod withdraw {
    use crate::{U256, Vec};
    use scale_info::prelude::string::String;
	use serde::__private::ToString;
	use sp_std::vec;

    pub(crate) fn crypto_req_data<'a>(address: &str, amt_value: U256) -> Option<(&'a str, Vec<String>)> {
        let amt_value_str = amt_value.to_string();
        let body = serde_json::json!({
            "action_name": crate::constants::ETHEREUM_CONTRACT_METHOD_NAME,
            "eth_address": address,
            "amount": amt_value_str,
        });
        let body_str = serde_json::to_string(&body).unwrap();
        let payload = vec!(body_str);
        Some((crate::global_constants::_WITHDRAW_REQUEST_API_ENDPOINT, payload))
    }
}

use crate::{U256, UsdRate, Error, Config};
use crate::*;
pub struct EthUsdRate<T>(T);
use crate::cryptos::utility::{VtbcUsdRate}; 

impl<T: Config> EthUsdRate<T> {
  
    /// fn convert_eth_to_usd: This function take one argument of U256 type.
    /// It have the logic to convert provided Eth value to equivalent Usd.
    /// Here, T::CryptoPrecision::get() - represent 18decimals such as 1_000_000_000_000_000_000
    /// for e.g
    /// let rate = U256::from(1_094_295_000_000_000_072_760_u128);
    /// let amount_eth = U256::from(6_538_804_839_682_267_u128);
    /// let usd = convert_eth_to_usd(amount_eth).unwrap();
    /// let expected_usd = U256::from(7_155_380_000_000_000_000_000);
    /// assert!(usd, expected_usd)
    pub fn convert_eth_to_usd(amount_eth: U256) -> Result<U256, Error<T>>   {
        
        let rate = <UsdRate<T>>::get().eth;
        let res = match amount_eth.checked_mul(rate).ok_or(Error::<T>::NumberIsTooBigForU256)?.checked_div(T::CryptoPrecision::get()){
            Some(res) => res,
            _ => return Err(Error::<T>::UsdEthConversionFailed),
        };

        Ok(res)
    }

    /// fn convert_usd_to_eth: This function take one argument of U256 type.
    /// It have the logic to convert provided Usd value to equivalent Eth.
    /// Here, T::CryptoPrecision::get() - represent 18decimals such as 1_000_000_000_000_000_000
    pub fn convert_usd_to_eth(amount_usd: U256) -> Result<U256, Error<T>>   {
        
        let rate = <UsdRate<T>>::get().eth;
        let res = match amount_usd.checked_mul(T::CryptoPrecision::get()).ok_or(Error::<T>::NumberIsTooBigForU256)?.checked_div(rate){
            Some(res) => res,
            _ => return Err(Error::<T>::EthUsdConversionFailed),
        };
        
        Ok(res)
    }

    /// fn convert_eth_to_vtbc: This function take one argument of U256 type.
    /// It have the logic to convert provided Eth value to equivalent Vtbc.
    /// Here, T::CryptoPrecision::get() - represent 18decimals such as 1_000_000_000_000_000_000
    pub fn convert_eth_to_vtbc(amount_eth: U256) -> Result<U256, Error<T>> {

        let amt_usd = Self::convert_eth_to_usd(amount_eth)?;

        VtbcUsdRate::<T>::convert_usd_to_vtbc(amt_usd)
    }

    /// fn convert_vtbc_to_eth: This function take one argument of U256 type.
    /// It have the logic to convert provided Vtbc value to equivalent Eth.
    /// Here, T::CryptoPrecision::get() - represent 18decimals such as 1_000_000_000_000_000_000
    pub fn convert_vtbc_to_eth(amount_vtbc: U256) -> Result<U256, Error<T>> {

        let amt_usd = VtbcUsdRate::<T>::convert_vtbc_to_usd(amount_vtbc)?;
    
        Self::convert_usd_to_eth(amt_usd)
    }
}