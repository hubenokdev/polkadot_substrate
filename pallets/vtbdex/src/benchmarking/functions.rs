//! This module have helper function to set the intial state for the extrinsic so benchmarking must be executed successfully.
use super::*;

use frame_system::RawOrigin as SystemOrigin;
use crate::Pallet as VtbDex;
use sp_std::{collections::vec_deque::VecDeque, prelude::*, str, collections::btree_map::BTreeMap};
use pallet_vtbc_token::{ReserveBalance};
pub use pallet_usd_rate::UsdRate;
use crate::cryptos::utility::VtbcUsdRate;
use frame_support::traits::Len;
use pallet_vtbt::Pallet as Assets;

pub fn create_wallet<T: Config>(pdot_address: &str, 
    user_address: &str, 
    crypto_type: TokenType,
    deposit: U256
    ) 
    -> T::AccountId
{
    let current_period = <Period<T>>::get();
  		
    let user_account_id = <VtbDex<T>>::convert_str_to_valid_account_id(pdot_address).unwrap();
    let account_key = user_account_id.encode();
    if <UserWallet<T>>::contains_key(&account_key) {
        <UserWallet<T>>::mutate(&account_key, |wallet_obj| {
            wallet_obj.update_crypto_details(crypto_type, Some(user_address));
        })
    }
    else {
        let mut wallet = UserType::new(Some(pdot_address), current_period);
		wallet.update_crypto_details(crypto_type, Some(user_address));
		
        <UserWallet<T>>::insert(&account_key, wallet.clone());
    }
   
    ClaimToken::<T>::mutate(&user_account_id, crypto_type, |user| {
        user.to_update_period = 0;
    });	
 
    let _ = <Pallet<T>>::add_update_balance(&crypto_type, &account_key, deposit, U256::from(0_u128));

    assert!(1 == 2 , "user: {:?}", UserWallet::<T>::get(&account_key));

    user_account_id
}

pub fn initialize_system<T: Config>() {
    <ReserveBalance<T>>::put(U256::from(40_0000_0000_0000_0000_0000_0000_u128));
    UsdRate::<T>::mutate(|get_usd_rate| {
        get_usd_rate.eth = U256::from(3003_985_000_000_000_127_329_u128);
        get_usd_rate.eos = U256::from(3003_985_000_000_000_127_329_u128);
        get_usd_rate.vtbc_current_price = U256::from(5459910363094032987_u128); 
    });

    let balances = crate::distribution::types::Balances {
        total_balance: U256::from(0_u64), 
        current_balance : U256::from(0_u64), 
    };

    let time_stamp = <pallet_timestamp::Pallet<T>>::now();
    Distribution::<T>::mutate(0, |distribute| {
        distribute.denominator =  U256::from(0_u64);
        distribute.inittimestamp = time_stamp;
        distribute.closed = false;
        distribute.balances.insert(TokenType::Eth, balances.clone());
        distribute.balances.insert(TokenType::Eos, balances.clone());
        distribute.balances.insert(TokenType::Vtbc, balances);
    });

    <Period<T>>::put(0);

    <VtbSystemRunning<T>>::put(true);

    let fee_collector = "5GCEPvr34nS5BNR4vg2exCxdoYmPnSjSwK2eBHYBds2sG4hA";
    let fee_collector_account_id = <VtbDex<T>>::convert_str_to_valid_account_id(fee_collector).unwrap();

    create_wallet::<T>(fee_collector, "0xfeeaddress", TokenType::Eth, U256::from(0_u128));
    create_wallet::<T>(fee_collector, "eos_account", TokenType::Eos, U256::from(0_u128));

    let fees = custom_types::FeeCollectorAccount {
        fee_collector_address: fee_collector_account_id.clone(),
        fee: U256::from(1_000_000_000_000_000_000_u128),
    };

    VtbdexTransactionFee::<T>::put(fees);
    let _ = <pallet_vtbt::Pallet<T>>::initialize_vtbt_token(T::VtbErc20AssetId::get(), fee_collector_account_id.clone());

}

