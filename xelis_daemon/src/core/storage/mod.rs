mod sled;
pub use self::sled::SledStorage;

use std::{collections::{HashSet, BTreeSet}, sync::Arc};
use async_trait::async_trait;
use xelis_common::{
    crypto::{key::PublicKey, hash::Hash},
    transaction::Transaction,
    block::{Block, BlockHeader, Difficulty}, account::{VersionedBalance, VersionedNonce}, immutable::Immutable, network::Network,
};

use crate::core::error::BlockchainError;

pub type Tips = HashSet<Hash>;

// this trait is useful for P2p to check itself the validty of a chain
#[async_trait]
pub trait DifficultyProvider {
    async fn get_height_for_block_hash(&self, hash: &Hash) -> Result<u64, BlockchainError>;
    async fn get_timestamp_for_block_hash(&self, hash: &Hash) -> Result<u128, BlockchainError>;
    async fn get_difficulty_for_block_hash(&self, hash: &Hash) -> Result<Difficulty, BlockchainError>;
    async fn get_cumulative_difficulty_for_block_hash(&self, hash: &Hash) -> Result<Difficulty, BlockchainError>;
    async fn get_past_blocks_for_block_hash(&self, hash: &Hash) -> Result<Arc<Vec<Hash>>, BlockchainError>;
    async fn get_block_header_by_hash(&self, hash: &Hash) -> Result<Arc<BlockHeader>, BlockchainError>;
}

