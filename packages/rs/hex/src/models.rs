use super::{FILL, Orientation, Projection, VOID};
use mrly_error::Result;
use mrly_tensor::Tensor;
use mrly_two::Cell2d;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CellHex {
    pub cell: Cell2d,
    pub projection: Projection,
    pub orientation: Orientation,
    pub start: u8,
}

impl CellHex {
    pub fn new(
        cell: Cell2d,
        projection: Projection,
        orientation: Orientation,
        start: u8,
    ) -> CellHex {
        CellHex {
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
    pub fn anti(mut self) -> CellHex {
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
    pub fn binarize(self, threshold: u8) -> CellHex {
        CellHex {
            cell: self.cell.binarize(threshold),
            ..self
        }
    }
    pub fn binarize_otsu(self) -> CellHex {
        CellHex {
            cell: self.cell.binarize_otsu(),
            ..self
        }
    }
    pub fn blur(self, mask: &Tensor, wrap: bool) -> Result<CellHex> {
        Ok(CellHex {
            cell: self.cell.blur(mask, wrap)?,
            ..self
        })
    }
    pub fn perforate(self, mask: &Tensor, value: u8) -> Result<CellHex> {
        Ok(CellHex {
            cell: self.cell.perforate(mask, value)?,
            ..self
        })
    }
}
