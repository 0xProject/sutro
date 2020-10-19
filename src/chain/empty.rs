//! Empty chain
//!
//! Chain with no state.

use super::ChainState;
use crate::evm::BlockInfo;
use zkp_u256::{Zero, U256};

struct Empty {}

impl ChainState for Empty {
    fn block(&self) -> BlockInfo {
        BlockInfo { timestamp: 0 }
    }

    fn nonce(&self, address: &U256) -> usize {
        0
    }

    fn balance(&self, address: &U256) -> usize {
        0
    }

    fn code(&self, address: &U256) -> Vec<u8> {
        Vec::new()
    }

    fn storage(&self, address: &U256, slot: &U256) -> U256 {
        U256::zero()
    }
}