#[async_trait]
pub trait Storage: DifficultyProvider + Sync + Send + 'static {
    fn get_pruned_topoheight(&self) -> Result<Option<u64>, BlockchainError>;
    fn set_pruned_topoheight(&mut self, pruned_topoheight: u64) -> Result<(), BlockchainError>;

    // delete block at topoheight, and all pointers (hash_at_topo, topo_by_hash, reward, supply, diff, cumulative diff...)
    async fn delete_block_at_topoheight(&mut self, topoheight: u64) -> Result<Arc<BlockHeader>, BlockchainError>;
    async fn delete_tx(&mut self, hash: &Hash) -> Result<Arc<Transaction>, BlockchainError>;
    // delete versioned balances at a specific topoheight
    async fn delete_versioned_balances_for_asset_at_topoheight(&mut self, asset: &Hash, topoheight: u64) -> Result<(), BlockchainError>;
    // delete versioned nonces at a specific topoheight
    async fn delete_versioned_nonces_at_topoheight(&mut self, topoheight: u64) -> Result<(), BlockchainError>;
    // delete all versions of balances under the specified topoheight
    // for those who don't have more recents, set it to the topoheight
    // for those above it, cut the chain by deleting the previous topoheight when it's going under
    async fn create_snapshot_balances_at_topoheight(&mut self, assets: &Vec<Hash>, topoheight: u64) -> Result<(), BlockchainError>;
    // same as above but for nonces
    async fn create_snapshot_nonces_at_topoheight(&mut self, topoheight: u64) -> Result<(), BlockchainError>;

    async fn get_partial_assets(&self, maximum: usize, skip: usize, maximum_topoheight: u64) -> Result<BTreeSet<Hash>, BlockchainError>;
    async fn get_partial_keys(&self, maximum: usize, skip: usize, maximum_topoheight: u64) -> Result<BTreeSet<PublicKey>, BlockchainError>;

    async fn get_balances<'a, I: Iterator<Item = &'a PublicKey> + Send>(&self, asset: &Hash, keys: I, maximum_topoheight: u64) -> Result<Vec<Option<u64>>, BlockchainError>;

    fn get_block_executer_for_tx(&self, tx: &Hash) -> Result<Hash, BlockchainError>;
    fn set_tx_executed_in_block(&mut self, tx: &Hash, block: &Hash) -> Result<(), BlockchainError>;
    fn remove_tx_executed(&mut self, tx: &Hash) -> Result<(), BlockchainError>;
    fn is_tx_executed_in_a_block(&self, tx: &Hash) -> Result<bool, BlockchainError>;
    fn is_tx_executed_in_block(&self, tx: &Hash, block: &Hash) -> Result<bool, BlockchainError>;
    fn set_blocks_for_tx(&mut self, tx: &Hash, blocks: &HashSet<Hash>) -> Result<(), BlockchainError>;

    fn get_network(&self) -> Result<Network, BlockchainError>;
    fn set_network(&mut self, network: &Network) -> Result<(), BlockchainError>;
    fn has_network(&self) -> Result<bool, BlockchainError>;

    async fn asset_exist(&self, asset: &Hash) -> Result<bool, BlockchainError>;
    async fn add_asset(&mut self, asset: &Hash, topoheight: u64) -> Result<(), BlockchainError>;
    async fn get_assets(&self) -> Result<Vec<Hash>, BlockchainError>;
    fn get_asset_registration_topoheight(&self, asset: &Hash) -> Result<u64, BlockchainError>;

    fn has_tx_blocks(&self, hash: &Hash) -> Result<bool, BlockchainError>;
    fn has_block_linked_to_tx(&self, tx: &Hash, block: &Hash) -> Result<bool, BlockchainError>;
    fn get_blocks_for_tx(&self, hash: &Hash) -> Result<Tips, BlockchainError>;
    fn add_block_for_tx(&mut self, tx: &Hash, block: &Hash) -> Result<(), BlockchainError>;

    fn set_last_topoheight_for_balance(&mut self, key: &PublicKey, asset: &Hash, topoheight: u64) -> Result<(), BlockchainError>;
    fn delete_last_topoheight_for_balance(&mut self, key: &PublicKey, asset: &Hash) -> Result<(), BlockchainError>;
    async fn get_last_topoheight_for_balance(&self, key: &PublicKey, asset: &Hash) -> Result<u64, BlockchainError>;
    async fn has_balance_for(&self, key: &PublicKey, asset: &Hash) -> Result<bool, BlockchainError>;
    async fn has_balance_at_exact_topoheight(&self, key: &PublicKey, asset: &Hash, topoheight: u64) -> Result<bool, BlockchainError>;
    async fn get_balance_at_exact_topoheight(&self, key: &PublicKey, asset: &Hash, topoheight: u64) -> Result<VersionedBalance, BlockchainError>;
    async fn get_balance_at_maximum_topoheight(&self, key: &PublicKey, asset: &Hash, topoheight: u64) -> Result<Option<(u64, VersionedBalance)>, BlockchainError>;
    async fn delete_balance_at_topoheight(&mut self, key: &PublicKey, asset: &Hash, topoheight: u64) -> Result<VersionedBalance, BlockchainError>;
    async fn get_new_versioned_balance(&self, key: &PublicKey, asset: &Hash, topoheight: u64) -> Result<VersionedBalance, BlockchainError>;
    async fn set_balance_to(&mut self, key: &PublicKey, asset: &Hash, topoheight: u64, version: &VersionedBalance) -> Result<(), BlockchainError>;
    async fn get_last_balance(&self, key: &PublicKey, asset: &Hash) -> Result<(u64, VersionedBalance), BlockchainError>;
    async fn set_balance_at_topoheight(&mut self, asset: &Hash, topoheight: u64, key: &PublicKey, balance: &VersionedBalance) -> Result<(), BlockchainError>;

    async fn has_nonce(&self, key: &PublicKey) -> Result<bool, BlockchainError>;
    async fn has_nonce_at_exact_topoheight(&self, key: &PublicKey, topoheight: u64) -> Result<bool, BlockchainError>;
    // returns its topoheight and its VersionedNonce
    async fn get_last_nonce(&self, key: &PublicKey) -> Result<(u64, VersionedNonce), BlockchainError>;
    async fn get_nonce_at_exact_topoheight(&self, key: &PublicKey, topoheight: u64) -> Result<VersionedNonce, BlockchainError>;
    async fn get_nonce_at_maximum_topoheight(&self, key: &PublicKey, topoheight: u64) -> Result<Option<(u64, VersionedNonce)>, BlockchainError>;
    // set the new highest topoheight for account
    fn set_last_topoheight_for_nonce(&mut self, key: &PublicKey, topoheight: u64) -> Result<(), BlockchainError>;
    async fn set_nonce_at_topoheight(&mut self, key: &PublicKey, nonce: u64, topoheight: u64) -> Result<(), BlockchainError>;

    fn get_block_reward(&self, hash: &Hash) -> Result<u64, BlockchainError>;
    fn set_block_reward(&mut self, hash: &Hash, reward: u64) -> Result<(), BlockchainError>;

    async fn get_transaction(&self, hash: &Hash) -> Result<Arc<Transaction>, BlockchainError>;
    fn count_transactions(&self) -> usize;
    async fn has_transaction(&self, hash: &Hash) -> Result<bool, BlockchainError>;

    async fn add_new_block(&mut self, block: Arc<BlockHeader>, txs: &Vec<Immutable<Transaction>>, difficulty: Difficulty, hash: Hash) -> Result<(), BlockchainError>;
    async fn pop_blocks(&mut self, mut height: u64, mut topoheight: u64, count: u64) -> Result<(u64, u64, Vec<(Hash, Arc<Transaction>)>), BlockchainError>;
    fn has_blocks(&self) -> bool;
    fn count_blocks(&self) -> usize;
    async fn has_block(&self, hash: &Hash) -> Result<bool, BlockchainError>;
    async fn has_blocks_at_height(&self, height: u64) -> Result<bool, BlockchainError>;
    async fn get_block_header_at_topoheight(&self, topoheight: u64) -> Result<(Hash, Arc<BlockHeader>), BlockchainError>;
    async fn get_top_block_hash(&self) -> Result<Hash, BlockchainError>;
    
    async fn get_block(&self, hash: &Hash) -> Result<Block, BlockchainError>;
    async fn get_top_block(&self) -> Result<Block, BlockchainError>;
    async fn get_top_block_header(&self) -> Result<(Arc<BlockHeader>, Hash), BlockchainError>;

    async fn get_blocks_at_height(&self, height: u64) -> Result<Tips, BlockchainError>;
    async fn add_block_hash_at_height(&mut self, hash: Hash, height: u64) -> Result<(), BlockchainError>;

    async fn get_topo_height_for_hash(&self, hash: &Hash) -> Result<u64, BlockchainError>;
    async fn set_topo_height_for_block(&mut self, hash: &Hash, topoheight: u64) -> Result<(), BlockchainError>;
    async fn is_block_topological_ordered(&self, hash: &Hash) -> bool;
    async fn get_hash_at_topo_height(&self, topoheight: u64) -> Result<Hash, BlockchainError>;

    async fn get_supply_at_topo_height(&self, topoheight: u64) -> Result<u64, BlockchainError>;

    fn get_supply_for_block_hash(&self, hash: &Hash) -> Result<u64, BlockchainError>;
    fn set_supply_for_block_hash(&mut self, hash: &Hash, supply: u64) -> Result<(), BlockchainError>;

    async fn set_cumulative_difficulty_for_block_hash(&mut self, hash: &Hash, cumulative_difficulty: u64) -> Result<(), BlockchainError>;

    fn get_top_topoheight(&self) -> Result<u64, BlockchainError>;
    fn set_top_topoheight(&mut self, topoheight: u64) -> Result<(), BlockchainError>;

    fn get_top_height(&self) -> Result<u64, BlockchainError>;
    fn set_top_height(&mut self, height: u64) -> Result<(), BlockchainError>;

    async fn get_tips(&self) -> Result<Tips, BlockchainError>;
    fn store_tips(&mut self, tips: &Tips) -> Result<(), BlockchainError>;

    //async fn execute_db_transaction<'a>(&mut self, transaction: DatabaseTransaction<'a, Self>) -> Result<(), BlockchainError>;
}