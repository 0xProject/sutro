mod evm;
mod interpreter;

use crate::{
    evm::{BlockInfo, CallInfo, ChainState, TransactionInfo},
    interpreter::evaluate,
};
use std::{collections::HashMap, str::FromStr};
use tokio;
use web3::types::{BlockId, BlockNumber, U64};
use zkp_macros_decl::u256h;
use zkp_u256::{One, Zero, U256};
// Copy source to destination, padding with zeros
// fn padded_copy(source: &[u8], destination: &[u8]) {}

#[derive(Clone, Debug)]
struct Fork {
    url:   String,
    block: Option<u64>,
}

fn h160_to_u256(h160: &web3::types::H160) -> U256 {
    let mut bytes = [0_u8; 32];
    bytes[12..32].copy_from_slice(h160.as_fixed_bytes());
    U256::from_bytes_be(&bytes)
}

fn h256_convert(value: &web3::types::H256) -> U256 {
    u256_convert(&web3::types::U256::from_big_endian(value.as_bytes()))
}

fn u256_convert(value: &web3::types::U256) -> U256 {
    let mut big_endian = [0_u8; 32];
    value.to_big_endian(&mut big_endian);
    U256::from_bytes_be(&big_endian)
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    env_logger::init();

    let transport = web3::transports::Http::new("http://localhost:8545")?;
    let web3 = web3::Web3::new(transport);

    let latest = web3.eth().block_number().await?.as_u64();
    println!("Latest block: {}", latest);
    let latest = 11017418;
    let block_number = BlockNumber::Number(U64([latest]));
    let block_id = BlockId::Number(block_number);
    let block = web3.eth().block_with_txs(block_id).await?.unwrap();
    dbg!(block.hash);

    // for tx in &block.transactions {
    //    println!("0x{}", hex::encode(&tx.input.0));
    //}

    let tx = &block.transactions[3];
    let input = &tx.input.0;
    let receiver = tx.to.unwrap_or_default();
    println!("Tx: {:?}", tx.hash);
    println!("Receiver: {:?}", receiver);
    println!("Input: 0x{}", hex::encode(&input));

    // NOTE: Ideally we'd specify the exact blocknumber, but that makes the RPC call
    // fail with `missing trie node`. So we fetch from latest instead.
    // Fortunately code is immutable except from being able to reset to empty.
    let code = web3.eth().code(receiver, None).await?.0;
    println!("Code: 0x{}", hex::encode(&code[0..100]));

    let mut chain = ChainState {
        code:    HashMap::new(),
        storage: HashMap::new(),
    };
    let block = BlockInfo {
        timestamp: block.timestamp.low_u64(),
    };
    let transaction = TransactionInfo {
        origin:    h160_to_u256(&tx.from),
        gas_price: u256_convert(&tx.gas_price),
    };
    let call = CallInfo {
        initial_gas: tx.gas.as_usize(),
        sender:      transaction.origin.clone(),
        address:     h160_to_u256(&tx.to.unwrap()),
        call_value:  u256_convert(&tx.value),
        input:       tx.input.0.clone(),
    };

    // Add code for current contract
    chain.code.insert(h160_to_u256(&receiver), code);

    // Add code for an aux contract being queried
    let ext = web3::types::H160::from_str("164ed0df02b3747315b50b806b79962ad9517578").unwrap();
    let code = web3.eth().code(ext, None).await?.0;
    chain.code.insert(h160_to_u256(&ext), code);

    // Add storage
    // NOTE: Hardcoding values from that block because we don't have an archival
    // node.
    let _value = web3
        .eth()
        .storage(ext, web3::types::U256::from(8), None)
        .await?;
    let value = u256h!("5f7f8b9000000000001ad42a56757431b7770000000000393051e01d03863a5f");
    chain
        .storage
        .insert((h160_to_u256(&ext), U256::from(8)), value);

    // Run transaction
    let result = evaluate(&mut chain, &block, &transaction, &call);
    println!("Result: {:?}", result);

    Ok(())
}

// https://www.4byte.directory/api/v1/signatures/?hex_signature=0x6e667db3

// curl -X POST -H "Content-Type: application/json" --data '{"method": "debug_traceTransaction", "params": ["0x4b2e0ebdd74ecbf49eafd21949b48d23ebd2d41cb0080eb8c9eadb96aaae8c91", {}]}' http://localhost:8545
