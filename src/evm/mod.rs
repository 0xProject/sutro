mod interpreter;
mod jit;
mod opcode;
pub mod precompiles;

pub use self::opcode::Opcode;
use zkp_u256::U256;

/// Constants for the current transaction
#[derive(Clone, Default, Debug)]
pub struct TransactionInfo {
    pub origin:    U256,
    pub gas_price: U256,
}

/// Constants for the current call
#[derive(Clone, Default, Debug)]
pub struct CallInfo {
    pub sender:      U256,
    pub address:     U256,
    pub call_value:  U256,
    pub initial_gas: usize,
    pub input:       Vec<u8>,
}

#[derive(Clone, Debug)]
pub enum ExecutionResult {
    Return(Vec<u8>),
    Revert(Vec<u8>),
}
