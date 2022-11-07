use crate::{U256, UsdRate, Error, Config};
use crate::*;


pub struct VtbcUsdRate<T>(T);
impl<T: Config> VtbcUsdRate<T> {

    /// fn convert_usd_to_vtbc: This function take one argument of U256 type.
    /// It have the logic to convert provided Usd value to equivalent Vtbc.
    /// Here, T::CryptoPrecision::get() - represent 18decimals such as 1_000_000_000_000_000_000
    pub fn convert_usd_to_vtbc(amount_usd: U256) -> Result<U256, Error<T>>   {

        let rate_vtbc_usd = <UsdRate<T>>::get().vtbc_current_price;
        let res = match amount_usd.checked_mul(T::CryptoPrecision::get()).ok_or(Error::<T>::NumberIsTooBigForU256)?.checked_div(rate_vtbc_usd){
            Some(res) => res,
            _ => return Err(Error::<T>::EthUsdConversionFailed),
        };
        Ok(res)
    }

    /// fn convert_vtbc_to_usd: This function take one argument of U256 type.
    /// It have the logic to convert provided Vtbc value to equivalent Usd.
    /// Here, T::CryptoPrecision::get() - represent 18decimals such as 1_000_000_000_000_000_000
    pub fn convert_vtbc_to_usd(amount_vtbc: U256) -> Result<U256, Error<T>> {

        let rate_vtbc_usd = <UsdRate<T>>::get().vtbc_current_price;
        let res = match amount_vtbc.checked_mul(rate_vtbc_usd).ok_or(Error::<T>::NumberIsTooBigForU256)?.checked_div(T::CryptoPrecision::get()){
            Some(res) => res,
            _ => return Err(Error::<T>::EthUsdConversionFailed),
        };
        Ok(res)
    }
}