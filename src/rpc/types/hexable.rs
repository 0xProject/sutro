use crate::prelude::*;
use std::num::ParseIntError;

pub trait Hexable: Sized {
    fn nibbles() -> usize;

    fn to_hex(&self) -> String;

    fn from_hex(str: &str) -> Result<Self, ParseIntError>;
}

#[allow(clippy::use_self)] // False positive due to macro expansion?
impl Hexable for u64 {
    fn nibbles() -> usize {
        16
    }

    fn to_hex(&self) -> String {
        format!("{:#x}", self)
    }

    fn from_hex(str: &str) -> Result<Self, ParseIntError> {
        let str = str.strip_prefix("0x").unwrap_or(str);
        Self::from_str_radix(str, 16)
    }
}

impl Hexable for U256 {
    fn nibbles() -> usize {
        64
    }

    fn to_hex(&self) -> String {
        let str = self.to_hex_string();
        // Remove `0x` prefix
        let str = &str[2..];
        // Remove leading zeros
        let str = str.trim_start_matches('0');
        // Have at least one digit
        let str = if str.is_empty() { "0" } else { str };
        // Add `0x` prefix
        format!("0x{}", str)
    }

    fn from_hex(str: &str) -> Result<Self, ParseIntError> {
        let str = str.strip_prefix("0x").unwrap_or(str);
        Ok(Self::from_hex_str(str))
    }
}
