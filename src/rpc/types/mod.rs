//! These types are mostly copied from the `web3` crate, but adjusted for our
//! needs.

mod address;
mod block_header;
mod block_number;
mod bloom_filter;
mod bytes;
mod call;
mod genesis_config;
mod hex;
mod hex_full;
mod hexable;
mod log;
mod log_filter;
mod transaction;
mod transaction_receipt;
mod value_or_array;

pub use self::{
    address::Address,
    block_header::{BlockHeader, TransactionEntries},
    block_number::BlockNumber,
    bloom_filter::BloomFilter,
    bytes::Bytes,
    call::CallRequest,
    genesis_config::GenesisConfig,
    hex::Hex,
    hex_full::HexFull,
    hexable::Hexable,
    log::{Log, LogBlock},
    log_filter::LogFilter,
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
