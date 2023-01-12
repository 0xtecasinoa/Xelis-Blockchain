use crate::crypto::hash::Hash;

pub const VERSION: &str = "alpha-0.0.1";
pub const NETWORK_ID: [u8; 16] = [0xA, 0xB, 0xC, 0xD, 0xE, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0xF];
pub const SEED_NODES: [&str; 1] = ["127.0.0.1:2125"]; // ["127.0.0.1:2125", "127.0.0.1:2126", "127.0.0.1:2127", "127.0.0.1:2128"];
pub const DEFAULT_P2P_BIND_ADDRESS: &str = "0.0.0.0:2125";
pub const DEFAULT_RPC_BIND_ADDRESS: &str = "0.0.0.0:8080";
pub const DEFAULT_DIR_PATH: &str = "mainnet";
pub const DEFAULT_CACHE_SIZE: usize = 1024;
pub const XELIS_ASSET: Hash = Hash::zero();
pub const SIDE_BLOCK_REWARD_PERCENT: u64 = 30; // only 30% of reward for side block
pub const BLOCK_TIME: u64 = 15 * 1000; // Block Time in milliseconds
pub const MINIMUM_DIFFICULTY: u64 = BLOCK_TIME * 10;
pub const GENESIS_BLOCK_DIFFICULTY: u64 = 1;
pub const MAX_BLOCK_SIZE: usize = (1024 * 1024) + (256 * 1024); // 1.25 MB
pub const FEE_PER_KB: u64 = 1000; // 0.01000 XLS per KB
pub const DEV_FEE_PERCENT: u64 = 5; // 5% per block going to dev address
pub const TIPS_LIMIT: usize = 3; // maximum 3 previous blocks
pub const STABLE_HEIGHT_LIMIT: u64 = 8;
pub const TIMESTAMP_IN_FUTURE_LIMIT: u128 = 2 * 1000; // 2 seconds maximum in future

pub const PREFIX_ADDRESS: &str = "xel"; // mainnet prefix address
pub const TESTNET_PREFIX_ADDRESS: &str = "xet"; // testnet prefix address
pub const COIN_VALUE: u64 = 100_000; // 5 decimals for a full coin
pub const MAX_SUPPLY: u64 = 18_400_000 * COIN_VALUE; // 18.4M full coin
pub const EMISSION_SPEED_FACTOR: u64 = 21;

pub const GENESIS_BLOCK: &str = "00000000000000000000000000000000000001846e1e9234000000000000000000000000000000000000000000000000000000000000000000000000000000000000006c24cdc1c8ee8f028b8cafe7b79a66a0902f26d89dd54eeff80abcf251a9a3bd"; // Genesis block in hexadecimal format
pub const GENESIS_BLOCK_HASH: &str = "81cf282f5818edb220d43ec79fdbd2d8f40e94a9e6afb786b3a45bb6a085e5e9";
pub const DEV_ADDRESS: &str = "xel1qyqxcfxdc8ywarcz3wx2leahnfn2pyp0ymvfm42waluq408j2x5680g05xfx5"; // Dev address

pub const MAX_BLOCK_REWIND: u64 = STABLE_HEIGHT_LIMIT - 1; // maximum X blocks can be rewinded
pub const CHAIN_SYNC_TIMEOUT_SECS: u64 = 3; // wait maximum between each chain sync request to peers
pub const CHAIN_SYNC_DELAY: u64 = 3; // minimum X seconds between each chain sync request per peer
pub const CHAIN_SYNC_REQUEST_MAX_BLOCKS: usize = 64; // allows up to X blocks id (hash + height) 
pub const P2P_PING_DELAY: u64 = 10; // time between each ping
pub const P2P_PING_PEER_LIST_DELAY: u64 = 15; // time in seconds between each update of peerlist
pub const P2P_PING_PEER_LIST_LIMIT: usize = 16; // maximum number of addresses to be send
pub const P2P_DEFAULT_MAX_PEERS: usize = 32; // default number of maximum peers
pub const PEER_TIMEOUT_REQUEST_OBJECT: u64 = 1500; // millis until we timeout

// Wallet config
pub const DEFAULT_DAEMON_ADDRESS: &str = DEFAULT_RPC_BIND_ADDRESS;