use crate::prelude::*;
use serde::{de, ser};

#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error("type can not be encoded in RLP")]
    UnsupportedType,

    #[error("trailing bytes in input or list")]
    TrailingBytes,

    #[error("input or list ended to early")]
    UnexpectedEnd,

    #[error("expected bytes, found list")]
    UnexpectedList,

    #[error("expected list, found bytes")]
    UnexpectedBytes,

    #[error("error reading string: {0}")]
    Utf8Error(#[from] std::str::Utf8Error),

    #[error("serde error: {0}")]
    Custom(String),
}

impl ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}

impl de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}
