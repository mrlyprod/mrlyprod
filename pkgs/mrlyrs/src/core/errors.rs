use thiserror::Error;

#[derive(Debug, Error)]
pub enum MrlyError {
    #[error("{0}")]
    Value(String),
}

pub type Result<T> = std::result::Result<T, MrlyError>;

pub fn value_error<T>(message: impl Into<String>) -> Result<T> {
    Err(MrlyError::Value(message.into()))
}
