//! Yul parser
//!
//! See <https://solidity.readthedocs.io/en/v0.7.3/yul.html#specification-of-yul>

pub mod ast;
mod ir;
mod parser;
mod token;

pub use parser::{parse_block, parse_file, parse_object};
pub use token::{tokenize, Token};
