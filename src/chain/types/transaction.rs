use super::Address;
use crate::{
    prelude::*,
    rpc::types::Bytes,
    serde::{fixed_u64, short_u64},
};

#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    #[serde(with = "short_u64")]
    nonce:     u64,
    #[serde(with = "short_u64")]
    gas_price: u64,
    #[serde(with = "short_u64")]
    gas_limit: u64,
    to:        Address,
    // #[serde(with = "short_u256")]
    value:     U256,
    data:      Bytes,
    #[serde(with = "short_u64")]
    v:         u64, // TODO u8
    r:         U256,
    s:         U256,
}
