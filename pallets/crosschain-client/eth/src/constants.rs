/** Module name: constants
 *  Description: Here all constants varibales is declared which will be used within this eth_vtb_contract pallet
 **/
 pub const NUM_VEC_LEN: usize = 10;
 /// The type to sign and send transactions.
 pub const UNSIGNED_TXS_PRIORITY: u64 = 100;
 
 pub const FETCH_TIMEOUT_PERIOD: u64 = 10000; // in milli-seconds
 pub const LOCK_TIMEOUT_EXPIRATION: u64 = FETCH_TIMEOUT_PERIOD + 5000; // in milli-seconds
 pub const LOCK_BLOCK_EXPIRATION: u32 = 3; // in block number
 pub const TOKEN_SYMBOL: &str = "ETH";
 
 /******************ETHEREUM CONTRACT DETAIL***************************/
 // ETHEREUM CONTRACT VTB EVENTS
 pub const ETH_VTB_TOPIC_ONBOARD: &str = "0x9954bce02f72fbdef0fa668ac6f42e434b5829615dac2fb22448054d2359d4f6";
 pub const ETH_VTB_TOPIC_DEPOSITETH: &str = "0x481f2c5f52e707f1a698b96b48d71f8e379a2f9a1fd62ae7b4a4482069583369";	
 pub const ETH_VTB_TOPIC_WITHDRAWETH: &str = "0xd8f8af852ed19cf5c8faaabb0f17c48f257593ede85efc2f1fe7ba6830de23e9";
 pub const BIRTHDAY_BLOCK_NUMBER: u64 = 0; //11679340 chris sync data
 /******************END OF ETHEREUM CONTRACT DETAIL***************************/

pub mod storage_keys {
    pub const PROCESS_RANGE_STORAGE_KEY: &[u8] = b"eth-vtb-contract::eth-processed-block-range-record";
    pub const CURRENT_BLOCK_RANGE_STORAGE_KEY: &[u8] = b"eth-vtb-contract::eth-current-block-range-record";
    pub const LOGS_CHECKED_QUEUE: &[u8] = b"light-client-worker::eos_blocks_log_queue";
    pub const AUTH_TOKEN_QUEUE: &[u8] = b"light-client-worker::diffuse_auth_token";
    pub const BLOCKS_QUEUE: &[u8] = b"light-client-worker::eos_blocks_queue";
}