//! These types are mostly copied from the `web3` crate, but adjusted for our
//! needs.

mod address;
mod block_header;
mod block_number;
mod bloom_filter;
mod bytes;
mod call;
mod hex;
mod log;
mod log_filter;
mod transaction_receipt;
mod value_or_array;

use crate::prelude::*;

pub use self::{
    address::Address,
    block_header::BlockHeader,
    block_number::BlockNumber,
    bloom_filter::BloomFilter,
    bytes::Bytes,
    call::CallRequest,
    hex::Hex,
    log::{Log, LogBlock},
    log_filter::LogFilter,
    transaction_receipt::{TransactionReceipt, TransactionStatus},
    value_or_array::ValueOrArray,
};

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::prelude::{assert_eq, *};
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
