use super::HexMid;
use crate::prelude::*;
use std::collections::HashMap;

/// See <https://github.com/ethereum/retesteth/wiki/RPC-Methods#debug_accountrange>
#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageRange {
    pub storage:  HashMap<U256, StorageSlot>,
    pub complete: bool,
}

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StorageSlot {
    pub key:   HexMid,
    pub value: U256,
}
