mod cache;
mod empty;
mod fork;
mod rpc_chain;

use crate::{evm::BlockInfo, prelude::*};
pub use rpc_chain::RpcChain;

/// Read only chain state
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
