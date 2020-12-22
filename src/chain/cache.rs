//! Cached chain
//!
//! Takes a given read-only chain implementation and implements and in-memory
//! cache on top of it. Every read is cached and never discarded.

use super::ChainState;
use crate::{evm::BlockInfo, prelude::*};
use std::{cell::RefCell, collections::HashMap};

/// # Panics
///
/// Panics if reads are called re-entrantly.
#[derive(Debug)]
pub struct Cache<Base: ChainState> {
    base:     Base,
    block:    RefCell<Option<BlockInfo>>,
    nonces:   RefCell<HashMap<U256, usize>>,
    balances: RefCell<HashMap<U256, U256>>,
    codes:    RefCell<HashMap<U256, Vec<u8>>>,
    storages: RefCell<HashMap<(U256, U256), U256>>,
}

impl<Base: ChainState> From<Base> for Cache<Base> {
    fn from(base: Base) -> Self {
        Self {
            base,
            block: RefCell::new(None),
            nonces: RefCell::new(HashMap::new()),
            balances: RefCell::new(HashMap::new()),
            codes: RefCell::new(HashMap::new()),
            storages: RefCell::new(HashMap::new()),
        }
    }
}

impl<Base: ChainState> ChainState for Cache<Base> {
    fn block(&self) -> BlockInfo {
        self.block
            .try_borrow_mut()
            .expect("Can not re-enter Cache.")
            .get_or_insert_with(|| self.base.block())
            .clone()
    }

    fn nonce(&self, address: &U256) -> usize {
        self.nonces
            .try_borrow_mut()
            .expect("Can not re-enter Cache.")
            .entry(address.clone())
            .or_insert_with(|| self.base.nonce(address))
            .clone()
    }

    fn balance(&self, address: &U256) -> U256 {
        self.balances
            .try_borrow_mut()
            .expect("Can not re-enter Cache.")
            .entry(address.clone())
            .or_insert_with(|| self.base.balance(address))
            .clone()
    }

    fn code(&self, address: &U256) -> Vec<u8> {
        self.codes
            .try_borrow_mut()
            .expect("Can not re-enter Cache.")
            .entry(address.clone())
            .or_insert_with(|| self.base.code(address))
            .clone()
    }

    fn storage(&self, address: &U256, slot: &U256) -> U256 {
        self.storages
            .try_borrow_mut()
            .expect("Can not re-enter Cache.")
            .entry((address.clone(), slot.clone()))
            .or_insert_with(|| self.base.storage(address, slot))
            .clone()
    }
}
