use derive_more::*;

#[derive(Debug, Display, From)]
pub struct IoError(pub std::io::Error);

impl PartialEq for IoError {
    fn eq(&self, rhs: &Self) -> bool {
        self.0.kind() == rhs.0.kind()
    }
}
