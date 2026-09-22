use crate::core::error::{value_error, Error, Result};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// The bitmask of filled corners that names a design.
///
/// It prints, parses and serialises as the bare decimal number.
///
/// ```
/// use mrlyrs::math::bang::Code;
/// assert_eq!(Code::from(402u64).to_string(), "402");
/// assert_eq!("402".parse::<Code>().unwrap().get(), 402);
/// ```
#[derive(
    Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Default, Serialize, Deserialize,
)]
#[serde(transparent)]
pub struct Code(pub(crate) u128);

impl Code {
    /// Returns the bitmask the code carries.
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn a_code_is_the_bare_decimal_number_in_every_form() {
        let code = Code::from(402u64);
        assert_eq!(code.to_string(), "402");
        assert_eq!(serde_json::to_string(&code).unwrap(), "402");
        assert_eq!(serde_json::from_str::<Code>("402").unwrap(), code);
        assert_eq!("402".parse::<Code>().unwrap(), code);
        assert!("x".parse::<Code>().is_err());
    }
}
