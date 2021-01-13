use crate::{prelude::*, serde::fixed20};
use std::{fmt, fmt::Debug};

/// Ethereum address
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct Address(#[serde(with = "fixed20")] [u8; 20]);

impl From<[u8; 20]> for Address {
    fn from(value: [u8; 20]) -> Self {
        Self(value)
    }
}

impl Address {
    pub fn to_array(self) -> [u8; 20] {
        self.0
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn as_slice_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl AsRef<[u8; 20]> for Address {
    fn as_ref(&self) -> &[u8; 20] {
        &self.0
    }
}

impl AsMut<[u8; 20]> for Address {
    fn as_mut(&mut self) -> &mut [u8; 20] {
        &mut self.0
    }
}

impl AsRef<[u8]> for Address {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for Address {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Address::from(hex!(\"{}\"))", hex::encode(self.0))
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
