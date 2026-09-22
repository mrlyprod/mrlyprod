use super::{Orientation, Projection, FILL, VOID};
use crate::core::error::Result;
use crate::core::tensor::Tensor;
use crate::math::two::Cell2d;

/// The projected cell: a triangle grid with its projection, orientation and start parity.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Cell6d {
    /// The triangle grid of type codes.
    pub cell: Cell2d,
    /// The projection that made the grid.
    pub projection: Projection,
    /// The way the hexagon points.
    pub orientation: Orientation,
    /// The parity of the first triangle.
    pub start: u8,
}

impl Cell6d {
    /// Builds a cell from its four parts.
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
    /// Returns the grid width in triangles.
    pub fn width(&self) -> usize {
        self.cell.width()
    }
    /// Returns the grid height in triangles.
    pub fn height(&self) -> usize {
        self.cell.height()
    }
    /// Swaps every fill triangle for a void and back.
    pub fn anti(mut self) -> Cell6d {
        let types = &mut self.cell.cell.types;
        for flat in 0..types.size() {
            let v = types.at(flat);
            if v == i64::from(FILL) {
                types.put(flat, i64::from(VOID));
            } else if v == i64::from(VOID) {
                types.put(flat, i64::from(FILL));
            }
        }
        self
    }
    /// Maps each triangle to one at or above the threshold, zero below.
    pub fn binarize(self, threshold: u8) -> Cell6d {
        Cell6d {
            cell: self.cell.binarize(threshold),
            ..self
        }
    }
    /// Binarizes the triangles at the threshold Otsu's method picks.
    pub fn binarize_otsu(self) -> Cell6d {
        Cell6d {
            cell: self.cell.binarize_otsu(),
            ..self
        }
    }
    /// Rounds each triangle to the mean of its masked neighborhood, wrapping on request.
    ///
    /// # Errors
    ///
    /// Errors when the mask does not match the cell's rank.
    pub fn blur(self, mask: &Tensor, wrap: bool) -> Result<Cell6d> {
        Ok(Cell6d {
            cell: self.cell.blur(mask, wrap)?,
            ..self
        })
    }
    /// Writes the value wherever the tiled mask is nonzero.
    ///
    /// # Errors
    ///
    /// Errors when the mask does not tile the cell.
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
    use crate::math::bang::Code;
    use crate::math::six::designs::iso_design;
    #[test]
    fn binarize_wrapper_keeps_projection_metadata() {
        let hex = iso_design(Code(23), 3, 1, 2).unwrap();
        let binarized = hex.clone().binarize(1);
        assert_eq!(binarized.projection, hex.projection);
        assert_eq!(binarized.orientation, hex.orientation);
        assert_eq!(binarized.start, hex.start);
    }
    #[test]
    fn perforate_wrapper_zero_mask_is_identity_6d() {
        let hex = iso_design(Code(23), 3, 1, 2).unwrap();
        let mask = Tensor::new(hex.cell.types().shape.clone());
        let perforated = hex.clone().perforate(&mask, 5).unwrap();
        assert_eq!(perforated.cell.types(), hex.cell.types());
    }
}
