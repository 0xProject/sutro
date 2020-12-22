//! Forked chain
//!
//! Takes a given read-only chain implementation and implements and in-memory
//! change buffer on top of it. The new chain acts as a fork of the underlying
//! chain.

use super::{BlockInfo, ChainState, StateSet, WriteableChainState};
use crate::prelude::*;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Fork<Base: ChainState> {
    base:  Base,
    state: StateSet,
}

impl<Base: ChainState> Fork<Base> {
    pub fn inner(&self) -> &Base {
        &self.base
    }

    pub fn inner_mut(&mut self) -> &mut Base {
        &mut self.base
    }

    pub fn into_inner(self) -> Base {
        self.base
    }
}

impl<Base: ChainState> From<Base> for Fork<Base> {
    fn from(base: Base) -> Self {
        Self {
            base,
            state: StateSet::default(),
        }
    }
}

impl<Base: ChainState> ChainState for Fork<Base> {
    fn block(&self) -> BlockInfo {
        self.state
            .block
            .clone()
            .unwrap_or_else(|| self.base.block())
    }

    fn nonce(&self, address: &U256) -> usize {
        self.state
            .nonces
            .get(address)
            .cloned()
            .unwrap_or_else(|| self.base.nonce(address))
    }

    fn balance(&self, address: &U256) -> U256 {
        self.state
            .balances
            .get(address)
            .cloned()
            .unwrap_or_else(|| self.base.balance(address))
    }

    fn code(&self, address: &U256) -> Vec<u8> {
        self.state
            .codes
            .get(address)
            .cloned()
            .unwrap_or_else(|| self.base.code(address))
    }

    fn storage(&self, address: &U256, slot: &U256) -> U256 {
        self.state
            .storages
            .get(&(address.clone(), slot.clone()))
            .cloned()
            .unwrap_or_else(|| self.base.storage(address, slot))
    }
}

impl<Base: ChainState> WriteableChainState for Fork<Base> {
    fn set_nonce(&mut self, address: &U256, nonce: usize) {
        let _previous = self.state.nonces.insert(address.clone(), nonce);
    }

    fn set_balance(&mut self, address: &U256, balance: &U256) {
        let _previous = self.state.balances.insert(address.clone(), balance.clone());
    }

    fn set_code(&mut self, address: &U256, code: &[u8]) {
        let _previous = self.state.codes.insert(address.clone(), code.to_vec());
    }

    fn set_storage(&mut self, address: &U256, slot: &U256, value: &U256) {
        let _previous = self
            .state
            .storages
            .insert((address.clone(), slot.clone()), value.clone());
    }
}
