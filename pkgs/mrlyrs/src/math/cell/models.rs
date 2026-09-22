use crate::core::cell::Cell;
use crate::core::cell::Mode;
use crate::core::colors::Color;
use crate::core::error::{shape_error, value_error, Result};
use crate::core::tensor::{Dtype, Tensor};
use std::collections::HashMap;

/// The two-dimensional cell.
pub type Cell2d = CellNd<2>;
/// The three-dimensional cell.
pub type Cell3d = CellNd<3>;

/// Returns the narrowest unsigned dtype that holds the peak value.
///
/// ```
/// use mrlyrs::core::tensor::Dtype;
/// assert_eq!(mrlyrs::math::cell::models::dtype_for(255), Dtype::U8);
/// assert_eq!(mrlyrs::math::cell::models::dtype_for(256), Dtype::U16);
/// assert_eq!(mrlyrs::math::cell::models::dtype_for(70000), Dtype::U32);
/// ```
pub fn dtype_for(peak: i64) -> Dtype {
    if peak <= Dtype::U8.max() {
        Dtype::U8
    } else if peak <= Dtype::U16.max() {
        Dtype::U16
    } else {
        Dtype::U32
    }
}

/// Returns the narrowest count dtype that fits the mask's popcount.
pub fn counting_dtype(mask: &Tensor) -> Dtype {
    dtype_for(mask.sum() as i64)
}

/// A cell whose tensor is pinned to N dimensions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CellNd<const N: usize> {
    /// The wrapped cell.
    pub cell: Cell,
}

impl<const N: usize> CellNd<N> {
    /// Builds a cell from an N-dimensional tensor of types.
    ///
    /// # Errors
    ///
    /// Errors when the tensor's rank is not N.
    pub fn new(types: Tensor) -> Result<CellNd<N>> {
        if types.shape.len() != N {
            return shape_error(format!(
                "a {N}d cell needs a {N}d tensor, got {}d.",
                types.shape.len()
            ));
        }
        Ok(CellNd {
            cell: Cell::new(types),
        })
    }
    /// Returns the size of axis 1.
    pub fn width(&self) -> usize {
        self.cell.types.shape[1]
    }
    /// Returns the size of axis 0.
    pub fn height(&self) -> usize {
        self.cell.types.shape[0]
    }
    /// Returns the tensor of types.
    pub fn types(&self) -> &Tensor {
        &self.cell.types
    }
    /// Swaps filled and empty sites.
    pub fn invert(self) -> CellNd<N> {
        CellNd {
            cell: self.cell.invert(),
        }
    }
    /// Inverts the cell.
    pub fn anti(self) -> CellNd<N> {
        self.invert()
    }
    /// Wraps the cell in count layers of the given value on every side.
    pub fn pad(self, count: usize, value: u8) -> CellNd<N> {
        CellNd {
            cell: self.cell.pad(count, value),
        }
    }
    /// Deepens the cell into its level-fold fractal.
    ///
    /// # Errors
    ///
    /// Errors below level one.
    pub fn fractal(self, level: usize) -> Result<CellNd<N>> {
        Ok(CellNd {
            cell: self.cell.fractal(level)?,
        })
    }
    /// Tags each site with its ring distance from the center.
    pub fn layers(self) -> CellNd<N> {
        CellNd {
            cell: self.cell.layers(Dtype::U8),
        }
    }
    /// Tags each site with its count of masked neighbors matching the target, wrapping on request.
    ///
    /// # Errors
    ///
    /// Errors when the mask does not match the cell's rank.
    pub fn neighbors(self, mask: &Tensor, target: u8, wrap: bool) -> Result<CellNd<N>> {
        let dtype = counting_dtype(mask);
        Ok(CellNd {
            cell: self.cell.neighbors(mask, target, wrap, dtype)?,
        })
    }
    /// Maps each site to one at or above the threshold, zero below.
    pub fn binarize(self, threshold: u8) -> CellNd<N> {
        CellNd {
            cell: self.cell.binarize(threshold),
        }
    }
    /// Binarizes the cell at the threshold Otsu's method picks.
    pub fn binarize_otsu(self) -> CellNd<N> {
        CellNd {
            cell: self.cell.binarize_otsu(),
        }
    }
    /// Rounds each site to the mean of its masked neighborhood, wrapping on request.
    ///
    /// # Errors
    ///
    /// Errors when the mask does not match the cell's rank.
    pub fn blur(self, mask: &Tensor, wrap: bool) -> Result<CellNd<N>> {
        Ok(CellNd {
            cell: self.cell.blur(mask, wrap)?,
        })
    }
    /// Writes the value wherever the tiled mask is nonzero.
    ///
    /// # Errors
    ///
    /// Errors when the mask does not tile the cell.
    pub fn perforate(self, mask: &Tensor, value: u8) -> Result<CellNd<N>> {
        Ok(CellNd {
            cell: self.cell.perforate(mask, value)?,
        })
    }
    /// Returns the Kronecker product of the two cells.
    pub fn combine(&self, other: &CellNd<N>) -> CellNd<N> {
        CellNd {
            cell: self.cell.combine(&other.cell),
        }
    }
    /// Colors each site by its type through the mapping in the given mode.
    pub fn paint(self, mapping: &HashMap<u8, Vec<Color>>, mode: Mode) -> CellNd<N> {
        CellNd {
            cell: self.cell.paint(mapping, mode),
        }
    }
}

