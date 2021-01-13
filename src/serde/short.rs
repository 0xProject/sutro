/// Implements Ethereum JSON-RPC quantity encoding.
///
/// * '0x' prefix
/// * Minimal number of nibbles, potentially odd, but at leas one.
/// * No leading zeros except in 0x0.
///
/// See <https://eth.wiki/json-rpc/API#hex-value-encoding>

macro_rules! short_length_serde {
    ($name:ident, $length:expr) => {
        pub mod $name {
            use crate::require;
            use serde::{de, ser};
            use std::{cmp::min, fmt};

            pub fn serialize<S>(bytes: &[u8; $length], serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ser::Serializer,
            {
                if serializer.is_human_readable() {
                    const BUFSIZE: usize = 2 + 2 * $length;
                    let nibbles = {
                        let mut result = [0_u8; BUFSIZE - 2];
                        hex::encode_to_slice(bytes, &mut result).map_err(ser::Error::custom)?;
                        result
                    };
                    let zeros = nibbles.iter().take_while(|b| **b == b'0').count();
                    let zeros = min(zeros, BUFSIZE - 3);
                    let nibbles = &nibbles[zeros..];
                    let hex_utf8 = {
                        let mut hex_utf8 = [0_u8; BUFSIZE];
                        hex_utf8[..2].copy_from_slice(b"0x");
                        hex_utf8[2..(BUFSIZE - zeros)].copy_from_slice(nibbles);
                        hex_utf8
                    };
                    let hex_utf8 = &hex_utf8[..(BUFSIZE - zeros)];
                    let hex_str = std::str::from_utf8(&hex_utf8).map_err(ser::Error::custom)?;
                    serializer.serialize_str(hex_str)
                } else {
                    let zeros = bytes.iter().take_while(|b| **b == 0).count();
                    let bytes = &bytes[zeros..];
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
                            let mut nibbles = [b'0'; 2 * $length];
                            let zeros = nibbles.len() - str.len();
                            nibbles[zeros..].copy_from_slice(str.as_bytes());
                            let mut result = [0; $length];
                            hex::decode_to_slice(&nibbles, &mut result)
                                .map_err(de::Error::custom)?;
                            Ok(result)
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
                            require!(
                                value.len() <= $length,
                                E::invalid_length(value.len(), &"too many bytes for target type")
                            );
                            let mut result = [0; $length];
                            result[($length - value.len())..].copy_from_slice(value);
                            Ok(result)
                        }
                    }
                    deserializer.deserialize_bytes(Visitor)
                }
            }
        }
    };
}

short_length_serde!(short8, 8);
short_length_serde!(short32, 32);
