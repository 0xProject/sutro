mod fixed;
pub mod rlp;
pub mod short;

pub use self::{fixed::*, short::*};
pub mod fixed_u64 {
    use super::fixed8;
    use serde::{de, ser};

    pub fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        fixed8::serialize(&value.to_be_bytes(), serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        fixed8::deserialize(deserializer).map(u64::from_be_bytes)
    }
}

pub mod short_u64 {
    use super::short8;
    use serde::{de, ser};

    pub fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        short8::serialize(&value.to_be_bytes(), serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        short8::deserialize(deserializer).map(u64::from_be_bytes)
    }
}
