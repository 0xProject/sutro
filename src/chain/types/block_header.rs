use super::{Address, BloomFilter, Nonce, Number, RlpHash};
use crate::{prelude::*, rpc::types::Bytes};

/// Constant for the current block
/// See <https://ethereum.github.io/yellowpaper/paper.pdf>
/// See <https://eth.wiki/json-rpc/API#eth_getblockbyhash>
#[derive(Clone, Default, PartialEq, PartialOrd, Eq, Ord, Debug, Serialize, Deserialize)]
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

#[cfg(test)]
mod test {
    use super::{super::rlp_hash, *};
    use crate::{
        serde::rlp::{from_rlp, to_rlp},
        test::prelude::assert_eq,
    };
    use serde_json::{from_value, json, to_value, Value as JsonValue};

    fn genesis_header() -> BlockHeader {
        BlockHeader {
            parent_hash:       u256h!(
                "0000000000000000000000000000000000000000000000000000000000000000"
            ),
            ommers_hash:       u256h!(
                "1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347"
            ),
            beneficiary:       Address::from(hex!("0000000000000000000000000000000000000000")),
            state_root:        u256h!(
                "d7f8974fb5ac78d9ac099b9ad5018bedc2ce0a72dad1827a1709da30580f0544"
            ),
            transactions_root: u256h!(
                "56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"
            ),
            receipts_root:     u256h!(
                "56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421"
            ),
            logs_bloom:        BloomFilter::empty(),
            difficulty:        17179869184.into(),
            number:            0.into(),
            gas_limit:         5000.into(),
            gas_used:          0.into(),
            timestamp:         0.into(),
            extra_data:        Bytes::from(
                hex!("11bbe8db4e347b4e8c937c1c8370e4b5ed33adb3db69cbdb7a38e1e50b1b82fa").to_vec(),
            ),
            mix_hash:          u256h!(
                "0000000000000000000000000000000000000000000000000000000000000000"
            ),
            nonce:             66.into(),
        }
    }

    fn genesis_json() -> JsonValue {
        // curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"eth_getBlockByNumber","params":["0x0",false],"id":67}' http://localhost:8545
        json!({
            "difficulty":"0x400000000","extraData":"0x11bbe8db4e347b4e8c937c1c8370e4b5ed33adb3db69cbdb7a38e1e50b1b82fa","gasLimit":"0x1388",
            "gasUsed":"0x0","hash":"0xd4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3","logsBloom":"0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000","miner":"0x0000000000000000000000000000000000000000","mixHash":"0x0000000000000000000000000000000000000000000000000000000000000000","nonce":"0x0000000000000042",
            "number":"0x0","parentHash":"0x0000000000000000000000000000000000000000000000000000000000000000","receiptsRoot":"0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421","sha3Uncles":"0x1dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347","size":"0x21c","stateRoot":"0xd7f8974fb5ac78d9ac099b9ad5018bedc2ce0a72dad1827a1709da30580f0544","timestamp":"0x0",
            "totalDifficulty":"0x400000000",
            "transactions":[],"transactionsRoot":"0x56e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421","uncles":[]
        })
    }

    fn genesis_rlp() -> Vec<u8> {
        // curl -X POST -H "Content-Type: application/json" --data '{"jsonrpc":"2.0","method":"debug_getBlockRlp","params":[0],"id":67}' http://localhost:8545
        // (cut out the header part)
        hex!( "f90214a00000000000000000000000000000000000000000000000000000000000000000a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347940000000000000000000000000000000000000000a0d7f8974fb5ac78d9ac099b9ad5018bedc2ce0a72dad1827a1709da30580f0544a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421b9010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000850400000000808213888080a011bbe8db4e347b4e8c937c1c8370e4b5ed33adb3db69cbdb7a38e1e50b1b82faa00000000000000000000000000000000000000000000000000000000000000000880000000000000042").to_vec()
    }

    #[test]
    fn genesis_hash() {
        assert_eq!(
            genesis_header().rlp_hash(),
            u256h!("d4e56740f876aef8c010b86a40d5f56745a118d0906a34e69aec8c0db1cb8fa3")
        );
    }

    #[test]
    fn genesis_json_encode() {
        assert_eq!(to_value(&genesis_header()).unwrap(), genesis_json());
    }

    #[test]
    fn genesis_json_decode() {
        assert_eq!(
            from_value::<BlockHeader>(genesis_json()).unwrap(),
            genesis_header()
        );
    }

    #[test]
    fn genesis_rlp_encode() {
        assert_eq!(
            hex::encode(to_rlp(&genesis_header()).unwrap()),
            hex::encode(genesis_rlp())
        );
    }

    #[test]
    fn genesis_rlp_decode() {
        assert_eq!(
            from_rlp::<BlockHeader>(&genesis_rlp()).unwrap(),
            genesis_header()
        );
    }
}
