use super::Lattice;
use crate::bang::code_to_corners;
use mrlycore::errors::Result;

pub fn axes(size: usize, lattice: Lattice, row: usize) -> (Vec<f64>, Vec<f64>) {
    let inv = 1.0 / size as f64;
    let v = (row as f64 + 0.5) * inv;
    let mut a = Vec::with_capacity(size);
    let mut b = Vec::with_capacity(size);
    for col in 0..size {
        let u = (col as f64 + 0.5) * inv;
        match lattice {
            Lattice::Square => {
                a.push(u);
                b.push(v);
            }
            Lattice::Hex => {
                let sqrt3 = 3.0f64.sqrt();
                a.push(u - v / sqrt3);
                b.push(2.0 * v / sqrt3);
            }
        }
    }
    (a, b)
}

pub fn membership(code: u128, base: usize, dimension: usize) -> Result<Vec<bool>> {
    let corners = code_to_corners(code, dimension, base)?;
    let total = base.pow(dimension as u32);
    let mut table = vec![false; total];
    for corner in corners {
        let mut idx = 0usize;
        for &d in &corner {
            idx = idx * base + d as usize;
        }
        table[idx] = true;
    }
    Ok(table)
}

#[inline]
pub fn pack(residues: &[usize], base: usize) -> usize {
    let mut idx = 0usize;
    for &r in residues {
        idx = idx * base + r;
    }
    idx
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn square_axes_are_pixel_centres() {
        let (a, b) = axes(4, Lattice::Square, 0);
        assert!((a[0] - 0.125).abs() < 1e-12);
        assert!((b[0] - 0.125).abs() < 1e-12);
        assert!((a[3] - 0.875).abs() < 1e-12);
    }
    #[test]
    fn membership_matches_corners() {
        let t = membership(1, 2, 2).unwrap();
        assert_eq!(t.len(), 4);
        assert!(t[pack(&[0, 0], 2)]);
        assert!(!t[pack(&[1, 1], 2)]);
    }
    #[test]
    fn full_code_is_all_true() {
        let t = membership(15, 2, 2).unwrap();
        assert!(t.iter().all(|&x| x));
    }
}
