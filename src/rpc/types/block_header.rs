use super::{Address, BloomFilter, Bytes, Hex};
use crate::prelude::*;

/// See <https://eth.wiki/json-rpc/API#eth_getblockbyhash>
#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockHeader {
    pub number:            Option<Hex<u64>>,
    pub hash:              Option<U256>,
    pub parent_hash:       U256,
    pub nonce:             Option<Hex<u64>>, // TODO: Always 8 bytes
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

    // Short form
    pub transactions: Vec<U256>,
    /* Full form
     * pub transactions:      Vec<Transaction>, */
}
