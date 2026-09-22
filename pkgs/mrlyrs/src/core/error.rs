use super::Json;
use std::fmt;

/// The one error type of the crate.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// A value that broke a rule, carrying the message.
    Value(String),
    /// A length, extent or dtype that does not match, carrying the message.
    Shape(String),
    /// A count that runs past the width of its integer, carrying the message.
    Overflow(String),
    /// A json text that would not parse, carrying the reader's own error.
    Json(serde_json::Error),
    /// A png or gif codec that refused, carrying its message.
    Codec(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Value(message) => write!(f, "{message}"),
            Error::Shape(message) => write!(f, "{message}"),
            Error::Overflow(message) => write!(f, "{message}"),
            Error::Json(error) => write!(f, "json: {error}"),
            Error::Codec(message) => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Json(error) => Some(error),
            _ => None,
        }
    }
}

/// The crate's result, erring with Error.
pub type Result<T> = std::result::Result<T, Error>;

/// Wraps a message in an Err of the value variant.
///
/// # Errors
///
/// Always errs; the Ok side is only there to fit the caller's return type.
pub fn value_error<T>(message: impl Into<String>) -> Result<T> {
    Err(Error::Value(message.into()))
}

/// Wraps a message in an Err of the shape variant.
///
/// # Errors
///
/// Always errs; the Ok side is only there to fit the caller's return type.
pub fn shape_error<T>(message: impl Into<String>) -> Result<T> {
    Err(Error::Shape(message.into()))
}

/// Wraps a message in an Err of the overflow variant.
///
/// # Errors
///
/// Always errs; the Ok side is only there to fit the caller's return type.
pub fn overflow_error<T>(message: impl Into<String>) -> Result<T> {
    Err(Error::Overflow(message.into()))
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Error {
        Error::Json(error)
    }
}

impl From<png::EncodingError> for Error {
    fn from(error: png::EncodingError) -> Error {
        Error::Codec(error.to_string())
    }
}

impl From<png::DecodingError> for Error {
    fn from(error: png::DecodingError) -> Error {
        Error::Codec(error.to_string())
    }
}

impl From<gif::EncodingError> for Error {
    fn from(error: gif::EncodingError) -> Error {
        Error::Codec(error.to_string())
    }
}

/// Parses JSON text into a value.
///
/// # Errors
///
/// Errs when the text is not valid json, naming where it broke.
///
/// ```
/// let v = mrlyrs::core::error::parse(r#"{"tags": [3, 5]}"#).unwrap();
/// assert_eq!(v["tags"][1], 5);
/// assert!(mrlyrs::core::error::parse("[1,").is_err());
/// ```
pub fn parse(text: &str) -> Result<Json> {
    Ok(serde_json::from_str(text)?)
}