pub fn create_default_minted_asset<T: Config>(
    account: T::AccountId,
	amount: U256,
) -> (T::AccountId, U256) {
	assert!(Assets::<T>::mint(
		RawOrigin::Signed(account.clone()).into(),
		T::VtbErc20AssetId::get(),
		account.clone(),
		amount,
	)
	.is_ok());

	(account, amount)
}

pub fn create_default_allowance_asset<T: Config>(
	caller: T::AccountId,
	spender: T::AccountId,
	amount: U256,
) -> (T::AccountId, U256) {
	assert!(Assets::<T>::approve(
		RawOrigin::Signed(caller.clone()).into(),
		T::VtbErc20AssetId::get(),
		spender.clone(),
		amount,
	)
	.is_ok());

	(spender, amount)
}

pub fn create_default_sell_orders<T: Config>() -> (T::AccountId, T::AccountId) {
	let seller_address = "5CyuKHaJmThyNLP2zTjrNwSA2U3RQcqWnFFmDBWv1QPyEJ5n";
    let seller_eth_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d4";
    let seller_account_id = create_wallet::<T>(seller_address, seller_eth_address, TokenType::Eth, U256::from(7_000_000_000_000_000_000_u128));

    // assert!(VtbDex::<T>::buy_vtbc(
	// 	SystemOrigin::Signed(seller_account_id.clone()).into(),
	// 	TokenType::Eth,
	// 	U256::from(1_000_000_000_000_000_000_u128),
	// )
	// .is_ok());
 
    // for _i in 0..2 { // Inserting two new sell orders
    //     assert!(VtbDex::<T>::sell_vtbc(
    //         SystemOrigin::Signed(seller_account_id.clone()).into(),
    //         TokenType::Eth,
    //         U256::from(30_000_000_000_000_000_000_u128),
    //     )
    //     .is_ok());
    // }
	
	(seller_account_id.clone(), seller_account_id)
}

pub fn create_default_buy_orders<T: Config>() -> (T::AccountId, T::AccountId) {
	let buyer_address = "5CyuKHaJmThyNLP2zTjrNwSA2U3RQcqWnFFmDBWv1QPyEJ5n";
    let buyer_eth_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d4";
    let buyer_account_id = create_wallet::<T>(buyer_address, buyer_eth_address,  TokenType::Eth, U256::from(70_000_000_000_000_000_u128));
    let account_key = buyer_account_id.encode();
    let _ = <Pallet<T>>::add_update_balance(&TokenType::Vtbc, &account_key, U256::from(0_u128), U256::from(0_u128));

	assert!(VtbDex::<T>::sell_vtbc(
		SystemOrigin::Signed(buyer_account_id.clone()).into(),
		TokenType::Eth,
		U256::from(3_000_000_000_000_000_000_u128),
	)
	.is_ok());

	(buyer_account_id.clone(), buyer_account_id)
}

