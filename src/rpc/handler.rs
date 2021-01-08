use super::{
    types::{
        Address, BlockHeader, BlockNumber, BloomFilter, Bytes, GenesisConfig, Hex, Log, LogFilter,
        TransactionReceipt,
    },
    EthereumRpc,
};
use crate::prelude::*;
use jsonrpc_core::Result as RpcResult;

#[allow(clippy::module_name_repetitions)]
pub struct RpcHandler {
    pub client_version: String,
    pub chain_id:       usize,
    pub gas_price:      U256,
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

    fn get_block_by_number(
        &self,
        _block_number: BlockNumber,
        _full: bool,
    ) -> RpcResult<Option<BlockHeader>> {
        let mut block_header = BlockHeader::default();
        block_header.logs_bloom = Some(BloomFilter::default());
        block_header.number = Some(1.into());
        block_header.nonce = Some(0.into());
        block_header.hash = Some(U256::zero());
        block_header.mix_hash = Some(U256::zero());
        block_header.extra_data = vec![0x42_u8].into();
        Ok(Some(block_header))
    }

    fn get_block_by_hash(&self, _block_hash: U256, full: bool) -> RpcResult<Option<BlockHeader>> {
        self.get_block_by_number(BlockNumber::Number(0), full)
    }

    fn get_nonce(&self, _address: Address, _block_number: BlockNumber) -> RpcResult<Hex<u64>> {
        Ok(1.into())
    }

    fn get_code(&self, _address: Address, _block_number: BlockNumber) -> RpcResult<Bytes> {
        Ok(b"code".to_vec().into())
    }

    fn estimate_gas(&self, _call: super::types::CallRequest) -> RpcResult<Hex<U256>> {
        Ok(U256::zero().into())
    }

    fn send_raw_transaction(&self, _data: Bytes) -> RpcResult<U256> {
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

    fn test_set_chain_params(&self, _genesis: GenesisConfig) -> RpcResult<bool> {
        Ok(true)
    }

    fn test_import_raw_block(&self, _block: Bytes) -> RpcResult<U256> {
        Ok(U256::zero())
    }
}
