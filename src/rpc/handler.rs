use super::EthereumRpc;
use crate::{
    chain::types::{
        rpc::{
            AccountRange, BlockNumber, Bytes, CallRequest, GenesisConfig, Hex, Log, LogFilter,
            StorageRange, StorageSlot, TransactionReceipt,
        },
        Address, Block, FullBlock, RpcTransaction,
    },
    prelude::*,
    utils::RlpHash,
};
use jsonrpc_core::Result as RpcResult;
use std::{collections::HashMap, sync::RwLock};

#[allow(clippy::module_name_repetitions)]
pub struct RpcHandler {
    pub client_version: String,
    pub chain_id:       usize,
    pub gas_price:      U256,
    pub genesis:        RwLock<Block>,
    pub header:         RwLock<Block>,
}

impl RpcHandler {
    fn return_block(&self, block: Block) -> RpcResult<Option<FullBlock>> {
        let header = block.header;
        let hash = header.rlp_hash();
        let ommers = block.ommers.iter().map(RlpHash::rlp_hash).collect();
        let transactions = block
            .transactions
            .into_iter()
            .enumerate()
            .map(|(index, transaction)| {
                let tx_hash = transaction.rlp_hash();
                let from = crate::chain::types::Address::default(); // TODO
                RpcTransaction {
                    transaction,
                    transaction_index: index as u64,
                    block_hash: hash.clone(),
                    block_number: header.number,
                    from,
                    hash: tx_hash,
                }
            })
            .collect();
        Ok(Some(FullBlock {
            header,
            hash,
            size: 0,
            total_difficulty: 0,
            ommers,
            transactions,
        }))
    }
}

impl EthereumRpc for RpcHandler {
    fn client_version(&self) -> RpcResult<String> {
        Ok(self.client_version.clone())
    }

    fn gas_price(&self) -> RpcResult<Hex<U256>> {
        Ok(U256::zero().into())
    }

    fn send_transaction(&self, _tx: web3::types::TransactionRequest) -> RpcResult<Hex<U256>> {
        Ok(U256::zero().into())
    }

    fn net_version(&self) -> RpcResult<String> {
        Ok(format!("{}", self.chain_id))
    }

    fn block_number(&self) -> RpcResult<Hex<u64>> {
        let lock = self.header.read().map_err(internal_error)?;
        let block = lock.clone();
        let number = block.header.number;
        Ok(number.into())
    }

    fn get_block_by_number(
        &self,
        block_number: BlockNumber,
        _full: bool,
    ) -> RpcResult<Option<FullBlock>> {
        // TODO: Use `block_number` and `full`
        let mutex = match block_number {
            BlockNumber::Number(0) => &self.genesis,
            BlockNumber::Number(1) => &self.header,
            _ => unimplemented!(),
        };
        let lock = mutex.read().map_err(internal_error)?;
        let block = lock.clone();
        self.return_block(block)
    }

    fn get_block_by_hash(&self, _block_hash: U256, full: bool) -> RpcResult<Option<FullBlock>> {
        // TODO: Use `block_hash` and `full`
        let lock = self.header.read().map_err(internal_error)?;
        self.return_block(lock.clone())
    }

    fn get_nonce(&self, address: Address, _block_number: BlockNumber) -> RpcResult<Hex<u64>> {
        Ok(match address.to_array() {
            hex!("a94f5374fce5edbc8e2a8697c15331677e6ebf0b") => 1,
            _ => 0,
        }
        .into())
    }

    fn get_balance(&self, address: Address, block_number: BlockNumber) -> RpcResult<Hex<U256>> {
        Ok(match address.to_array() {
            hex!("0f572e5295c57f15886f9b263e2f6d2d6c7b5ec6") => {
                u256h!("00000000000000000000000000000000000000000000d3c21bcecceda100000b")
            }
            hex!("2adc25665018aa1fe0e6bc666dac8fc2697ff9ba") => {
                u256h!("0000000000000000000000000000000000000000000000001bc16d674ecf8270")
            }
            hex!("a94f5374fce5edbc8e2a8697c15331677e6ebf0b") => {
                u256h!("0000000000000000000000000000000000000000000000007ffffffffff87d75")
            }
            _ => U256::zero(),
        }
        .into())
    }

    fn get_code(&self, address: Address, _block_number: BlockNumber) -> RpcResult<Bytes> {
        Ok(match address.to_array() {
            hex!("0f572e5295c57f15886f9b263e2f6d2d6c7b5ec6") => {
                hex!("7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff7feeeeeeeeeeeeeeeeeeeeeeeeeeeeefeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee1860005500").to_vec()
            }
            _ => Vec::new(),
        }
        .into())
    }

    fn estimate_gas(&self, _call: CallRequest) -> RpcResult<Hex<U256>> {
        Ok(U256::zero().into())
    }

