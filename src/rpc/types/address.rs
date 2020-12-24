use crate::prelude::*;
use serde::{de, ser};
use std::{fmt, fmt::Debug};

/// Ethereum addresses with Serialization to 0x prefixed hex string.
///
/// # To do
///
/// * Add check sum
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Address([u8; 20]);

impl Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address(hex!(\"{}\"))", hex::encode(self.0))
    }
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        // TODO: Add checksum
        // OPT: Avoid allocations
        serializer.serialize_str(&format!("0x{}", hex::encode(self.0)))
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Address;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a hexadecimal address string")
            }

            fn visit_str<E>(self, str: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let str = str.strip_prefix("0x").unwrap_or(str);
                let mut buffer = [0_u8; 20];
                hex::decode_to_slice(str, &mut buffer).map_err::<E, _>(de::Error::custom)?;
                Ok(Address(buffer))
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::prelude::assert_eq;
    use serde_json::{from_value, json, to_value};

    #[test]
    fn test_serialize_default() {
        let obj = Address::default();
        let json = to_value(&obj).unwrap();
        assert_eq!(&json, &json!("0x0000000000000000000000000000000000000000"));
        let de: Address = from_value(json).unwrap();
        assert_eq!(de, obj);
    }
}
