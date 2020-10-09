mod evm;

use evm::Opcode;
use std::{collections::HashMap, str::FromStr};
use tiny_keccak::{Hasher, Keccak};
use tokio;
use web3::types::{BlockId, BlockNumber, U64};
use zkp_u256::{One, Zero, U256};

fn keccak256(bytes: &[u8]) -> U256 {
    let mut keccak = Keccak::v256();
    keccak.update(bytes);
    let mut output = [0u8; 32];
    keccak.finalize(&mut output);
    U256::from_bytes_be(&output)
}

// Copy source to destination, padding with zeros
// fn padded_copy(source: &[u8], destination: &[u8]) {}

#[derive(Clone, Debug)]
struct Fork {
    url:   String,
    block: Option<u64>,
}
#[derive(Clone, Debug)]
struct ChainState {
    code:    HashMap<U256, Vec<u8>>,
    storage: HashMap<(U256, U256), U256>,
}

/// Constant for the current block
#[derive(Clone, Debug)]
struct BlockInfo {
    timestamp: u64,
}

/// Constants for the current transaction
#[derive(Clone, Debug)]
struct TransactionInfo {
    origin:    U256,
    gas_price: U256,
}

/// Constants for the current call
#[derive(Clone, Debug)]
struct CallInfo {
    sender:      U256,
    address:     U256,
    call_value:  U256,
    initial_gas: usize,
    input:       Vec<u8>,
}

/// Variables during execution
#[derive(Debug)]
struct ExecutionState<'a> {
    chain:       &'a mut ChainState,
    block:       &'a BlockInfo,
    transaction: &'a TransactionInfo,
    call:        &'a CallInfo,
    code:        &'a [u8],
    pc:          usize,
    gas:         usize,
    stack:       Vec<U256>,
    memory:      Vec<u8>,
    return_data: Vec<u8>,
}

#[derive(Clone, Debug)]
enum ExecutionResult {
    Return(Vec<u8>),
    Revert(Vec<u8>),
}

fn evaluate(
    chain: &mut ChainState,
    block: &BlockInfo,
    transaction: &TransactionInfo,
    call: &CallInfo,
) -> ExecutionResult {
    let code = chain.code[&call.address].clone();
    let mut exec = ExecutionState {
        chain,
        block,
        transaction,
        call,
        code: code.as_slice(),
        pc: 0,
        gas: call.initial_gas,
        stack: Vec::new(),
        memory: vec![0_u8; 1_000_000],
        return_data: Vec::new(),
    };
    exec.run()
}

impl<'a> ExecutionState<'a> {
    pub fn run(&mut self) -> ExecutionResult {
        loop {
            if let Some(result) = self.step() {
                return result;
            }
        }
    }

