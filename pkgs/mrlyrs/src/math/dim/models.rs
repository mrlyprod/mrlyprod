use crate::core::cell::Cell;
use crate::core::colors::Color;
use crate::core::enums::Mode;
use crate::core::errors::{value_error, Result};
use crate::core::tensor::{Dtype, Tensor};
use std::collections::HashMap;

pub type Cell2d = CellNd<2>;
pub type Cell3d = CellNd<3>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellNd<const N: usize> {
    pub cell: Cell,
}

impl<const N: usize> CellNd<N> {
    pub fn new(types: Tensor) -> CellNd<N> {
        assert_eq!(types.shape.len(), N, "CellNd requires a {N}d tensor");
        CellNd {
            cell: Cell::new(types),
        }
    }
    pub fn width(&self) -> usize {
        self.cell.types.shape[1]
    }
    pub fn height(&self) -> usize {
        self.cell.types.shape[0]
    }
    pub fn types(&self) -> &Tensor {
        &self.cell.types
    }
    pub fn invert(self) -> CellNd<N> {
        CellNd {
            cell: self.cell.invert(),
        }
    }
    pub fn anti(self) -> CellNd<N> {
        self.invert()
    }
    pub fn pad(self, count: usize, value: u8) -> CellNd<N> {
        CellNd {
            cell: self.cell.pad(count, value),
        }
    }
    pub fn fractal(self, level: usize) -> Result<CellNd<N>> {
        Ok(CellNd {
            cell: self.cell.fractal(level)?,
        })
    }
    pub fn layers(self) -> CellNd<N> {
        CellNd {
            cell: self.cell.layers(Dtype::U8),
        }
    }
    pub fn neighbors(self, mask: &Tensor, target: u8, wrap: bool) -> Result<CellNd<N>> {
        Ok(CellNd {
            cell: self.cell.neighbors(mask, target, wrap, Dtype::U8)?,
        })
    }
    pub fn binarize(self, threshold: u8) -> CellNd<N> {
        CellNd {
            cell: self.cell.binarize(threshold),
        }
    }
    pub fn binarize_otsu(self) -> CellNd<N> {
        CellNd {
            cell: self.cell.binarize_otsu(),
        }
    }
    pub fn blur(self, mask: &Tensor, wrap: bool) -> Result<CellNd<N>> {
        Ok(CellNd {
            cell: self.cell.blur(mask, wrap)?,
        })
    }
    pub fn perforate(self, mask: &Tensor, value: u8) -> Result<CellNd<N>> {
        Ok(CellNd {
            cell: self.cell.perforate(mask, value)?,
        })
    }
    pub fn combine(&self, other: &CellNd<N>) -> CellNd<N> {
        CellNd {
            cell: self.cell.combine(&other.cell),
        }
    }
    pub fn paint(self, mapping: &HashMap<u8, Vec<Color>>, mode: Mode) -> CellNd<N> {
        CellNd {
            cell: self.cell.paint(mapping, mode),
        }
    }
}

impl CellNd<2> {
    pub fn rotate(self, k: usize) -> Cell2d {
        CellNd {
            cell: self.cell.rotate(k, (0, 1)),
        }
    }
    pub fn tile(self, width: usize, height: usize) -> Cell2d {
        CellNd {
            cell: self.cell.tile(&[height, width]),
        }
    }
}

impl CellNd<3> {
    pub fn depth(&self) -> usize {
        self.cell.types.shape[2]
    }
    pub fn rotate(self, k: usize, axes: (usize, usize)) -> Cell3d {
        CellNd {
            cell: self.cell.rotate(k, axes),
        }
    }
    pub fn orient(self, index: usize) -> Result<Cell3d> {
        let table = crate::math::three::orientations();
        match table.get(index) {
            Some(&(a, b, c)) => Ok(self.rotate(a, (1, 2)).rotate(b, (0, 2)).rotate(c, (0, 1))),
            None => value_error(format!("orientation index {index} out of range (0..23).")),
        }
    }
    pub fn tile(self, width: usize, height: usize, depth: usize) -> Cell3d {
        CellNd {
            cell: self.cell.tile(&[height, width, depth]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::atoms;
    #[test]
    fn binarize_wrapper_thresholds_pointwise() {
        let cell = Cell2d::new(atoms::carpet_2d(3));
        let binarized = cell.clone().binarize(1);
        assert_eq!(binarized.types(), cell.types());
    }
    #[test]
    fn blur_wrapper_preserves_shape() {
        let cell = Cell2d::new(atoms::carpet_2d(3));
        let mask = Tensor::full(vec![3, 3], 1);
        let blurred = cell.clone().blur(&mask, true).unwrap();
        assert_eq!(blurred.types().shape, cell.types().shape);
    }
    #[test]
    fn perforate_wrapper_zero_mask_is_identity() {
        let cell = Cell2d::new(atoms::carpet_2d(3));
        let mask = Tensor::new(cell.types().shape.clone());
        let perforated = cell.clone().perforate(&mask, 5).unwrap();
        assert_eq!(perforated.types(), cell.types());
    }
    #[test]
    fn blur_wrapper_preserves_shape_3d() {
        let cell = Cell3d::new(atoms::carpet_3d(3));
        let mask = Tensor::full(vec![3, 3, 3], 1);
        let blurred = cell.clone().blur(&mask, true).unwrap();
        assert_eq!(blurred.types().shape, cell.types().shape);
    }
    #[test]
    fn perforate_wrapper_zero_mask_is_identity_3d() {
        let cell = Cell3d::new(atoms::carpet_3d(3));
        let mask = Tensor::new(cell.types().shape.clone());
        let perforated = cell.clone().perforate(&mask, 5).unwrap();
        assert_eq!(perforated.types(), cell.types());
    }
}
