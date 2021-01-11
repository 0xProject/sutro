use serde::{de, ser};
use std::{cmp::min, fmt};

pub fn serialize<S>(value: &u64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ser::Serializer,
{
    if serializer.is_human_readable() {
        let nibbles = {
            let mut result = [0_u8; 16];
            let bytes = value.to_be_bytes();
            hex::encode_to_slice(bytes, &mut result).map_err(ser::Error::custom)?;
            result
        };
        let zeros = value.leading_zeros() as usize / 4;
        let zeros = min(zeros, 15);
        let nibbles = &nibbles[zeros..];
        let hex_utf8 = {
            let mut hex_utf8 = [0_u8; 2 + 16];
            hex_utf8[..2].copy_from_slice(b"0x");
            hex_utf8[2..(18 - zeros)].copy_from_slice(nibbles);
            hex_utf8
        };
        let hex_utf8 = &hex_utf8[..(18 - zeros)];
        let hex_str = std::str::from_utf8(&hex_utf8).map_err(ser::Error::custom)?;
        serializer.serialize_str(hex_str)
    } else {
        let bytes = value.to_be_bytes();
        let zeros = value.leading_zeros() as usize / 8;
        let bytes = &bytes[zeros..];
        serializer.serialize_bytes(bytes)
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: de::Deserializer<'de>,
{
    if deserializer.is_human_readable() {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = u64;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("hexadecimal characters with optional prefix")
            }

            fn visit_str<E>(self, str: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let str = str.strip_prefix("0x").unwrap_or(str);
                u64::from_str_radix(str, 16).map_err(de::Error::custom)
            }
        }
        deserializer.deserialize_bytes(Visitor)
    } else {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = u64;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("bytes")
            }

            fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let mut result = [0; 8];
                result[(8 - value.len())..].copy_from_slice(value);
                Ok(u64::from_be_bytes(result))
            }
        }
        deserializer.deserialize_bytes(Visitor)
    }
}
