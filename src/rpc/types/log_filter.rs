use super::{Address, BlockNumber, ValueOrArray};
use crate::prelude::*;
use serde_with::skip_serializing_none;

/// Log Filter
#[skip_serializing_none]
#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogFilter {
    pub from_block: Option<BlockNumber>,
    pub to_block:   Option<BlockNumber>,
    pub block_hash: Option<U256>,
    // pub address:    Option<ValueOrArray<Address>>,
    // pub topics:     Option<Vec<Option<ValueOrArray<Address>>>>,
    pub limit:      Option<usize>,
}
