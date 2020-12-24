//! Implements a partial Ethereum JSON-RPC interface.
//!
//! See <https://eth.wiki/json-rpc/API>
//!
//! Uses <https://github.com/paritytech/jsonrpc>

mod handler;
mod interface;
mod logger;
pub mod types;

pub use self::{handler::RpcHandler, interface::EthereumRpc, logger::Logger};
use crate::prelude::*;
use jsonrpc_core::MetaIoHandler;
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, Server, ServerBuilder};

pub fn serve(rpc_handler: RpcHandler) -> AnyResult<Server> {
    let mut io_handler = MetaIoHandler::<(), Logger>::with_middleware(Logger::default());
    io_handler.extend_with(rpc_handler.to_delegate());
    let server = ServerBuilder::new(io_handler)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Null,
        ]))
        .start_http(&"127.0.0.1:8545".parse()?)
        .context("Starting RPC server")?;
    Ok(server)
}
