use super::BlockInfo;
use crate::prelude::*;
use std::collections::HashMap;

/// A subset of chain state
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct StateSet {
    pub block:    Option<BlockInfo>,
    pub nonces:   HashMap<U256, usize>,
    pub balances: HashMap<U256, U256>,
    pub codes:    HashMap<U256, Vec<u8>>,
    pub storages: HashMap<(U256, U256), U256>,
}
