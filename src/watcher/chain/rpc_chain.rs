use super::ChainState;
use crate::{evm::BlockInfo, prelude::*};
use web3::{
    block_on,
    types::{BlockId, BlockNumber, H160, U256 as W256},
    Transport, Web3,
};

pub struct RpcChain<T: Transport> {
    connection:   Web3<T>,
    block_number: BlockNumber,
}

impl<T: Transport> RpcChain<T> {
    pub fn new(transport: T, block_number: BlockNumber) -> Self {
        Self {
            connection: Web3::new(transport),
            block_number,
        }
    }
}

impl<T: Transport> ChainState for RpcChain<T> {
    fn block(&self) -> BlockInfo {
        let block_id = BlockId::Number(self.block_number);
        let block = block_on(self.connection.eth().block(block_id))
            .expect("Fetch block failed")
            .expect("Block not found");
        BlockInfo {
            timestamp: block.timestamp.as_u64(),
        }
    }

    fn nonce(&self, address: &U256) -> usize {
        let address = H160::from_slice(&address.to_bytes_be()[12..]);
        let nonce = block_on(
            self.connection
                .eth()
                .transaction_count(address, Some(self.block_number)),
        )
        .expect("Get transaction count (nonce) failed");
        nonce.as_usize()
    }

    fn balance(&self, address: &U256) -> U256 {
        let address = H160::from_slice(&address.to_bytes_be()[12..]);
        let balance = block_on(
            self.connection
                .eth()
                .balance(address, Some(self.block_number)),
        )
        .expect("Fetch balance failed");
        let mut bytes = [0_u8; 32];
        balance.to_big_endian(&mut bytes);
        U256::from_bytes_be(&bytes)
    }

    fn code(&self, address: &U256) -> Vec<u8> {
        let address = H160::from_slice(&address.to_bytes_be()[12..]);
        let code = block_on(self.connection.eth().code(address, Some(self.block_number)))
            .expect("Fetch code failed");
        code.0
    }

    fn storage(&self, address: &U256, slot: &U256) -> U256 {
        let address = H160::from_slice(&address.to_bytes_be()[12..]);
        let idx = W256::from_big_endian(&slot.to_bytes_be());
        let storage = block_on(self.connection.eth().storage(
            address,
            idx,
            Some(self.block_number),
        ))
        .expect("Fetch storage failed");
        U256::from_bytes_be(storage.as_fixed_bytes())
    }
}
