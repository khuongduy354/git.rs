use std::io;

#[derive(Debug)]
pub enum dgitError {
    NoDirectory,
    InvalidCommit,
    InvalidIndex,
    IoError(io::Error),
}

impl std::fmt::Display for dgitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            dgitError::NoDirectory => write!(f, "No directory"),
            dgitError::InvalidCommit => write!(f, "Invalid commit"),
            dgitError::InvalidIndex => write!(f, "Invalid index"),
            dgitError::IoError(e) => write!(f, "IO error: {}", e),
        }
    }
}
impl From<io::Error> for dgitError {
    fn from(e: io::Error) -> Self {
        dgitError::IoError(e)
    }
}
