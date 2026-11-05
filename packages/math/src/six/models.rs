use super::{FILL, Orientation, Projection, VOID};
use crate::two::Cell2d;
use mrly_core::error::Result;
use mrly_core::tensor::Tensor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
