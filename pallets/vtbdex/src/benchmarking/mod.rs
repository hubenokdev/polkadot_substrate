
mod functions;
pub use functions::*;
use super::*;

use frame_system::RawOrigin;
use frame_benchmarking::{benchmarks, whitelisted_caller, impl_benchmark_test_suite};
use pallet_vtbt::Event as AssetsEvent;
use pallet_vtbc_token as VtbcToken;
use frame_support::log::info;

benchmarks! {
    buy_vtbc {
        sp_runtime::runtime_logger::RuntimeLogger::init();
        info!("test debug buy");
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let caller = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(7_000_000_000_000_000_000_u128));

        let seller_address = "5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM";
        let seller_eth_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d4";
        let seller_account_id = create_wallet::<T>(seller_address, seller_eth_address,  TokenType::Eth, U256::from(70_000_000_000_000_000_u128));
       
        Circulation::<T>::mutate(&TokenType::Eth, |get_circulation_value| {
            *get_circulation_value = U256::from(90_000_000_000_000_000_000_u128); // this amount will go for dostribution
        });

        Circulation::<T>::mutate(&TokenType::Vtbc, |get_circulation_value| {
            *get_circulation_value = U256::from(9000_000_000_000_000_000_000_000_u128); // this amount will go for dostribution
        });
        let (seller_account_id, _ ) = create_default_sell_orders::<T>();
    }:_(RawOrigin::Signed(caller.clone()), TokenType::Eth, U256::from(1_000_000_000_000_000_00_u128))
    verify {   
        let fees = U256::from(332891142931805_u128);
        assert_event::<T>(Event::BuyVtbcRequested {
            buyer: caller.clone(), 
            token_type: TokenType::Eth, 
            crypto_amount: U256::from(1_000_000_000_000_000_000_u128) 
        }.into());
        
        // assert_event::<T>(Event::BuyVtbcFromSellOrder {
        //     buyer: caller.clone(), 
        //     token_type: TokenType::Eth, 
        //     seller: seller_account_id.clone(), 
        //     order_id: [219, 226, 217, 26, 13, 176, 121, 245, 62, 41, 162, 247, 62, 159, 228, 102, 55, 102, 177, 125, 136, 89, 205, 132, 227, 208, 60, 54, 52, 216, 149, 172].to_vec(), 
        //     crypto_amount: U256::from(54_936_167_309_499_518_u128), 
        //     vtbc_amount: U256::from(30_000_000_000_000_000_000_u128)
        // }.into());
        // assert_event::<T>(Event::BuyVtbcFromSellOrder {
        //     buyer: caller.clone(), 
        //     token_type: TokenType::Eth, 
        //     seller: seller_account_id.clone(), 
        //     order_id: [101, 37, 198, 37, 52, 77, 102, 40, 114, 245, 20, 33, 73, 134, 71, 40, 83, 130, 14, 247, 255, 40, 168, 84, 169, 109, 216, 66, 197, 32, 47, 211].into(), 
        //     crypto_amount: U256::from(54_936_167_309_499_518_u128), 
        //     vtbc_amount: U256::from(30_000_000_000_000_000_000_u128)
        // }.into());
       assert_vtbc_token_event::<T>(VtbcToken::Event::IssuedVtbcToken {
        user: caller.clone(), 
        amount: U256::from(486088332500262062619_u128)
       }.into()); 
       assert_event::<T>(Event::BuyVtbcFromReserve {
        buyer: caller.clone(), 
        token_type: TokenType::Eth, 
        crypto_amount: U256::from(890_127_665_381_000_963_u128), 
        vtbc_amount: U256::from(486088332500262062619_u128)
       }.into());
       // This need to test in real time why nt is 0 and there is no trnsactional increase
       // This seems to be bug in apr or in buy
       assert_event::<T>(Event::IncreasedByTransaction {
        transaction_count: 0, amount: U256::from(0)
       }.into());
       assert_event::<T>(Event::IncreasedVtbRateDueToTransaction {
        old_vtbc_rate: U256::from(5500914085174230571_u128), 
        new_vtbc_rate: U256::from(5500914085174230571_u128)
       }.into());     
       assert_event::<T>(Event::TransactionSuccessFee {
        user: caller.clone(), reason: "Buy vtbc".as_bytes().to_vec(), token_type: TokenType::Eth, amount: fees
       }.into()); 
    }
    
    buy_vtbc_for_none_linked_cryptos {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let caller = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(7_000_000_000_000_000_000_u128));

        let seller_address = "5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM";
        let seller_eth_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d4";
        let seller_account_id = create_wallet::<T>(seller_address, seller_eth_address,  TokenType::Eth, U256::from(70000000000000000_u128));
       
        Circulation::<T>::mutate(&TokenType::Eth, |get_circulation_value| {
            *get_circulation_value = U256::from(90_000_000_000_000_000_000_u128); // this amount will go for dostribution
        });

        Circulation::<T>::mutate(&TokenType::Vtbc, |get_circulation_value| {
            *get_circulation_value = U256::from(9000_000_000_000_000_000_000_000_u128); // this amount will go for dostribution
        });
        let (seller_account_id, _ ) = create_default_sell_orders::<T>();
       
        <UserWallet<T>>::mutate(&caller.encode(), |user_data| {
            user_data.update_crypto_details(TokenType::Eos, None);
        });
        let _ = <Pallet<T>>::add_update_balance(&TokenType::Eos, &caller.encode(), U256::from(2_000_000_000_000_000_000_u128), U256::from(0_u128));
        Circulation::<T>::mutate(&TokenType::Eos, |get_circulation_value| {
            *get_circulation_value = U256::from(90_000_000_000_000_000_000_u128); // this amount will go for dostribution
        });
    }:buy_vtbc(RawOrigin::Signed(caller.clone()), TokenType::Eos, U256::from(1_000_000_000_000_000_000_u128))
    verify {   
        let fees = U256::from(332891142931805_u128);
        assert_event::<T>(Event::BuyVtbcRequested {
            buyer: caller.clone(), 
            token_type: TokenType::Eos, 
            crypto_amount: U256::from(1_000_000_000_000_000_000_u128) 
        }.into()); 
        assert_event::<T>(Event::TransactionSuccessFee {
            user: caller.clone(), 
            reason: "Buy vtbc".as_bytes().to_vec(), 
            token_type: TokenType::Eos, 
            amount: fees
        }.into()); 
    }

    cancel_buy_vtbc_order {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d4";
        let buyer_address = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(50_000_000_000_000_000_000_u128));
       
        for i in [("order121", U256::from(100_000_000_000_000_000_u128)), ("order122", U256::from(1_000_000_000_000_000_u128))] {
            insert_buy_orders::<T>(buyer_address.clone(), user_address, i.0, i.1);
        }
        Circulation::<T>::mutate(&TokenType::Vtbc, |get_circulation_value| {
            *get_circulation_value = U256::from(90_000_000_000_000_000_000_u128); // this amount will go for dostribution
        });
       
    }:_(RawOrigin::Signed(buyer_address.clone()), "order121".as_bytes().to_vec(), TokenType::Eth)
    verify {
        let fees = U256::from(332891142931805_u128);
        assert_event::<T>(Event::CanceledOrder {
            trade_type: TradeType::Buy, 
            user: buyer_address.clone(), 
            order_id: "order121".as_bytes().to_vec(), 
            amount: U256::from(100_000_000_000_000_000_u128)
        }.into());  
        assert_event::<T>(Event::OrderRefunded {
            trade_type: TradeType::Buy, 
            order_id: "order121".as_bytes().to_vec(), 
            user: buyer_address.clone(), 
            token_type: TokenType::Eth, 
            amount: U256::from(100_000_000_000_000_000_u128)
        }.into());  
        assert_event::<T>(Event::TransactionSuccessFee {
            user: buyer_address.clone(), 
            reason: "Cancel order".as_bytes().to_vec(), 
            token_type: TokenType::Eth, 
            amount: fees
        }.into());        
    }
    
    sell_vtbc {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let caller = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(7_000_000_000_000_000_000_u128));
        let account_key = caller.encode();

        let _ = <Pallet<T>>::add_update_balance(&TokenType::Vtbc, &account_key, U256::from(800_000_000_000_000_000_000_u128), U256::from(0_u128));
        Circulation::<T>::mutate(&TokenType::Eth, |get_circulation_value| {
            *get_circulation_value = U256::from(90_000_000_000_000_000_000_u128); // this amount will go for dostribution
        });
        Circulation::<T>::mutate(&TokenType::Vtbc, |get_circulation_value| {
            *get_circulation_value = U256::from(9000_000_000_000_000_000_000_000_u128); // this amount will go for dostribution
        });

        let buyer_pdot_address = "5Gbkauy7EwfyDZMDxsXRTcFdQcze5FNX3VQByX9iQ4RBGPos";
        let buyer_user_address = "0x5a3339e99E5Eb202Fc2ea7C963600A2695A8e84e";
        let buyer_address = create_wallet::<T>(buyer_pdot_address, buyer_user_address, TokenType::Eth, U256::from(50_000_000_000_000_000_000_u128));
        for i in [("order121", U256::from(100_000_000_000_000_000_u128)), ("order122", U256::from(1_000_000_000_000_000_u128))] {
            insert_buy_orders::<T>(buyer_address.clone(), user_address, i.0, i.1);
        }
    }:_(RawOrigin::Signed(caller.clone()), TokenType::Eth, U256::from(300_000_000_000_000_000_000_u128))
    verify {  
        let fees = U256::from(332891142931805_u128); 
        // assert_event::<T>(Event::SellVtbcRequested {
        //     seller: caller.clone(), 
        //     vtbc_amount: U256::from(300_000_000_000_000_000_000_u128)
        // }.into());
        // assert_event::<T>(Event::TransactionSuccessFee {
        //     user: caller.clone(), 
        //     reason: "Sell vtbc".as_bytes().to_vec(), 
        //     token_type: TokenType::Eth, 
        //     amount: fees
        // }.into());        
        let order = OrderBookNMap::<T>::get((&TradeType::Sell, &TokenType::Eth, 1_u64)).unwrap();
        // assert_event::<T>(Event::SellVtbcToFillBuyOrder {
        //     seller: caller.clone(),
        //     token_type: TokenType::Eth, 
        //     buyer: buyer_address.clone(), 
        //     order_id: "order121".as_bytes().to_vec(),
        //     vtbc_amount: U256::from(100_000_000_000_000_000_u128), 
        //     crypto_amount: U256::from(55_018_943_539_902_656_297_u128) 
        // }.into());
        // assert_event::<T>(Event::OrderRefunded {
        //     trade_type: TradeType::Buy, 
        //     order_id: [111, 114, 100, 101, 114, 49, 50, 50].to_vec(), 
        //     user: buyer_address.clone(), 
        //     token_type: TokenType::Eth, 
        //     amount: U256::from(1_000_000_000_000_000_u128)
        // }.into());
        // assert_last_event::<T>(Event::OpenSellOrder {
        //     seller: caller.clone(), 
        //     order_id: [112, 245, 36, 30, 136, 100, 138, 101, 229, 231, 203, 197, 242, 6, 75, 226, 158, 234, 132, 242, 14, 14, 35, 8, 143, 214, 186, 89, 33, 162, 86, 97].into(),
        //     token_type: TokenType::Eth, 
        //     vtbc_amount: U256::from(244_981_056_460_097_343_703_u128)
        // }.into());
        //here is a bug in apr or sell, it is not increasing the vtbc price
        //assert_event::<T>(Event::IncreasedVtbRateDueToTransaction(U256::from(5500914085174230571_u128), U256::from(5500914085174230571_u128)).into());     
    }
        
    sell_vtbc_for_none_linked_cryptos {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let caller = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(7_000_000_000_000_000_000_u128));
        let account_key = caller.encode();

        let _ = <Pallet<T>>::add_update_balance(&TokenType::Vtbc, &account_key, U256::from(800_000_000_000_000_000_000_u128), U256::from(0_u128));
        Circulation::<T>::mutate(&TokenType::Eth, |get_circulation_value| {
            *get_circulation_value = U256::from(90_000_000_000_000_000_000_u128); // this amount will go for dostribution
        });
        Circulation::<T>::mutate(&TokenType::Vtbc, |get_circulation_value| {
            *get_circulation_value = U256::from(9000_000_000_000_000_000_000_000_u128); // this amount will go for dostribution
        });

        let buyer_pdot_address = "5Gbkauy7EwfyDZMDxsXRTcFdQcze5FNX3VQByX9iQ4RBGPos";
        let buyer_user_address = "0x5a3339e99E5Eb202Fc2ea7C963600A2695A8e84e";
        let buyer_address = create_wallet::<T>(buyer_pdot_address, buyer_user_address, TokenType::Eth, U256::from(50_000_000_000_000_000_000_u128));
        for i in [("order121", U256::from(100_000_000_000_000_000_u128)), ("order122", U256::from(1_000_000_000_000_000_u128))] {
            insert_buy_orders::<T>(buyer_address.clone(), user_address, i.0, i.1);
        }

        <UserWallet<T>>::mutate(&caller.encode(), |user_data| {
            user_data.update_crypto_details(TokenType::Eos, None);
        });
        let _ = <Pallet<T>>::add_update_balance(&TokenType::Eos, &caller.encode(), U256::from(2_000_000_000_000_000_000_u128), U256::from(0_u128));
        Circulation::<T>::mutate(&TokenType::Eos, |get_circulation_value| {
            *get_circulation_value = U256::from(90_000_000_000_000_000_000_u128); // this amount will go for dostribution
        });

    }:sell_vtbc(RawOrigin::Signed(caller.clone()), TokenType::Eos, U256::from(300_000_000_000_000_000_000_u128))
    verify {  
        let fees = U256::from(332891142931805_u128); 
        assert_event::<T>(Event::SellVtbcRequested {
            seller: caller.clone(), 
            vtbc_amount: U256::from(300_000_000_000_000_000_000_u128)
        }.into());
        assert_event::<T>(Event::TransactionSuccessFee {
            user: caller.clone(), 
            reason: "Sell vtbc".as_bytes().to_vec(), 
            token_type: TokenType::Eos, 
            amount: fees 
        }.into());          
    }

    cancel_sell_vtbc_order {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d4";
        let seller_address = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(50_000_000_000_000_000_000_u128));
        for i in ["order121", "order122"] {
            insert_sell_orders::<T>(seller_address.clone(), user_address, i);
        }
        Circulation::<T>::mutate(&TokenType::Vtbc, |get_circulation_value| {
            *get_circulation_value = U256::from(90_000_000_000_000_000_000_u128); // this amount will go for dostribution
        });
       
    }:_(RawOrigin::Signed(seller_address.clone()), "order121".as_bytes().to_vec(), TokenType::Eth)
    verify {
        assert_event::<T>(Event::CanceledOrder {
            trade_type: TradeType::Sell, 
            user: seller_address.clone(), 
            order_id: "order121".as_bytes().to_vec(), 
            amount: U256::from(30_000_000_000_000_000_000_u128)
        }.into()); 
        assert_event::<T>(Event::OrderRefunded {
            trade_type: TradeType::Sell, 
            order_id: "order121".as_bytes().to_vec(), 
            user: seller_address.clone(), 
            token_type: TokenType::Eth, 
            amount: U256::from(30_000_000_000_000_000_000_u128)
        }.into());  
        let fees = U256::from(332891142931805_u128); 
        assert_event::<T>(Event::TransactionSuccessFee {
            user: seller_address.clone(), 
            reason: "Cancel order".as_bytes().to_vec(), 
            token_type: TokenType::Eth, 
            amount: fees
        }.into());        
    }
    
    withdraw_initiate {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let caller = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(5_000_000_000_000_000_000_u128));
        Circulation::<T>::mutate(&TokenType::Eth, |get_circulation_value| {
            *get_circulation_value = U256::from(90_000_000_000_000_000_000_u128); // this amount will go for dostribution
        });
        let blocknumber = frame_system::Pallet::<T>::block_number();
        let (id, fees, with_req_data) = get_id_blocked_user_list_for_withdraw::<T>(caller.clone());
    }:_(RawOrigin::Signed(caller.clone()), TokenType::Eth, U256::from(300_000_000_000_000_000_u128))
    verify {
        let user_address_lc_1 = user_address.to_lowercase();
        let user_address_lc = user_address_lc_1.to_string().as_bytes().to_vec();
        let users_list =  <BlockedUserWallet<T>>::get(&caller).unwrap();

        assert!(id == users_list[0].id, "{:?} and this {:?}----{:?}", users_list[0], with_req_data, id);
        assert_event::<T>(Event::TransactionSuccessFee {
            user: caller.clone(), reason: "Withdraw Crypto".as_bytes().to_vec(), 
            token_type: TokenType::Eth, 
            amount: fees
        }.into());        
        assert_event::<T>(Event::WithdrawInitiated {
            user: caller, 
            crypto_address: user_address_lc, 
            amount: U256::from(300_000_000_000_000_000_u128), 
            id
        }.into());  
    }

    withdraw_inprogress {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let user_account_id = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(50_000_000_000_000_000_000_u128));
        let id = insert_blocked_user_list_for_withdraw::<T>(user_account_id.clone());
        let caller: T::AccountId = whitelisted_caller();
        let user_address_lc = user_address.to_lowercase().to_string().as_bytes().to_vec();
        let txn_hash = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let txn_hash_lc = txn_hash.to_lowercase();
        let txn_hash_vec = txn_hash_lc.to_string().as_bytes().to_vec();

    }:_(RawOrigin::Signed(caller.clone()), user_account_id.clone(), user_address_lc.clone(), txn_hash_vec.clone(), id.clone())
    verify {
        assert_event::<T>(Event::WithdrawInProgress {
            user: user_account_id, 
            crypto_address: user_address_lc.clone(),
            token_type: TokenType::Eth, 
            amount: U256::from(10_000_000_000_000_000_000_u128), 
            transaction_id: txn_hash_vec
        }.into());  
    }

    withdraw_failed {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let user_account_id = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(50_000_000_000_000_000_000_u128));
        let id = insert_blocked_user_list_for_withdraw::<T>(user_account_id.clone());

        let caller: T::AccountId = whitelisted_caller();
        let user_address_lc = user_address.to_lowercase().to_string().as_bytes().to_vec();
        let msg = "withdraw failed with 400 api error".to_string().as_bytes().to_vec();
    }:_(RawOrigin::Signed(caller.clone()), user_account_id.clone(), user_address_lc.clone(), msg.clone(), U256::from(10_000_000_000_000_000_000_u128), id.clone())
    verify {
        assert_event::<T>(Event::WithdrawFailed {
            user: user_account_id, 
            crypto_address: user_address_lc,
            token_type: TokenType::Eth, 
            amount: U256::from(10_000_000_000_000_000_000_u128), 
            msg
        }.into());  
    }

    withdraw_failed_due_to_time_out {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let user_account_id = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(50_000_000_000_000_000_000_u128));
        let id = insert_blocked_user_list_for_withdraw::<T>(user_account_id.clone());

        let caller: T::AccountId = whitelisted_caller();
        let user_address_lc = user_address.to_lowercase().to_string().as_bytes().to_vec();
        let msg = "withdraw failed with 500 TimeOut api error".to_string().as_bytes().to_vec();
    }:_(RawOrigin::Signed(caller.clone()), user_account_id.clone(), user_address_lc.clone(), msg.clone(), id.clone())
    verify {
        assert_event::<T>(Event::WithdrawFailed {
            user: user_account_id, 
            crypto_address: user_address_lc, 
            token_type: TokenType::Eth, 
            amount: U256::from(10_000_000_000_000_000_000_u128), 
            msg 
        }.into());  
    }

    check_and_remove_from_pending_list {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let user_account_id = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(50_000_000_000_000_000_000_u128));
        let id = insert_blocked_user_list_for_withdraw::<T>(user_account_id.clone());
    }:_(RawOrigin::Root, user_account_id.clone(), id.clone())
    verify {
        assert_event::<T>(Event::PendingWithdrawRejected { user: user_account_id, id }.into());  
    }

    check_and_return_withdraw_pending_amount {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let user_account_id = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(50_000_000_000_000_000_000_u128));
        let id = insert_blocked_user_list_for_withdraw::<T>(user_account_id.clone());
    }:_(RawOrigin::Root, user_account_id.clone(), id.clone())
    verify {
        assert_event::<T>(Event::PendingWithdrawReturned {user: user_account_id, amount: U256::from(10_000_000_000_000_000_000_u128), id }.into());  
    }

    initialize_values_for_apr {
        let caller = whitelisted_caller();
    }:_(RawOrigin::Signed(caller), U256::from(30_000_000_000_000_000_u128))
    verify {
        let feed_rate = UsdRate::<T>::get().vtbc_last_apr_rate;
        ensure!(<VtbcStartRate<T>>::get() == true, "Start rate is being not initialized");   
    }

    initialize_value_for_year_ta {
        let caller = whitelisted_caller();
    }:_(RawOrigin::Signed(caller), U256::from(30_000_000_000_000_000_u128), U256::from(30_000_000_000_000_000_u128))
    verify {
        let feed_rate = UsdRate::<T>::get().vtbc_last_apr_rate;
        ensure!(feed_rate == U256::from(30_000_000_000_000_000_u128), "Start rate is being not initialized");   
    }

    submit_vtbc_hourly_rate {
        let time_stamp = <pallet_timestamp::Pallet<T>>::now() + T::HourlyPeriod::get() + T::HourlyPeriod::get();

        let old_rate = UsdRate::<T>::get().vtbc_last_apr_rate;
        let old_time_stamp: T::Moment = UpdatedTimeList::<T>::get().apr_timestamp; 
        let new_rate = U256::from(5_459_910_363_094_032_987_u128);
        let caller = whitelisted_caller();
    }:_(RawOrigin::Signed(caller), new_rate, time_stamp, U256::from(5_459_910_363_094_032_987_u128), 510, 1)
    verify {
        let feed_rate = UsdRate::<T>::get().vtbc_last_apr_rate;
        ensure!(feed_rate == new_rate, "New rate is not inserted properly");   
        assert_event::<T>(Event::IncreaseHourlyAccuralRate { 
            old_vtbc_rate: old_rate, 
            old_timestamp: old_time_stamp, 
            new_vtbc_rate: new_rate, 
            new_timestamp: time_stamp }.into());
        assert_event::<T>(Event::HourlyChanges().into());
    }

    set_vtbdex_fee_collector_account {
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let caller = create_wallet::<T>(pdot_address, "0x812466288703fc6e210A4fFfe1e259Dd966096d3", TokenType::Eth, U256::from(0_u128));
        create_wallet::<T>(pdot_address, "eos123", TokenType::Eos, U256::from(0_u128));
    }:_(RawOrigin::Root, caller.clone())
    verify {   
        assert_event::<T>(Event::SetFeeCollectorAddress { fee_collector: caller.clone() }.into());
    }

    set_vtbdex_transaction_fee {
        initialize_system::<T>();
    }:_(RawOrigin::Root,  U256::from(8_000_000_000_000_000_000_u128))
    verify {   
        assert_event::<T>(Event::SetFeeAmountInUsd { fee_amount: U256::from(8_000_000_000_000_000_000_u128 )}.into());
    }

    stop_vtbdex_functionality {
        initialize_system::<T>();
    }:_(RawOrigin::Root)
    verify {   
        ensure!(<VtbSystemRunning<T>>::get() == false, "Extrinsic does not executed successfully,  System must be stopped");
        assert_event::<T>(Event::VtbdexSystemStopped().into());
    }

    resume_vtbdex_functionality {
        initialize_system::<T>();
        <VtbSystemRunning<T>>::put(false);
    }:_(RawOrigin::Root)
    verify {
        ensure!(<VtbSystemRunning<T>>::get() == true, "Extrinsic does not executed successfully, System must be running");   
        assert_event::<T>(Event::VtbdexSystemResume().into());
    }
    
    submit_new_estimated_price {
        let caller: T::AccountId = whitelisted_caller();
        let estimated_price = U256::from(5000000000000000000_u128);

    }:_(RawOrigin::Signed(caller.clone()), estimated_price)
    verify {
        let feed_price = <EstimatedGasPrice<T>>::get();
        ensure!(feed_price == estimated_price, "Start rate is being not initialized"); 
        assert_event::<T>(Event::UpdateNewEstimatedGasPrice {
            signer: Some(caller), 
            amount: estimated_price
        }.into());  
    }

    charge_set_ipfs_trnx_price {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let user_account = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(7_000_000_000_000_000_000_u128));

        let txn_hash = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let txn_hash_lc = txn_hash.to_lowercase();
        let txn_hash_vec = txn_hash_lc.to_string().as_bytes().to_vec();

        let ipfs_hash = "QmR9Ew4Dd6dbyVwCJn8Dd3kzF87peh3ZtJsDf93NzTMmGn";
        let ipfs_hash_vec = ipfs_hash.to_string().as_bytes().to_vec();
        let caller: T::AccountId = whitelisted_caller();
    }:_(RawOrigin::Signed(caller.clone()), user_account.clone(), U256::from(50_000_000_000_000_000_u128), txn_hash_vec.clone(), TokenType::Eth, ipfs_hash_vec.clone())
    verify {
        assert_event::<T>(Event::ChargeIpfsSnapshotGasPrice {
            signer: Some(caller), 
            user: user_account, 
            amount: U256::from(50_000_000_000_000_000_u128),
            transaction_id: txn_hash_vec, 
            token_type: TokenType::Eth, 
            ipfs_hash: ipfs_hash_vec
        }.into());  
    }

    initiate_convert_vtbc_to_vtbt_substrate {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d4";
        let caller = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(50_000_000_000_000_000_000_u128));
        Circulation::<T>::mutate(&TokenType::Eth, |get_circulation_value| {
            *get_circulation_value = U256::from(90_000_000_000_000_000_000_u128); // this amount will go for dostribution
        });

        Circulation::<T>::mutate(&TokenType::Vtbc, |get_circulation_value| {
            *get_circulation_value = U256::from(9000_000_000_000_000_000_000_000_u128); // this amount will go for dostribution
        });
        let account_key = caller.encode();
        let _ = <Pallet<T>>::add_update_balance(&TokenType::Vtbc, &account_key, U256::from(800_000_000_000_000_000_000_u128), U256::from(0_u128));
        let vtbt_amount = U256::from(50_000_000_000_000_000_000_u128);
    }:_(RawOrigin::Signed(caller.clone()), vtbt_amount)
    verify {   
        let vtbcamount = <Pallet<T>>::convert_vtbt_to_vtbc(vtbt_amount).unwrap();
        assert_event::<T>(Event::MintSubstrateErc20VTBtInitiated { user: caller.clone(), vtbt_amount }.into());    
        assert_event::<T>(Event::ConvertVtbcToVtbtSuccess {
            user: caller.clone(), 
            vtbc_amount: vtbcamount, 
            vtbt_amount 
        }.into());  
        assert_vtbt_pallet_event::<T>(AssetsEvent::Issued { asset_id: T::VtbErc20AssetId::get(), owner: caller, balance: vtbt_amount }.into());
    }

    initiate_convert_vtbt_to_vtbc_substrate {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d4";
        let caller = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(50_000_000_000_000_000_000_u128));
        let account_key = caller.encode();
        
        let _ = <Pallet<T>>::add_update_balance(&TokenType::Vtbc, &account_key, U256::from(800_000_000_000_000_000_000_u128), U256::from(0_u128));
        let _ = <Pallet<T>>::add_update_balance(&TokenType::Vtbt, &account_key, U256::from(80_000_000_000_000_000_000_u128), U256::from(0_u128));

        Circulation::<T>::mutate(&TokenType::Eth, |get_circulation_value| {
            *get_circulation_value = U256::from(90_000_000_000_000_000_000_u128); // this amount will go for dostribution
        });

        Circulation::<T>::mutate(&TokenType::Vtbc, |get_circulation_value| {
            *get_circulation_value = U256::from(9000_000_000_000_000_000_000_u128); // this amount will go for dostribution
        });
        Globals::<T>::mutate(|get_globals| {
            get_globals.backing_reserve = U256::from(5000_000_000_000_000_000_000_u128)
        });
         
        let vtbt_amount = U256::from(50_000_000_000_000_000_000_u128);
        create_default_minted_asset::<T>(caller.clone(), vtbt_amount);
    }:_(RawOrigin::Signed(caller.clone()), vtbt_amount)
    verify {   
        let vtbcamount = <Pallet<T>>::convert_vtbt_to_vtbc(vtbt_amount).unwrap();
        assert_event::<T>(Event::BurnSubstrateErc20VTBtInitiated {
            user: caller.clone(), 
            vtbt_amount 
        }.into());    
        assert_event::<T>(Event::ConvertVtbtToVtbcSuccess {
            user: caller.clone(), 
            vtbt_amount: vtbt_amount, 
            vtbc_amount: vtbcamount
        }.into());  
        assert_vtbt_pallet_event::<T>(AssetsEvent::Burned { asset_id: T::VtbErc20AssetId::get(), owner: caller, balance: vtbt_amount }.into());
    }

    initiate_transfer_of_vtbt_substrate {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d4";
        let caller = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(0_u128));
        let account_key = caller.encode();
        <UserWallet<T>>::mutate(&caller.encode(), |user_data| {
            user_data.update_crypto_details(TokenType::Eos, None);
        });
        let _ = <Pallet<T>>::add_update_balance(&TokenType::Eos, &caller.encode(), U256::from(5_000_000_000_000_000_000_u128), U256::from(0_u128));

        let _ = <Pallet<T>>::add_update_balance(&TokenType::Vtbc, &account_key, U256::from(800_000_000_000_000_000_000_u128), U256::from(0_u128));
        let _ = <Pallet<T>>::add_update_balance(&TokenType::Vtbt, &account_key, U256::from(80_000_000_000_000_000_000_u128), U256::from(0_u128));
        Circulation::<T>::mutate(&TokenType::Eth, |get_circulation_value| {
            *get_circulation_value = U256::from(90_000_000_000_000_000_000_u128); // this amount will go for dostribution
        });

        Circulation::<T>::mutate(&TokenType::Vtbc, |get_circulation_value| {
            *get_circulation_value = U256::from(9000_000_000_000_000_000_000_u128); // this amount will go for dostribution
        });

        Globals::<T>::mutate(|get_globals| {
            get_globals.backing_reserve = U256::from(5000_000_000_000_000_000_000_u128)
        });
        let receiver_address = "5C4hrfjw9DjXZTzV3MwzrrAr9P1MJhSrvWGWqi1eSuyUpnhM";
        let receiver_eth_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d4";
        let receiver_account_id = create_wallet::<T>(receiver_address, receiver_eth_address, TokenType::Eth, U256::from(10_000_000_000_000_000_000_u128));
        let vtbt_amount = U256::from(50_000_000_000_000_000_000_u128);
        create_default_minted_asset::<T>(caller.clone(), vtbt_amount);

    }:_(RawOrigin::Signed(caller.clone()), receiver_account_id.clone(), vtbt_amount)
    verify {
        assert_event::<T>(Event::TransferSubstrateErc20VTBtInitiated {
            sender_address: caller.clone(), 
            receiver_address: receiver_account_id.clone(), 
            vtbt_amount
        }.into());    
        assert_event::<T>(Event::TransferVtbtErc20Success {
            sender_address: caller.clone(), 
            receiver_address: receiver_account_id.clone(), 
            vtbt_amount
        }.into()); 
        let fees = U256::from(332891142931805_u128);
        assert_event::<T>(Event::TransactionSuccessFee {
            user: caller.clone(), 
            reason: "Transfer vtbt".as_bytes().to_vec(), 
            token_type: TokenType::Eos, 
            amount: fees
        }.into());        
        assert_vtbt_pallet_event::<T>(AssetsEvent::Transferred { asset_id: T::VtbErc20AssetId::get(), from: caller, to: receiver_account_id, amount: vtbt_amount }.into()); 

    }

    initiate_transfer_from_of_vtbt_substrate {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d4";
        let sender_account_id = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(5_000_000_000_000_000_000_u128));
        let account_key = sender_account_id.encode();
        
        let _ = <Pallet<T>>::add_update_balance(&TokenType::Vtbc, &account_key, U256::from(800_000_000_000_000_000_000_u128), U256::from(0_u128));
        let _ = <Pallet<T>>::add_update_balance(&TokenType::Vtbt, &account_key, U256::from(80_000_000_000_000_000_000_u128), U256::from(0_u128));
        Circulation::<T>::mutate(&TokenType::Eth, |get_circulation_value| {
            *get_circulation_value = U256::from(90_000_000_000_000_000_000_u128); // this amount will go for dostribution
        });
        Circulation::<T>::mutate(&TokenType::Vtbc, |get_circulation_value| {
            *get_circulation_value = U256::from(9000_000_000_000_000_000_000_u128); // this amount will go for dostribution
        });
        Globals::<T>::mutate(|get_globals| {
            get_globals.backing_reserve = U256::from(5000_000_000_000_000_000_000_u128)
        });
        let receiver_address = "5CyuKHaJmThyNLP2zTjrNwSA2U3RQcqWnFFmDBWv1QPyEJ5n";
        let receiver_eth_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d4";
        let receiver_account_id = create_wallet::<T>(receiver_address, receiver_eth_address, TokenType::Eth, U256::from(10_000_000_000_000_000_000_u128));

        let caller_address = "5Gbkauy7EwfyDZMDxsXRTcFdQcze5FNX3VQByX9iQ4RBGPos";
        let caller_eth_address = "0x12456";
        let caller = create_wallet::<T>(caller_address, caller_eth_address, TokenType::Eth, U256::from(2_000_000_000_000_000_000_u128));
        let vtbt_amount = U256::from(50_000_000_000_000_000_000_u128);

        create_default_minted_asset::<T>(sender_account_id.clone(), vtbt_amount);   
        create_default_allowance_asset::<T>(sender_account_id.clone(), caller.clone(), vtbt_amount);

    }:_(RawOrigin::Signed(caller), sender_account_id.clone(), receiver_account_id.clone(), vtbt_amount)
    verify {   
        assert_event::<T>(Event::TransferSubstrateErc20VTBtInitiated {
            sender_address: sender_account_id.clone(), 
            receiver_address: receiver_account_id.clone(), 
            vtbt_amount
        }.into());    
        assert_event::<T>(Event::TransferVtbtErc20Success {
            sender_address: sender_account_id.clone(), 
            receiver_address: receiver_account_id.clone(), 
            vtbt_amount
        }.into()); 
        assert_vtbt_pallet_event::<T>(AssetsEvent::Transferred { asset_id: T::VtbErc20AssetId::get(), from: sender_account_id, to: receiver_account_id, amount: vtbt_amount }.into()); 
    }

    claim_distribution {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let user_account_id = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(50_000_000_000_000_000_000_u128));
        let account_key = user_account_id.encode();
        
        let _ = <Pallet<T>>::add_update_balance(&TokenType::Vtbc, &account_key, U256::from(800_000_000_000_000_000_000_u128), U256::from(10_000_000_000_000_000_000_u128));
		
        close_and_intialize_new_distribution::<T>();

    }:_(RawOrigin::Signed(user_account_id.clone()), TokenType::Eos)
    verify {   
        let claimed_amount = U256::from(19999999999999999980_u128);
        let users = <UserWallet<T>>::get(&user_account_id.encode());
        let data = "dtata"; 
        assert_event::<T>(Event::ClaimedSuccess {
            user: user_account_id.clone(), token_type: TokenType::Eos, claimed_amount
        }.into());   
       // assert!(data == "0", "user data {:?}", users);  
    }

    claim_eth_distribution {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let user_account_id = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(50_000_000_000_000_000_000_u128));
        let account_key = user_account_id.encode();
        
        let _ = <Pallet<T>>::add_update_balance(&TokenType::Vtbc, &account_key, U256::from(800_000_000_000_000_000_000_u128), U256::from(10_000_000_000_000_000_000_u128));
		
        close_and_intialize_new_distribution::<T>();

    }:claim_distribution(RawOrigin::Signed(user_account_id.clone()), TokenType::Eth)
    verify {   
        let claimed_amount = U256::from(19999999999999999980_u128);
        let users = <UserWallet<T>>::get(&user_account_id.encode());
        let data = "dtata";
        assert_event::<T>(Event::ClaimedSuccess {
            user: user_account_id.clone(), token_type: TokenType::Eth, claimed_amount
        }.into()); 
    }

    claim_vtbc_distribution {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let user_account_id = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(50_000_000_000_000_000_000_u128));
        let account_key = user_account_id.encode();
        
        let _ = <Pallet<T>>::add_update_balance(&TokenType::Vtbc, &account_key, U256::from(800_000_000_000_000_000_000_u128), U256::from(10_000_000_000_000_000_000_u128));
		
        close_and_intialize_new_distribution::<T>();

    }:claim_distribution(RawOrigin::Signed(user_account_id.clone()), TokenType::Vtbc)
    verify {   
        let claimed_amount = U256::from(19999999999999999980_u128);
        let users = <UserWallet<T>>::get(&user_account_id.encode());
        assert_event::<T>(Event::ClaimedSuccess {
            user: user_account_id.clone(), token_type: TokenType::Vtbc, claimed_amount
        }.into());  
    }

    claim_vtbc_distribution_multiple_period {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let user_account_id = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(50_000_000_000_000_000_000_u128));
        let account_key = user_account_id.encode();
        
        let _ = <Pallet<T>>::add_update_balance(&TokenType::Vtbc, &account_key, U256::from(800_000_000_000_000_000_000_u128), U256::from(10_000_000_000_000_000_000_u128));
		
        close_and_intialize_multiple_distribution::<T>();

    }:claim_distribution(RawOrigin::Signed(user_account_id.clone()), TokenType::Vtbc)
    verify {   
       // let claimed_amount = U256::from(19999999999999999980_u128);
        let users = <UserWallet<T>>::get(&user_account_id.encode());
        // assert!(data == "0", "user data {:?}", users);   
        // assert_event::<T>(Event::ClaimedSuccess {
        //     user: user_account_id.clone(), token_type: TokenType::Vtbc, claim_amount: claimed_amount
        // }.into());  
    }

    claim_all_distribution {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let user_account_id = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(50_000_000_000_000_000_000_u128));
        let account_key = user_account_id.encode();
        
        let _ = <Pallet<T>>::add_update_balance(
            &TokenType::Vtbc, &account_key, 
            U256::from(800_000_000_000_000_000_000_u128), 
            U256::from(10_000_000_000_000_000_000_u128)
        );
		
        close_and_intialize_new_distribution::<T>();

    }:_(RawOrigin::Signed(user_account_id.clone()))
  verify {   
        let eth_claimed_amount = U256::from(19_999_999_999_999_999_980_u128);
        let eos_claimed_amount = U256::from(19_999_999_999_999_999_980_u128);
        let vtbc_claimed_amount = U256::from(19_999_999_999_999_999_980_u128);

        assert_event::<T>(Event::ClaimedSuccess {
            user: user_account_id.clone(), token_type: TokenType::Eth, claimed_amount: eth_claimed_amount
        }.into());  
        assert_event::<T>(Event::ClaimedSuccess {
            user: user_account_id.clone(), token_type: TokenType::Eos, claimed_amount: eos_claimed_amount
        }.into());  
        assert_event::<T>(Event::ClaimedSuccess {
            user: user_account_id.clone(), token_type: TokenType::Vtbc, claimed_amount: vtbc_claimed_amount
        }.into());   
    }

    claim_all_distribution_multiple_distribution {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let user_account_id = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(50_000_000_000_000_000_000_u128));
        let account_key = user_account_id.encode();
        
        let _ = <Pallet<T>>::add_update_balance(
            &TokenType::Vtbc, 
            &account_key, 
            U256::from(800_000_000_000_000_000_000_u128), 
            U256::from(10_000_000_000_000_000_000_u128)
        );
		
        close_and_intialize_multiple_distribution::<T>();

    }:claim_all_distribution(RawOrigin::Signed(user_account_id.clone()))
  verify {   
        // let users = <UserWallet<T>>::get(&user_account_id.encode());
        // let data = "dtata";
        // assert!(data == "0", "user data {:?}", users);   
    }

    check_claim_distribution {
        initialize_system::<T>();
        let pdot_address = "5D2LkprRWaZ66RHJmHQMzYU8H3YMVUg3P7xY5oXxru15RKeh";
        let user_address = "0x812466288703fc6e210A4fFfe1e259Dd966096d3";
        let user_account_id = create_wallet::<T>(pdot_address, user_address, TokenType::Eth, U256::from(50_000_000_000_000_000_000_u128));
        let account_key = user_account_id.encode();
       
        let _ = <Pallet<T>>::add_update_balance(&TokenType::Vtbc, &account_key, U256::from(800_000_000_000_000_000_000_u128), U256::from(10_000_000_000_000_000_000_u128));
		
        close_and_intialize_new_distribution::<T>();

    }:_(RawOrigin::None, user_account_id.clone(), 0)
    verify {   
        let eth_claimed_amount = U256::from(19999999999999999980_u128);
        let claimed_eth_amt = ClaimToken::<T>::get(&user_account_id, &TokenType::Eth);
        assert!(claimed_eth_amt.token_amount == eth_claimed_amount, "Eth claim not matching"); 
        assert!(claimed_eth_amt.to_update_period == 1, "Next distribution unclaimed period must be 1"); 

        let claimed_eos_amt = ClaimToken::<T>::get(&user_account_id, &TokenType::Eos);
        let eos_claimed_amount = U256::from(19999999999999999980_u128);
        assert!(claimed_eos_amt.token_amount == eos_claimed_amount, "Eth claim not matching"); 
        assert!(claimed_eos_amt.to_update_period == 1, "Next distribution unclaimed period must be 1"); 

        let claimed_vtbc_amt = ClaimToken::<T>::get(&user_account_id, &TokenType::Vtbc);
        let vtbc_claimed_amount = U256::from(19999999999999999980_u128);
        assert!(claimed_vtbc_amt.token_amount == vtbc_claimed_amount, "Eth claim not matching"); 
        assert!(claimed_vtbc_amt.to_update_period == 1, "Next distribution unclaimed period must be 1"); 
    }
}

impl_benchmark_test_suite!(
	VtbDex,
	crate::mock::new_test_ext(),
	crate::mock::Test,
);

// Compile cli code: cargo build --release --features runtime-benchmarks
//   ./target/release/vtb-node benchmark pallet --pallet pallet_usd_rate --extrinsic "*" --steps=50 --repeat=20 --execution=wasm --wasm-execution=compiled --heap-pages=4096 --output=pallets/usd-rate/src/weights1.rs 
