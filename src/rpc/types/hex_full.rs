use super::Hexable;
use crate::prelude::*;
use serde::{de, ser};
use std::{fmt, marker::PhantomData};

/// Serialize number types as hex strings with prefix and all leading zeros.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
pub struct HexFull<T: Hexable>(T);

impl<T: Hexable> From<T> for HexFull<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Hexable> HexFull<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Hexable> AsRef<T> for HexFull<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T: Hexable> AsMut<T> for HexFull<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: Hexable> Serialize for HexFull<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        let hex_str = self.as_ref().to_hex();
        let nibbles = &hex_str[2..];
        let target = T::nibbles();
        dbg!(&target, nibbles.len());
        let padding = target - nibbles.len();
        let mut result = String::from("0x");
        result.extend(std::iter::repeat('0').take(padding));
        result.push_str(nibbles);
        serializer.serialize_str(&result)
    }
}

impl<'de, T: Hexable> Deserialize<'de> for HexFull<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor<T: Hexable>(PhantomData<T>);
        impl<'de, T: Hexable> de::Visitor<'de> for Visitor<T> {
            type Value = HexFull<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a hexadeximal number string")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let t = <T as Hexable>::from_hex(s)
                    .map_err(|_err| de::Error::invalid_value(de::Unexpected::Str(s), &self))?;
                Ok(HexFull(t))
            }
        }
        deserializer.deserialize_str(Visitor(PhantomData))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::prelude::assert_eq;
    use serde_json::{from_value, json, to_value};

    #[test]
    fn test_u64_zero() {
        let obj = HexFull(0_u64);
        let json = to_value(&obj).unwrap();
        assert_eq!(&json, &json!("0x0000000000000000"));
        let de: HexFull<u64> = from_value(json).unwrap();
        assert_eq!(de, obj);
    }

    #[test]
    fn test_u64() {
        let obj = HexFull(42_u64);
        let json = to_value(&obj).unwrap();
        assert_eq!(&json, &json!("0x000000000000002a"));
        let de: HexFull<u64> = from_value(json).unwrap();
        assert_eq!(de, obj);
    }

    #[test]
    fn test_u64_3_nibbles() {
        let obj = HexFull(300_u64);
        let json = to_value(&obj).unwrap();
        assert_eq!(&json, &json!("0x000000000000012c"));
        let de: HexFull<u64> = from_value(json).unwrap();
        assert_eq!(de, obj);
    }

    #[test]
    fn test_u256_zero() {
        let obj = HexFull(U256::zero());
        let json = to_value(&obj).unwrap();
        assert_eq!(
            &json,
            &json!("0x0000000000000000000000000000000000000000000000000000000000000000")
        );
        let de: HexFull<U256> = from_value(json).unwrap();
        assert_eq!(de, obj);
    }

    #[test]
    fn test_u256_3_nibbles() {
        let obj = HexFull(U256::from(300));
        let json = to_value(&obj).unwrap();
        assert_eq!(
            &json,
            &json!("0x000000000000000000000000000000000000000000000000000000000000012c")
        );
        let de: HexFull<U256> = from_value(json).unwrap();
        assert_eq!(de, obj);
    }
}
