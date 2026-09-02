use crate::bang::factory;
use crate::bang::universe::Code;
use crate::two;
use mrlycore::errors::{value_error, Result};
use mrlycore::tensor::Tensor;

/// Builds the base-2 design mask a code names at an odd side grown to the given Kronecker level, its centre popped.
pub fn design_mask(dimension: usize, code: Code, number: usize, level: usize) -> Result<Tensor> {
    if !(1..=2).contains(&dimension) {
        return value_error("a mask lives in dimension 1 or 2.");
    }
    if number < 1 || number.is_multiple_of(2) {
        return value_error("the mask side must be odd.");
    }
    if level < 1 {
        return value_error("level must be at least 1.");
    }
    let mut mask = if dimension == 1 {
        factory::create(code, number, 1, 2, level)?
    } else {
        two::create(code, number, level, 0, 2)?.types().clone()
    };
    let centre = (number.pow(level as u32) - 1) / 2;
    mask.set(&vec![centre; dimension], 0);
    Ok(mask)
}

fn gcd(a: i64, b: i64) -> i64 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        let rest = a % b;
        a = b;
        b = rest;
    }
    a
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a.abs(), if a < 0 { -1 } else { 1 }, 0)
    } else {
        let (g, s, t) = egcd(b, a % b);
        (g, t, s - (a / b) * t)
    }
}

/// Returns the offsets a mask's filled sites take from its centre, the centre itself dropped.
pub fn mask_offsets(mask: &Tensor) -> Vec<Vec<i64>> {
    let dimension = mask.shape.len();
    let centre: Vec<i64> = mask.shape.iter().map(|&n| (n as i64 - 1) / 2).collect();
    let mut out = Vec::new();
    for flat in 0..mask.size() {
        if mask.bytes()[flat] != 1 {
            continue;
        }
        let mut rest = flat;
        let mut offset = vec![0i64; dimension];
        for axis in (0..dimension).rev() {
            offset[axis] = (rest % mask.shape[axis]) as i64 - centre[axis];
            rest /= mask.shape[axis];
        }
        if offset.iter().any(|&v| v != 0) {
            out.push(offset);
        }
    }
    out
}

/// Returns the index of the lattice the mask offsets generate together with the centre, zero when they do not span the dimension.
pub fn lattice_index(mask: &Tensor) -> usize {
    let offsets = mask_offsets(mask);
    if mask.shape.len() == 1 {
        return offsets.iter().fold(0i64, |g, v| gcd(g, v[0])) as usize;
    }
    let (mut pivot, mut shear) = (0i64, 0i64);
    for offset in &offsets {
        let (g, s, t) = egcd(pivot, offset[0]);
        shear = s * shear + t * offset[1];
        pivot = g;
    }
    if pivot == 0 {
        return 0;
    }
    let mut rise = 0i64;
    for offset in &offsets {
        rise = gcd(rise, offset[1] - (offset[0] / pivot) * shear);
    }
    if rise == 0 {
        return 0;
    }
    (pivot * rise) as usize
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn the_moore_mask_couples() {
        let mask = design_mask(2, 7, 3, 1).unwrap();
        assert_eq!((mask.shape.clone(), mask.sum()), (vec![3, 3], 8));
        assert_eq!(lattice_index(&mask), 1);
    }
    #[test]
    fn the_parity_tiles_alternate_by_side() {
        let read: Vec<usize> = [3, 5, 7, 9]
            .iter()
            .map(|&n| lattice_index(&design_mask(1, 1, n, 1).unwrap()))
            .collect();
        assert_eq!(read, vec![1, 2, 1, 2]);
    }
    #[test]
    fn the_cantor_tower_runs_one_two_one() {
        let read: Vec<usize> = (1..=3)
            .map(|level| lattice_index(&design_mask(1, 1, 3, level).unwrap()))
            .collect();
        assert_eq!(read, vec![1, 2, 1]);
    }
    #[test]
    fn the_diagonal_mask_decouples_and_the_von_neumann_one_does_not() {
        assert_eq!(lattice_index(&design_mask(2, 9, 3, 1).unwrap()), 2);
        assert_eq!(lattice_index(&design_mask(2, 6, 3, 1).unwrap()), 1);
    }
    #[test]
    fn a_rank_deficient_mask_reads_zero() {
        let flat = Tensor::of(vec![0, 0, 0, 1, 0, 1, 0, 0, 0], vec![3, 3]);
        assert_eq!(lattice_index(&flat), 0);
        assert_eq!(lattice_index(&Tensor::new(vec![3, 3])), 0);
    }
    #[test]
    fn even_sides_and_wide_dimensions_are_rejected() {
        assert!(design_mask(2, 7, 4, 1).is_err());
        assert!(design_mask(3, 7, 3, 1).is_err());
        assert!(design_mask(1, 1, 3, 0).is_err());
    }
    #[test]
    fn the_centre_is_popped_at_every_level() {
        let mask = design_mask(2, 7, 3, 2).unwrap();
        assert_eq!(mask.shape, vec![9, 9]);
        assert_eq!(mask.get(&[4, 4]), 0);
    }
}
