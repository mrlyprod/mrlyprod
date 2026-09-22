use crate::core::error::{value_error, Result};
use crate::core::tensor::Tensor;
use std::collections::HashSet;

/// The default residue base.
pub const BASE: usize = 2;

/// Builds a hypercube of the given side and rank, marking each cell whose coordinate residues are in the filled list.
///
/// ```
/// let carpet: Vec<Vec<u8>> = vec![vec![0, 0], vec![0, 1], vec![1, 0]];
/// let t = mrlyrs::math::rules::render(&carpet, 3, 2, 2).unwrap();
/// assert_eq!(t.bytes().unwrap(), vec![1, 1, 1, 1, 0, 1, 1, 1, 1]);
/// ```
///
/// # Errors
///
/// Errors when the number, the dimension or the base is below one.
pub fn render(filled: &[Vec<u8>], number: usize, dimension: usize, base: usize) -> Result<Tensor> {
    if number < 1 {
        return value_error("number must be at least 1.");
    }
    if dimension < 1 {
        return value_error("dimension must be at least 1.");
    }
    if base < 1 {
        return value_error("base must be at least 1.");
    }
    let wanted: HashSet<&[u8]> = filled.iter().map(Vec::as_slice).collect();
    let mut out = Tensor::new(vec![number; dimension]);
    let mut residue = vec![0u8; dimension];
    for flat in 0..out.size() {
        let mut rem = flat;
        for axis in (0..dimension).rev() {
            residue[axis] = ((rem % number) % base) as u8;
            rem /= number;
        }
        out.put(flat, i64::from(wanted.contains(residue.as_slice()) as u8));
    }
    Ok(out)
}

/// Returns every axis but the free one.
pub fn tree_axes(dimension: usize, free_axis: usize) -> Vec<usize> {
    (0..dimension).filter(|&axis| axis != free_axis).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn render_carpet_rule() {
        let t = render(&[vec![0, 0], vec![0, 1], vec![1, 0]], 3, 2, 2).unwrap();
        assert_eq!(t.bytes().unwrap(), vec![1, 1, 1, 1, 0, 1, 1, 1, 1]);
    }
    #[test]
    fn refuses_a_zero_number_dimension_or_base() {
        assert!(render(&[], 0, 2, 2).is_err());
        assert!(render(&[], 3, 0, 2).is_err());
        assert!(render(&[], 3, 2, 0).is_err());
    }
}
