mod opcode;

use crate::opcode::Opcode;
use hex_literal::hex;

type U256 = u64;

trait ChainState {
    fn sload(&self, slot: U256) -> U256;
    fn sstore(&mut self, slot: U256, value: U256);
}

const MAX_MEMORY: usize = 3_213_708; // Max for 20MGas

struct ExecutionContext {
    gas_left: usize,
    stack: [U256; 1024],
    memory: [u8; MAX_MEMORY],
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Instruction(Opcode, Vec<u8>);

#[derive(Clone, Debug, Eq, PartialEq)]
struct Block {
    instructions: Vec<Instruction>,
}

impl From<&[u8]> for Block {
    fn from(bytecode: &[u8]) -> Self {
        let mut instructions = Vec::default();
        let mut reader = &bytecode[0..];
        loop {
            // Read next opcode
            // Programs are implicitly zero padded
            let opcode = if reader.is_empty() {
                Opcode::from(0)
            } else {
                let opcode = Opcode::from(reader[0]);
                reader = &reader[1..];
                opcode
            };

            // Read payload for Push opcodes
            let payload = if let Opcode::Push(n) = opcode {
                let n = n as usize;
                let payload = &reader[0..n];
                reader = &reader[n..];
                payload
            } else {
                &reader[0..0]
            };

            // Append to block
            instructions.push(Instruction(opcode, payload.to_vec()));
            if opcode.is_block_final() {
                break;
            }
        }
        Block { instructions }
    }
}

fn main() {
    let bytecode = hex!(
        "6080604052600436106049576000357c0100000000000000000000000000000
        000000000000000000000000000900463ffffffff16806360fe47b114604e57
        80636d4ce63c146078575b600080fd5b348015605957600080fd5b506076600
        4803603810190808035906020019092919050505060a0565b005b3480156083
        57600080fd5b50608a60aa565b6040518082815260200191505060405180910
        390f35b8060008190555050565b600080549050905600a165627a7a72305820
        99c66a25d59f0aa78f7ebc40748fa1d1fbc335d8d780f284841b30e0365acd9
        60029"
    );

    let block = Block::from(&bytecode[0..]);
    println!("{:?}", block);
}
