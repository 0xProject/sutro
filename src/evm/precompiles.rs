use tiny_keccak::{Hasher, Keccak};
use zkp_u256::U256;

/// Ethereum's Keccak256 hash function
///
/// Matches the opcode. While not technically a precompile, it is usefull
/// to consider it one.
pub fn keccak256(bytes: &[u8]) -> U256 {
    let mut keccak = Keccak::v256();
    keccak.update(bytes);
    let mut output = [0_u8; 32];
    keccak.finalize(&mut output);
    U256::from_bytes_be(&output)
}
