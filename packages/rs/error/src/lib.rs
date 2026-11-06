use std::fmt;

#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    Value(String),
    Shape(String),
    Overflow(String),
    Json(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Value(message) => write!(f, "{message}"),
            Error::Shape(message) => write!(f, "{message}"),
            Error::Overflow(message) => write!(f, "{message}"),
            Error::Json(error) => write!(f, "json: {error}"),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

pub fn value_error<T>(message: impl Into<String>) -> Result<T> {
    Err(Error::Value(message.into()))
}

pub fn shape_error<T>(message: impl Into<String>) -> Result<T> {
    Err(Error::Shape(message.into()))
}

pub fn overflow_error<T>(message: impl Into<String>) -> Result<T> {
    Err(Error::Overflow(message.into()))
}
