use crate::Error;
use crate::Instruction;
use crate::Opcode;
use zkp_u256::Binary;
use zkp_u256::{Zero, U256};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Block {
    pub instructions: Vec<Instruction>,
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
                let mut padded = [0_u8; 32];
                padded[(32 - n)..].copy_from_slice(&reader[0..n]);
                reader = &reader[n..];
                U256::from_bytes_be(&padded)
            } else {
                U256::zero()
            };

            // Append to block
            instructions.push(Instruction(opcode, payload));
            if opcode.is_block_final() {
                break;
            }
        }
        Block { instructions }
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for inst in &self.instructions {
            writeln!(f, "{}", inst)?
        }
        Ok(())
    }
}

impl Block {
    pub fn jump_targets(
        &self,
        stack: &mut Vec<Option<U256>>,
    ) -> Result<Vec<(usize, Vec<Option<U256>>)>, Error> {
        let mut result = Vec::default();
        for inst in &self.instructions {
            match inst.0 {
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

    pub fn apply(&self, stack: &mut Vec<Option<U256>>) {
        for inst in &self.instructions {
            // println!("{:?}", &stack);
            println!("{}", &inst);

            match inst.0 {
                Opcode::Jump | Opcode::JumpI => {
                    let dest = stack[stack.len() - 1].as_ref().unwrap().clone();
                    inst.apply(stack);
                    dbg!(dest.as_usize(), &stack);

                    // Recursively parse identified blocks.
                }
                _ => {
                    inst.apply(stack);
                }
            }
        }
    }
}
