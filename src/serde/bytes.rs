use crate::require;
use hex::encode_to_slice;
use serde::{de, ser};
use std::{cmp::min, fmt, iter::FromIterator, marker::PhantomData, str::from_utf8};

pub fn serialize<T, S>(bytes: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S: ser::Serializer,
{
    let bytes = bytes.as_ref();
    if serializer.is_human_readable() {
        let mut hex_utf8 = vec![0_u8; 2 + 2 * bytes.len()];
        hex_utf8[..2].copy_from_slice(b"0x");
        encode_to_slice(bytes, &mut hex_utf8[2..]).map_err(ser::Error::custom)?;
        let hex_str = from_utf8(&hex_utf8).map_err(ser::Error::custom)?;
        serializer.serialize_str(hex_str)
    } else {
        serializer.serialize_bytes(bytes)
    }
}

pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
where
    T: FromIterator<u8>,
    D: de::Deserializer<'de>,
{
    if deserializer.is_human_readable() {
        struct Visitor<T>(PhantomData<T>);
        impl<'de, T: FromIterator<u8>> de::Visitor<'de> for Visitor<T> {
            type Value = T;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("hexadecimal characters with optional prefix")
            }

            fn visit_str<E>(self, str: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let str = str.strip_prefix("0x").unwrap_or(str);
                let vec = hex::decode(&str).map_err(de::Error::custom)?;
                Ok(vec.into_iter().collect())
            }
        }
        deserializer.deserialize_bytes(Visitor(PhantomData))
    } else {
        struct Visitor<T>(PhantomData<T>);
        impl<'de, T: FromIterator<u8>> de::Visitor<'de> for Visitor<T> {
            type Value = T;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("bytes")
            }

            fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(value.iter().cloned().collect())
            }
        }
        deserializer.deserialize_bytes(Visitor(PhantomData))
    }
}
