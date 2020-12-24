// TODO: async_trait and error handling

// TODO: Instead of hashmaps we can record all the read values. When
// re-executing, we can feed the values in in the same order they where read,
// ignoring parameters.

mod cache;
mod empty;
mod fork;
mod rpc_chain;
mod state_set;

pub use self::{cache::Cache, empty::Empty, fork::Fork, rpc_chain::RpcChain, state_set::StateSet};

use crate::prelude::*;

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

/// Create a fork from a JSON-RPC URL.
pub async fn fork(url: &str) -> AnyResult<Fork<Cache<RpcChain<web3::transports::Http>>>> {
    let transport = web3::transports::Http::new(url).context("Creating transport")?;

    let web3 = web3::Web3::new(transport.clone());
    let latest = tokio_compat_02::FutureExt::compat(web3.eth().block_number())
        .await
        .context("Fetching block number")?;
    info!("Forking from block number {}", latest);
    let block_number = web3::types::BlockNumber::Number(latest);

    Ok(Fork::from(Cache::from(RpcChain::new(
        transport,
        block_number,
    ))))
}
