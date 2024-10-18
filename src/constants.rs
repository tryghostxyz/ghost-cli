pub const CHAIN_ETH: u64 = 1;
pub const CHAIN_BASE: u64 = 8453;
pub const CHAIN_BASE_TESTNET: u64 = 84532;
pub const CHAIN_BLAST: u64 = 81457;
pub const CHAIN_BERA_TESTNET: u64 = 80084;
pub const CHAIN_ABS_TESTNET: u64 = 11124;
pub const CHAIN_ETH_SEPOLIA: u64 = 11155111;
pub const CHAIN_UNI_TESTNET: u64 = 1301;


pub const CHAIN_NAMES: [(&str, u64); 8] = [
    ("eth", CHAIN_ETH),
    ("eth_testnet", CHAIN_ETH_SEPOLIA),
    ("base", CHAIN_BASE),
    ("base_testnet", CHAIN_BASE_TESTNET),
    ("bera_testnet", CHAIN_BERA_TESTNET),
    ("blast", CHAIN_BLAST),
    ("abs_testnet", CHAIN_ABS_TESTNET),
    ("uni_testnet", CHAIN_UNI_TESTNET),
];
