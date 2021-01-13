use super::{
    super::{Address, BloomFilter},
    Hex, Log,
};
use crate::prelude::*;

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionReceipt {
    pub transaction_hash:    U256,
    pub transaction_index:   Hex<u64>,
    pub block_hash:          Option<U256>,
    pub block_number:        Option<Hex<u64>>,
    pub from:                Address,
    pub to:                  Option<Address>,
    pub cumulative_gas_used: Hex<u64>,
    pub gas_used:            Hex<u64>,
    pub contract_address:    Option<Address>,
    pub logs:                Vec<Log>,
    pub logs_bloom:          BloomFilter,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TransactionStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<U256>,
}

// TODO: This does not accept leading zeros.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionStatus {
    #[serde(rename = "0x1")]
    Success,
    #[serde(rename = "0x0")]
    Failure,
}

impl Default for TransactionStatus {
    fn default() -> Self {
        Self::Failure
    }
}