pub fn insert_sell_orders<T: Config>(seller_account_id: T::AccountId, seller_eth_address: &str, order_id: &str ) {
    // Insert 1st sel order
    let account_key = seller_account_id.encode();

    let _ = <Pallet<T>>::add_update_balance(&TokenType::Vtbc, &account_key, U256::from(1000000000000000000_u128), U256::from(300_000_000_000_000_000_000_u128));
    let sell_amt = U256::from(30_000_000_000_000_000_000_u128);
    let new_sell_order_entry = OrderBookStruct {
        order_id: order_id.as_bytes().to_vec(),
        address: Some(seller_account_id.clone()),
        crypto_address: Some(seller_eth_address.as_bytes().to_vec()),
        amount: sell_amt, 
        usd_rate: U256::from(5_459_910_363_094_032_987_u128),
    };
    let mut push_index = 0_u64;
    OrderIndexedNMap::<T>::mutate((&TradeType::Buy, &TokenType::Eth), |vec_obj|{
        push_index = if let Some(last_index) = vec_obj.last() {
            last_index + 1_u64
        }
        else {
            1_u64
        };
        vec_obj.push(push_index);
    });

    let account_key: Vec<u8> = seller_account_id.encode();
    UserWallet::<T>::mutate(&account_key, |user_data| {
        let user_crypto_data = user_data.crypto_addresses.get_mut(&TokenType::Eth).unwrap();
        match user_crypto_data.order.get_mut(&TradeType::Sell) {
            Some(orders) => {
                orders.insert(new_sell_order_entry.order_id.clone(), push_index);
            },
            None => {
                let mut new_map = BTreeMap::new();
                new_map.insert(new_sell_order_entry.order_id.clone(), push_index);
                user_crypto_data.order.insert(TradeType::Sell, new_map);
            }
        }
        
    });

    OrderBookNMap::<T>::insert((&TradeType::Sell, 
                                &TokenType::Eth, 
                                &push_index), 
                                &new_sell_order_entry); 
    //Insert 2nd sell order
    TotalOrdersCountTillDate::<T>::mutate(|count| {
        *count = *count + 1_u64;
    });
    let order_usd = VtbcUsdRate::<T>::convert_vtbc_to_usd(sell_amt).unwrap();

    <TotalSellsJournal<T>>::mutate(|total| {
        *total = total.checked_add(order_usd).unwrap();
    });
    <Globals<T>>::mutate(|globals_data| {
        globals_data.controlled = globals_data.controlled + sell_amt;
    })
}

pub fn insert_buy_orders<T: Config>(
    seller_account_id: T::AccountId, 
    seller_eth_address: &str, 
    order_id: &str,
    amount: U256 
) {
    // Insert 1st sel order
    let account_key = seller_account_id.encode();

    let _ = <Pallet<T>>::add_update_balance(&TokenType::Eth, &account_key, U256::from(1_000_000_000_000_000_000_u128), amount);

    let new_buy_order_entry = OrderBookStruct {
        order_id: order_id.as_bytes().to_vec(),
        address: Some(seller_account_id.clone()),
        crypto_address: Some(seller_eth_address.as_bytes().to_vec()),
        amount: amount,
        usd_rate: U256::from(5_459_910_363_094_032_987_u128),
    };
    let mut push_index = 0_u64;
    OrderIndexedNMap::<T>::mutate((&TradeType::Buy, &TokenType::Eth), |vec_obj|{
        push_index = if let Some(last_index) = vec_obj.last() {
            last_index + 1_u64
        }
        else {
            1_u64
        };
        vec_obj.push(push_index);
    });

    let account_key: Vec<u8> = seller_account_id.encode();
    UserWallet::<T>::mutate(&account_key, |user_data| {
        let user_crypto_data = user_data.crypto_addresses.get_mut(&TokenType::Eth).unwrap();
        match user_crypto_data.order.get_mut(&TradeType::Buy) {
            Some(orders) => {
                orders.insert(new_buy_order_entry.order_id.clone(), push_index);
            },
            None => {
                let mut new_map = BTreeMap::new();
                new_map.insert(new_buy_order_entry.order_id.clone(), push_index);
                user_crypto_data.order.insert(TradeType::Buy, new_map);
            }
        }
        
    });

    OrderBookNMap::<T>::insert((&TradeType::Buy, 
                                &TokenType::Eth, 
                                &push_index), 
                                &new_buy_order_entry); 

    TotalOrdersCountTillDate::<T>::mutate(|count| {
        *count = *count + 1_u64;
    });
}

