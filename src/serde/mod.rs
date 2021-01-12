mod fixed;
pub mod rlp;
pub mod short;

pub use self::{fixed::*, short::*};
use crate::prelude::*;

macro_rules! uint {
    ($name:ident, $base:ident, $type:ident, $to:expr, $from:expr) => {
        pub mod $name {
            use super::*;
            use serde::{de, ser};

            pub fn serialize<S>(value: &$type, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ser::Serializer,
            {
                $base::serialize(&$to(value), serializer)
            }

            pub fn deserialize<'de, D>(deserializer: D) -> Result<$type, D::Error>
            where
                D: de::Deserializer<'de>,
            {
                $base::deserialize(deserializer).map($from)
            }
        }
    };
}

uint!(fixed_u64, fixed8, u64, u64_bytes, u64::from_be_bytes);
uint!(short_u64, short8, u64, u64_bytes, u64::from_be_bytes);
uint!(fixed_u256, fixed32, U256, U256::to_bytes_be, bytes_u256);
uint!(short_u256, short32, U256, U256::to_bytes_be, bytes_u256);

fn u64_bytes(n: &u64) -> [u8; 8] {
    n.to_be_bytes()
}

fn bytes_u256(bytes: [u8; 32]) -> U256 {
    U256::from_bytes_be(&bytes)
}
