//! Implements a partial Ethereum JSON-RPC interface.
//!
//! See <https://eth.wiki/json-rpc/API>
//!
//! Uses <https://github.com/paritytech/jsonrpc>

mod hex_number;
mod logger;
mod server;

use self::{hex_number::HexNumber, logger::Logger, server::Server};
use crate::evm_jit::Program;
use jsonrpc_core::{MetaIoHandler, Params};
use jsonrpc_http_server::{AccessControlAllowOrigin, DomainsValidation, ServerBuilder};
use log::{debug, info};
use serde_json::{json, Value};
use std::sync::{Arc, RwLock};

fn eip_155_v(chain_id: u64) -> u64 {
    chain_id * 2 + 35
}

pub fn main() {
    let server = Server::default();
    let server = Arc::new(RwLock::new(server));

    let mut io = MetaIoHandler::<(), Logger>::with_middleware(Logger::default());
    io.add_method("web3_clientVersion", |params: Params| {
        params.expect_no_params()?;
        Ok(json!(format!(
            "{} {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        )))
    });
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
            let server = server.read().unwrap();
            Ok(Value::Array(vec![Value::String(
                "0x407d73d8a49eeb85d32cf465507dd71d507100c1".to_string(),
            )]))
        }
    });
    // See <https://eth.wiki/json-rpc/API#eth_sendtransaction>
    io.add_method("eth_sendTransaction", {
        let server = server.clone();
        move |params: Params| {
            // let (tx,) = params.parse::<(web3::types::CallRequest,)>()?;

            dbg!();
            let mut server = server.write().unwrap();
            dbg!();

            let obj = if let Params::Array(arr) = params {
                arr[0].clone()
            } else {
                panic!()
            };

            dbg!();
            use ethereum::{TransactionAction, TransactionSignature};
            use ethereum_types::{H160, H256, U256};
            use std::str::FromStr;

            let chain_id = 1;

            dbg!();
            dbg!(H256::from_low_u64_be(1));
            let tx = ethereum::Transaction {
                action:    match obj.get("to") {
                    Some(Value::String(address)) => {
                        dbg!(address);
                        // TODO: Check for "0x" prefix.
                        TransactionAction::Call(H160::from_str(&address[2..]).unwrap())
                    }
                    None => TransactionAction::Create,
                    _ => panic!("Invalid to"),
                },
                // TODO:
                nonce:     U256::default(),
                gas_limit: U256::default(),
                gas_price: U256::default(),
                value:     U256::default(),
                signature: TransactionSignature::new(
                    eip_155_v(chain_id),
                    H256::from_low_u64_be(1),
                    H256::from_low_u64_be(1),
                )
                .unwrap(),
                input:     Vec::default(),
            };
            dbg!(&tx);
            let hash = tx.message_hash(Some(chain_id));
            dbg!(hash);

            // TODO: Compute and return transaction hash.
            Ok(Value::String(format!("{:?}", hash)))
        }
    });
    // See <https://eth.wiki/json-rpc/API#eth_gettransactionbyhash>
    io.add_method("eth_getTransactionByHash", {
        let server = server.clone();
        move |params: Params| {
            let (hash,) = params.parse::<(String,)>()?;
            let server = server.read().unwrap();

            // TODO: Retrieve transaction
            Ok(json!({
                "blockHash":"0x1d59ff54b1eb26b013ce3cb5fc9dab3705b415a67127a003c3e61eb445bb8df2",
                "blockNumber":"0x5daf3b", // 6139707
                "from":"0xa7d9ddbe1f17865597fbd27ec712455208b6b76d",
                "gas":"0xc350", // 50000
                "gasPrice":"0x4a817c800", // 20000000000
                "hash":hash,
                "input":"0x68656c6c6f21",
                "nonce":"0x15", // 21
                "to":"0xf02c1c8e6114b1dbe8937a39260b5b0a374432bb",
                "transactionIndex":"0x41", // 65
                "value":"0xf3dbb76162000", // 4290000000000000
                "v":"0x25", // 37
                "r":"0x1b5e176d927f8e9ab405058b2d2457392da3e20f328b16ddabcebc33eaac5fea",
                "s":"0x4ba69724e8f69de52f0125ad8b3c5c2cef33019bac3249e2c0a2192766d1721c"
            }))
        }
    });
    // See <https://eth.wiki/json-rpc/API#eth_gettransactionreceipt>
    io.add_method("eth_getTransactionReceipt", {
        let server = server.clone();
        move |params:Params| {
             let (hash,) = params.parse::<(String,)>()?;
                        let server = server.read().unwrap();

            // TODO: Retrieve transaction receipt
            Ok(json!({
                "transactionHash": "0x88df016429689c079f3b2f6ad39fa052532c56795b733da78a91ebe6a713944b",
                "transactionIndex":  "0x1", // 1
                "blockNumber": "0xb", // 11
                "blockHash": "0x1d59ff54b1eb26b013ce3cb5fc9dab3705b415a67127a003c3e61eb445bb8df2",
                "cumulativeGasUsed": "0x33bc", // 13244
                "gasUsed": "0x4dc", // 1244
                "contractAddress": "0xb60e8dd61c5d32be8058bb8eb970870f07233155", // or null, if none was created
                "logs": [],
                "logsBloom": "0x00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000", // 256 byte bloom filter
                "status": "0x1"
            }))
        }
    });
    // See <https://eth.wiki/json-rpc/API#eth_estimategas>
    io.add_method("eth_estimateGas", {
        let server = server.clone();
        move |params: Params| {
            // TODO: Support (optional) block number/tag
            let (tx,) = params.parse::<(web3::types::CallRequest,)>()?;
            dbg!(tx);
            let server = server.read().unwrap();

            Ok(json!("0x5208"))
        }
    });
    // See <https://eth.wiki/json-rpc/API#eth_gasprice>
    io.add_method("eth_gasPrice", {
        let server = server.clone();
        move |params: Params| {
            params.expect_no_params()?;
            let server = server.read().unwrap();

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
