use super::*;

use std::{ffi::OsString, fmt, option::NoneError};
use std::num::ParseIntError;

#[derive(Clone, Debug, Fail, PartialEq)]
#[allow(pub_enum_variant_names)]
pub enum Error {
    InvalidUtf8Arg(OsString),
    NoneError,
    InvalidInt(ParseIntError),
    NoImportData,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Error::InvalidUtf8Arg(ref os_string) => format!("{}: {:?}", MSG_ERR_INVALID_UTF8_ARG, os_string),
            Error::NoneError => MSG_ERR_INTERNAL_NONE_ERROR.to_string(),
            Error::InvalidInt(ref string) => format!("{}: {:?}", MSG_ERR_INVALID_INT, string),
            Error::NoImportData => MSG_ERR_NO_IMPORT_DATA.to_owned(),
        })
    }
}

impl From<OsString> for Error {
    fn from(err: OsString) -> Self {
        Error::InvalidUtf8Arg(err)
    }
}

impl From<NoneError> for Error {
    fn from(_: NoneError) -> Self {
        Error::NoneError
    }
}

impl From<ParseIntError> for Error {
    fn from(err: ParseIntError) -> Self {
        Error::InvalidInt(err)
    }
}
