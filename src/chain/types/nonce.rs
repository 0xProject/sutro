use crate::{prelude::*, serde::fixed8};
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
        fixed8::serialize(&self.0.to_be_bytes(), serializer)
    }
}

impl<'de> Deserialize<'de> for Nonce {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        fixed8::deserialize(deserializer)
            .map(u64::from_be_bytes)
            .map(Nonce::from)
    }
}
