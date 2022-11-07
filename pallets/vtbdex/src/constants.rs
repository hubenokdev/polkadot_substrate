pub const FETCH_TIMEOUT_PERIOD: u64 = 30000; // in milli-seconds
pub const LOCK_TIMEOUT_EXPIRATION: u64 = FETCH_TIMEOUT_PERIOD + 3000; // in milli-seconds
pub const LOCK_BLOCK_EXPIRATION: u32 = 5; // in block number

pub const ONCHAIN_TX_KEY: &[u8] = b"vtbdex::storage::tx";

pub const ETHEREUM_CONTRACT_METHOD_NAME: &str = "withdrawEth"; // Eth withdraw Method name
pub const _TARGET_RATES: &[[u32;2];30] = &[
     [2022,100],[2023,90],[2024,85],
     [2025,82],[2026,80],[2027,78],
     [2028,75],[2029,72],[2030,70],
     [2031,68],[2032,65],[2033,62],
     [2034,60],[2035,58],[2036,55],
     [2037,50],[2038,48],[2039,45],
     [2040,42],[2041,40],[2042,38],
     [2043,35],[2044,30],[2045,28],
     [2046,25],[2047,24],[2048,23],
     [2049,22],[2050,21],[2051,20],
    ];

