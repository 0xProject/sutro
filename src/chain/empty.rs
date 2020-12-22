//! Empty chain
//!
//! Chain with no state.

use super::{BlockInfo, ChainState};
use crate::prelude::*;

/// An empty chain state with a given block header
///
/// All balances, nonces and storage is zero, there is no contract code.
#[derive(Clone, Default, Debug)]
pub struct Empty;

impl ChainState for Empty {
    fn block(&self) -> BlockInfo {
        BlockInfo::default()
    }

    fn nonce(&self, _address: &U256) -> usize {
        0
    }

    fn balance(&self, _address: &U256) -> U256 {
        U256::zero()
    }

    fn code(&self, _address: &U256) -> Vec<u8> {
        Vec::new()
    }

    fn storage(&self, _address: &U256, _slot: &U256) -> U256 {
        U256::zero()
    }
}
