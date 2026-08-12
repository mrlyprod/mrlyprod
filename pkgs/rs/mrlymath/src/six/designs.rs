use super::geometry::{cut, iso, pro};
use super::models::Cell6d;
use crate::bang::universe::Code;
use crate::three;
use mrlycore::errors::Result;

/// Builds the coded 3d design and projects it isometrically.
pub fn iso_design(code: Code, number: usize, level: usize, base: usize) -> Result<Cell6d> {
    iso(&three::create(code, number, level, base)?)
}

/// Builds the coded 3d design and projects its facing sides.
pub fn pro_design(code: Code, number: usize, level: usize, base: usize) -> Result<Cell6d> {
    pro(&three::create(code, number, level, base)?)
}

/// Builds the coded 3d design and slices its central hexagon.
pub fn cut_design(code: Code, number: usize, level: usize, base: usize) -> Result<Cell6d> {
    cut(&three::create(code, number, level, base)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn menger_projections_build() {
        let i = iso_design(23, 3, 1, 2).unwrap();
        let p = pro_design(23, 3, 1, 2).unwrap();
        let c = cut_design(23, 3, 1, 2).unwrap();
        assert!(i.cell.types().sum() > 0);
        assert!(p.cell.types().sum() > 0);
        assert!(c.cell.types().sum() > 0);
    }
}
