use super::Lattice;
use crate::bang::{Code, code_to_corners};
use mrly_core::error::Result;

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
    let corners = code_to_corners(Code::from(code), dimension, base)?;
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
