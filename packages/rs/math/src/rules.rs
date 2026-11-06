use mrly_error::{Result, value_error};
use mrly_tensor::Tensor;
use std::collections::HashSet;

pub const BASE: usize = 2;

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

pub fn tree_axes(dimension: usize, free_axis: usize) -> Vec<usize> {
    (0..dimension).filter(|&axis| axis != free_axis).collect()
}
