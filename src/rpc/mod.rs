//! Implements a partial Ethereum JSON-RPC interface.
//!
//! See <https://eth.wiki/json-rpc/API>
//!
//! Uses <https://github.com/paritytech/jsonrpc>

mod logger;

use crate::evm_jit::Program;
use jsonrpc_core::{MetaIoHandler, Params};
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, ServerBuilder};
use log::{debug, info};
use logger::Logger;
use serde_json::{json, Value};

pub fn main() {
    let mut io = MetaIoHandler::<(), Logger>::with_middleware(Logger::default());

    io.add_method("say_hello", |_| Ok(json!("hello")));
    io.add_method("web3_clientVersion", |_| {
        Ok(json!(format!(
            "{} {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )))
    });
    // TODO: Return chain_id
    io.add_method("net_version", |_| Ok(json!("1")));
    // TODO: Generate key pairs
    io.add_method("eth_accounts", |_| {
        Ok(Value::Array(vec![Value::String(
            "0x407d73d8a49eeb85d32cf465507dd71d507100c1".to_string(),
        )]))
    });
    // See <https://eth.wiki/json-rpc/API#eth_sendtransaction>
    io.add_method("eth_sendTransaction", |params| {
        let obj = if let Params::Array(arr) = params {
            arr[0].clone()
        } else {
            panic!()
        };
        let data = if let Value::Object(obj) = obj {
            obj["data"].clone()
        } else {
            panic!()
        };
        let data = if let Value::String(string) = data {
            string.clone()
        } else {
            panic!()
        };

        let contract = hex::decode(&data[2..]).unwrap();

        let prog = Program::from(contract[0..].to_vec()).unwrap();
        for (pc, block) in &prog.blocks {
            // println!("{}: ({} gas)", pc, block.gas_cost());
            // println!("{}", block);
        }

        let prog = Program::from(contract[31..].to_vec()).unwrap();
        for (pc, block) in &prog.blocks {
            // println!("{}: ({} gas)", pc, block.gas_cost());
            // println!("{}", block);
        }

        Ok(json!("hello"))
    });

    let server = ServerBuilder::new(io)
        .cors(DomainsValidation::AllowOnly(vec![
            AccessControlAllowOrigin::Null,
        ]))
        .start_http(&"127.0.0.1:8545".parse().unwrap())
        .expect("Unable to start RPC server");

    server.wait();
}
