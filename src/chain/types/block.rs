use super::{BlockHeader, Transaction};
use crate::prelude::*;

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
