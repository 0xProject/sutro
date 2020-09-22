use crate::Opcode;
use zkp_u256::{Zero, U256};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Instruction(pub Opcode, pub U256);

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Opcode::Push(_) = self.0 {
            write!(f, "Push({})", self.1)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

impl Instruction {
    pub fn apply(&self, stack: &mut Vec<Option<U256>>) {
        match self.0 {
            Opcode::Push(_) => stack.push(Some(self.1.clone())),
            Opcode::Dup(n) => stack.push(stack[stack.len() - (n as usize)].clone()),
            Opcode::Swap(n) => {
                dbg!(n);
                let last = stack.len() - 1;
                stack.swap(last, last - (n as usize));
            }
            other => {
                let (pop, push) = other.stack();
                stack.resize(stack.len() - pop, None);
                stack.resize(stack.len() + push, None);
            }
        }
    }
}
