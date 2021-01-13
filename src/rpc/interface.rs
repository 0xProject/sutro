use crate::{
    chain::types::{
        rpc::{
            AccountRange, BlockNumber, Bytes, CallRequest, GenesisConfig, Hex, Log, LogFilter,
            StorageRange, Transaction, TransactionReceipt,
        },
        Address, FullBlock,
    },
    prelude::*,
};
use jsonrpc_core::Result as RpcResult;
use jsonrpc_derive::rpc;

#[rpc]
pub trait EthereumRpc {
    /// See <https://eth.wiki/json-rpc/API#web3_clientversion>
    #[rpc(name = "web3_clientVersion")]
    fn client_version(&self) -> RpcResult<String>;

    /// See <https://eth.wiki/json-rpc/API#net_version>
    #[rpc(name = "net_version", alias("eth_chainId"))]
    fn net_version(&self) -> RpcResult<String>;

    /// See <https://eth.wiki/json-rpc/API#eth_sendtransaction>
    #[rpc(name = "eth_sendTransaction")]
    fn send_transaction(&self, tx: Transaction) -> RpcResult<Hex<U256>>;

    #[rpc(name = "eth_blockNumber")]
    fn block_number(&self) -> RpcResult<Hex<u64>>;

    #[rpc(name = "eth_getBlockByNumber")]
    fn get_block_by_number(
        &self,
        block_number: BlockNumber,
        full: bool,
    ) -> RpcResult<Option<FullBlock>>;

    /// See <https://eth.wiki/json-rpc/API#eth_getblockbyhash>
    #[rpc(name = "eth_getBlockByHash")]
    fn get_block_by_hash(&self, block_hash: U256, full: bool) -> RpcResult<Option<FullBlock>>;

    #[rpc(name = "eth_gasPrice")]
    fn gas_price(&self) -> RpcResult<Hex<U256>>;

    #[rpc(name = "eth_getTransactionCount")]
    fn get_nonce(&self, address: Address, block_number: BlockNumber) -> RpcResult<Hex<u64>>;

    /// See <https://eth.wiki/json-rpc/API#eth_getbalance>
    #[rpc(name = "eth_getBalance")]
    fn get_balance(&self, address: Address, block_number: BlockNumber) -> RpcResult<Hex<U256>>;

    /// See <https://eth.wiki/json-rpc/API#eth_getlogs>
    #[rpc(name = "eth_getLogs")]
    fn get_logs(&self, filter: LogFilter) -> RpcResult<Vec<Log>>;

    /// See <https://eth.wiki/json-rpc/API#eth_getcode>
    #[rpc(name = "eth_getCode")]
    fn get_code(&self, address: Address, block_number: BlockNumber) -> RpcResult<Bytes>;

    /// See <https://eth.wiki/json-rpc/API#eth_estimategas>
    #[rpc(name = "eth_estimateGas")]
    fn estimate_gas(&self, call: CallRequest) -> RpcResult<Hex<U256>>;

    /// See <https://eth.wiki/json-rpc/API#eth_sendrawtransaction>
    #[rpc(name = "eth_sendRawTransaction")]
    fn send_raw_transaction(&self, data: Vec<u8>) -> RpcResult<U256>;

    /// See <https://eth.wiki/json-rpc/API#eth_gettransactionreceipt>
    #[rpc(name = "eth_getTransactionReceipt")]
    fn get_transaction_receipt(
        &self,
        transaction_hash: U256,
    ) -> RpcResult<Option<TransactionReceipt>>;

    // Ganache extensions for testing
    //
    // See <https://github.com/trufflesuite/ganache-cli/blob/9c1c0a3fc206e673ee4cb214798b2d80e2e82e40/README.md#custom-methods>

    #[rpc(name = "evm_snapshot")]
    fn evm_snapshot(&self) -> RpcResult<Hex<u64>>;

    #[rpc(name = "evm_revert")]
    fn evm_revert(&self, snapshot: Hex<u64>) -> RpcResult<bool>;

    #[rpc(name = "evm_increaseTime")]
    fn evm_increase_time(&self, amount_sec: u64) -> RpcResult<u64>;

    #[rpc(name = "evm_mine")]
    fn evm_mine(&self, timestamp: Option<u64>) -> RpcResult<Hex<u64>>;

    #[rpc(name = "evm_unlockUnknownAccount")]
    fn evm_unlock_unknown_account(&self, address: Address) -> RpcResult<bool>;

    #[rpc(name = "evm_lockUnknownAccount")]
    fn evm_lock_unknown_account(&self, address: Address) -> RpcResult<bool>;

    // Ethereum Test
    //
    // See <https://github.com/ethereum/retesteth/wiki/RPC-Methods>

    #[rpc(name = "test_setChainParams")]
    fn test_set_chain_params(&self, genesis: GenesisConfig) -> RpcResult<bool>;

    #[rpc(name = "test_importRawBlock")]
    fn test_import_raw_block(&self, block: Bytes) -> RpcResult<U256>;

    /// See <https://github.com/ethereum/retesteth/wiki/RPC-Methods#debug_accountrange>
    #[rpc(name = "debug_accountRange")]
    fn account_range(
        &self,
        block_id: String,
        tx_index: u64,
        start: U256,
        max_results: usize,
    ) -> RpcResult<AccountRange>;

    /// See <https://github.com/ethereum/retesteth/wiki/RPC-Methods#debug_storagerangeat>
    #[rpc(name = "debug_storageRangeAt")]
    fn storage_range(
        &self,
        block_id: String,
        tx_index: u64,
        address: Address,
        start: U256,
        max_results: usize,
    ) -> RpcResult<StorageRange>;

    // Geth debug extensions
    //
    // See <https://geth.ethereum.org/docs/rpc/ns-debug>

    /// Note: Result by geth does not contain the `0x` prefix. The `Bytes` type
    /// will accept both with and without, but will always encode with.
    #[rpc(name = "debug_getBlockRlp")]
    fn get_block_rlp(&self, block_number: u64) -> RpcResult<Bytes>;
}
