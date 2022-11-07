
use super::*;

use frame_system::RawOrigin;
use frame_support::ensure;
use frame_benchmarking::{benchmarks, whitelisted_caller, impl_benchmark_test_suite};
use pallet_vtbdex::benchmarking::*;
use serde::__private::ToString;
use codec::Encode;

fn assert_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_has_event(generic_event.into());
}

benchmarks! {
    submit_signed_user_new_wallet_record {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_account_id = <pallet_vtbdex::Pallet<T>>::convert_str_to_valid_account_id(pdot_address).unwrap();
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let token_type = TokenType::Eth;
        let current_period = <pallet_vtbdex::Pallet<T>>::get_current_period();
		let mut wallet = pallet_vtbdex::UserType::new(Some(pdot_address), current_period);
		wallet.update_crypto_details(token_type, Some(user_address));

        let txn_hash = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let txn_hash_lc = txn_hash.to_lowercase();
        let txn_hash_vec = txn_hash_lc.to_string().as_bytes().to_vec();

        let caller: T::AccountId = whitelisted_caller();
    }:_(RawOrigin::Signed(caller.clone()), token_type, wallet.clone(), user_account_id.clone(), txn_hash_vec)
    verify {   
        ensure!(<pallet_vtbdex::WalletStorage<T>>::is_user_exist(&user_account_id) == true, "Extrinsic does not executed successfully,User wallet not created must be stopped");
        assert_event::<T>(Event::CreateWallet {
            signer: Some(caller), 
            token_type, 
            user_wallet_data: wallet 
        }.into());
    }

    submit_signed_user_wallet_add_new_crypto {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let user_account_id = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(7_000_000_000_000_000_000_u128));

        let new_crypto_address = "eos-linked-name";
        let txn_hash = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let txn_hash_lc = txn_hash.to_lowercase();
        let txn_hash_vec = txn_hash_lc.to_string().as_bytes().to_vec();
        let token_type = TokenType::Eos;
        let crypto_detail = pallet_vtbdex::UserCryptoBasedData::new(token_type, new_crypto_address);
        let caller: T::AccountId = whitelisted_caller();
    }:_(RawOrigin::Signed(caller.clone()), token_type, user_account_id.clone(), crypto_detail.clone(), txn_hash_vec)
    verify {   
        ensure!(<pallet_vtbdex::WalletStorage<T>>::is_user_exist(&user_account_id) == true, "Extrinsic does not executed successfully,User wallet not created must be stopped");
        assert_event::<T>(Event::AddedNewCrypto {
            signer: Some(caller), 
            token_type, 
            user_address: user_account_id, 
            crypto_data: crypto_detail
        }.into());
    }

    crypto_deposit_success {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let token_type = TokenType::Eth;
        let user_account_id = create_wallet::<T>(pdot_address, user_address, token_type, U256::from(7_000_000_000_000_000_000_u128));
        let txn_hash = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let txn_hash_lc = txn_hash.to_lowercase();
        let txn_hash_vec = txn_hash_lc.to_string().as_bytes().to_vec();
        let deposit_amount = U256::from(20_000_000_000_000_000_000_u128); 
        let crypto_detail = pallet_vtbdex::UserCryptoBasedData::new(token_type, user_address);
        let current_period = <pallet_vtbdex::Pallet<T>>::get_current_period();
		let mut wallet = pallet_vtbdex::UserType::new(None, current_period);

        let caller: T::AccountId = whitelisted_caller();
    }:_(RawOrigin::Signed(caller.clone()), token_type, Some(user_account_id.clone()), deposit_amount, txn_hash_vec.clone(), Some(crypto_detail), Some(wallet))
    verify {   
        ensure!(<pallet_vtbdex::WalletStorage<T>>::is_user_exist(&user_account_id) == true, "Extrinsic does not executed successfully,User deposit not updated");
        assert_event::<T>(Event::DepositSuccess {
            signer: Some(caller.clone()), 
            token_type, 
            user_address: Some(user_account_id.clone()), 
            transaction_id: txn_hash_vec.clone(), 
            amount: deposit_amount
        }.into());
        ensure!(<TransactionHashList<T>>::get(&token_type).contains(&txn_hash_vec) == true, "Transaction hash not updated for deposit extrinsic");
        let user_data = <pallet_vtbdex::WalletStorage<T>>::get_wallet_record(&user_account_id.encode());
        ensure!(user_data.crypto_addresses.get(&token_type).unwrap().deposit_balance == U256::from(27_000_000_000_000_000_000_u128), "Deposit balance is incorrect");
        assert_event::<T>(Event::UpdateBalance {
            signer: Some(caller), 
            token_type, 
            user_wallet_data: user_data
        }.into());
    }

    crypto_withdraw_success {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let token_type = TokenType::Eth;
        let user_account_id = create_wallet::<T>(pdot_address, user_address, token_type, U256::from(7_000_000_000_000_000_000_u128));
        let txn_hash = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let txn_hash_lc = txn_hash.to_lowercase();
        let txn_hash_vec = txn_hash_lc.to_string().as_bytes().to_vec();
        let withdraw_amount = U256::from(10_000_000_000_000_000_000_u128); 
        insert_blocked_user_list_for_withdraw::<T>(user_account_id.clone());
        insert_transaction_hash_in_blocked_user_wallet::<T>(&user_account_id, txn_hash_vec.clone(), withdraw_amount);
        let caller: T::AccountId = whitelisted_caller();
    }:_(RawOrigin::Signed(caller.clone()), token_type, user_account_id.clone(), withdraw_amount, txn_hash_vec.clone())
   verify {   
        ensure!(<pallet_vtbdex::WalletStorage<T>>::is_user_exist(&user_account_id) == true, "Extrinsic does not executed successfully,User withdrawn event not updated yet");
        assert_event::<T>(Event::WithdrawSuccess {
            signer: Some(caller), 
            token_type, 
            user_address: user_account_id, 
            transaction_id: txn_hash_vec.clone(), 
            amount: withdraw_amount
        }.into());
        ensure!(<TransactionHashList<T>>::get(&token_type).contains(&txn_hash_vec) == true, "Transaction hash not updated for withdraw extrinsic");
    }
}

impl_benchmark_test_suite!(
	CrossChainContract,
	crate::mock::new_test_ext(),
	crate::mock::Test,
);



// ./target/release/vtb-node benchmark pallet --pallet pallet_cross_chain --extrinsic "*" --steps=50 --repeat=20 --execution=wasm --wasm-execution=compiled --heap-pages=4096 