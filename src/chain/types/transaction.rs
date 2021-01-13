use super::Address;
use crate::{
    prelude::*,
    serde::{bytes, fixed_u256, short_u256, short_u64},
};

#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    #[serde(with = "short_u64")]
    nonce:     u64,
    #[serde(with = "short_u64")]
    gas_price: u64,
    #[serde(with = "short_u64", rename = "gas")]
    gas_limit: u64,
    to:        Address, // To do: encode as null for contract creation
    #[serde(with = "short_u256")]
    value:     U256,
    #[serde(rename = "input", with = "bytes")]
    data:      Vec<u8>,
    #[serde(with = "short_u64")]
    v:         u64, // TODO u8
    #[serde(with = "fixed_u256")]
    r:         U256,
    #[serde(with = "fixed_u256")]
    s:         U256,
}

#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransaction {
    #[serde(flatten)]
    pub transaction: Transaction,

    #[serde(with = "fixed_u256")]
    pub block_hash:        U256,
    #[serde(with = "short_u64")]
    pub block_number:      u64,
    #[serde(with = "short_u64")]
    pub transaction_index: u64,
    pub from:              Address,
    #[serde(with = "fixed_u256")]
    pub hash:              U256,
}
