//! Implements a partial Ethereum JSON-RPC interface.
//!
//! See <https://eth.wiki/json-rpc/API>
//!
//! Uses <https://github.com/paritytech/jsonrpc>

mod logger;
mod server;

use self::{logger::Logger, server::Server};
use futures::compat::Compat;
use jsonrpc_core::{MetaIoHandler, Params, Result as RpcResult};
use jsonrpc_derive::rpc;
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, ServerBuilder};
use serde_json::{json, Value};
use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, RwLock},
};

// jsonrpc uses an ancient version of futures that needs a workaround.
// See <https://github.com/paritytech/jsonrpc/issues/485>
pub type BoxFuture<T> = Compat<Pin<Box<dyn Future<Output = RpcResult<T>> + Send>>>;

#[rpc(server)]
pub trait EthereumJsonRpc {
    #[rpc(name = "web3_clientVersion")]
    fn client_version(&self) -> RpcResult<String>;

    #[rpc(name = "eth_sendTransaction")]
    fn send_transaction(&self, tx: web3::types::TransactionRequest)
        -> BoxFuture<web3::types::H256>;
}

struct EJRServer {
    server: Arc<RwLock<Server>>,
}

impl EthereumJsonRpc for EJRServer {
    fn client_version(&self) -> RpcResult<String> {
        Ok(format!(
            "{} {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
    }

    fn send_transaction(
        &self,
        tx: web3::types::TransactionRequest,
    ) -> BoxFuture<web3::types::H256> {
        let _future = async move {
            let mut server = self.server.write().unwrap();
            let result = server.transact(tx).await;
            result
        };
        // TODO: future.boxed().compat()
        todo!()
    }
}

pub fn main() {
    let server = Server::new();
    let server = Arc::new(RwLock::new(server));

    let mut io = MetaIoHandler::<(), Logger>::with_middleware(Logger::default());

    let ejrs = EJRServer {
        server: server.clone(),
    };
    io.extend_with(ejrs.to_delegate());

    // See <https://eth.wiki/json-rpc/API#net_version>
    io.add_method("net_version", {
        let server = server.clone();
        move |params: Params| {
            params.expect_no_params()?;
            let server = server.read().unwrap();
            let chain_id = server.chain_id();
            // Value is returned as decimal string
            Ok(Value::String(format!("{}", chain_id)))
        }
    });
    // See <https://eth.wiki/json-rpc/API#eth_blocknumber>
    io.add_method("eth_blockNumber", {
        let server = server.clone();
        move |params: Params| {
            params.expect_no_params()?;
            let server = server.read().unwrap();
            Ok(Value::String(format!("0x{:x}", server.block_number())))
        }
    });
    // TODO: Generate key pairs
    io.add_method("eth_accounts", {
        let server = server.clone();
        move |params: Params| {
            params.expect_no_params()?;
            let _server = server.read().unwrap();
            Ok(Value::Array(vec![Value::String(
                "0x407d73d8a49eeb85d32cf465507dd71d507100c1".to_string(),
            )]))
        }
    });
    // See <https://eth.wiki/json-rpc/API#eth_sendtransaction>
    // io.add_method("eth_sendTransaction", {
    //     let server = server.clone();
    //     move |params: Params| {
    //         let future = async move {
    //             let (tx,) =
    // params.parse::<(web3::types::TransactionRequest,)>()?;             let
    // mut server = server.write().unwrap();             let hash =
    // server.transact(tx).await;             Ok(json!(hash))
    //         };
    //         Box::new(future.boxed().compat())
    //     }
    // });
    // See <https://eth.wiki/json-rpc/API#eth_gettransactionbyhash>
    io.add_method("eth_getTransactionByHash", {
        let server = server.clone();
        move |params: Params| {
            let (hash,) = params.parse::<(web3::types::H256,)>()?;
            let _server = server.read().unwrap();
            dbg!(&hash);

            // TODO: Retrieve transaction
            let mut tx = web3::types::RawTransaction::default().tx;
            tx.hash = hash;
            dbg!(&tx);

            Ok(json!(tx))
        }
    });
    // See <https://eth.wiki/json-rpc/API#eth_gettransactionreceipt>
    io.add_method("eth_getTransactionReceipt", {
        let server = server.clone();
        move |params: Params| {
            let (hash,) = params.parse::<(web3::types::H256,)>()?;
            let _server = server.read().unwrap();

            // TODO: Retrieve transaction receipt
            let mut receipt = web3::types::TransactionReceipt::default();
            receipt.transaction_hash = hash;

            Ok(json!(receipt))
        }
    });
    // See <https://eth.wiki/json-rpc/API#eth_estimategas>
    io.add_method("eth_estimateGas", {
        let server = server.clone();
        move |params: Params| {
            // TODO: Support (optional) block number/tag
            let (tx,) = params.parse::<(web3::types::CallRequest,)>()?;
            dbg!(tx);
            let _server = server.read().unwrap();

            Ok(json!("0x5208"))
        }
    });
    // See <https://eth.wiki/json-rpc/API#eth_gasprice>
    io.add_method("eth_gasPrice", {
        let server = server.clone();
        move |params: Params| {
            params.expect_no_params()?;
            let _server = server.read().unwrap();

            Ok(json!("0x1dfd14000"))
        }
    });

    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Null,
        ]))
        .start_http(&"127.0.0.1:8545".parse().unwrap())
        .expect("Unable to start RPC server");

    server.wait();
}
