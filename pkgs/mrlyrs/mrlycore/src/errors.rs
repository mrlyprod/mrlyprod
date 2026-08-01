use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MrlyError {
    Value(String),
}

impl fmt::Display for MrlyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MrlyError::Value(message) => write!(f, "{message}"),
        }
    }
}

impl Error for MrlyError {}

pub type Result<T> = std::result::Result<T, MrlyError>;

pub fn value_error<T>(message: impl Into<String>) -> Result<T> {
    Err(MrlyError::Value(message.into()))
}
