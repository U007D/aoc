use crate::consts::msg;
use derive_more::*;
use std::io::Error as StdIoError;
pub use wrapped::IoError as IoError;
mod wrapped;

#[derive(Debug, Display, From, PartialEq)]
pub enum Error {
    #[display(fmt = "{}", "msg::ERR_OVERFLOW_I32")]
    Overflow,
    InvalidInputError(std::num::ParseIntError),
    IoError(IoError),
    EnvVarError(std::env::VarError),
}

impl From<StdIoError> for Error {
    fn from(err: StdIoError) -> Self {
        Error::IoError(IoError(err))
    }
}
