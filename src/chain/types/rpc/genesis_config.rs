use super::super::{Address, BlockHeader};
use crate::{
    prelude::*,
    serde::{bytes, short_u64},
};
use arrayvec::ArrayVec;
use std::collections::HashMap;

// See <https://github.com/ethereum/retesteth/wiki/RPC-Methods#test_setchainparams>
#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenesisConfig {
    pub accounts:    HashMap<Address, GenesisAccount>,
    pub genesis:     BlockHeader,
    pub seal_engine: SealEngine,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenesisAccount {
    pub balance: U256,
    #[serde(with = "bytes")]
    pub code:    Vec<u8>,
    #[serde(with = "short_u64")]
    pub nonce:   u64,
    pub storage: HashMap<U256, U256>,
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
