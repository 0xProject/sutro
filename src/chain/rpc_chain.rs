use super::{BlockInfo, ChainState};
use crate::{
    prelude::*,
    rpc::{
        types::{BlockHeader, BlockNumber},
        EthereumRpcClient,
    },
};
use futures::executor::block_on;

pub struct RpcChain {
    client:       EthereumRpcClient,
    block_number: BlockNumber,
}

impl RpcChain {
    pub fn new(client: EthereumRpcClient, block_number: BlockNumber) -> Self {
        Self {
            client,
            block_number,
        }
    }
}

// TODO: Async & Result ?
impl ChainState for RpcChain {
    fn block(&self) -> BlockInfo {
        todo!();
        // let result: AnyResult<BlockHeader> = block_on(async {
        //     Ok(self
        //         .client
        //         .get_block_by_number(BlockNumber::Latest, false)
        //         .await
        //         .map_err(|err| anyhow!("Error: {}", err))
        //         .context("Fetching latest block number")?
        //         .ok_or_else(|| anyhow!("Latest block not found"))?)
        // });
        // let block = result.unwrap();
        // BlockInfo {
        //     timestamp: block.timestamp.into_inner(),
        // }
    }

    fn nonce(&self, _address: &U256) -> usize {
        todo!()
    }

    fn balance(&self, _address: &U256) -> U256 {
        todo!()
    }

    fn code(&self, _address: &U256) -> Vec<u8> {
        todo!()
    }

    fn storage(&self, _address: &U256, _slot: &U256) -> U256 {
        todo!()
    }
}
