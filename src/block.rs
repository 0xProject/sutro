use crate::Error;
use crate::Instruction;
use crate::Opcode;
use cranelift::prelude::{Block as JitBlock, *};
use zkp_u256::Binary;
use zkp_u256::{Zero, U256};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block {
    pub instructions: Vec<Instruction>,
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for inst in &self.instructions {
            writeln!(f, "{}", inst)?
        }
        Ok(())
    }
}
impl From<&[u8]> for Block {
    fn from(bytecode: &[u8]) -> Self {
        Block::from(bytecode, 0)
    }
}

impl Block {
    fn from(bytecode: &[u8], mut pc: usize) -> Self {
        let mut instructions = Vec::default();
        let mut reader = &bytecode[0..];
        loop {
            // Read next opcode
            // Programs are implicitly zero padded
            let opcode = bytecode.get(pc).cloned().map_or(Opcode::Stop, Opcode::from);
            pc += 1;

            // Add instruction
            instructions.push(match opcode {
                Opcode::Push(n) => {
                    // Read payload for Push instructions
                    // TODO: Does this also zero extend?
                    let n = n as usize;
                    let mut padded = [0_u8; 32];
                    padded[(32 - n)..].copy_from_slice(&bytecode[pc..pc + n]);
                    pc += n;
                    Instruction::Push(U256::from_bytes_be(&padded))
                }
                Opcode::Jump => Instruction::Jump(0),
                Opcode::JumpI => Instruction::CondJump(0, pc),
                Opcode::JumpDest => Instruction::Fallthrough(pc),
                opcode => Instruction::Opcode(opcode),
            });

            // End block after a block-final opcode (
            if instructions.last().unwrap().is_block_final() {
                break;
            }
        }
        Block { instructions }
    }

    pub fn gas_cost(&self) -> usize {
        let mut result = 0;
        for inst in &self.instructions {
            result += inst.opcode().map_or(0, |op| op.base_gas());
        }
        result
    }

    pub fn apply(&self, stack: &mut Vec<Option<U256>>) {
        for inst in &self.instructions {
            inst.apply(stack);
        }
    }

    pub fn jump_targets(
        &self,
        stack: &mut Vec<Option<U256>>,
    ) -> Result<Vec<(usize, Vec<Option<U256>>)>, Error> {
        let mut result = Vec::default();
        for inst in &self.instructions {
            //println!("{:?}", &stack);
            //println!("{}", &inst.0);
            match inst.opcode().unwrap() {
                Opcode::Jump | Opcode::JumpI => {
                    let dest = stack[stack.len() - 1]
                        .as_ref()
                        .ok_or(Error::ControlFlowEscaped)?
                        .clone();
                    if dest.bits() > 32 {
                        return Err(Error::InvalidJump);
                    }
                    inst.apply(stack)?;
                    result.push((dest.as_usize(), stack.clone()));
                }
                _ => {
                    inst.apply(stack)?;
                }
            }
        }
        Ok(result)
    }

    pub fn render<'a>(&self, builder: &mut FunctionBuilder<'a>) -> JitBlock {
        let block = builder.create_block();
        builder.switch_to_block(block);
        builder.seal_block(block);
        for inst in &self.instructions {
            inst.render(builder)
        }
        builder.ins().trap(TrapCode::User(0));
        block
    }
}
