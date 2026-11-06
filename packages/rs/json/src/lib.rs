use mrly_error::{Error, Result};
use serde::Serialize;
use serde::de::DeserializeOwned;

pub use serde_json::{Number, Value as Json, json};

pub type Map = serde_json::Map<String, Json>;

fn fail(error: serde_json::Error) -> Error {
    Error::Json(error.to_string())
}

pub fn read<T: DeserializeOwned>(text: &str) -> Result<T> {
    serde_json::from_str(text).map_err(fail)
}

pub fn parse(text: &str) -> Result<Json> {
    read(text)
}

pub fn take<T: DeserializeOwned>(value: &Json) -> Result<T> {
    T::deserialize(value).map_err(fail)
}

pub fn write<T: Serialize>(value: &T) -> Result<String> {
    serde_json::to_string(value).map_err(fail)
}

pub fn pretty<T: Serialize>(value: &T) -> Result<String> {
    serde_json::to_string_pretty(value).map_err(fail)
}
