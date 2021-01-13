//! Implements a partial Ethereum JSON-RPC interface.
//!
//! See <https://eth.wiki/json-rpc/API>
//!
//! Uses <https://github.com/paritytech/jsonrpc>

mod handler;
mod interface;
mod logger;

pub use self::{
    handler::RpcHandler,
    interface::{EthereumRpc, EthereumRpcClient},
    logger::Logger,
};
use crate::prelude::*;
use jsonrpc_core::MetaIoHandler;
use jsonrpc_core_client::transports::http;
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, Server, ServerBuilder};

pub fn serve(addr: &std::net::SocketAddr, rpc_handler: RpcHandler) -> AnyResult<Server> {
    let mut io_handler = MetaIoHandler::<(), Logger>::with_middleware(Logger::default());
    io_handler.extend_with(rpc_handler.to_delegate());
    let server = ServerBuilder::new(io_handler)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Null,
        ]))
        .start_http(addr)
        .context("Starting RPC server")?;
    Ok(server)
}

pub async fn client(url: &str) -> AnyResult<EthereumRpcClient> {
    http::connect(url)
        .await
        .map_err(|err| anyhow!("Error: {}", err))
        .context("Connecting to RPC client")
}
