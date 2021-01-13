use super::{BlockHeader, RpcTransaction, Transaction};
use crate::{
    prelude::*,
    serde::{fixed_u256, short_u64},
};

/// Block with transactions and ommers
///
/// See <https://ethereum.github.io/yellowpaper/paper.pdf>
/// See <https://eth.wiki/json-rpc/API#eth_getblockbyhash>
#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    pub header:       BlockHeader,
    pub transactions: Vec<Transaction>,
    pub ommers:       Vec<BlockHeader>,
}

/// Variant used for JSON-RPC for non-pending blocks.
///
/// The variant for Pending blocks which `null`s some of the [`BlockHeader`]
/// fields is not supported.
#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcBlock<T> {
    #[serde(flatten)]
    pub header:           BlockHeader,
    #[serde(with = "fixed_u256")]
    pub hash:             U256,
    #[serde(with = "short_u64")]
    pub total_difficulty: u64,
    #[serde(with = "short_u64")]
    pub size:             u64,
    pub transactions:     Vec<T>,
    #[serde(rename = "uncles")]
    pub ommers:           Vec<U256>,
}

pub type ConciseBlock = RpcBlock<U256>;
pub type FullBlock = RpcBlock<RpcTransaction>;
