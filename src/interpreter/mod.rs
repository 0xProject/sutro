use crate::evm::{
    precompiles::keccak256, BlockInfo, CallInfo, ChainState, ExecutionResult, Opcode,
    TransactionInfo,
};
use zkp_u256::{One, Zero, U256};

/// Variables during execution
#[derive(Debug)]
struct ExecutionState<'a> {
    chain:       &'a mut ChainState,
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
    chain: &mut ChainState,
    block: &BlockInfo,
    transaction: &TransactionInfo,
    call: &CallInfo,
) -> ExecutionResult {
    let code = chain.code[&call.address].clone();
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

    pub fn step(&mut self) -> Option<ExecutionResult> {
        // Read from zero-extended bytecode
        // NOTE: Does the zero-extending work for Push(..) too?
        let op = self
            .code
            .get(self.pc)
            .map_or(Opcode::Stop, |b| Opcode::from(*b));
        match op {
            Opcode::Push(_) => {}
            op => println!("{:05} {}", self.pc, op),
        }
        self.pc += 1;

        // Dispatch opcode
        match op {
            Opcode::Push(n) => {
                // Read payload for Push instructions
                // TODO: Does this also zero extend?
                let n = n as usize;
                let mut padded = [0_u8; 32];
                padded[(32 - n)..].copy_from_slice(&self.code[self.pc..self.pc + n]);
                let argument = U256::from_bytes_be(&padded);
                println!("{:05} {} {}", self.pc - 1, op, argument);
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
            Opcode::IsZero => {
                let value = self.stack.pop().unwrap();
                self.stack.push(if value.is_zero() {
                    U256::one()
                } else {
                    U256::zero()
                });
            }
            Opcode::Lt => {
                let left = self.stack.pop().unwrap();
                let right = self.stack.pop().unwrap();
                self.stack.push(if left < right {
                    U256::one()
                } else {
                    U256::zero()
                });
            }
            Opcode::Gt => {
                let left = self.stack.pop().unwrap();
                let right = self.stack.pop().unwrap();
                self.stack.push(if left > right {
                    U256::one()
                } else {
                    U256::zero()
                });
            }
            Opcode::Eq => {
                let left = self.stack.pop().unwrap();
                let right = self.stack.pop().unwrap();
                self.stack.push(if left == right {
                    U256::one()
                } else {
                    U256::zero()
                });
            }
            Opcode::And => {
                let left = self.stack.pop().unwrap();
                let right = self.stack.pop().unwrap();
                self.stack.push(left & right);
            }
            Opcode::Or => {
                let left = self.stack.pop().unwrap();
                let right = self.stack.pop().unwrap();
                self.stack.push(left | right);
            }
            Opcode::Add => {
                let left = self.stack.pop().unwrap();
                let right = self.stack.pop().unwrap();
                self.stack.push(left + right);
            }
            Opcode::Sub => {
                let left = self.stack.pop().unwrap();
                let right = self.stack.pop().unwrap();
                self.stack.push(left - right);
            }
            Opcode::Mul => {
                let left = self.stack.pop().unwrap();
                let right = self.stack.pop().unwrap();
                self.stack.push(left * right);
            }
            Opcode::Div => {
                let left = self.stack.pop().unwrap();
                let right = self.stack.pop().unwrap();
                self.stack.push(left / right);
            }
            Opcode::Shl => {
                let shift = self.stack.pop().unwrap().as_usize();
                let value = self.stack.pop().unwrap();
                self.stack.push(value << shift);
            }
            Opcode::Shr => {
                let shift = self.stack.pop().unwrap().as_usize();
                let value = self.stack.pop().unwrap();
                self.stack.push(value >> shift);
            }
            Opcode::Sha3 => {
                let source = self.stack.pop().unwrap().as_usize();
                let size = self.stack.pop().unwrap().as_usize();
                let bytes = &self.memory[source..source + size];
                self.stack.push(keccak256(bytes))
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
                    println!("Branch taken");
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
            Opcode::CallDataCopy => {
                let destination = self.stack.pop().unwrap().as_usize();
                let source = self.stack.pop().unwrap().as_usize();
                let size = self.stack.pop().unwrap().as_usize();
                if source + size < self.call.input.len() {
                    self.memory[destination..destination + size]
                        .copy_from_slice(&self.call.input[source..source + size]);
                } else {
                    let n = self.call.input.len() - source;
                    self.memory[destination..destination + n]
                        .copy_from_slice(&self.call.input[source..source + n]);
                    for byte in &mut self.memory[destination + n..destination + size] {
                        *byte = 0;
                    }
                }
            }
            Opcode::ReturnDataSize => {
                self.stack.push(U256::from(self.return_data.len()));
            }
            Opcode::ReturnDataCopy => {
                let destination = self.stack.pop().unwrap().as_usize();
                let source = self.stack.pop().unwrap().as_usize();
                let size = self.stack.pop().unwrap().as_usize();
                if source + size < self.return_data.len() {
                    self.memory[destination..destination + size]
                        .copy_from_slice(&self.return_data[source..source + size]);
                } else {
                    let n = self.return_data.len() - source;
                    self.memory[destination..destination + n]
                        .copy_from_slice(&self.return_data[source..source + n]);
                    for byte in &mut self.memory[destination + n..destination + size] {
                        *byte = 0;
                    }
                }
            }
            Opcode::SLoad => {
                let slot = self.stack.pop().unwrap();
                println!("SLOAD {:?}", slot);
                self.stack
                    .push(self.chain.storage[&(self.call.address.clone(), slot)].clone());
            }
            Opcode::ExtCodeSize => {
                let address = self.stack.pop().unwrap();
                let size = self.chain.code[&address].len();
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
                    input: self.memory[in_size..in_size + in_offset].to_vec(),
                };
                println!("Calling {:?}", &call.address);
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
}
