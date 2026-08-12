use std::error::Error;
use std::fmt;

/// The one error type of the crate.
#[derive(Debug)]
pub enum MrlyError {
    /// A value that broke a rule, carrying the message.
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

/// The crate's result, erring with MrlyError.
pub type Result<T> = std::result::Result<T, MrlyError>;

/// Wraps a message in an Err of the value variant.
pub fn value_error<T>(message: impl Into<String>) -> Result<T> {
    Err(MrlyError::Value(message.into()))
}
