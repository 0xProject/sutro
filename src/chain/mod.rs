mod eth_json_rpc;

use crate::evm::BlockInfo;
pub use eth_json_rpc::EthJsonRpc;
use zkp_u256::U256;

pub trait ChainState {
    fn block(&self) -> BlockInfo;
    fn nonce(&self, address: &U256) -> usize;
    fn balance(&self, address: &U256) -> usize;
    fn code(&self, address: &U256) -> Vec<u8>;
    fn storage(&self, address: &U256, slot: &U256) -> U256;
}
