use crate::Error;
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
    /// Super simple symbolic executor
    pub fn apply(&self, stack: &mut Vec<Option<U256>>) -> Result<(), Error> {
        let (pop, push) = self.0.stack();
        if pop > stack.len() {
            return Err(Error::StackUnderflow);
        }
        match self.0 {
            Opcode::Push(_) => stack.push(Some(self.1.clone())),
            Opcode::Dup(n) => stack.push(stack[stack.len() - (n as usize)].clone()),
            Opcode::Swap(n) => {
                let last = stack.len() - 1;
                stack.swap(last, last - (n as usize));
            }
            Opcode::Invalid(_) => return Err(Error::InvalidOpcode),
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
}
