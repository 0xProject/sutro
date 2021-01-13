use super::Hexable;
use crate::prelude::*;
use serde::{de, ser};
use std::{fmt, marker::PhantomData};

/// Serialize number types as hex strings with prefix and no leading zeros.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
pub struct Hex<T: Hexable>(T);

impl<T: Hexable> From<T> for Hex<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

impl<T: Hexable> Hex<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

impl<T: Hexable> AsRef<T> for Hex<T> {
    fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T: Hexable> AsMut<T> for Hex<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

impl<T: Hexable> Serialize for Hex<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(&self.as_ref().to_hex())
    }
}

impl<'de, T: Hexable> Deserialize<'de> for Hex<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor<T: Hexable>(PhantomData<T>);
        impl<'de, T: Hexable> de::Visitor<'de> for Visitor<T> {
            type Value = Hex<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                write!(formatter, "a hexadeximal number string")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                let t = <T as Hexable>::from_hex(s)
                    .map_err(|_err| de::Error::invalid_value(de::Unexpected::Str(s), &self))?;
                Ok(Hex(t))
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
        let obj = Hex(0_u64);
        let json = to_value(&obj).unwrap();
        assert_eq!(&json, &json!("0x0"));
        let de: Hex<u64> = from_value(json).unwrap();
        assert_eq!(de, obj);
    }

    #[test]
    fn test_u64() {
        let obj = Hex(42_u64);
        let json = to_value(&obj).unwrap();
        assert_eq!(&json, &json!("0x2a"));
        let de: Hex<u64> = from_value(json).unwrap();
        assert_eq!(de, obj);
    }

    #[test]
    fn test_u64_3_nibbles() {
        let obj = Hex(300_u64);
        let json = to_value(&obj).unwrap();
        assert_eq!(&json, &json!("0x12c"));
        let de: Hex<u64> = from_value(json).unwrap();
        assert_eq!(de, obj);
    }

    #[test]
    fn test_u256_zero() {
        let obj = Hex(U256::zero());
        let json = to_value(&obj).unwrap();
        assert_eq!(&json, &json!("0x0"));
        let de: Hex<U256> = from_value(json).unwrap();
        assert_eq!(de, obj);
    }

    #[test]
    fn test_u256_3_nibbles() {
        let obj = Hex(U256::from(300));
        let json = to_value(&obj).unwrap();
        assert_eq!(&json, &json!("0x12c"));
        let de: Hex<U256> = from_value(json).unwrap();
        assert_eq!(de, obj);
    }
}
