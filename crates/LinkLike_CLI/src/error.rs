use color_eyre::Result;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum CliError {
    InvalidInput(String),
    OperationFailed(String),
    IoError(std::io::Error),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            CliError::OperationFailed(msg) => write!(f, "Operation failed: {}", msg),
            CliError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl Error for CliError {}

pub type CliResult<T> = Result<T>;