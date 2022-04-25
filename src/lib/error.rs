use std::io;

pub enum rgitError {
    NoDirectory,
    InvalidCommit,
    InvalidIndex,
    IoError(io::Error),
}

impl std::fmt::Display for rgitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            rgitError::NoDirectory => write!(f, "No directory"),
            rgitError::InvalidCommit => write!(f, "Invalid commit"),
            rgitError::InvalidIndex => write!(f, "Invalid index"),
            rgitError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}
impl From<io::Error> for rgitError {
    fn from(e: io::Error) -> Self {
        rgitError::IoError(e)
    }
}
