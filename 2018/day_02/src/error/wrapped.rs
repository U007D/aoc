use derive_more::*;
use std::io::Error as StdIoError;

#[derive(Debug, Display, From)]
pub struct IoError(pub StdIoError);

impl Clone for IoError {
    fn clone(&self) -> Self {
        Self(StdIoError::from(self.0.kind()))
    }
}

impl PartialEq for IoError {
    fn eq(&self, rhs: &Self) -> bool {
        self.0.kind() == rhs.0.kind()
    }
}
