//! Module trade
//! It contains trade features of vtb system.
//! Such as BuyVTbc, SellVtbc, CancelSell order, CancelBuy order.
//! 
use serde::{Deserialize, Serialize};
use sp_std::{str, fmt, slice::Iter};
use serde::__private::ToString;
use scale_info::TypeInfo;
use crate::{
    Vec, U256, TokenType, 
    Encode, Decode,
};

/// TradeType is custom enum defined to support different trade features
/// Default value of this enum is SELL
#[derive( Encode, Decode, Clone, Copy, Debug, PartialEq, Eq, Ord, PartialOrd, Deserialize, Serialize, TypeInfo)]
pub enum TradeType {
    Buy,
    Sell
}

impl TradeType {
    pub fn _iterator() -> Iter<'static, TradeType> {
        static _TRADETYPES: [TradeType; 2] = [TradeType::Buy, TradeType::Sell];
        _TRADETYPES.iter()
    }
}

impl fmt::Display for TradeType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Default for TradeType {
    fn default() -> Self { TradeType::Sell }
}

/// Indexed is custom struct defined to support for LIFO principle for OrderBook
/// It has two field
/// Field start_index of U256 type, this will increse by +1 when ever new BUY/SELL order be fulfilled.
/// Field end_index of U256 type, this will increse by +1 when ever new BUY/SELL order will be inserted.
#[derive( Encode, Decode, Clone, Debug, PartialEq, Eq, Deserialize, Serialize, Default, TypeInfo)]
pub struct Indexed {
    pub start_index: U256,
    pub end_index: U256, 
}

impl fmt::Display for Indexed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "start_index: {:?}, end_index: {:?}", self.start_index, self.end_index)
    }
}

/// OrderBookStruct is custom struct defined for maintain OrderBook detail for BUY/SELL Orders.
///  This struct will be the value type of OrderBookNMap runtime storage defined in lib.rs.
/// It has five field
/// Field1: order_id -> This is generated using keccak256 algo and encoded to bytes.
/// Field2: address -> This is Seller/Buyer Ss58 AccountId.
/// Field3: crypto_address -> This is Seller/Buyer ETH/EOS address.
/// Field4: amount -> If the struct is declared for SellOrder then this amount will be VTBC amount to sell.
/// Otherwise if the struct is defined for BuyOrder then this amout field will hold ETH/EOS crypto amount for buy.
/// Field5: usd_rate -> In SellOrder this field will store the VTBC rate at the time of struct declaratio.
/// And in BuyOrder this field will be empty.
#[derive( Encode, Decode, Clone, Debug, PartialEq, Eq, Deserialize, Serialize, TypeInfo)]
pub struct OrderBookStruct<T> {
    pub order_id: Vec<u8>,
    pub address: Option<T>,
    pub crypto_address: Option<Vec<u8>>,
    pub amount: U256,
    pub usd_rate: U256,
}

impl<T> OrderBookStruct<T> 
where T: sp_std::clone::Clone
{ 
    pub(crate) fn new(req_data: &TradeRequest<T>, len: u64) ->  OrderBookStruct<T> {

        let len_order_id = (len + 200_u64).to_string().as_bytes().to_vec();    
        OrderBookStruct {
            order_id: len_order_id,
            address: req_data.address.clone(),
            crypto_address: req_data.crypto_address.clone(),
            amount: U256::from(0_u8),
            usd_rate: U256::from(0_u8),
        }
    }

    pub(crate) fn update_amount(&mut self, amt: U256) {
        self.amount = amt;
    }

    pub(crate) fn update_usd_rate(&mut self, amt: U256) {
        self.usd_rate = amt;
    }
}

#[derive(Clone)]
pub struct TradeRequest<T> {
    pub crypto_type: TokenType,
    pub trade_type: TradeType, 
    pub address: Option<T>, 
    pub crypto_address: Option<Vec<u8>>, 
    pub crypto_amt: U256, 
    pub vtbc_amt: U256,
    pub controlled_amt: U256,
    pub id: Vec<u8>,
    pub usd_rate: U256,
    pub index: u64,
}

impl<T> TradeRequest<T> {
        
    pub(crate) fn new(token_type: TokenType, trade_type: TradeType, address: T, amt: U256) ->  TradeRequest<T> {
        match trade_type {
            TradeType::Buy => {
                TradeRequest {
                    crypto_type: token_type, 
                    trade_type,
                    address: Some(address), 
                    crypto_address: None, 
                    crypto_amt: amt, 
                    vtbc_amt: U256::from(0_u8),
                    controlled_amt: U256::from(0_u8),
                    id: Vec::new(), 
                    usd_rate: U256::from(0_u8),
                    index: 0,
                }
            },
            TradeType::Sell => {
                TradeRequest {
                    crypto_type: token_type, 
                    trade_type,
                    address: Some(address), 
                    crypto_address: None, 
                    crypto_amt: U256::from(0_u8),
                    vtbc_amt: amt, 
                    controlled_amt: U256::from(0_u8),
                    id: Vec::new(), 
                    usd_rate: U256::from(0_u8),
                    index: 0,
                }
            }
        }
    }
        
    pub(crate) fn insert_crypto_address(&mut self, address: Option<Vec<u8>>) {
        self.crypto_address = address;
    }

    pub(crate) fn new_from(token_type: TokenType, trade_type: TradeType, order_req: OrderBookStruct<T>, index: u64) ->  TradeRequest<T> {
         match trade_type {
            TradeType::Buy => {
                TradeRequest {
                    crypto_type: token_type, 
                    trade_type,
                    address: order_req.address, 
                    crypto_address: order_req.crypto_address, 
                    crypto_amt: order_req.amount, 
                    vtbc_amt: U256::from(0_u8),
                    controlled_amt: U256::from(0_u8), 
                    id: order_req.order_id, 
                    usd_rate: order_req.usd_rate,
                    index,
                }
             },
            TradeType::Sell => {
                TradeRequest {
                    crypto_type: token_type, 
                    trade_type,
                    address: order_req.address, 
                    crypto_address: order_req.crypto_address, 
                    crypto_amt: U256::from(0_u8), 
                    vtbc_amt: order_req.amount,
                    controlled_amt: U256::from(0_u8), 
                    id: order_req.order_id, 
                    usd_rate: order_req.usd_rate,
                    index,
                }
            }
        }
    }
}