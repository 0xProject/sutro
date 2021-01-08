use super::{Address, Bytes, Hex};
use crate::prelude::*;
use std::collections::HashMap;

// See <https://github.com/ethereum/retesteth/wiki/RPC-Methods#test_setchainparams>
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenesisConfig {
    pub accounts:    HashMap<String, GenesisAccount>,
    pub genesis:     GenesisBlock,
    pub seal_engine: SealEngine,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenesisAccount {
    pub balance: U256,
    pub code:    Bytes,
    pub nonce:   Hex<u64>,
    pub storage: HashMap<U256, U256>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenesisBlock {
    pub author:     Address,
    pub difficulty: Hex<u64>,
    pub extra_data: Bytes, // Max 32 bytes.
    pub gas_limit:  Hex<u64>,
    pub mix_hash:   U256,
    pub nonce:      Hex<u64>, // TODO: Always 8 bytes
    pub timestamp:  Hex<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum SealEngine {
    NoProof,
}

impl Default for SealEngine {
    fn default() -> Self {
        Self::NoProof
    }
}
