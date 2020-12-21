//! Forked chain
//!
//! Takes a given read-only chain implementation and implements and in-memory
//! change buffer on top of it. The new chain acts as a fork of the underlying
//! chain.

use super::ChainState;
use crate::evm::BlockInfo;
use zkp_u256::U256;

struct Fork<Base: ChainState> {
    base: Base,
}

impl<Base: ChainState> ChainState for Fork<Base> {
    fn block(&self) -> BlockInfo {
        self.base.block()
    }

    fn nonce(&self, address: &U256) -> usize {
        // TODO: Add local accounts
        self.base.nonce(address)
    }

    fn balance(&self, address: &U256) -> usize {
        self.base.balance(address)
    }

    fn code(&self, address: &U256) -> Vec<u8> {
        self.base.code(address)
    }

    fn storage(&self, address: &U256, slot: &U256) -> U256 {
        self.base.storage(address, slot)
    }
}
