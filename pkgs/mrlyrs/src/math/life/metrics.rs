use crate::math::two::Cell2d;

pub fn entropy(grid: &Cell2d) -> f64 {
    let bytes = grid.types().bytes();
    let total = bytes.len();
    if total == 0 {
        return 0.0;
    }
    let ones = bytes.iter().filter(|&&b| b == 1).count();
    let p = ones as f64 / total as f64;
    if p == 0.0 || p == 1.0 {
        return 0.0;
    }
    -(p * p.log2() + (1.0 - p) * (1.0 - p).log2())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::tensor::Tensor;

    fn grid(bits: &[u8], side: usize) -> Cell2d {
        Cell2d::new(Tensor::of(bits.to_vec(), vec![side, side]))
    }

    #[test]
    fn empty_and_full_have_no_entropy() {
        assert_eq!(entropy(&grid(&[0, 0, 0, 0], 2)), 0.0);
        assert_eq!(entropy(&grid(&[1, 1, 1, 1], 2)), 0.0);
    }
    #[test]
    fn half_filled_is_one_bit() {
        assert_eq!(entropy(&grid(&[1, 0, 0, 1], 2)), 1.0);
    }
    #[test]
    fn quarter_filled_matches_shannon() {
        let h = entropy(&grid(&[1, 0, 0, 0], 2));
        assert!((h - 0.8112781244591328).abs() < 1e-9);
    }
}
