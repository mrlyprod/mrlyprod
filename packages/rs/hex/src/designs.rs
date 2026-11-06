use super::geometry::{cut, iso, pro};
use super::models::CellHex;
use mrly_error::Result;
use mrly_math::code::Code;
use mrly_three as three;

pub fn iso_design(code: Code, number: usize, level: usize, base: usize) -> Result<CellHex> {
    iso(&three::create(code, number, level, base)?)
}

pub fn pro_design(code: Code, number: usize, level: usize, base: usize) -> Result<CellHex> {
    pro(&three::create(code, number, level, base)?)
}

pub fn cut_design(code: Code, number: usize, level: usize, base: usize) -> Result<CellHex> {
    cut(&three::create(code, number, level, base)?)
}