    fn send_raw_transaction(&self, _data: Vec<u8>) -> RpcResult<U256> {
        Ok(U256::zero())
    }

    fn get_transaction_receipt(
        &self,
        _transaction_hash: U256,
    ) -> RpcResult<Option<TransactionReceipt>> {
        Ok(Some(TransactionReceipt::default()))
    }

    fn get_logs(&self, _filter: LogFilter) -> RpcResult<Vec<Log>> {
        Ok(Vec::new())
    }

    fn evm_snapshot(&self) -> RpcResult<Hex<u64>> {
        Ok(1.into())
    }

    fn evm_revert(&self, _snapshot: Hex<u64>) -> RpcResult<bool> {
        Ok(true)
    }

    fn evm_increase_time(&self, _amount_sec: u64) -> RpcResult<u64> {
        todo!()
    }

    fn evm_mine(&self, _timestamp: Option<u64>) -> RpcResult<Hex<u64>> {
        // Always returns zero
        Ok(0.into())
    }

    fn evm_unlock_unknown_account(&self, _address: Address) -> RpcResult<bool> {
        todo!()
    }

    fn evm_lock_unknown_account(&self, _address: Address) -> RpcResult<bool> {
        todo!()
    }

    fn test_set_chain_params(&self, genesis: GenesisConfig) -> RpcResult<bool> {
        dbg!(genesis);
        // TODO: Get from input
        let genesis = crate::serde::rlp::from_rlp(&hex!("f901fdf901f8a00000000000000000000000000000000000000000000000000000000000000000a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347942adc25665018aa1fe0e6bc666dac8fc2697ff9baa0829e96e4071585b5e6f3f966a5ff59b8a2021a3827571c25bc8a739982f3f9e9a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421a056e81f171bcc55a6ff8345e692c0f86e5b48e01b996cadc001622fb5e363b421b90100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000008302000080887fffffffffffffff808042a00000000000000000000000000000000000000000000000000000000000000000880000000000000000c0c0")).unwrap();
        let mut lock = self.genesis.write().unwrap();
        *lock = genesis;

        Ok(true)
    }

    fn test_import_raw_block(&self, bytes: Bytes) -> RpcResult<U256> {
        dbg!(&bytes);
        let block = crate::serde::rlp::from_rlp::<Block>(bytes.as_slice()).map_err(parse_error)?;
        debug!("Block: {:#?}", &block);
        debug!("Tx hash = {:?}", block.transactions.trie_hash());
        debug!("Ommer hash = {:?}", block.ommers.rlp_hash());
        debug!("Block hash = {:?}", block.header.rlp_hash());
        let block_hash = block.header.rlp_hash();

        let mut lock = self.header.write().map_err(internal_error)?;
        *lock = block;

        Ok(block_hash)
    }

    fn get_block_rlp(&self, block_number: u64) -> RpcResult<Bytes> {
        todo!()
    }

    fn account_range(
        &self,
        block_id: String,
        tx_index: u64,
        start: U256,
        max_results: usize,
    ) -> RpcResult<AccountRange> {
        let addresses = [
            Address::from(hex!("0f572e5295c57f15886f9b263e2f6d2d6c7b5ec6")),
            Address::from(hex!("2adc25665018aa1fe0e6bc666dac8fc2697ff9ba")),
            Address::from(hex!("a94f5374fce5edbc8e2a8697c15331677e6ebf0b")),
        ];
        let mut address_map = HashMap::new();
        for addr in &addresses {
            address_map.insert(addr.rlp_hash(), addr.clone());
        }
        Ok(AccountRange {
            address_map,
            next_key: U256::zero(),
        })
    }

    fn storage_range(
        &self,
        block_id: String,
        tx_index: u64,
        address: Address,
        start: U256,
        max_results: usize,
    ) -> RpcResult<StorageRange> {
        if address.to_array() == hex!("0f572e5295c57f15886f9b263e2f6d2d6c7b5ec6") {
            let mut storage = HashMap::new();
            let key = U256::zero().into();
            let value = u256h!("1111111111111111111111111111101111111111111111111111111111111111");
            storage.insert(-U256::one(), StorageSlot { key, value });
            Ok(StorageRange {
                storage,
                complete: true,
            })
        } else {
            Ok(StorageRange {
                storage:  HashMap::new(),
                complete: true,
            })
        }
    }
}

fn parse_error<T: std::fmt::Display>(err: T) -> jsonrpc_core::Error {
    warn!("Parse error in RPC handler: {}", err);
    jsonrpc_core::Error::invalid_params(err.to_string())
}

fn internal_error<T: std::fmt::Display>(err: T) -> jsonrpc_core::Error {
    error!("Internal error in RPC handler: {}", err);
    jsonrpc_core::Error::internal_error()
}
