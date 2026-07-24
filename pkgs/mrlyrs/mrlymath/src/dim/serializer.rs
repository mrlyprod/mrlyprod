use mrlycore::errors::{MrlyError, Result};
use serde::de::DeserializeOwned;
use serde_json::Value;

pub fn parse(text: &str) -> Result<Value> {
    serde_json::from_str(text).map_err(|e| MrlyError::Value(e.to_string()))
}

pub fn types_field(data: &Value) -> Result<&Value> {
    data.get("types")
        .ok_or_else(|| MrlyError::Value("missing types.".to_string()))
}

pub fn field<T: DeserializeOwned>(value: &Value) -> Result<T> {
    serde_json::from_value(value.clone()).map_err(|e| MrlyError::Value(e.to_string()))
}
