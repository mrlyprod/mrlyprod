use crate::two::Cell2d;
use mrlycore::logs;
use std::f64::consts::LN_2;

/// Returns the grid's binary Shannon entropy in millibits.
pub fn entropy(grid: &Cell2d) -> i64 {
    let bytes = grid.types().bytes();
    let total = bytes.len();
    if total == 0 {
        return 0;
    }
    let ones = bytes.iter().filter(|&&b| b == 1).count();
    let p = ones as f64 / total as f64;
    if p == 0.0 || p == 1.0 {
        return 0;
    }
    let bits = -(p * (logs::ln(p) / LN_2) + (1.0 - p) * (logs::ln(1.0 - p) / LN_2));
    (bits * 1000.0).round() as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use mrlycore::tensor::Tensor;

    fn grid(bits: &[u8], side: usize) -> Cell2d {
        Cell2d::new(Tensor::of(bits.to_vec(), vec![side, side]))
    }

    #[test]
    fn empty_and_full_have_no_entropy() {
        assert_eq!(entropy(&grid(&[0, 0, 0, 0], 2)), 0);
        assert_eq!(entropy(&grid(&[1, 1, 1, 1], 2)), 0);
    }
    #[test]
    fn half_filled_is_one_bit() {
        assert_eq!(entropy(&grid(&[1, 0, 0, 1], 2)), 1000);
    }
    #[test]
    fn quarter_filled_matches_shannon() {
        assert_eq!(entropy(&grid(&[1, 0, 0, 0], 2)), 811);
    }
}
