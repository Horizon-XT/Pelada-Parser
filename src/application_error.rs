use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ApplicationError {
    InvalidOperation,
    FileNotFound(String),
    InvalidInput(String),
    UnsupportedFile,
    IOError(String),
}

impl Error for ApplicationError {}

// Implement the Display trait for ApplicationError to provide a user-friendly error message
impl fmt::Display for ApplicationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ApplicationError::InvalidOperation => write!(f, ""),
            ApplicationError::FileNotFound(filename) => write!(f, "File: {} not found.", filename),
            ApplicationError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            ApplicationError::UnsupportedFile => write!(
                f,
                "Given file is not supported! Please, input a valid file: [.dat, .txt]"
            ),
            ApplicationError::IOError(msg) => write!(f, "IO Error while reading the file: {}", msg),
        }
    }
}
