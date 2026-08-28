use crate::errors::{MrlyError, Result};
use crate::Json;

impl From<serde_json::Error> for MrlyError {
    fn from(error: serde_json::Error) -> MrlyError {
        MrlyError::Value(format!("json: {error}"))
    }
}

/// Parses JSON text into a value, or an error naming where it broke.
///
/// ```
/// let v = mrlycore::json::parse(r#"{"tags": [3, 5]}"#).unwrap();
/// assert_eq!(v["tags"][1], 5);
/// assert!(mrlycore::json::parse("[1,").is_err());
/// ```
pub fn parse(text: &str) -> Result<Json> {
    Ok(serde_json::from_str(text)?)
}
