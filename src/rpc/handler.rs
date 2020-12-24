use super::{
    types::{
        Address, BlockHeader, BlockNumber, BloomFilter, Bytes, Hex, Log, LogFilter,
        TransactionReceipt,
    },
    EthereumRpc,
};
use crate::prelude::*;
use jsonrpc_core::Result as RpcResult;

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

    fn send_transaction(&self, tx: web3::types::TransactionRequest) -> RpcResult<Hex<U256>> {
        Ok(U256::zero().into())
    }

    fn net_version(&self) -> RpcResult<String> {
        Ok(format!("{}", self.chain_id))
    }

    fn get_block_by_number(
        &self,
        block_number: BlockNumber,
        full: bool,
    ) -> RpcResult<Option<BlockHeader>> {
        let mut block_header = BlockHeader::default();
        block_header.logs_bloom = Some(BloomFilter::default());
        block_header.number = Some(42.into());
        block_header.nonce = Some(23.into());
        block_header.hash = Some(U256::zero());
        Ok(Some(block_header))
    }

    fn get_nonce(&self, address: Address, block_number: BlockNumber) -> RpcResult<Hex<u64>> {
        Ok(1.into())
    }

    fn get_code(&self, address: Address, block_number: BlockNumber) -> RpcResult<Bytes> {
        Ok(b"code".to_vec().into())
    }

    fn estimate_gas(&self, call: super::types::CallRequest) -> RpcResult<Hex<U256>> {
        Ok(U256::zero().into())
    }

    fn send_raw_transaction(&self, data: Bytes) -> RpcResult<U256> {
        Ok(U256::zero())
    }

    fn get_transaction_receipt(
        &self,
        transaction_hash: U256,
    ) -> RpcResult<Option<TransactionReceipt>> {
        Ok(Some(TransactionReceipt::default()))
    }

    fn get_logs(&self, filter: LogFilter) -> RpcResult<Vec<Log>> {
        Ok(Vec::new())
    }

    fn evm_snapshot(&self) -> RpcResult<Hex<u64>> {
        Ok(1.into())
    }

    fn evm_revert(&self, snapshot: Hex<u64>) -> RpcResult<bool> {
        Ok(true)
    }

    fn evm_increaseTime(&self, amount_sec: u64) -> RpcResult<u64> {
        todo!()
    }

    fn evm_mine(&self, timestamp: Option<u64>) -> RpcResult<Hex<u64>> {
        // Always returns zero
        Ok(0.into())
    }

    fn evm_unlock_unknown_account(&self, address: Address) -> RpcResult<bool> {
        todo!()
    }

    fn evm_lock_unknown_account(&self, address: Address) -> RpcResult<bool> {
        todo!()
    }
}
