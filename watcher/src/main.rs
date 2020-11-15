// Be less strict during early development.
#![allow(dead_code)]

mod chain;
mod evm;
mod evm_jit;
mod interpreter;
mod rpc;
mod yul;

use crate::{
    evm::{BlockInfo, CallInfo, ExecutionResult, TransactionInfo},
    interpreter::evaluate,
};
use chain::EthJsonRpc;
use hex_literal::hex;
use tokio;
use zkp_macros_decl::u256h;
use zkp_u256::{Zero, U256};

// Copy source to destination, padding with zeros
// fn padded_copy(source: &[u8], destination: &[u8]) {}

struct Chain {}

struct AccountState {
    nonce:   usize,
    balance: U256,
    code:    Vec<u8>,
}

#[macro_export]
macro_rules! require {
    ($condition:expr, $err:expr) => {
        if !$condition {
            return Err($err);
        }
    };
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    env_logger::init();

    rpc::main();

    // Chain state
    let mut chain_state = EthJsonRpc::new().await?;

    let block = BlockInfo {
        // number : 11017418
        timestamp: 1602194355,
    };
    let transaction = TransactionInfo {
        origin:    u256h!("000000000000000000000000f82ffee7eda1dd212dd0d867e57aa174dc207d7e"),
        gas_price: U256::from(1),
    };
    let call = CallInfo {
        sender: transaction.origin.clone(),
        address: u256h!("0000000000000000000000007a250d5630b4cf539739df2c5dacb4c659f2488d"),
        call_value: U256::zero(),
        initial_gas: 153_840,
        input: hex!("7ff36ab50000000000000000000000000000000000000000000000003c8902c25aa4f85d0000000000000000000000000000000000000000000000000000000000000080000000000000000000000000f82ffee7eda1dd212dd0d867e57aa174dc207d7e000000000000000000000000000000000000000000000000000000005f7f90380000000000000000000000000000000000000000000000000000000000000002000000000000000000000000c02aaa39b223fe8d0a0e5c4f27ead9083c756cc2000000000000000000000000468ab3b1f63a1c14b361bc367c3cc92277588da1").to_vec(),
    };

    // Run transaction
    let result = evaluate(&mut chain_state, &block, &transaction, &call);
    println!("Result: {:?}", result);
    if let ExecutionResult::Revert(result) = result {
        println!("Revert: {}", hex::encode(&result));

        let _param_type = ethabi::param_type::Reader::read("Error(string)").unwrap();
        // TODO: Decode ABI using param_type.
        let decoded = ethabi::decode(&[ethabi::ParamType::String], &result[4..]);
        dbg!(decoded).unwrap();
    }

    Ok(())
}

// https://www.4byte.directory/api/v1/signatures/?hex_signature=0x6e667db3

// curl -X POST -H "Content-Type: application/json" --data '{"method": "debug_traceTransaction", "params": ["0x4b2e0ebdd74ecbf49eafd21949b48d23ebd2d41cb0080eb8c9eadb96aaae8c91", {}]}' http://localhost:8545