    pub fn step(&mut self) -> Option<ExecutionResult> {
        // Read from zero-extended bytecode
        // NOTE: Does the zero-extending work for Push(..) too?
        let op = self
            .code
            .get(self.pc)
            .map_or(Opcode::Stop, |b| Opcode::from(*b));
        match op {
            Opcode::Push(_) => {}
            op => println!("{:05} {}", self.pc, op),
        }
        self.pc += 1;

        // Dispatch opcode
        match op {
            Opcode::Push(n) => {
                // Read payload for Push instructions
                // TODO: Does this also zero extend?
                let n = n as usize;
                let mut padded = [0_u8; 32];
                padded[(32 - n)..].copy_from_slice(&self.code[self.pc..self.pc + n]);
                let argument = U256::from_bytes_be(&padded);
                println!("{:05} {} {}", self.pc - 1, op, argument);
                self.pc += n;
                self.stack.push(argument);
            }
            Opcode::MStore => {
                let offset = self.stack.pop().unwrap().as_usize();
                let value = self.stack.pop().unwrap().to_bytes_be();
                self.memory[offset..offset + 32].copy_from_slice(&value);
            }
            Opcode::MLoad => {
                let offset = self.stack.pop().unwrap().as_usize();
                let mut bytes32 = [0_u8; 32];
                bytes32.copy_from_slice(&self.memory[offset..offset + 32]);
                self.stack.push(U256::from_bytes_be(&bytes32));
            }
            Opcode::IsZero => {
                let value = self.stack.pop().unwrap();
                self.stack.push(if value.is_zero() {
                    U256::one()
                } else {
                    U256::zero()
                });
            }
            Opcode::Lt => {
                let left = self.stack.pop().unwrap();
                let right = self.stack.pop().unwrap();
                self.stack.push(if left < right {
                    U256::one()
                } else {
                    U256::zero()
                });
            }
            Opcode::Gt => {
                let left = self.stack.pop().unwrap();
                let right = self.stack.pop().unwrap();
                self.stack.push(if left > right {
                    U256::one()
                } else {
                    U256::zero()
                });
            }
            Opcode::Eq => {
                let left = self.stack.pop().unwrap();
                let right = self.stack.pop().unwrap();
                self.stack.push(if left == right {
                    U256::one()
                } else {
                    U256::zero()
                });
            }
            Opcode::And => {
                let left = self.stack.pop().unwrap();
                let right = self.stack.pop().unwrap();
                self.stack.push(left & right);
            }
            Opcode::Or => {
                let left = self.stack.pop().unwrap();
                let right = self.stack.pop().unwrap();
                self.stack.push(left | right);
            }
            Opcode::Add => {
                let left = self.stack.pop().unwrap();
                let right = self.stack.pop().unwrap();
                self.stack.push(left + right);
            }
            Opcode::Sub => {
                let left = self.stack.pop().unwrap();
                let right = self.stack.pop().unwrap();
                self.stack.push(left - right);
            }
            Opcode::Mul => {
                let left = self.stack.pop().unwrap();
                let right = self.stack.pop().unwrap();
                self.stack.push(left * right);
            }
            Opcode::Div => {
                let left = self.stack.pop().unwrap();
                let right = self.stack.pop().unwrap();
                self.stack.push(left / right);
            }
            Opcode::Shl => {
                let shift = self.stack.pop().unwrap().as_usize();
                let value = self.stack.pop().unwrap();
                self.stack.push(value << shift);
            }
            Opcode::Shr => {
                let shift = self.stack.pop().unwrap().as_usize();
                let value = self.stack.pop().unwrap();
                self.stack.push(value >> shift);
            }
            Opcode::Sha3 => {
                let source = self.stack.pop().unwrap().as_usize();
                let size = self.stack.pop().unwrap().as_usize();
                let bytes = &self.memory[source..source + size];
                self.stack.push(keccak256(bytes))
            }
            Opcode::Pop => {
                self.stack.pop();
            }
            Opcode::Dup(i) => {
                let i = i as usize;
                let value = self.stack[self.stack.len() - i].clone();
                self.stack.push(value);
            }
            Opcode::Swap(i) => {
                let top = self.stack.len() - 1;
                let i = top - (i as usize);
                self.stack.swap(i, top);
            }
            Opcode::JumpDest => {
                // TODO: Require on jump
            }
            Opcode::Jump => {
                let target = self.stack.pop().unwrap();
                self.pc = target.as_usize();
            }
            Opcode::JumpI => {
                let target = self.stack.pop().unwrap();
                let condition = self.stack.pop().unwrap();
                if !condition.is_zero() {
                    println!("Branch taken");
                    self.pc = target.as_usize();
                }
            }
            Opcode::Timestamp => {
                self.stack.push(U256::from(self.block.timestamp));
            }
            Opcode::CallValue => {
                self.stack.push(self.call.call_value.clone());
            }
            Opcode::CallDataSize => {
                self.stack.push(U256::from(self.call.input.len()));
            }
            Opcode::CallDataLoad => {
                // TODO: Pad input by 32 zero bytes to avoid this half-copy nonsense
                let source = self.stack.pop().unwrap().as_usize();
                let mut bytes32 = [0_u8; 32];
                for (i, b) in bytes32.iter_mut().enumerate() {
                    *b = self.call.input.get(source + i).cloned().unwrap_or_default();
                }
                self.stack.push(U256::from_bytes_be(&bytes32));
            }
            Opcode::CallDataCopy => {
                let destination = self.stack.pop().unwrap().as_usize();
                let source = self.stack.pop().unwrap().as_usize();
                let size = self.stack.pop().unwrap().as_usize();
                if source + size < self.call.input.len() {
                    self.memory[destination..destination + size]
                        .copy_from_slice(&self.call.input[source..source + size]);
                } else {
                    let n = self.call.input.len() - source;
                    self.memory[destination..destination + n]
                        .copy_from_slice(&self.call.input[source..source + n]);
                    for byte in &mut self.memory[destination + n..destination + size] {
                        *byte = 0;
                    }
                }
            }
            Opcode::ReturnDataSize => {
                self.stack.push(U256::from(self.return_data.len()));
            }
            Opcode::ReturnDataCopy => {
                let destination = self.stack.pop().unwrap().as_usize();
                let source = self.stack.pop().unwrap().as_usize();
                let size = self.stack.pop().unwrap().as_usize();
                if source + size < self.return_data.len() {
                    self.memory[destination..destination + size]
                        .copy_from_slice(&self.return_data[source..source + size]);
                } else {
                    let n = self.return_data.len() - source;
                    self.memory[destination..destination + n]
                        .copy_from_slice(&self.return_data[source..source + n]);
                    for byte in &mut self.memory[destination + n..destination + size] {
                        *byte = 0;
                    }
                }
            }
            Opcode::SLoad => {
                let slot = self.stack.pop().unwrap();
                println!("SLOAD {:?}", slot);
                self.stack
                    .push(self.chain.storage[&(self.call.address.clone(), slot)].clone());
            }
            Opcode::ExtCodeSize => {
                let address = self.stack.pop().unwrap();
                let size = self.chain.code[&address].len();
                self.stack.push(U256::from(size));
            }
            Opcode::StaticCall => {
                let initial_gas = self.stack.pop().unwrap().as_usize();
                let address = self.stack.pop().unwrap();
                let in_offset = self.stack.pop().unwrap().as_usize();
                let in_size = self.stack.pop().unwrap().as_usize();
                let out_offset = self.stack.pop().unwrap().as_usize();
                let out_size = self.stack.pop().unwrap().as_usize();
                let call = CallInfo {
                    sender: self.call.address.clone(),
                    initial_gas,
                    call_value: U256::zero(),
                    address,
                    input: self.memory[in_size..in_size + in_offset].to_vec(),
                };
                println!("Calling {:?}", &call.address);
                let result = evaluate(self.chain, self.block, self.transaction, &call);
                self.stack.push(match result {
                    ExecutionResult::Return(_) => U256::one(),
                    ExecutionResult::Revert(_) => U256::zero(),
                });
                self.return_data = match result {
                    ExecutionResult::Return(a) => a,
                    ExecutionResult::Revert(a) => a,
                };
                // Zero return memory
                // TODO: Use slice::fill
                for byte in self.memory[out_offset..out_offset + out_size].iter_mut() {
                    *byte = 0;
                }
                self.memory
                    [out_offset..out_offset + std::cmp::min(out_size, self.return_data.len())]
                    .copy_from_slice(self.return_data.as_slice());
            }
            Opcode::Return => {
                let offset = self.stack.pop().unwrap().as_usize();
                let size = self.stack.pop().unwrap().as_usize();
                let return_data = &self.memory[offset..offset + size];
                println!("Return 0x{}", hex::encode(return_data));
                return Some(ExecutionResult::Return(return_data.to_vec()));
            }
            Opcode::Revert => {
                let offset = self.stack.pop().unwrap().as_usize();
                let size = self.stack.pop().unwrap().as_usize();
                let return_data = &self.memory[offset..offset + size];
                println!("Revert 0x{}", hex::encode(return_data));
                return Some(ExecutionResult::Revert(return_data.to_vec()));
            }
            Opcode::Gas => {
                // Fake no gas consumption
                self.stack.push(U256::from(self.gas))
            }
            op => todo!("opcode {:?} is not yet implemented", op),
        };

        None
    }
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
    let value = web3
        .eth()
        .storage(ext, web3::types::U256::from(8), None)
        .await?;
    chain.storage.insert(
        (h160_to_u256(&ext), U256::from(8)),
        h256_convert(&value.into()),
    );

    // Run transaction
    let result = evaluate(&mut chain, &block, &transaction, &call);
    println!("Result: {:?}", result);

    Ok(())
}

// https://www.4byte.directory/api/v1/signatures/?hex_signature=0x6e667db3

// curl -X POST -H "Content-Type: application/json" --data '{"method": "debug_traceTransaction", "params": ["0x4b2e0ebdd74ecbf49eafd21949b48d23ebd2d41cb0080eb8c9eadb96aaae8c91", {}]}' http://localhost:8545
