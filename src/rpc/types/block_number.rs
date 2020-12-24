use crate::prelude::*;
use serde::{de, ser};
use std::fmt;

/// Block Number
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BlockNumber {
    /// Latest block
    Latest,
    /// Earliest block (genesis)
    Earliest,
    /// Pending block (not yet part of the blockchain)
    Pending,
    /// Block by number from canon chain
    Number(u64),
}

impl From<u64> for BlockNumber {
    fn from(number: u64) -> Self {
        Self::Number(number)
    }
}

impl Default for BlockNumber {
    fn default() -> Self {
        Self::Latest
    }
}

impl Serialize for BlockNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        match *self {
            Self::Number(ref x) => serializer.serialize_str(&format!("0x{:x}", x)),
            Self::Latest => serializer.serialize_str("latest"),
            Self::Earliest => serializer.serialize_str("earliest"),
            Self::Pending => serializer.serialize_str("pending"),
        }
    }
}

impl<'de> Deserialize<'de> for BlockNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = BlockNumber;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    formatter,
                    "a hexadeximal number string or one of \"latest\", \"earliest\" or \"pending\""
                )
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(match s {
                    "latest" => BlockNumber::Latest,
                    "earliest" => BlockNumber::Earliest,
                    "pending" => BlockNumber::Pending,
                    str => {
                        let str = str.strip_prefix("0x").unwrap_or(str);
                        let number = u64::from_str_radix(str, 16).map_err(|_err| {
                            de::Error::invalid_value(de::Unexpected::Str(s), &self)
                        })?;
                        BlockNumber::Number(number)
                    }
                })
                // Err()
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}
