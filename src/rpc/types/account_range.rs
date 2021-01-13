use super::{Address, BloomFilter, Bytes, Hex, HexFull, Transaction};
use crate::prelude::*;
use std::collections::HashMap;

/// See <https://github.com/ethereum/retesteth/wiki/RPC-Methods#debug_accountrange>
#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountRange {
    pub address_map: HashMap<U256, Address>,
    pub next_key:    U256,
}
