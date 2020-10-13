//! Yul parser
//!
//! See <https://solidity.readthedocs.io/en/v0.7.3/yul.html#specification-of-yul>

pub mod ast;
mod token;

pub use token::Token;

// https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
