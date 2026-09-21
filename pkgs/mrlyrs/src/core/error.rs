use super::Json;
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

impl From<serde_json::Error> for MrlyError {
    fn from(error: serde_json::Error) -> MrlyError {
        MrlyError::Value(format!("json: {error}"))
    }
}

/// Parses JSON text into a value, or an error naming where it broke.
///
/// ```
/// let v = mrlyrs::core::error::parse(r#"{"tags": [3, 5]}"#).unwrap();
/// assert_eq!(v["tags"][1], 5);
/// assert!(mrlyrs::core::error::parse("[1,").is_err());
/// ```
pub fn parse(text: &str) -> Result<Json> {
    Ok(serde_json::from_str(text)?)
}
