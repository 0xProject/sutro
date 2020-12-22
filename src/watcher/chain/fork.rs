//! Forked chain
//!
//! Takes a given read-only chain implementation and implements and in-memory
//! change buffer on top of it. The new chain acts as a fork of the underlying
//! chain.

use super::{ChainState, WriteableChainState};
use crate::{evm::BlockInfo, prelude::*};
use std::collections::HashMap;

#[derive(Clone, Debug)]
struct Fork<Base: ChainState> {
    base:     Base,
    nonces:   HashMap<U256, usize>,
    balances: HashMap<U256, U256>,
    codes:    HashMap<U256, Vec<u8>>,
    storages: HashMap<(U256, U256), U256>,
}

impl<Base: ChainState> From<Base> for Fork<Base> {
    fn from(base: Base) -> Self {
        Self {
            base,
            nonces: HashMap::new(),
            balances: HashMap::new(),
            codes: HashMap::new(),
            storages: HashMap::new(),
        }
    }
}

impl<Base: ChainState> ChainState for Fork<Base> {
    fn block(&self) -> BlockInfo {
        self.base.block()
    }

    fn nonce(&self, address: &U256) -> usize {
        self.nonces
            .get(address)
            .cloned()
            .unwrap_or_else(|| self.base.nonce(address))
    }

    fn balance(&self, address: &U256) -> U256 {
        self.balances
            .get(address)
            .cloned()
            .unwrap_or_else(|| self.base.balance(address))
    }

    fn code(&self, address: &U256) -> Vec<u8> {
        self.codes
            .get(address)
            .cloned()
            .unwrap_or_else(|| self.base.code(address))
    }

    fn storage(&self, address: &U256, slot: &U256) -> U256 {
        self.storages
            .get(&(address.clone(), slot.clone()))
            .cloned()
            .unwrap_or_else(|| self.base.storage(address, slot))
    }
}

impl<Base: ChainState> WriteableChainState for Fork<Base> {
    fn set_nonce(&mut self, address: &U256, nonce: usize) {
        let _previous = self.nonces.insert(address.clone(), nonce);
    }

    fn set_balance(&mut self, address: &U256, balance: &U256) {
        let _previous = self.balances.insert(address.clone(), balance.clone());
    }

    fn set_code(&mut self, address: &U256, code: &[u8]) {
        let _previous = self.codes.insert(address.clone(), code.to_vec());
    }

    fn set_storage(&mut self, address: &U256, slot: &U256, value: &U256) {
        let _previous = self
            .storages
            .insert((address.clone(), slot.clone()), value.clone());
    }
}