pub fn insert_blocked_user_list_for_withdraw<T: Config>(user_account_id: T::AccountId) -> Vec<u8> {
    let now_time = <Pallet<T>>::u32_to_moment(1658249448000_u64).unwrap(); //  Tuesday, 19 July 2022 16:50:48
    pallet_timestamp::Now::<T>::put(now_time);
    //let time_stamp1 = <pallet_timestamp::Pallet<T>>::now();
    let old_len = BlockedUserWallet::<T>::get(&user_account_id).len();
    let blocknumber = frame_system::Pallet::<T>::block_number();
    let time_stamp = <pallet_timestamp::Pallet<T>>::now() - T::PendingWithdrawMinTime::get() - T::PendingWithdrawMinTime::get(); //
    let mut withdraw_claim_data = crate::WithdrawClaim {
        polkadot_address: user_account_id.clone(),
        id: old_len.to_string().as_bytes().to_vec(),
        token_type: TokenType::Eth,
        withdraw_amount: U256::from(10_000_000_000_000_000_000_u128),
        fee: U256::from(1_000_000_000_000_000_000_u128),
        transaction_hash: Vec::new(),
        timestamp: time_stamp,
        node_block_number: blocknumber,
        requested: false
    };
    let bytes: &[u8] = &withdraw_claim_data.encode();
    
    let order_keccak_hash = sp_io::hashing::keccak_256(bytes);
    log::info!("sp_core::hashing::keccak_256: {:?}", &order_keccak_hash);
    let id: Vec<u8> = order_keccak_hash.to_vec();
    withdraw_claim_data.id = id.clone();

    if <BlockedUserWallet<T>>::contains_key(&user_account_id) {
		BlockedUserWallet::<T>::mutate(&user_account_id, |list_opt| {
			let list = list_opt.as_mut().unwrap();
			list.push_back(withdraw_claim_data);
		});
	}
	else {
		let mut data = VecDeque::new();
		data.push_back(withdraw_claim_data);
		<BlockedUserWallet<T>>::insert(&user_account_id, data);
	}

    id
}

pub fn insert_transaction_hash_in_blocked_user_wallet<T: Config>(user_account_id: &T::AccountId, transaction_hash: Vec<u8>, amount: U256) {
    if <BlockedUserWallet<T>>::contains_key(&user_account_id) {
		BlockedUserWallet::<T>::mutate(&user_account_id, |list_opt| {
			let list_mut = list_opt.as_mut().unwrap();
			match list_mut.iter().position(|u| u.withdraw_amount == amount ) {
                Some(index) => {
                    let data = list_mut.get_mut(index).unwrap();
                    data.transaction_hash = transaction_hash;
                   
                },
                None => log::error!("User does not exist in blocked user list")
            }
		});
	}
}

pub fn get_id_blocked_user_list_for_withdraw<T: Config>(user_account_id: T::AccountId) -> (Vec<u8>, U256, WithdrawRequest::<T> ) {
    let now_time = <Pallet<T>>::u32_to_moment(1658249448000_u64).unwrap(); //  Tuesday, 19 July 2022 16:50:48
    pallet_timestamp::Now::<T>::put(now_time);
    let time_stamp1 = <pallet_timestamp::Pallet<T>>::now();
    let old_len = BlockedUserWallet::<T>::get(&user_account_id).len();
    let blocknumber = frame_system::Pallet::<T>::block_number();
   // let time_stamp = <pallet_timestamp::Pallet<T>>::now() - T::PendingWithdrawMinTime::get() - T::PendingWithdrawMinTime::get(); //
    let fee_amt = <Pallet<T>>::check_and_calculate_fee(&TokenType::Eth).unwrap();	
    let mut withdraw_claim_data = crate::WithdrawClaim {
        polkadot_address: user_account_id.clone(),
        id: old_len.to_string().as_bytes().to_vec(),
        token_type: TokenType::Eth,
        withdraw_amount: U256::from(300_000_000_000_000_000_u128),
        fee: fee_amt,
        transaction_hash: Vec::new(),
        timestamp: time_stamp1,
        node_block_number: blocknumber + 1u32.into(),
        requested: false
    };
    let bytes: &[u8] = &withdraw_claim_data.encode();
    
    let order_keccak_hash = sp_io::hashing::keccak_256(bytes);
    log::info!("sp_core::hashing::keccak_256: {:?}", &order_keccak_hash);
    let id: Vec<u8> = order_keccak_hash.to_vec();
    withdraw_claim_data.id = id.clone();

    (id, fee_amt, withdraw_claim_data)
}

