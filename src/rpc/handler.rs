use super::{
    types::{
        Address, BlockHeader, BlockNumber, BloomFilter, Bytes, Hex, Log, LogFilter,
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
        block_header.number = Some(42.into());
        block_header.nonce = Some(23.into());
        block_header.hash = Some(U256::zero());
        Ok(Some(block_header))
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
}
