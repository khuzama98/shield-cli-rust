use std::fmt;

#[derive(Debug)]
pub enum ShieldError {
    MissingArgument(String),
    IoError(std::io::Error),
    Other(String),
}

impl fmt::Display for ShieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShieldError::MissingArgument(arg) => write!(f, "Missing argument: {}", arg),
            ShieldError::IoError(err) => write!(f, "IO error: {}", err),
            ShieldError::Other(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for ShieldError {}
