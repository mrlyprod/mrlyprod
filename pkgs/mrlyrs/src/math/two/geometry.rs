use super::models::Cell2d;
use crate::core::errors::{value_error, Result};
use crate::core::tensor::Tensor;
use crate::math::dim::geometry;

pub use crate::math::dim::geometry::{magic, mosaic, perforate};

pub fn merge(cells: &[Cell2d], width: usize, height: usize) -> Result<Cell2d> {
    geometry::merge_reps(cells, &[height, width])
}

pub fn special(mask: &Tensor, cell: &Cell2d) -> Result<Cell2d> {
    if mask.shape.len() != 2 {
        return value_error("special mask must be 2d.");
    }
    if mask.bytes().iter().any(|&v| v > 3) {
        return value_error("Invalid rotation value. Must be 0, 1, 2, or 3.");
    }
    let rotated: Vec<Cell2d> = mask
        .bytes()
        .iter()
        .map(|&k| cell.clone().rotate(k as usize))
        .collect();
    merge(&rotated, mask.shape[1], mask.shape[0])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::two::designs;
    #[test]
    fn special_rotations_preserve_sum() {
        let tree = designs::htree(3, 1).unwrap();
        let mask = Tensor::of(vec![0, 1, 3, 2], vec![2, 2]);
        let s = special(&mask, &tree).unwrap();
        assert_eq!(s.width(), 6);
        assert_eq!(s.types().sum(), 4 * tree.types().sum());
        assert!(special(&Tensor::of(vec![4], vec![1, 1]), &tree).is_err());
    }
    #[test]
    fn special_identity_mask_is_tile() {
        let c = designs::carpet(3, 1).unwrap();
        let mask = Tensor::new(vec![2, 3]);
        let s = special(&mask, &c).unwrap();
        assert_eq!(s, c.clone().tile(3, 2));
    }
    #[test]
    fn perforate_zero_mask_is_identity() {
        let c = designs::carpet(3, 1).unwrap();
        let mask = Tensor::new(c.types().shape.clone());
        let p = perforate(&mask, &c, 9).unwrap();
        assert_eq!(p, c);
    }
}
