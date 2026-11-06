use mrly_error::{Error, Result, value_error};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(
    Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Code(#[doc(hidden)] pub u128);

impl Code {
    pub const fn get(self) -> u128 {
        self.0
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Code {
    type Err = Error;
    fn from_str(text: &str) -> Result<Code> {
        match text.parse() {
            Ok(bits) => Ok(Code(bits)),
            Err(_) => value_error(format!("code {text:?} is not a decimal number.")),
        }
    }
}

impl From<u64> for Code {
    fn from(bits: u64) -> Code {
        Code(bits as u128)
    }
}

impl From<u128> for Code {
    fn from(bits: u128) -> Code {
        Code(bits)
    }
}

impl From<Code> for u128 {
    fn from(code: Code) -> u128 {
        code.0
    }
}
