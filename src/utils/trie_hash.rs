use crate::{prelude::*, serde::rlp::to_rlp};
use std::collections::HashMap;

pub trait TrieHash {
    fn trie_hash(self) -> U256;
}

impl<'a, T: 'a + Serialize, I: IntoIterator<Item = &'a T>> TrieHash for I {
    fn trie_hash(self) -> U256 {
        let mut map = HashMap::new();
        for (i, t) in self.into_iter().enumerate() {
            let key = to_rlp(&i).unwrap();
            let rlp = to_rlp(t).unwrap();
            map.insert(key, rlp);
        }
        let (root, _) = trie::build(&map);
        U256::from_bytes_be(&root.0)
    }
}
