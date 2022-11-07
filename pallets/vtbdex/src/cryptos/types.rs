use codec::{Decode, Encode};
use sp_std::{ prelude::*, fmt, slice::Iter};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};

#[derive( Encode, Decode, Debug, Clone, TypeInfo, Copy, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub enum TokenType {
	Eos,
	Eth,
	Vtbc,
	Vtbt,
	None
}

impl Default for TokenType {
    fn default() -> Self { TokenType::None }
}

impl TokenType {
    pub fn _iterator() -> Iter<'static, TokenType> {
        static _TOKENTYPES: [TokenType; 4] = [TokenType::Eos, TokenType::Eth, TokenType::Vtbc, TokenType::Vtbt];
        _TOKENTYPES.iter()
    }
	pub fn _distributable_iterator() -> Iter<'static, TokenType> {
        static _DISTTOKEN: [TokenType; 3] = [TokenType::Eos, TokenType::Eth, TokenType::Vtbc];
        _DISTTOKEN.iter()
    }
	pub fn _crypto_iterator() -> Iter<'static, TokenType> {
        static _CRYPTOTOKEN: [TokenType; 2] = [TokenType::Eos, TokenType::Eth];
        _CRYPTOTOKEN.iter()
	}
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