impl CellNd<2> {
    /// Rotates the cell k quarter turns in the plane.
    ///
    /// # Errors
    ///
    /// Errors for a cell without two axes.
    pub fn rotate(self, k: usize) -> Result<Cell2d> {
        Ok(CellNd {
            cell: self.cell.rotate(k, (0, 1))?,
        })
    }
    /// Repeats the cell into a width-by-height array of copies.
    ///
    /// # Errors
    ///
    /// Errors for a cell without two axes.
    pub fn tile(self, width: usize, height: usize) -> Result<Cell2d> {
        Ok(CellNd {
            cell: self.cell.tile(&[height, width])?,
        })
    }
}

impl CellNd<3> {
    /// Returns the size of axis 2.
    pub fn depth(&self) -> usize {
        self.cell.types.shape[2]
    }
    /// Rotates the cell k quarter turns about the given pair of axes.
    ///
    /// # Errors
    ///
    /// Errors for axes the cell does not hold.
    pub fn rotate(self, k: usize, axes: (usize, usize)) -> Result<Cell3d> {
        Ok(CellNd {
            cell: self.cell.rotate(k, axes)?,
        })
    }
    /// Turns the cell into one of the 24 cube orientations.
    ///
    /// # Errors
    ///
    /// Errors past orientation twenty-three.
    pub fn orient(self, index: usize) -> Result<Cell3d> {
        let table = crate::math::three::orientations();
        match table.get(index) {
            Some(&(a, b, c)) => self.rotate(a, (1, 2))?.rotate(b, (0, 2))?.rotate(c, (0, 1)),
            None => value_error(format!("orientation index {index} out of range (0..23).")),
        }
    }
    /// Repeats the cell into a width-by-height-by-depth array of copies.
    ///
    /// # Errors
    ///
    /// Errors for a cell without three axes.
    pub fn tile(self, width: usize, height: usize, depth: usize) -> Result<Cell3d> {
        Ok(CellNd {
            cell: self.cell.tile(&[height, width, depth])?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::atoms;
    #[test]
    fn binarize_wrapper_thresholds_pointwise() {
        let cell = Cell2d::new(atoms::carpet_2d(3)).unwrap();
        let binarized = cell.clone().binarize(1);
        assert_eq!(binarized.types(), cell.types());
    }
    #[test]
    fn blur_keeps_the_shape_at_every_rank() {
        let flat = Cell2d::new(atoms::carpet_2d(3)).unwrap();
        let blurred = flat
            .clone()
            .blur(&Tensor::full(vec![3, 3], 1), true)
            .unwrap();
        assert_eq!(blurred.types().shape, flat.types().shape);
        let cube = Cell3d::new(atoms::carpet_3d(3)).unwrap();
        let blurred = cube
            .clone()
            .blur(&Tensor::full(vec![3, 3, 3], 1), true)
            .unwrap();
        assert_eq!(blurred.types().shape, cube.types().shape);
    }
    #[test]
    fn a_zero_mask_perforates_nothing_at_every_rank() {
        let flat = Cell2d::new(atoms::carpet_2d(3)).unwrap();
        let mask = Tensor::new(flat.types().shape.clone());
        assert_eq!(
            flat.clone().perforate(&mask, 5).unwrap().types(),
            flat.types()
        );
        let cube = Cell3d::new(atoms::carpet_3d(3)).unwrap();
        let mask = Tensor::new(cube.types().shape.clone());
        assert_eq!(
            cube.clone().perforate(&mask, 5).unwrap().types(),
            cube.types()
        );
    }
    #[test]
    fn counting_dtype_widens_with_the_mask() {
        assert_eq!(counting_dtype(&Tensor::full(vec![3, 3], 1)), Dtype::U8);
        assert_eq!(counting_dtype(&Tensor::full(vec![15, 15], 1)), Dtype::U8);
        assert_eq!(counting_dtype(&Tensor::full(vec![17, 17], 1)), Dtype::U16);
        assert_eq!(counting_dtype(&Tensor::full(vec![63, 63], 1)), Dtype::U16);
    }
    #[test]
    fn neighbors_wrapper_survives_a_wide_mask() {
        let mut mask = Tensor::full(vec![17, 17], 1);
        mask.set(&[8, 8], 0).unwrap();
        let grid = Cell2d::new(Tensor::full(vec![21, 21], 1)).unwrap();
        let counted = grid.neighbors(&mask, 1, true).unwrap();
        let tags = counted.cell.tags.as_ref().unwrap();
        assert_eq!(tags.at(0), 17 * 17 - 1);
    }

    #[test]
    fn refuses_a_tensor_of_the_wrong_rank() {
        assert!(Cell2d::new(Tensor::new(vec![2, 2, 2])).is_err());
        assert!(Cell3d::new(Tensor::new(vec![2, 2])).is_err());
        assert!(CellNd::<1>::new(Tensor::new(vec![2, 2])).is_err());
    }
}
