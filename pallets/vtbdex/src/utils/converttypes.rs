
use crate::{
    Error, Config, Pallet,
    Vec, String, AccountId32, constants,
};
use sp_runtime::DispatchError;
use codec::{Encode, Decode};
use sp_std::convert::TryFrom;
use serde::__private::ToString;

impl<T: Config> Pallet<T> {

    ///fn derive_index_key(block_number: T::BlockNumber, key_type: &Vec<u8>, counter: u64) -> Vec<u8>;
    ///This function derive the key via combining BlockNumber, u64, &str and finally it collect the combined key in
    ///Vec<u8>
    pub fn derive_index_key(block_number: T::BlockNumber, key_type: &[u8], counter: u64) -> Vec<u8> {
		block_number.using_encoded(|encoded_bn| {
            (*constants::ONCHAIN_TX_KEY).iter()
				.chain(b"/".iter())
				.chain(key_type.iter())
				.chain(encoded_bn)
				.chain(counter.to_string().as_bytes().iter())
				.copied()
				.collect::<Vec<u8>>()
		})
	}

    ///fn is_leap_year(year: u64) -> bool
    ///This function is to check the given year is leap year or not.
    ///It return true if the year is leap year else it return false.
    pub fn is_leap_year(year: u64) -> bool {
        //"This should return if a year is either a leap one or not";
        year % 4 == 0 && year % 100 != 0 || year % 400 == 0
    }

    ///fn convert_str_to_valid_account_id(account_address: &str) -> Result<T::AccountId, Error<T>>
    ///This function is to convert given string of SS58 address to AccountId type.
    pub fn convert_str_to_valid_account_id(account_address: &str) -> Result<T::AccountId, DispatchError> 
    //where <T as frame_system::Config>::AccountId: sp_std::default::Default
    {
        let mut output = [0xFF; 48];
        let checksum_len = 2; //for substrate address
        let decoded = bs58::decode(account_address).into(&mut output).map_err(|e| {
            log::info!("Unable to decode: {:?}", e);
            <Error<T>>::NoneValue
        })?;
        let address_32: sp_core::crypto::AccountId32 = AccountId32::try_from(&output[1..decoded-checksum_len]).map_err(|e| {
            log::info!("Unable to accountId: {:?}", e);
            <Error<T>>::NoneValue
        })?; 
        let account_id: T::AccountId = T::AccountId::decode(& mut AccountId32::as_ref(&address_32)).map_err(|e| {
            log::info!("Unable to decode accountid: {:?}", e);
            <Error<T>>::NoneValue
        })?;
        Ok(account_id)
    }

    ///fn fn bytes_to_string(bytes: &[u8]) -> Result<String, Error<T>>
    ///This function is to convert given bytes to String type.
    pub fn bytes_to_string(bytes: &Option<Vec<u8>>) -> Result<String, Error<T>> {

        let address = if let Some(x) = bytes {    
            x
        }
        else {
            "".as_bytes()
        };
        match sp_std::str::from_utf8(address) {
            Ok(data_str) => Ok(data_str.to_string()),
            Err(_err) => Err(Error::<T>::ByteToStringConversionError)
        }
    }
}