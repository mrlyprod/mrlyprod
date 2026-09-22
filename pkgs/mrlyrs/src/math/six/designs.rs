use super::geometry::{cut, iso, pro};
use super::models::Cell6d;
use crate::core::error::Result;
use crate::math::bang::Code;
use crate::math::three;

/// Builds the coded 3d design and projects it isometrically.
///
/// # Errors
///
/// Errors when the code is out of range, or the level is below one.
pub fn iso_design(code: Code, number: usize, level: usize, base: usize) -> Result<Cell6d> {
    iso(&three::create(code, number, level, base)?)
}

/// Builds the coded 3d design and projects its facing sides.
///
/// # Errors
///
/// Errors when the code is out of range, or the level is below one.
pub fn pro_design(code: Code, number: usize, level: usize, base: usize) -> Result<Cell6d> {
    pro(&three::create(code, number, level, base)?)
}

/// Builds the coded 3d design and slices its central hexagon.
///
/// # Errors
///
/// Errors when the code is out of range, or the level is below one.
pub fn cut_design(code: Code, number: usize, level: usize, base: usize) -> Result<Cell6d> {
    cut(&three::create(code, number, level, base)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn menger_projections_build() {
        let i = iso_design(Code(23), 3, 1, 2).unwrap();
        let p = pro_design(Code(23), 3, 1, 2).unwrap();
        let c = cut_design(Code(23), 3, 1, 2).unwrap();
        assert!(i.cell.types().sum() > 0);
        assert!(p.cell.types().sum() > 0);
        assert!(c.cell.types().sum() > 0);
    }
}
