use super::{Address, BloomFilter, Bytes, Hex, HexFull, Transaction};
use crate::prelude::*;

/// See <https://eth.wiki/json-rpc/API#eth_getblockbyhash>
#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockHeader {
    pub number:            Option<Hex<u64>>,
    pub hash:              Option<U256>,
    pub mix_hash:          Option<U256>,
    pub parent_hash:       U256,
    pub nonce:             Option<HexFull<u64>>,
    #[serde(rename = "sha3Uncles")]
    pub uncles_hash:       U256,
    pub uncles:            Vec<U256>,
    pub logs_bloom:        Option<BloomFilter>,
    pub transactions_root: U256,
    pub state_root:        U256,
    pub receipts_root:     U256,
    pub miner:             Address,
    pub difficulty:        Hex<u64>,
    pub total_difficulty:  Hex<u64>,
    pub extra_data:        Bytes, // Max 32 bytes.
    pub gas_limit:         Hex<u64>,
    pub gas_used:          Hex<u64>,
    pub timestamp:         Hex<u64>,
    pub transactions:      TransactionEntries,
    pub size:              Hex<u64>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransactionEntries {
    Hash(Vec<U256>),
    Full(Vec<Transaction>),
}

impl Default for TransactionEntries {
    fn default() -> Self {
        Self::Full(vec![])
    }
}
