use super::models::Cell3d;
use crate::core::errors::{value_error, Result};
use crate::core::tensor::Tensor;
use crate::math::dim::geometry;
use std::sync::OnceLock;

pub use crate::math::dim::geometry::{magic, mosaic, perforate};

pub fn orientations() -> &'static Vec<(usize, usize, usize)> {
    static TABLE: OnceLock<Vec<(usize, usize, usize)>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut probe = Tensor::new(vec![3, 3, 3]);
        for (flat, item) in probe.bytes_mut().iter_mut().enumerate() {
            *item = flat as u8;
        }
        let mut seen: Vec<Vec<u8>> = Vec::new();
        let mut table = Vec::new();
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    let image = probe.rot90(a, (1, 2)).rot90(b, (0, 2)).rot90(c, (0, 1));
                    if !seen.contains(&image.bytes().to_vec()) {
                        seen.push(image.bytes().to_vec());
                        table.push((a, b, c));
                    }
                }
            }
        }
        table
    })
}

pub fn merge(cells: &[Cell3d], width: usize, height: usize, depth: usize) -> Result<Cell3d> {
    geometry::merge_reps(cells, &[height, width, depth])
}

pub fn special(mask: &Tensor, cell: &Cell3d) -> Result<Cell3d> {
    if mask.shape.len() != 3 {
        return value_error("special mask must be 3d.");
    }
    if mask.bytes().iter().any(|&v| v > 23) {
        return value_error("Invalid orientation value. Must be 0..23.");
    }
    let oriented: Result<Vec<Cell3d>> = mask
        .bytes()
        .iter()
        .map(|&k| cell.clone().orient(k as usize))
        .collect();
    let oriented = oriented?;
    merge(&oriented, mask.shape[1], mask.shape[0], mask.shape[2])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::three::designs;
    #[test]
    fn exactly_24_orientations() {
        let table = orientations();
        assert_eq!(table.len(), 24);
        assert_eq!(table[0], (0, 0, 0));
    }
    #[test]
    fn orientations_preserve_sum_and_shape() {
        let c = designs::carpet(3, 1).unwrap();
        for i in 0..24 {
            let o = c.clone().orient(i).unwrap();
            assert_eq!(o.types().sum(), c.types().sum());
            assert_eq!(o.types().shape, c.types().shape);
        }
        assert!(c.clone().orient(24).is_err());
    }
    #[test]
    fn orientations_are_distinct_on_chiral_design() {
        let tree = designs::xtree(3, 1).unwrap();
        let images: Vec<Vec<u8>> = (0..24)
            .map(|i| tree.clone().orient(i).unwrap().types().bytes().to_vec())
            .collect();
        let mut unique = images.clone();
        unique.sort();
        unique.dedup();
        assert!(unique.len() >= 3);
    }
    #[test]
    fn special_identity_is_tile() {
        let c = designs::carpet(3, 1).unwrap();
        let mask = Tensor::new(vec![2, 2, 2]);
        let s = special(&mask, &c).unwrap();
        assert_eq!(s, c.clone().tile(2, 2, 2));
    }
    #[test]
    fn perforate_zero_mask_is_identity() {
        let c = designs::carpet(3, 1).unwrap();
        let mask = Tensor::new(c.types().shape.clone());
        let p = perforate(&mask, &c, 9).unwrap();
        assert_eq!(p, c);
    }
}
