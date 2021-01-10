use crate::prelude::*;
use rlp::{Encodable, RlpStream};
use serde::{de, ser};
use std::{fmt, fmt::Debug};

#[derive(PartialEq, Clone)]
pub struct BloomFilter([u8; 256]);

impl Default for BloomFilter {
    fn default() -> Self {
        Self([0; 256])
    }
}

impl Debug for BloomFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BloomFilter(hex!(\"{}\"))", hex::encode(self.0))
    }
}

impl Encodable for BloomFilter {
    fn rlp_append(&self, s: &mut RlpStream) {
        let slice: &[u8] = &self.0;
        s.append(&slice);
    }
}

impl Serialize for BloomFilter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if serializer.is_human_readable() {
            // OPT: Avoid allocations
            serializer.serialize_str(&format!("0x{}", hex::encode(self.0)))
        } else {
            serializer.serialize_bytes(&self.0)
        }
    }
}

impl<'de> Deserialize<'de> for BloomFilter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = BloomFilter;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a hexadecimal bloom filter string")
            }

            fn visit_str<E>(self, str: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let str = str.strip_prefix("0x").unwrap_or(str);
                let mut buffer = [0_u8; 256];
                hex::decode_to_slice(str, &mut buffer).map_err::<E, _>(de::Error::custom)?;
                Ok(BloomFilter(buffer))
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}
