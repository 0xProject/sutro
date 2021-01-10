use super::Error;
use serde::{de, Deserialize};

pub fn from_rlp<'a, T>(value: &'a [u8]) -> Result<T, Error>
where
    T: Deserialize<'a>,
{
    todo!()
}
