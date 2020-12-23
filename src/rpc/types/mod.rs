//! These types are mostly copied from the `web3` crate, but adjusted for our
//! needs.

mod address;
mod block_number;
mod log_filter;
mod value_or_array;

pub use self::{
    address::Address, block_number::BlockNumber, log_filter::LogFilter,
    value_or_array::ValueOrArray,
};
