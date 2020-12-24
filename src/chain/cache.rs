//! Cached chain
//!
//! Takes a given read-only chain implementation and implements and in-memory
//! cache on top of it. Every read is cached and never discarded.

use super::{BlockInfo, ChainState, StateSet};
use crate::prelude::*;
use std::cell::{RefCell, RefMut};

/// # Panics
///
/// Panics if reads are called re-entrantly.
#[derive(Debug)]
pub struct Cache<Base: ChainState> {
    base:  Base,
    state: RefCell<StateSet>,
}

impl<Base: ChainState> Cache<Base> {
    fn state_set_mut(&self) -> RefMut<StateSet> {
        self.state
            .try_borrow_mut()
            .expect("Can not re-enter Cache.")
    }
}

impl<Base: ChainState> From<Base> for Cache<Base> {
    fn from(base: Base) -> Self {
        Self {
            base,
            state: RefCell::new(StateSet::default()),
        }
    }
}

impl<Base: ChainState> ChainState for Cache<Base> {
    fn block(&self) -> BlockInfo {
        self.state_set_mut()
            .block
            .get_or_insert_with(|| self.base.block())
            .clone()
    }

    fn nonce(&self, address: &U256) -> usize {
        *self
            .state_set_mut()
            .nonces
            .entry(address.clone())
            .or_insert_with(|| self.base.nonce(address))
    }

    fn balance(&self, address: &U256) -> U256 {
        self.state_set_mut()
            .balances
            .entry(address.clone())
            .or_insert_with(|| self.base.balance(address))
            .clone()
    }

    fn code(&self, address: &U256) -> Vec<u8> {
        self.state_set_mut()
            .codes
            .entry(address.clone())
            .or_insert_with(|| self.base.code(address))
            .clone()
    }

    fn storage(&self, address: &U256, slot: &U256) -> U256 {
        self.state_set_mut()
            .storages
            .entry((address.clone(), slot.clone()))
            .or_insert_with(|| self.base.storage(address, slot))
            .clone()
    }
}
