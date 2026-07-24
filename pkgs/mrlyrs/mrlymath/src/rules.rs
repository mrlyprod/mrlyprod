use mrlycore::errors::{value_error, Result};
use mrlycore::tensor::Tensor;

pub const BASE: usize = 2;

pub fn render<F>(rule: F, number: usize, dimension: usize, base: usize) -> Result<Tensor>
where
    F: Fn(&[u8]) -> bool,
{
    if number < 1 {
        return value_error("number must be at least 1.");
    }
    if dimension < 1 {
        return value_error("dimension must be at least 1.");
    }
    if base < 1 {
        return value_error("base must be at least 1.");
    }
    let mut out = Tensor::new(vec![number; dimension]);
    let mut residue = vec![0u8; dimension];
    for flat in 0..out.size() {
        let mut rem = flat;
        for axis in (0..dimension).rev() {
            residue[axis] = ((rem % number) % base) as u8;
            rem /= number;
        }
        out.bytes_mut()[flat] = rule(&residue) as u8;
    }
    Ok(out)
}

pub fn tree_axes(dimension: usize, free_axis: usize) -> Vec<usize> {
    (0..dimension).filter(|&axis| axis != free_axis).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn render_carpet_rule() {
        let t = render(
            |p| p.iter().map(|&b| b as usize).sum::<usize>() <= 1,
            3,
            2,
            2,
        )
        .unwrap();
        assert_eq!(t.bytes(), vec![1, 1, 1, 1, 0, 1, 1, 1, 1]);
    }
    #[test]
    fn render_rejects_bad_input() {
        assert!(render(|_| true, 0, 2, 2).is_err());
        assert!(render(|_| true, 3, 0, 2).is_err());
    }
}
