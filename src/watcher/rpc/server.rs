//! Chain instance handling JSON-RPC requests.

use std::collections::HashMap;
use tiny_keccak::{Hasher, Keccak};
use web3::types::{CallRequest, Transaction, TransactionRequest, H256};

#[derive(Clone, Debug)]
pub struct Server {
    connection:   web3::Web3<web3::transports::Http>,
    transactions: HashMap<H256, Transaction>,
}

impl Server {
    pub fn new() -> Self {
        let transport = web3::transports::Http::new("http://localhost:8555").unwrap();
        let connection = web3::Web3::new(transport);
        Server {
            connection,
            transactions: HashMap::default(),
        }
    }

    pub fn chain_id(&self) -> u64 {
        1
    }

    pub fn block_number(&self) -> u64 {
        1
    }

    /// Process transaction, mine new block and return transaction hash.
    // TODO: Async processing.
    pub async fn transact(&mut self, tx: TransactionRequest) -> H256 {
        let data = tx.data.clone().unwrap_or_default();

        let mut rlp = rlp::RlpStream::new();
        rlp.begin_list(9);
        rlp.append(&tx.nonce.unwrap_or_default());
        rlp.append(&tx.gas_price.unwrap_or_default());
        rlp.append(&tx.gas.unwrap_or_default());
        rlp.append(&tx.to.unwrap_or_default());
        rlp.append(&tx.value.unwrap_or_default());
        rlp.append(&data.0);
        rlp.append(&self.chain_id());
        rlp.append(&0_u64);
        rlp.append(&0_u64);

        let hash = hash(&rlp.out());

        // Delegate
        let result = self.connection.eth().send_transaction(tx).await.unwrap();
        assert_eq!(result, hash);

        hash
    }
}

fn hash(bytes: &[u8]) -> H256 {
    let mut keccak = Keccak::v256();
    keccak.update(bytes);
    let mut output = [0u8; 32];
    keccak.finalize(&mut output);
    web3::types::H256::from_slice(&output)
}
