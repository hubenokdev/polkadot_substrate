/***********************
 * EOS GO API ENDPOINTS
 */
 pub const _EOS_CURRENT_BLOCK_NUMBER_API_ENDPOINT: &str = "https://gobackend.vtbtestnet.com/eos/getcurrentblock"; // API ENDPOINT TO FETCH EOS CURRENT BLOCK NUMBER
 pub const _EOS_BLOCK_INFO_API_ENDPOINT: &str = "https://gobackend.vtbdev.io/eos/rangeblocks";  // API ENDPOINT TO FETCH EOS BLOCK INFO (RANGE API)
 pub const _EOS_POWERUP_ACCOUNT_API: &str = "https://gobackend.vtbdev.io/eos/powerup";   // API ENDPOINT TO SEND EOS ACCOUNT POWERUP REQUEST
 pub const _EOS_BUY_RAM_IN_EOS_ACCOUNT_API: &str = "https://gobackend.vtbdev.io/eos/buyram"; // API ENDPOINT TO SEND BUYRAM REQUEST
 pub const _EOS_WITHDRAW_REQUEST_API_ENDPOINT: &str = "https://gobackend.vtbdev.io/eos/withdraw"; // API ENDPOINT TO SEND EOS WITHDRAW REQUEST
 pub const _EOS_USD_RATE_API_ENDPOINT: &str = "http://freecurrencyrates.com/api/action.php?do=cvals&iso=USD&f=EOS&v=1&s=cbr"; // API ENDPOINT TO FETCH EOS RATE
 pub const _EOS_CONTRACT_ACCOUNT: &str = "vtbownertes1";  // EOS VTB CONTRACT NAME
 pub const _EOS_ACCOUNT_NAME: &str = "vtbownertes1";     // EOS VTB ACCOUNT NAME
 pub const _EOS_SUBSTRATE_ACCOUNT: &str = "vtbownertes1";  // EOS VTB SUBSTRATE USER ACCOUNT NAME
 pub const _EOS_EOSIO_MAX_PAYMENT: &str = "0.0050 EOS";  // MAX PAYMENT FOR POWERUP
 
 /***********************
  * ETH GO API ENDPOINTS
  */
 pub const _ETHEREUM_CONTRACT_ADDRESS: &str = "0xe7cf2db8daf42f1e33902131b494d4b995bf792d"; 
 pub const _ETH_CURRENT_BLOCK_NUMBER_API_ENDPOINT: &str = "https://gobackend.vtbtestnet.com/eth/getcurrentblock"; // API ENDPOINT TO FETCH ETH CURRENT BLOCK NUMBER
 pub const _ETH_BLOCK_INFO_RANGE_API_ENDPOINT: &str = "https://gobackend.vtbdev.io/eth/rangeblocks"; // API ENDPOINT TO FETCH ETH BLOCK INFO (RANGE API)
 pub const _ETH_USD_RATE_API_ENDPOINT: &str = "https://gobackend.vtbdev.io/eth/getethrate"; // API ENDPOINT TO FETCH ETH RATE
 pub const _ETH_HODLC_USD_RATE_READ_API: &str = "https://gobackend.vtbdev.io/eth/gethodlcrate"; // API ENDPOINT TO FETCH HODLC RATE
 pub const _WITHDRAW_REQUEST_API_ENDPOINT: &str = "https://gobackend.vtbdev.io/eth/withdrawtxn"; // API ENDPOINT TO SEND ETH WITHDRAW REQUEST
 
 /***********************
  * OTHER DETAILS
  */
 pub const _IPFS_REQUEST_API_ENDPOINT: &str = "https://gobackend.vtbdev.io/eth/ipfs"; // API ENDPOINT TO SEND IPFS REQUEST
 pub const _IPFS_GAS_ESTIMATION_API_ENDPOINT: &str = "https://gobackend.vtbdev.io/eth/ipfsestimategas"; // API ENDPOINT TO FETCH ESTIMATED GAS PRICE
 /***********************
  * GO API BASE64 ENCODED AUTHORIZATION KEY
  */
 pub const _BASIC_AUTHORIZATION_KEY: &str = "Basic YWRtaW4xMjM6YWRtaW5wYXNz";
 
 /***********************
  * SET UPCOMING BLOCKNUMBER OF VTBDEX FOR HOTFIX
  */
 pub const _HOT_FIX_BLOCK_NUMBER: u32 = 15;
 
 /***********************
  * ETH HODLT GO API ENDPOINTS ( UPCOMING FEATURE)
  */
 pub const _MINT_REQUEST_API_ENDPOINT: &str = "https://gobackend.vtbdev.io/eth/mintvtbt";
 pub const _BURN_REQUEST_API_ENDPOINT: &str = "https://gobackend.vtbdev.io/eth/burnvtbt";
 pub const _TRANSFER_REQUEST_API_ENDPOINT: &str = "https://gobackend.vtbdev.io/eth/transfervtbt";
 pub const _GET_VTBT_TOKEN_BALANCE_API: &str = "https://gobackend.vtbdev.io/eth/balanceof";
 pub const _ETH_VTB_GET_POLKADOT_ADDRESS_API: &str = "https://gobackend.vtbdev.io/eth/getpolkadotaddr";
 

/***************
* EOS NATION DIFFUSE KEY
*/
pub const _EOS_DIFFUSE_API_KEY: &str = "770305df81771e3fa01de078c85990a2";
pub const _EOS_GENERATE_AUTH_TOKEN_ENDPOINT: &str = "https://auth.eosnation.io/v1/auth/issue";
pub const _EOS_DIFFUSE_GET_BLOCK: &str = "https://eos.dfuse.eosnation.io/v1/chain/get_block";
pub const _EOS_DIFFUSE_GET_TRANSACTIONS: &str = "https://eos.dfuse.eosnation.io/v0/transactions/";
pub const _EOS_DIFFUSE_GET_INFO: &str = "https://eos.dfuse.eosnation.io/v1/chain/get_info";
// pub const _EOS_DIFFUSE_API_KEY: &str = "770305df81771e3fa01de078c85990a2";
