use super::ChainState;
use web3::{
    block_on,
    transports::Http,
    types::{BlockNumber, H160, H256, U256 as W256},
    Error, Web3,
};
use zkp_u256::U256;

pub struct EthJsonRpc {
    connection: Web3<Http>,
    block:      Option<BlockNumber>,
}

impl EthJsonRpc {
    pub async fn new() -> Result<Self, Error> {
        let transport = web3::transports::Http::new("http://localhost:8555")?;
        let connection = web3::Web3::new(transport);

        let latest = connection.eth().block_number().await?.as_u64();
        println!("Latest block: {}", latest);

        Ok(EthJsonRpc {
            connection,
            block: None,
        })
    }

    pub fn web3(&self) -> &Web3<Http> {
        &self.connection
    }
}

impl ChainState for EthJsonRpc {
    fn block(&self) -> crate::evm::BlockInfo {
        todo!()
    }

    fn nonce(&self, _address: &U256) -> usize {
        todo!()
    }

    fn balance(&self, _address: &U256) -> U256 {
        todo!()
    }

    fn code(&self, address: &U256) -> Vec<u8> {
        let address = address_to(address);
        block_on(self.connection.eth().code(address, self.block))
            .unwrap()
            .0
    }

    fn storage(&self, address: &U256, slot: &U256) -> U256 {
        let address = address_to(address);
        let idx = u256_to(&slot);
        u256_from_h(&block_on(self.connection.eth().storage(address, idx, self.block)).unwrap())
    }
}

fn address_to(u256: &U256) -> H160 {
    H160::from_slice(&u256.to_bytes_be()[12..])
}

fn address_from(h160: &H160) -> U256 {
    let mut bytes = [0_u8; 32];
    bytes[12..32].copy_from_slice(h160.as_fixed_bytes());
    U256::from_bytes_be(&bytes)
}

fn u256_to(value: &U256) -> W256 {
    W256::from_big_endian(&value.to_bytes_be())
}

fn u256_from_h(value: &H256) -> U256 {
    U256::from_bytes_be(&value.to_fixed_bytes())
}
