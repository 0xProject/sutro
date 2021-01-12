use super::Address;
use crate::{
    prelude::*,
    rpc::types::Bytes,
    serde::{fixed_u64, short},
};

#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    #[serde(with = "short")]
    nonce:     u64,
    #[serde(with = "short")]
    gas_price: u64,
    #[serde(with = "short")]
    gas_limit: u64,
    to:        Address,
    value:     U256,
    data:      Bytes,
    #[serde(with = "short")]
    v:         u64, // TODO u8
    r:         U256,
    s:         U256,
}
