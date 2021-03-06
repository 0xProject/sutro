use super::{Error, Instruction, Opcode};
use crate::prelude::*;
use cranelift::prelude::{Block as JitBlock, *};
use std::collections::HashSet;

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
        Self::from_pc(bytecode, 0)
    }
}

impl Block {
    pub fn from_pc(bytecode: &[u8], mut pc: usize) -> Self {
        let mut instructions = Vec::default();
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
                Opcode::Jump => Instruction::Jump(HashSet::default()),
                Opcode::JumpI => Instruction::CondJump(HashSet::default(), pc),
                Opcode::JumpDest => {
                    if instructions.is_empty() {
                        Instruction::Opcode(Opcode::JumpDest)
                    } else {
                        Instruction::Fallthrough(pc - 1)
                    }
                }
                opcode => Instruction::Opcode(opcode),
            });

            // End block after a block-final opcode (except JumDest at start)
            // Single instruction blocks are invalid?
            if instructions.last().unwrap().is_block_final()
                && !(instructions.len() == 1 && opcode == Opcode::JumpDest)
            {
                break;
            }
        }
        Self { instructions }
    }

    pub fn gas_cost(&self) -> usize {
        let mut result = 0;
        for inst in &self.instructions {
            result += inst.opcode().map_or(0, Opcode::base_gas);
        }
        result
    }

    pub fn apply(&self, stack: &mut Vec<Option<U256>>) {
        for inst in &self.instructions {
            inst.apply(stack).unwrap();
        }
    }

    pub fn jump_targets(
        &mut self,
        mut stack: Vec<Option<U256>>,
    ) -> Result<Vec<(usize, Vec<Option<U256>>)>, Error> {
        for inst in &self.instructions[..self.instructions.len() - 1] {
            inst.apply(&mut stack)?;
        }
        let last = self.instructions.last_mut().unwrap();
        Ok(match last {
            Instruction::CondJump(branch_set, fallthrough) => {
                let fallthrough = *fallthrough;
                let branch = &stack
                    .last()
                    .ok_or(Error::StackUnderflow)?
                    .as_ref()
                    .ok_or(Error::ControlFlowEscaped)?;
                require!(branch.bits() < 32, Error::InvalidJump);
                let branch = branch.as_usize();
                branch_set.insert(branch);
                last.apply(&mut stack)?;
                vec![(fallthrough, stack.clone()), (branch, stack)]
            }
            Instruction::Jump(branch_set) => {
                let branch = stack
                    .last()
                    .ok_or(Error::StackUnderflow)?
                    .as_ref()
                    .ok_or(Error::ControlFlowEscaped)?;
                require!(branch.bits() < 32, Error::InvalidJump);
                let branch = branch.as_usize();
                branch_set.insert(branch);
                last.apply(&mut stack)?;
                vec![(branch, stack)]
            }
            Instruction::Fallthrough(fallthrough) => vec![(*fallthrough, stack)],
            _ => vec![],
        })
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
