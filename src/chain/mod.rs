// TODO: async_trait and error handling

// TODO: Instead of hashmaps we can record all the read values. When
// re-executing, we can feed the values in in the same order they where read,
// ignoring parameters.

mod cache;
mod empty;
mod fork;
mod rpc_chain;
mod state_set;
pub mod types;

pub use self::{cache::Cache, empty::Empty, fork::Fork, rpc_chain::RpcChain, state_set::StateSet};

use crate::{
    prelude::*,
    rpc::{self, types::BlockNumber},
};

/// Constant for the current block
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct BlockInfo {
    pub timestamp: u64,
}

/// Read only chain state
#[allow(clippy::module_name_repetitions)]
pub trait ChainState {
    fn block(&self) -> BlockInfo;
    fn nonce(&self, address: &U256) -> usize;
    fn balance(&self, address: &U256) -> U256;
    fn code(&self, address: &U256) -> Vec<u8>;
    fn storage(&self, address: &U256, slot: &U256) -> U256;
}

pub trait WriteableChainState: ChainState {
    fn set_nonce(&mut self, address: &U256, nonce: usize);
    fn set_balance(&mut self, address: &U256, balance: &U256);
    fn set_code(&mut self, address: &U256, code: &[u8]);
    fn set_storage(&mut self, address: &U256, slot: &U256, value: &U256);
}

/// Create an empty chain
pub async fn new() -> AnyResult<Fork<Empty>> {
    Ok(Fork::from(Empty))
}

/// Create a fork from a JSON-RPC URL.
pub async fn fork(url: &str) -> AnyResult<Fork<Cache<RpcChain>>> {
    let client = rpc::client(url)
        .await
        .context("Creating RPC client to fork from")?;

    // Pin to latest block
    let latest = client
        .get_block_by_number(BlockNumber::Latest, false)
        .await
        .map_err(|err| anyhow!("Error: {}", err))
        .context("Fetching latest block number")?
        .ok_or_else(|| anyhow!("Latest block not found"))?
        .header
        .number;
    info!("Forking from block number {}", latest);
    let block_number = BlockNumber::Number(latest);

    // Create monad stack
    Ok(Fork::from(Cache::from(RpcChain::new(client, block_number))))
}
