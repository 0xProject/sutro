use crate::prelude::*;
use rlp::{Encodable, RlpStream};
use serde::{de, ser};
use std::{fmt, fmt::Debug};

/// 64 bit nonce, always encoded as eight bytes
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
pub struct Nonce(u64);

impl From<u64> for Nonce {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl Nonce {
    pub fn to_u64(self) -> u64 {
        self.0
    }
}

impl AsRef<u64> for Nonce {
    fn as_ref(&self) -> &u64 {
        &self.0
    }
}

impl AsMut<u64> for Nonce {
    fn as_mut(&mut self) -> &mut u64 {
        &mut self.0
    }
}

impl Serialize for Nonce {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if serializer.is_human_readable() {
            serializer.serialize_str(&format!("{:#016x}", self.0))
        } else {
            serializer.serialize_bytes(&self.0.to_be_bytes())
        }
    }
}

impl<'de> Deserialize<'de> for Nonce {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            struct Visitor;
            impl<'de> de::Visitor<'de> for Visitor {
                type Value = Nonce;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    write!(formatter, "a hexadecimal number string")
                }

                fn visit_str<E>(self, str: &str) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    // Silently accept hex without prefix
                    let str = str.strip_prefix("0x").unwrap_or(str);
                    let n = u64::from_str_radix(str, 16).map_err(E::custom)?;
                    Ok(Nonce(n))
                }
            }
            deserializer.deserialize_str(Visitor)
        } else {
            struct Visitor;
            impl<'de> de::Visitor<'de> for Visitor {
                type Value = Nonce;

                fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                    write!(formatter, "one to eight bytes")
                }

                fn visit_bytes<E>(self, b: &[u8]) -> Result<Self::Value, E>
                where
                    E: de::Error,
                {
                    // Silently accept b.len() == 0
                    if b.len() > 8 {
                        return Err(E::custom("number too large"));
                    }
                    let mut bytes = [0_u8; 8];
                    let zeros = 8 - b.len();
                    bytes[zeros..].copy_from_slice(b);
                    Ok(Nonce(u64::from_be_bytes(bytes)))
                }
            }
            deserializer.deserialize_bytes(Visitor)
        }
    }
}
