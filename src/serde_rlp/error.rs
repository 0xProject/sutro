use crate::prelude::*;
use serde::ser::Error as SerdeError;

#[derive(Clone, Debug, Error)]
pub enum Error {
    #[error("type can not be encoded in RLP")]
    UnsupportedType,

    #[error("{0}")]
    Custom(String),
}

impl SerdeError for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: std::fmt::Display,
    {
        Self::Custom(msg.to_string())
    }
}
