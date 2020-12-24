use super::Error;
use crate::evm::Opcode;
use cranelift::prelude::*;
use std::collections::HashSet;
use zkp_u256::{Binary, U256};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Instruction {
    // Plain instruction
    Opcode(Opcode),

    // Push with data
    Push(U256),

    // Identified jumps
    Jump(HashSet<usize>),

    // Conditional brach (then, else)
    CondJump(HashSet<usize>, usize),

    // Fallthrough to next block
    Fallthrough(usize),
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Push(value) => {
                if value.bits() > 16 {
                    write!(f, "Push({})", value)
                } else {
                    write!(f, "Push({})", value.as_u128())
                }
            }
            Self::Opcode(op) => std::fmt::Display::fmt(op, f),
            _ => write!(f, "{:?}", self),
        }
    }
}

impl Instruction {
    pub fn opcode(&self) -> Option<Opcode> {
        match self {
            Self::Opcode(opcode) => Some(*opcode),
            Self::Push(value) => Some(Opcode::Push((1 + value.bits() / 8) as u8)),
            Self::Jump(_) => Some(Opcode::Jump),
            Self::CondJump(..) => Some(Opcode::JumpI),
            Self::Fallthrough(_) => None,
        }
    }

    pub const fn is_block_final(&self) -> bool {
        match self {
            Self::Opcode(opcode) => opcode.is_block_final(),
            Self::Push(_) => false,
            _ => true,
        }
    }

    /// Super simple symbolic executor
    pub fn apply(&self, stack: &mut Vec<Option<U256>>) -> Result<(), Error> {
        let (pop, push) = self.opcode().map_or((0, 0), Opcode::stack);
        if pop > stack.len() {
            return Err(Error::StackUnderflow);
        }
        match self {
            Self::Push(value) => stack.push(Some(value.clone())),
            Self::Opcode(Opcode::Dup(n)) => stack.push(stack[stack.len() - (*n as usize)].clone()),
            Self::Opcode(Opcode::Swap(n)) => {
                let last = stack.len() - 1;
                stack.swap(last, last - (*n as usize));
            }
            Self::Fallthrough(_) => {}
            Self::Opcode(Opcode::Unknown(_)) => return Err(Error::InvalidOpcode),
            _ => {
                stack.truncate(stack.len() - pop);
                stack.resize(stack.len() + push, None);
            }
        }
        if stack.len() > 1024 {
            return Err(Error::StackOverflow);
        }
        Ok(())
    }

    pub fn render<'a>(&self, builder: &mut FunctionBuilder<'a>) {
        match self {
            Self::Push(value) => {
                let stack_slot = builder.create_stack_slot(StackSlotData {
                    kind:   StackSlotKind::ExplicitSlot,
                    size:   32,
                    offset: None,
                });
                for (i, limb) in value.as_limbs().iter().enumerate() {
                    let x = builder.ins().iconst(types::I64, *limb as i64);
                    builder.ins().stack_store(x, stack_slot, (i * 8) as i32);
                }
            }
            _ => {} // todo!(),
        }
    }
}
