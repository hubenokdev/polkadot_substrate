/** Module name: constants
 *  Description: Here all constants varibales is declared which will be used within this eth_vtb_contract pallet
 **/
 pub const NUM_VEC_LEN: usize = 10;
 /// The type to sign and send transactions.
 pub const UNSIGNED_TXS_PRIORITY: u64 = 100;
 
 pub const FETCH_TIMEOUT_PERIOD: u64 = 10000; // in milli-seconds
 pub const LOCK_TIMEOUT_EXPIRATION: u64 = FETCH_TIMEOUT_PERIOD + 6000; // in milli-seconds
 pub const LOCK_BLOCK_EXPIRATION: u32 = 3; // in block number
 pub const TOKEN_SYMBOL: &str = "EOS";

 /******************EOS CONTRACT DETAIL***************************/
 // EOS VTB events name
 pub const EOS_VTB_ACTION_ONBOARD_USER: &str = "notifyonbrd";
 pub const EOS_VTB_ACTION_WITHDRAW_EOS: &str = "notifywdrw";	
 pub const EOS_VTB_ACTION_AUTH_SUBSTRATE: &str = "authsubstrate";	
 pub const EOS_VTB_ACTION_EOSIO_TRANSFER: &str = "notifydpst";	
 pub const BIRTHDAY_BLOCK_NUMBER: u64 = 0; 
 /******************END OF EOS CONTRACT DETAIL***************************/

pub mod storage_keys {
    pub const PROCESS_RANGE_STORAGE_KEY: &[u8] = b"eos-vtb-contract::eos-processed-block-range-record";
    pub const CURRENT_BLOCK_RANGE_STORAGE_KEY: &[u8] = b"light-client-worker::eos_blocks_queue";
    pub const LOGS_CHECKED_QUEUE: &[u8] = b"light-client-worker::eos_blocks_log_queue";
    pub const AUTH_TOKEN_QUEUE: &[u8] = b"light-client-worker::diffuse_auth_token";
    pub const BLOCKS_QUEUE: &[u8] = b"light-client-worker::eos_blocks_queue";
}