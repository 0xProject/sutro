pub struct RpcHandler {
    pub client_version: String,
    pub chain_id:       usize,
    pub gas_price:      U256,
}

impl EthereumRpc for RpcHandler {
    fn client_version(&self) -> RpcResult<String> {
        Ok(self.client_version.clone())
    }

    fn gas_price(&self) -> RpcResult<W256> {
        Ok(W256::zero())
    }

    fn send_transaction(&self, tx: web3::types::TransactionRequest) -> RpcResult<W256> {
        Ok(W256::zero())
    }

    fn net_version(&self) -> RpcResult<String> {
        Ok(format!("{}", self.chain_id))
    }

    fn evm_snapshot(&self) -> RpcResult<String> {
        Ok("0x1".into())
    }

    fn evm_revert(&self, snapshot: String) -> RpcResult<bool> {
        Ok(true)
    }

    fn get_block_by_number(
        &self,
        block_number: String,
        full: bool,
    ) -> RpcResult<web3::types::BlockHeader> {
        Ok(web3::types::BlockHeader {
            hash:              Some(H256::zero()),
            parent_hash:       H256::zero(),
            uncles_hash:       H256::zero(),
            author:            H160::zero(),
            state_root:        H256::zero(),
            transactions_root: H256::zero(),
            receipts_root:     H256::zero(),
            number:            Some(U64::zero()),
            gas_used:          W256::zero(),
            gas_limit:         W256::zero(),
            extra_data:        Bytes::default(),
            logs_bloom:        H2048::zero(),
            timestamp:         W256::zero(),
            difficulty:        W256::zero(),
            mix_hash:          Some(H256::zero()),
            nonce:             Some(H64::zero()),
        })
    }

    fn get_nonce(&self, address: web3::types::H160, block_number: String) -> RpcResult<String> {
        Ok("0x1".into())
    }

    fn get_logs(&self, filter: LogFilter) -> Result<Vec<Log>> {
        Ok(Vec::new())
    }
}
