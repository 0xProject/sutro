use crate::{evm::precompiles::keccak256, prelude::*, serde::rlp::to_rlp};

pub trait RlpHash {
    fn rlp_hash(&self) -> U256;
}

impl<T: Serialize> RlpHash for T {
    fn rlp_hash(&self) -> U256 {
        let bytes =
            to_rlp(self).expect("type needs should be RLP serializable for rlp_hash() to work");
        keccak256(&bytes)
    }
}
