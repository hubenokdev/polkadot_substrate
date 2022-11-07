pub mod distribute;
pub mod claim;
pub mod types;
use crate::{
    users::types::Balances, TokenType,
    UserWallet, ClaimToken, Config, Pallet, Encode, Vec
};

impl<T: Config> Pallet<T> {

    /// fn initialize_user_for_distribution(account: &T::AccountId, current_period: u64, token_type: TokenType)
    /// This take three parameters.
    /// It initializes space for the user to maintain each period balance which will be used during processing claim distributable-token.
    /// This function is invoked during onboarding process of the user.
    pub fn initialize_user_for_distribution(account: &T::AccountId, current_period: u64, token_type: TokenType) {
        let userbal = Balances::new();
		let account_key: Vec<u8> = account.encode();

		for assets in [TokenType::Vtbc, TokenType::Vtbt, token_type] {
			if UserWallet::<T>::contains_key(&account_key)
			{
				log::info!("User struct exist");
				Self::check_and_add_new_period_in_user(current_period, account);
			}
			else {			
				UserWallet::<T>::mutate(&account_key, |user| {
                    if let Some(x) = user.processing_distribution_index.get_mut(&token_type) {
                        *x = current_period;
                    }
                    user.latest_period = current_period;
					user.vtbc_period_balance.insert(current_period, userbal);
				});
			
                ClaimToken::<T>::mutate(&account, assets, |user| {
                    user.to_update_period = current_period;
                });	
            }
		}
    }
}