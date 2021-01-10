/// Fixed length lower-case hex encoding with "0x" prefix
///
/// Falls back to bytes encoding in non-human-readable formats.
///
/// TODO: Version with checksums for [u8; 20] addresses.

macro_rules! fixed_length_serde {
    ($name:ident, $length:expr) => {
        pub mod $name {
            use serde::{de, ser};
            use std::fmt;

            pub fn serialize<S>(bytes: &[u8; $length], serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ser::Serializer,
            {
                if serializer.is_human_readable() {
                    let mut hex_utf8 = [0_u8; 2 + 2 * $length];
                    hex_utf8[..2].copy_from_slice(b"0x");
                    hex::encode_to_slice(bytes, &mut hex_utf8[2..]).map_err(ser::Error::custom)?;
                    let hex_str = std::str::from_utf8(&hex_utf8).map_err(ser::Error::custom)?;
                    serializer.serialize_str(hex_str)
                } else {
                    serializer.serialize_bytes(bytes)
                }
            }

            pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; $length], D::Error>
            where
                D: de::Deserializer<'de>,
            {
                if deserializer.is_human_readable() {
                    struct Visitor;
                    impl<'de> de::Visitor<'de> for Visitor {
                        type Value = [u8; $length];

                        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                            formatter.write_str("hexadecimal characters with optional prefix")
                        }

                        fn visit_str<E>(self, str: &str) -> Result<Self::Value, E>
                        where
                            E: de::Error,
                        {
                            let str = str.strip_prefix("0x").unwrap_or(str);
                            if str.len() != 2 * $length {
                                Err(de::Error::invalid_length(
                                    str.len(),
                                    &"expected hexadecimal digits",
                                ))
                            } else {
                                let mut result = [0_u8; $length];
                                hex::decode_to_slice(str, &mut result)
                                    .map_err(de::Error::custom)?;
                                Ok(result)
                            }
                        }
                    }
                    deserializer.deserialize_bytes(Visitor)
                } else {
                    struct Visitor;
                    impl<'de> de::Visitor<'de> for Visitor {
                        type Value = [u8; $length];

                        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                            formatter.write_str("bytes")
                        }

                        fn visit_bytes<E>(self, value: &[u8]) -> Result<Self::Value, E>
                        where
                            E: de::Error,
                        {
                            if value.len() != $length {
                                Err(de::Error::invalid_length(value.len(), &"expected bytes"))
                            } else {
                                let mut result = [0; $length];
                                result.copy_from_slice(value);
                                Ok(result)
                            }
                        }
                    }
                    deserializer.deserialize_bytes(Visitor)
                }
            }
        }
    };
}

fixed_length_serde!(fixed8, 8);
fixed_length_serde!(fixed20, 20);
fixed_length_serde!(fixed32, 32);
fixed_length_serde!(fixed256, 256);
