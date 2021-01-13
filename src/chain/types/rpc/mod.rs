//! These types are mostly copied from the `web3` crate, but adjusted for our
//! needs.

mod account_range;
mod block_number;
mod bytes;
mod call;
mod genesis_config;
mod hex;
mod hex_full;
mod hex_mid;
mod hexable;
mod log;
mod log_filter;
mod storage_range;
mod transaction;
mod transaction_receipt;
mod value_or_array;

pub use self::{
    account_range::AccountRange,
    block_number::BlockNumber,
    bytes::Bytes,
    call::CallRequest,
    genesis_config::GenesisConfig,
    hex::Hex,
    hex_full::HexFull,
    hex_mid::HexMid,
    hexable::Hexable,
    log::{Log, LogBlock},
    log_filter::LogFilter,
    storage_range::{StorageRange, StorageSlot},
    transaction::Transaction,
    transaction_receipt::{TransactionReceipt, TransactionStatus},
    value_or_array::ValueOrArray,
};

#[cfg(test)]
mod test {
    use crate::{prelude::*, test::prelude::assert_eq};
    use serde_json::{from_value, json, to_value};

    #[test]
    fn test_u256_zero() {
        let obj = U256::zero();
        let json = to_value(&obj).unwrap();
        assert_eq!(
            &json,
            &json!("0x0000000000000000000000000000000000000000000000000000000000000000")
        );
        let de: U256 = from_value(json).unwrap();
        assert_eq!(de, obj);
    }
}
