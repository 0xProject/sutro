//! Simple EVM interpreter
// TODO: Error handling

use crate::{
    chain::{BlockInfo, ChainState},
    evm::{precompiles::keccak256, CallInfo, ExecutionResult, Opcode, TransactionInfo},
    prelude::*,
};

/// Variables during execution
struct ExecutionState<'a> {
    chain:       &'a mut dyn ChainState,
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

pub fn evaluate(
    chain: &mut dyn ChainState,
    block: &BlockInfo,
    transaction: &TransactionInfo,
    call: &CallInfo,
) -> ExecutionResult {
    let code = chain.code(&call.address);
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

    #[allow(clippy::too_many_lines)] // TODO: Simplify
    pub fn step(&mut self) -> Option<ExecutionResult> {
        // Read from zero-extended bytecode
        // NOTE: Does the zero-extending work for Push(..) too?
        let op = self
            .code
            .get(self.pc)
            .map_or(Opcode::Stop, |b| Opcode::from(*b));
        // match op {
        // Opcode::Push(_) => {}
        // op => println!("{:05} {}", self.pc, op),
        // }
        self.pc += 1;

        // Dispatch opcode
        #[allow(clippy::match_same_arms)]
        match op {
            Opcode::Stop => todo!(),
            Opcode::Add => self.op2(|left, right| left + right),
            Opcode::Mul => self.op2(|left, right| left * right),
            Opcode::Sub => self.op2(|left, right| left - right),
            Opcode::Div => self.op2(|left, right| left / right),
            Opcode::SDiv => todo!(), // self.op2(|left, right| ),
            Opcode::Mod => self.op2(|left, right| left % right),
            Opcode::SMod => todo!(),   // self.op2(|left, right| ),
            Opcode::AddMod => todo!(), // self.op3(|left, right, modulus| ),
            Opcode::MulMod => self.op3(|left, right, modulus| left.mulmod(&right, &modulus)),
            Opcode::Exp => todo!(),        // self.op2(|base, exponent| ),
            Opcode::SignExtend => todo!(), // self.op2(|value, bytes| ),

            Opcode::Lt => self.op2(|left, right| left < right),
            Opcode::Gt => self.op2(|left, right| left > right),
            Opcode::SLt => todo!(), // self.op2(|left, right| ),
            Opcode::SGt => todo!(), // self.op2(|left, right| ),
            Opcode::Eq => self.op2(|left, right| left == right),
            Opcode::IsZero => self.op1(|value| value.is_zero()),
            Opcode::And => self.op2(|left, right| left & right),
            Opcode::Or => self.op2(|left, right| left | right),
            Opcode::Xor => self.op2(|left, right| left ^ right),
            Opcode::Not => self.op1(|value| !value),
            Opcode::Byte => todo!(),
            // TODO: Fix truncation of large shift amounts
            Opcode::Shl => self.op2(|shift, value| value << shift.as_usize()),
            Opcode::Shr => self.op2(|shift, value| value >> shift.as_usize()),
            Opcode::Sar => todo!(), // self.op2(|shift, value|),

            // TODO: Fix overly large offsets/sizes
            Opcode::Sha3 => {
                let source = self.stack.pop().unwrap().as_usize();
                let size = self.stack.pop().unwrap().as_usize();
                let bytes = &self.memory[source..source + size];
                self.stack.push(keccak256(bytes))
            }

            Opcode::Push(n) => {
                // Read payload for Push instructions
                // TODO: Does this also zero extend?
                let n = n as usize;
                let mut padded = [0_u8; 32];
                padded[(32 - n)..].copy_from_slice(&self.code[self.pc..self.pc + n]);
                let argument = U256::from_bytes_be(&padded);
                // println!("{:05} {} {}", self.pc - 1, op, argument);
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
            Opcode::ReturnDataSize => {
                self.stack.push(U256::from(self.return_data.len()));
            }
            Opcode::CallDataCopy => self.handle_copy(&self.call.input),
            Opcode::ReturnDataCopy => {
                // HACK: Temporarily swap out return_data without cloning.
                let mut return_data = Vec::new();
                std::mem::swap(&mut self.return_data, &mut return_data);
                self.handle_copy(&return_data);
                std::mem::swap(&mut self.return_data, &mut return_data);
            }
            Opcode::CodeCopy => self.handle_copy(self.code),
            Opcode::SLoad => {
                let slot = self.stack.pop().unwrap();
                println!("SLOAD {:?}", slot);
                self.stack
                    .push(self.chain.storage(&self.call.address, &slot));
            }
            Opcode::ExtCodeSize => {
                let address = self.stack.pop().unwrap();
                let size = self.chain.code(&address).len();
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
                    input: self.memory[in_offset..in_offset + in_size].to_vec(),
                };
                // TODO: Print using bytes4-dictionary based ABI decoder.
                info!("Calling {:?} {}", &call.address, hex::encode(&call.input));
                let result = evaluate(self.chain, self.block, self.transaction, &call);
                self.stack.push(match result {
                    ExecutionResult::Return(_) => U256::one(),
                    ExecutionResult::Revert(_) => U256::zero(),
                });
                self.return_data = match result {
                    ExecutionResult::Return(a) => a,
                    ExecutionResult::Revert(a) => a,
                };
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

    fn op1<F, T>(&mut self, f: F)
    where
        F: FnOnce(U256) -> T,
        T: Into<U256>,
    {
        let arg = self.stack.pop().unwrap();
        let result = f(arg);
        self.stack.push(result.into());
    }

    fn op2<F, T>(&mut self, f: F)
    where
        F: FnOnce(U256, U256) -> T,
        T: Into<U256>,
    {
        let arg_0 = self.stack.pop().unwrap();
        let arg_1 = self.stack.pop().unwrap();
        let result = f(arg_0, arg_1);
        self.stack.push(result.into());
    }

    fn op3<F, T>(&mut self, f: F)
    where
        F: FnOnce(U256, U256, U256) -> T,
        T: Into<U256>,
    {
        let arg_0 = self.stack.pop().unwrap();
        let arg_1 = self.stack.pop().unwrap();
        let arg_3 = self.stack.pop().unwrap();
        let result = f(arg_0, arg_1, arg_3);
        self.stack.push(result.into());
    }

    /// Handle copy operations from a source array to memory
    ///
    /// Offsets and sizes are popped from stack. `source` is implicitly
    /// zero extended.
    fn handle_copy(&mut self, source: &[u8]) {
        let offset = self.stack.pop().unwrap().as_usize();
        let source_offset = self.stack.pop().unwrap().as_usize();
        let want_size = self.stack.pop().unwrap().as_usize();
        let size = std::cmp::min(want_size, source.len() - source_offset);
        let source_slice = &source[source_offset..source_offset + size];
        self.memory[offset..offset + size].copy_from_slice(source_slice);
        for byte in self.memory[offset + size..offset + want_size].iter_mut() {
            *byte = 0;
        }
    }
}
