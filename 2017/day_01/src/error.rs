use crate::consts::*;
use failure::Fail;
use std::{
    fmt::{
        Display,
        Formatter,
        Result as FmtResult,
    },
    option::NoneError,
};

#[derive(Debug, Fail, PartialEq)]
pub enum Error {
    SumOverflowed,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", match self {
            Error::SumOverflowed => format!("{}.", msg::ERR_SUM_OVERFLOWED),
        })
    }
}
impl From<NoneError> for Error {
    fn from(_: NoneError) -> Self {
        Error::SumOverflowed
    }
}