pub fn close_and_intialize_new_distribution<T: Config>() {

    let balances = crate::distribution::types::Balances {
        total_balance: U256::from(20_000_000_000_000_000_000_u128), 
        current_balance : U256::from(20_000_000_000_000_000_000_u128), 
    };

    let time_stamp = <pallet_timestamp::Pallet<T>>::now();
    // This is closed period, Ready for distribution among users
    Distribution::<T>::mutate(0, |distribute| {
        distribute.denominator =  U256::from(810_000_000_000_000_000_000_u128);
        distribute.inittimestamp = time_stamp;
        distribute.closed = true;
        distribute.balances.insert(TokenType::Eth, balances.clone());
        distribute.balances.insert(TokenType::Eos, balances.clone());
        distribute.balances.insert(TokenType::Vtbc, balances);
    });

    let balances = crate::distribution::types::Balances {
        total_balance: U256::from(0_u128), 
        current_balance : U256::from(0_u128), 
    };

    let time_stamp = <pallet_timestamp::Pallet<T>>::now();
    Distribution::<T>::mutate(1, |distribute| {
        distribute.denominator =  U256::from(0_u64);
        distribute.inittimestamp = time_stamp;
        distribute.closed = false;
        distribute.balances.insert(TokenType::Eth, balances.clone());
        distribute.balances.insert(TokenType::Eos, balances.clone());
        distribute.balances.insert(TokenType::Vtbc, balances);
    });

    <Period<T>>::put(1);
}

pub fn close_and_intialize_multiple_distribution<T: Config>() {

    let balances = crate::distribution::types::Balances {
        total_balance: U256::from(20_000_000_000_000_000_000_u128), 
        current_balance : U256::from(20_000_000_000_000_000_000_u128), 
    };

    let time_stamp = <pallet_timestamp::Pallet<T>>::now();
    // This is closed period, Ready for distribution among users
    Distribution::<T>::mutate(0, |distribute| {
        distribute.denominator =  U256::from(810_000_000_000_000_000_000_u128);
        distribute.inittimestamp = time_stamp;
        distribute.closed = true;
        distribute.balances.insert(TokenType::Eth, balances.clone());
        distribute.balances.insert(TokenType::Eos, balances.clone());
        distribute.balances.insert(TokenType::Vtbc, balances);
    });

    let balances = crate::distribution::types::Balances {
        total_balance: U256::from(30_000_000_000_000_000_000_u128), 
        current_balance : U256::from(30_000_000_000_000_000_000_u128), 
    };

    let time_stamp = <pallet_timestamp::Pallet<T>>::now();
    // This is closed period, Ready for distribution among users
    Distribution::<T>::mutate(1, |distribute| {
        distribute.denominator =  U256::from(810_000_000_000_000_000_000_u128);
        distribute.inittimestamp = time_stamp;
        distribute.closed = true;
        distribute.balances.insert(TokenType::Eth, balances.clone());
        distribute.balances.insert(TokenType::Eos, balances.clone());
        distribute.balances.insert(TokenType::Vtbc, balances);
    });

    let balances = crate::distribution::types::Balances {
        total_balance: U256::from(0_u128), 
        current_balance : U256::from(0_u128), 
    };

    let time_stamp = <pallet_timestamp::Pallet<T>>::now();
    Distribution::<T>::mutate(2, |distribute| {
        distribute.denominator =  U256::from(0_u64);
        distribute.inittimestamp = time_stamp;
        distribute.closed = false;
        distribute.balances.insert(TokenType::Eth, balances.clone());
        distribute.balances.insert(TokenType::Eos, balances.clone());
        distribute.balances.insert(TokenType::Vtbc, balances);
    });

    <Period<T>>::put(2);
}

pub fn assert_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_has_event(generic_event.into());
}

pub fn assert_last_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

pub fn assert_vtbt_pallet_event<T: Config>(generic_event: <T as pallet_vtbt::Config>::Event) {
	frame_system::Pallet::<T>::assert_has_event(generic_event.into());
}

pub fn assert_vtbc_token_event<T: Config>(generic_event: <T as pallet_vtbc_token::Config>::Event) {
	frame_system::Pallet::<T>::assert_has_event(generic_event.into());
}
