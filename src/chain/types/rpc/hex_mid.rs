use super::Hexable;
use crate::prelude::*;
use serde::{de, ser};
use std::{fmt, marker::PhantomData};

/// Serialize number types as hex strings with prefix and no leading zeros.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
pub struct HexMid(U256);

impl From<U256> for HexMid {
    fn from(value: U256) -> Self {
        Self(value)
    }
}

impl HexMid {
    pub fn into_inner(self) -> U256 {
        self.0
    }
}

impl AsRef<U256> for HexMid {
    fn as_ref(&self) -> &U256 {
        &self.0
    }
}

impl AsMut<U256> for HexMid {
    fn as_mut(&mut self) -> &mut U256 {
        &mut self.0
    }
}

impl Serialize for HexMid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let bytes = self.0.to_bytes_be();
        let zeros = bytes.iter().take_while(|b| **b == 0).count();
        let zeros = std::cmp::min(31, zeros);
        let bytes = &bytes[zeros..];
        serializer.serialize_str(&format!("0x{}", &hex::encode(bytes)))
    }
}

impl<'de> Deserialize<'de> for HexMid {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = HexMid;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a hexadecimal number string")
            }

            fn visit_str<E>(self, str: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let str = str.strip_prefix("0x").unwrap_or(str);
                let mut nibbles = [b'0'; 64];
                let zeros = nibbles.len() - str.len();
                nibbles[zeros..].copy_from_slice(str.as_bytes());
                let mut result = [0; 32];
                hex::decode_to_slice(&nibbles, &mut result).map_err(de::Error::custom)?;
                Ok(U256::from_bytes_be(&result).into())
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}
