use super::{Address, Bytes, Hex};
use crate::prelude::*;

/// See <https://eth.wiki/json-rpc/API#eth_gettransactionbyhash>
#[derive(Default, Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub block_hash:        Option<U256>,
    pub block_number:      Option<Hex<u64>>,
    pub from:              Address,
    pub gas:               Hex<u64>,
    pub gas_price:         Hex<U256>,
    pub hash:              U256,
    pub input:             Bytes,
    pub nonce:             Hex<u64>,
    pub to:                Option<Address>,
    pub transaction_index: Option<Hex<u64>>,
    pub value:             Hex<U256>,
    pub v:                 Hex<u64>,
    pub r:                 U256,
    pub s:                 U256,
}
