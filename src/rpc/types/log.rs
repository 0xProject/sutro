use super::{Address, Hex};
use crate::prelude::*;

/// See <https://eth.wiki/json-rpc/API#eth_getfilterchanges>
#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub address: Address,
    pub topics:  Vec<U256>,
    pub data:    Vec<u8>,
    pub removed: bool,

    #[serde(flatten)]
    pub block: Option<LogBlock>,
}

#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogBlock {
    pub block_hash:        U256,
    pub block_number:      Hex<u64>,
    pub transaction_hash:  U256,
    pub transaction_index: Hex<u64>,
    pub log_index:         U256,
}
