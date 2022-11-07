#![cfg_attr(not(feature = "std"), no_std)]
use frame_system::{
	offchain::{
		AppCrypto
	},
};
use sp_std::{vec::Vec, str };
use scale_info::prelude::string::String;
use pallet_cross_chain::{TokenType};
use primitive_types::U256;
pub use pallet::*;

pub mod eos_block;
pub mod constants;
#[path = "../../../../global_constants.rs"] mod global_constants;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	/// This pallet's configuration trait
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_cross_chain::Config {
		/// The identifier type for an offchain worker.
		type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {

		/// Offchain Worker entry point.
		fn offchain_worker(block_number: T::BlockNumber) {
			log::info!("Hello from offchain workers of eth-vtb-contract pallet!");
	
			//To synsc with current blocknumber on ethereum network
			if block_number > 1u32.into(){
				match Self::fetch_eos_current_block_number() {
					Ok(current_eth_block_num) => {	
						let _ = Self::fetch_eos_block_info(current_eth_block_num);
					}
					Err(err) => {
						log::info!("Error: {:?} ", err);
					}
				}
			}
		}
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		// Error returned when not sure which ocw function to executed
		UnknownOffchainMux,

		// Error returned when making signed transactions in off-chain worker
		NoLocalAcctForSigning,
		OffchainSignedTxError,

		// Error returned when making unsigned transactions in off-chain worker
		OffchainUnsignedTxError,

		// Error returned when making unsigned transactions with signed payloads in off-chain worker
		OffchainUnsignedTxSignedPayloadError,

		// Error returned when fetching info from remote
		HttpFetchingError,
		StringParsingError,
		ByteToStringConversionError,
		ResultNotAStringError,
		HexDecodingError,

		// Error returned when fetching ocw store data
		OcwStoreFetchingCurrentBlockError,
		OcwStoreFetchingProcessedBlockError,
		OcwStoreCurrentBlockFetching0thPositionError,

		InvalidValueEos,
		LockAcquiredFailed,
		OnBoardFailed,
		ValueAlreadylinkedWithThisKey,
		PolkadotAddressNotLinked,
		InvalidSs58Address,
		AccountNotOnboardedInSubstrate,
		UserPeriodsDataIsNone,
		RequireOcwKeyOwner,
		EthTransactionHashAlreadyProcessedByAnotherOcw
	}

}

// impl<T: SigningTypes> SignedPayload<T> for custom_types::PricePayload<T::Public, T::BlockNumber> {
// 	fn public(&self) -> T::Public {
// 		self.public.clone()
// 	}
// }

impl<T: Config> Pallet<T> {

	fn vtb_onboard_user(action_params: &serde_json::Map<String, serde_json::Value>, transacion_id:  &[u8]) -> Result<(), sp_runtime::DispatchError> {

		log::info!("action params: {:?}", &action_params);
		
		let user_eos_address = action_params["user"].as_str().ok_or(Error::<T>::InvalidValueEos)?;
		let user_pdot_address = action_params["pdot_address"].as_str().ok_or(Error::<T>::InvalidValueEos)?;

		<pallet_cross_chain::Pallet<T>>::vtb_onboard_user(
			user_pdot_address,
			user_eos_address,
			transacion_id.to_vec(),
			TokenType::Eos
		)?;

		Ok(())
	}

	fn vtb_eos_deposits(action_params: &serde_json::Map<String, serde_json::Value>, transacion_id:  &[u8]) -> Result<(), sp_runtime::DispatchError> {
	
		let user_pdot_address = action_params["pdot_address"].as_str().ok_or(Error::<T>::InvalidValueEos)?;
		let user_eos_address = action_params["user"].as_str().ok_or(Error::<T>::InvalidValueEos)?;
		let amt: Vec<&str> = action_params["quantity"].as_str().ok_or(Error::<T>::InvalidValueEos)?.split_whitespace().collect();
		let f = amt[0].parse::<f64>().map_err(|e| {
            log::info!("UNable to parse: {:?}", e);
            <Error<T>>::InvalidValueEos
        })?;
		let precision = 1_000_000_000_000_000_000.00;
		let amt_in_18_decimal = f * precision;
		let amount = U256::from(amt_in_18_decimal as i128);

		<pallet_cross_chain::Pallet<T>>::vtb_crypto_deposits_sign_trnx(
			Some(user_pdot_address),
			user_eos_address,
			amount,
			transacion_id.to_vec(),
			TokenType::Eos
		)?;

		Ok(())
	}

	fn vtb_eos_withdrawn(action_params: &serde_json::Map<String, serde_json::Value>, transacion_id:  &[u8]) -> Result<(), Error<T>> {

		let user_pdot_address = action_params["pdot_address"].as_str().ok_or(Error::<T>::InvalidValueEos)?;
		let amt: Vec<&str> = action_params["quantity"].as_str().ok_or(Error::<T>::InvalidValueEos)?.split_whitespace().collect();
		let f = amt[0].parse::<f64>().map_err(|e| {
            log::info!("UNable to parse: {:?}", e);
            <Error<T>>::InvalidValueEos
        })?;
		let precision = 1000000000000000000.00;
		let amt_in_18_decimal = f * precision;

		let amount = U256::from(amt_in_18_decimal as i128);

		let _ = <pallet_cross_chain::Pallet<T>>::vtb_crypto_withdraw_sign_trnx(
			user_pdot_address,
			amount,
			transacion_id.to_vec(),
			TokenType::Eos
		);

		Ok(())
	}
}
