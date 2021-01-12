use super::Keccak256;
use crate::{prelude::*, serde::rlp::ser::Serializer};

pub trait RlpHash {
    fn rlp_hash(&self) -> U256;
}

impl<T: Serialize> RlpHash for T {
    fn rlp_hash(&self) -> U256 {
        let mut serializer = Serializer::new(Keccak256::new());
        self.serialize(&mut serializer)
            .expect("error in RLP serialization");
        let keccak = serializer.finish().expect("error in RLP serialization");
        keccak.finish()
    }
}
