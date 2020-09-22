mod block;
mod block_jit;
mod error;
mod instruction;
mod opcode;

use crate::block::Block;
use crate::error::Error;
use crate::instruction::Instruction;
use crate::opcode::Opcode;
use cranelift::prelude::*;
use cranelift_module::{DataContext, Linkage, Module};
use cranelift_simplejit::{SimpleJITBackend, SimpleJITBuilder};
use hex_literal::hex;
use zkp_u256::U256;

macro_rules! require {
    ($requirement:expr, $error:expr) => {
        if !$requirement {
            Err($error)?;
        }
    };
}

type Map<K, V> = std::collections::HashMap<K, V>;

#[derive(Clone, Debug, Eq, PartialEq, Default)]
struct Program {
    bytecode: Vec<u8>,
    blocks: Map<usize, Block>,
}

impl Program {
    fn recover_control_flow(
        &mut self,
        pc: usize,
        mut stack: Vec<Option<U256>>,
    ) -> Result<(), Error> {
        // Decompile block if not done already
        if !self.blocks.contains_key(&pc) {
            let block = Block::from(&self.bytecode[pc..]);
            require!(
                pc == 0 || block.instructions[0].0 == Opcode::JumpDest,
                Error::InvalidJump
            );
            println!("{}: ({} gas)", pc, block.gas_cost());
            println!("{}", block);

            self.blocks.insert(pc, block);
        }
        let block = &self.blocks[&pc];

        // Find more blocks
        let jump_targets = block.jump_targets(&mut stack)?;
        for (dest, stack) in jump_targets {
            // TODO: Fix potential infinite recursion
            self.recover_control_flow(dest, stack)?;
        }

        Ok(())
    }

    pub fn render<'a>(&self, builder: &mut FunctionBuilder<'a>) {
        for (_, block) in &self.blocks {
            block.render(builder);
        }
        builder.finalize();
    }
}

impl From<Vec<u8>> for Program {
    fn from(bytecode: Vec<u8>) -> Self {
        let mut result = Program {
            bytecode,
            blocks: Map::default(),
        };
        result.recover_control_flow(0, Vec::default());
        result
    }
}

fn main() -> anyhow::Result<()> {
    println!("Sizeof Opcode {}", std::mem::size_of::<Opcode>());
    println!("Sizeof U256 {}", std::mem::size_of::<U256>());
    println!("Sizeof Instruction {}", std::mem::size_of::<Instruction>());
    println!();

    // ZeroEx
    let bytecode = hex!(
        "6080604052600436106100225760003560e01c8063972fdd261461013857610029565b3661002957005b600061006f600080368080601f016020809104026020016040519081016040528093929190818152602001838380828437600092019190915250929392505061016e9050565b9050600061007c826101ba565b905073ffffffffffffffffffffffffffffffffffffffff81166100aa576100aa6100a583610213565b6102cb565b600060608273ffffffffffffffffffffffffffffffffffffffff166000366040516100d69291906103f5565b600060405180830381855af49150503d8060008114610111576040519150601f19603f3d011682016040523d82523d6000602084013e610116565b606091505b50915091508161012957610129816102cb565b610132816102d3565b50505050005b34801561014457600080fd5b506101586101533660046103ae565b6101ba565b6040516101659190610405565b60405180910390f35b6000816004018351101561018f5761018f6100a56003855185600401610309565b5001602001517fffffffff000000000000000000000000000000000000000000000000000000001690565b60006101c46102db565b7fffffffff0000000000000000000000000000000000000000000000000000000092909216600090815260209290925250604090205473ffffffffffffffffffffffffffffffffffffffff1690565b60607f734e6e1c6ec3f883cac8d13d3e7390b280f5e94424662aa29e27394ed56586c9826040516024016102479190610426565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fffffffff00000000000000000000000000000000000000000000000000000000909316929092179091529050919050565b805160208201fd5b805160208201f35b6000806102e860006102ee565b92915050565b600060808260058111156102fe57fe5b600101901b92915050565b6060632800659560e01b84848460405160240161032893929190610453565b604080517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe08184030181529190526020810180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff167fffffffff000000000000000000000000000000000000000000000000000000009093169290921790915290509392505050565b6000602082840312156103bf578081fd5b81357fffffffff00000000000000000000000000000000000000000000000000000000811681146103ee578182fd5b9392505050565b6000828483379101908152919050565b73ffffffffffffffffffffffffffffffffffffffff91909116815260200190565b7fffffffff0000000000000000000000000000000000000000000000000000000091909116815260200190565b606081016008851061046157fe5b93815260208101929092526040909101529056fea26469706673582212204011e5efaad3c8b897b9f518079a3f612fa6dac9577f8fe651130f5f3b423c8164736f6c634300060c0033"
    );

    let prog = Program::from(bytecode.to_vec());

    let builder = SimpleJITBuilder::new(cranelift_module::default_libcall_names());
    let module: Module<SimpleJITBackend> = Module::new(builder);
    let mut ctx = module.make_context();
    let data_ctx = DataContext::new();
    let mut func_ctx = FunctionBuilderContext::new();
    let mut builder = FunctionBuilder::new(&mut ctx.func, &mut func_ctx);

    prog.render(&mut builder);

    let shared_builder = settings::builder();
    let shared_flags = settings::Flags::new(shared_builder);
    let isa = cranelift::codegen::isa::lookup_by_name("x86_64")?.finish(shared_flags);

    println!("{}", builder.display(Some(isa.as_ref())));

    Ok(())
}
