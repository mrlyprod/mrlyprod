use super::{Orientation, Projection, FILL, VOID};
use crate::core::errors::Result;
use crate::core::tensor::Tensor;
use crate::math::two::Cell2d;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell6d {
    pub cell: Cell2d,
    pub projection: Projection,
    pub orientation: Orientation,
    pub start: u8,
}

impl Cell6d {
    pub fn new(
        cell: Cell2d,
        projection: Projection,
        orientation: Orientation,
        start: u8,
    ) -> Cell6d {
        Cell6d {
            cell,
            projection,
            orientation,
            start,
        }
    }
    pub fn width(&self) -> usize {
        self.cell.width()
    }
    pub fn height(&self) -> usize {
        self.cell.height()
    }
    pub fn anti(mut self) -> Cell6d {
        for v in self.cell.cell.types.bytes_mut().iter_mut() {
            if *v == FILL {
                *v = VOID;
            } else if *v == VOID {
                *v = FILL;
            }
        }
        self
    }
    pub fn binarize(self, threshold: u8) -> Cell6d {
        Cell6d {
            cell: self.cell.binarize(threshold),
            ..self
        }
    }
    pub fn binarize_otsu(self) -> Cell6d {
        Cell6d {
            cell: self.cell.binarize_otsu(),
            ..self
        }
    }
    pub fn blur(self, mask: &Tensor, wrap: bool) -> Result<Cell6d> {
        Ok(Cell6d {
            cell: self.cell.blur(mask, wrap)?,
            ..self
        })
    }
    pub fn perforate(self, mask: &Tensor, value: u8) -> Result<Cell6d> {
        Ok(Cell6d {
            cell: self.cell.perforate(mask, value)?,
            ..self
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::six::designs::iso_design;
    #[test]
    fn binarize_wrapper_keeps_projection_metadata() {
        let hex = iso_design(23, 3, 1, 2).unwrap();
        let binarized = hex.clone().binarize(1);
        assert_eq!(binarized.projection, hex.projection);
        assert_eq!(binarized.orientation, hex.orientation);
        assert_eq!(binarized.start, hex.start);
    }
    #[test]
    fn perforate_wrapper_zero_mask_is_identity_6d() {
        let hex = iso_design(23, 3, 1, 2).unwrap();
        let mask = Tensor::new(hex.cell.types().shape.clone());
        let perforated = hex.clone().perforate(&mask, 5).unwrap();
        assert_eq!(perforated.cell.types(), hex.cell.types());
    }
}
