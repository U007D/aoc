use crate::consts::msg;
use derive_more::*;
use std::io::Error as StdIoError;
pub use wrapped::IoError as IoError;
mod wrapped;

#[derive(Clone, Debug, Display, From, PartialEq)]
pub enum Error {
    EnvVarError(std::env::VarError),
    InvalidInputError(std::num::ParseIntError),
    IoError(IoError),
    #[display(fmt = "{}", "msg::ERR_OVERFLOW_I32")]
    Overflow,
    #[display(fmt = "{}", "msg::ERR_INTERNAL_EXHAUSTED_DELTA_VALUES")]
    ExhaustedDeltaValues,
}

impl From<StdIoError> for Error {
    fn from(err: StdIoError) -> Self {
        Error::IoError(IoError(err))
    }
}
