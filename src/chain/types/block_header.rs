use super::{Address, BloomFilter, Nonce, Number};
use crate::{
    evm::precompiles::keccak256,
    prelude::*,
    rpc::types::{Bytes, Hex, HexFull},
};
use rlp::Encodable;

/// Constant for the current block
/// See <https://ethereum.github.io/yellowpaper/paper.pdf>
/// See <https://eth.wiki/json-rpc/API#eth_getblockbyhash>
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockHeader {
    pub parent_hash:       U256,
    #[serde(rename = "sha3Uncles")]
    pub ommers_hash:       U256,
    #[serde(rename = "miner")]
    pub beneficiary:       Address,
    pub state_root:        U256,
    pub transactions_root: U256,
    pub receipts_root:     U256,
    pub logs_bloom:        BloomFilter,
    pub difficulty:        Number,
    pub number:            Number,
    pub gas_limit:         Number,
    pub gas_used:          Number,
    pub timestamp:         Number,
    pub extra_data:        Bytes, // 32 bytes or less.
    pub mix_hash:          U256,
    pub nonce:             Nonce,
}

trait RlpStreamExt {
    fn append_u256(&mut self, u256: &U256) -> &mut Self;
}

impl RlpStreamExt for rlp::RlpStream {
    fn append_u256(&mut self, u256: &U256) -> &mut Self {
        let bytes = u256.to_bytes_be();
        let slice: &[u8] = &bytes;
        self.append(&slice);
        self
    }
}

trait EncodableExt {
    fn rlp_hash(&self) -> U256;
}

impl<T: Encodable> EncodableExt for T {
    fn rlp_hash(&self) -> U256 {
        let bytes = self.rlp_bytes();
        keccak256(&bytes)
    }
}

impl Encodable for BlockHeader {
    fn rlp_append(&self, s: &mut rlp::RlpStream) {
        s.begin_unbounded_list();
        s.append_u256(&self.parent_hash);
        s.append_u256(&self.ommers_hash);
        s.append(&self.beneficiary);
        s.append_u256(&self.state_root);
        s.append_u256(&self.transactions_root);
        s.append_u256(&self.receipts_root);
        s.append(&self.logs_bloom);
        s.append(self.difficulty.as_ref());
        s.append(self.number.as_ref());
        s.append(self.gas_limit.as_ref());
        s.append(self.gas_used.as_ref());
        s.append(self.timestamp.as_ref());
        s.append(&self.extra_data);
        s.append_u256(&self.mix_hash);
        let bytes = self.nonce.as_ref().to_be_bytes();
        let slice: &[u8] = &bytes;
        s.append(&slice);
        s.finalize_unbounded_list();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::prelude::assert_eq;
    use serde_json::{from_value, json};

    #[test]
    fn genesis_block() {
        // curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["0x0",false],"id":67}' http://localhost:8545
        let mut header: BlockHeader = from_value(json!({
            "difficulty":"0x400000000","extraData":"0x11bbe8db4e347b4e8c937c1c8370e4b5ed33adb3db69cbdb7a38e1e50b1b82fa","gasLimit":"0x1388",
            "gasUsed":"0x0","hash":"0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3","logsBloom":"0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000","miner":"0x0000000000000000000000000000000000000000","mixHash":"0x0000000000000000000000000000000000000000000000000000000000000000","nonce":"0x0000000000000042",
            "number":"0x0","parentHash":"0x0000000000000000000000000000000000000000000000000000000000000000","receiptsRoot":"0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421","sha3Uncles":"0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","size":"0x21c","stateRoot":"0xd7f8974fb5ac78d9ac099b9ad5018bedc2ce0a72dad1827a1709da30580f0544","timestamp":"0x0",
            "totalDifficulty":"0x400000000",
            "transactions":[],"transactionsRoot":"0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421","uncles":[]
        }))
        .unwrap();
        // curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"debug_getBlockRlp","params":[0],"id":67}' http://localhost:8545
        // (cut out the header part)
        assert_eq!(
            hex::encode(header.rlp_bytes()),
            "f90214a00000000000000000000000000000000000000000000000000000000000000000a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347940000000000000000000000000000000000000000a0d7f8974fb5ac78d9ac099b9ad5018bedc2ce0a72dad1827a1709da30580f0544a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421b9010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000850400000000808213888080a011bbe8db4e347b4e8c937c1c8370e4b5ed33adb3db69cbdb7a38e1e50b1b82faa00000000000000000000000000000000000000000000000000000000000000000880000000000000042"
        );
        let rlp = super::super::serde_rlp::to_rlp(&header).unwrap();
        assert_eq!(
            hex::encode(&rlp),
            "f90214a00000000000000000000000000000000000000000000000000000000000000000a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347940000000000000000000000000000000000000000a0d7f8974fb5ac78d9ac099b9ad5018bedc2ce0a72dad1827a1709da30580f0544a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421b9010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000850400000000808213888080a011bbe8db4e347b4e8c937c1c8370e4b5ed33adb3db69cbdb7a38e1e50b1b82faa00000000000000000000000000000000000000000000000000000000000000000880000000000000042"
        );
        assert_eq!(
            header.rlp_hash(),
            u256h!("d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3")
        );
    }
}
