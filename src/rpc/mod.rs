//! Implements a partial Ethereum JSON-RPC interface.
//!
//! See <https://eth.wiki/json-rpc/API>
//!
//! Uses <https://github.com/paritytech/jsonrpc>

mod logger;
pub mod types;

use self::logger::Logger;
use crate::prelude::*;

pub struct RpcHandler;
pub struct Server;

pub fn serve(rpc_handler: RpcHandler) -> Result<Server> {
    todo!();
    // let mut io_handler = MetaIoHandler::<(),
    // Logger>::with_middleware(Logger::default()); io_handler.
    // extend_with(rpc_handler.to_delegate()); let server =
    // ServerBuilder::new(io_handler)     .cors(DomainsValidation::
    // AllowOnly(vec![         AccessControlAllowOrigin::Null,
    //     ]))
    //     .start_http(&"127.0.0.1:8545".parse()?)
    //     .context("Starting RPC server")?;
    // Ok(server)
}
