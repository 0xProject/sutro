use crate::prelude::*;
use std::io::Write;
use tiny_keccak::{Hasher, Keccak};

/// Ethereum's Keccak256 hash function
pub fn keccak256(bytes: &[u8]) -> U256 {
    let mut keccak = Keccak256::new();
    keccak.write_all(bytes).expect("keccak never fails");
    keccak.finish()
}

/// Ethereum's Keccak256 hash writer
pub struct Keccak256(Keccak);

impl Keccak256 {
    pub fn new() -> Self {
        Self(Keccak::v256())
    }

    pub fn finish(self) -> U256 {
        let mut output = [0_u8; 32];
        self.0.finalize(&mut output);
        U256::from_bytes_be(&output)
    }
}

impl Write for Keccak256 {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.update(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
