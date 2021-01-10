use crate::{prelude::*, serde::fixed256};
use std::{fmt, fmt::Debug};

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Serialize, Deserialize)]
pub struct BloomFilter(#[serde(with = "fixed256")] [u8; 256]);

impl BloomFilter {
    pub fn empty() -> Self {
        Self([0; 256])
    }
}

impl From<[u8; 256]> for BloomFilter {
    fn from(value: [u8; 256]) -> Self {
        Self(value)
    }
}

impl Default for BloomFilter {
    fn default() -> Self {
        Self::empty()
    }
}

impl Debug for BloomFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BloomFilter::from(hex!(\"{}\"))", hex::encode(self.0))
    }
}
