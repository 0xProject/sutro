mod block;
mod instruction;
mod opcode;

use crate::block::Block;
use crate::instruction::Instruction;
use crate::opcode::Opcode;
use hex_literal::hex;
use zkp_u256::U256;

fn main() {
    println!("Sizeof Opcode {}", std::mem::size_of::<Opcode>());
    println!("Sizeof U256 {}", std::mem::size_of::<U256>());
    println!("Sizeof Instruction {}", std::mem::size_of::<Instruction>());

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
    println!("{}", block);

    let mut stack = Vec::default();
    block.apply(&mut stack);
}
