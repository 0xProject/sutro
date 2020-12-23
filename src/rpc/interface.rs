use jsonrpc_core::{BoxFuture, MetaIoHandler, Params, Result as RpcResult};
use jsonrpc_derive::rpc;
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, Server, ServerBuilder};
use serde_json::{json, Value};
use std::sync::Arc;
use tokio::sync::RwLock;
use types::LogFilter;

#[rpc(server)]
pub trait EthereumRpc {
    /// See <https://eth.wiki/json-rpc/API#web3_clientversion>
    #[rpc(name = "web3_clientVersion")]
    fn client_version(&self) -> RpcResult<String>;

    /// See <https://eth.wiki/json-rpc/API#eth_sendtransaction>
    #[rpc(name = "eth_sendTransaction")]
    fn send_transaction(&self, tx: web3::types::TransactionRequest) -> RpcResult<W256>;

    /// See <https://eth.wiki/json-rpc/API#net_version>
    #[rpc(name = "net_version", alias("eth_chainId"))]
    fn net_version(&self) -> RpcResult<String>;

    /// See <https://github.com/trufflesuite/ganache-cli/blob/9c1c0a3fc206e673ee4cb214798b2d80e2e82e40/README.md#custom-methods>
    #[rpc(name = "evm_snapshot")]
    fn evm_snapshot(&self) -> RpcResult<String>;

    /// See <https://github.com/trufflesuite/ganache-cli/blob/9c1c0a3fc206e673ee4cb214798b2d80e2e82e40/README.md#custom-methods>
    #[rpc(name = "evm_revert")]
    fn evm_revert(&self, snapshot: String) -> RpcResult<bool>;

    #[rpc(name = "eth_getBlockByNumber")]
    fn get_block_by_number(&self, block_number: String, full: bool) -> RpcResult<BlockHeader>;

    #[rpc(name = "eth_gasPrice")]
    fn gas_price(&self) -> RpcResult<W256>;

    #[rpc(name = "eth_getTransactionCount")]
    fn get_nonce(&self, address: web3::types::H160, block_number: String) -> RpcResult<String>;

    #[rpc(name = "eth_getLogs")]
    fn get_logs(&self, filter: Filter) -> Result<Vec<Log>>;
}

/// Ganache extensions for testing
///
/// See <https://github.com/trufflesuite/ganache-cli/blob/9c1c0a3fc206e673ee4cb214798b2d80e2e82e40/README.md#custom-methods>
#[rpc(server)]
pub trait TestRpc {
    #[rpc(name = "evm_snapshot")]
    fn evm_snapshot(&self) -> RpcResult<Hex<u64>>;

    #[rpc(name = "evm_revert")]
    fn evm_revert(&self, snapshot: Hex<u64>) -> RpcResult<bool>;

    #[rpc(name = "evm_increaseTime")]
    fn evm_revert(&self, amount_sec: u64) -> RpcResult<Dec<u64>>;

    #[rpc(name = "evm_mine")]
    fn evm_revert(&self, timestamp: Option<u64>) -> RpcResult<HexZero>;

    #[rpc(name = "evm_unlockUnknownAccount")]
    fn evm_revert(&self, address: Address) -> RpcResult<bool>;

    #[rpc(name = "evm_lockUnknownAccount")]
    fn evm_revert(&self, address: Address) -> RpcResult<bool>;
}
