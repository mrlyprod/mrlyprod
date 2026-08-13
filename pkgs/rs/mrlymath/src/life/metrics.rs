use crate::saga::spread;
use crate::two::Cell2d;
use mrlycore::logs;
use std::f64::consts::LN_2;

/// Returns the mean fraction of sites changed between consecutive grids.
pub fn churn(grids: &[Cell2d]) -> f64 {
    if grids.len() < 2 {
        return 0.0;
    }
    let total: f64 = grids
        .windows(2)
        .map(|pair| spread(pair[0].types(), pair[1].types()))
        .sum();
    total / (grids.len() - 1) as f64
}

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
    #[test]
    fn churn_averages_the_changed_fractions() {
        let a = grid(&[0, 0, 0, 0], 2);
        let b = grid(&[1, 0, 0, 0], 2);
        let c = grid(&[0, 1, 0, 0], 2);
        assert_eq!(churn(&[]), 0.0);
        assert_eq!(churn(std::slice::from_ref(&a)), 0.0);
        assert_eq!(churn(&[a.clone(), b.clone()]), 0.25);
        assert_eq!(churn(&[a.clone(), b, c]), 0.375);
        assert_eq!(churn(&[a.clone(), a]), 0.0);
    }
    #[test]
    fn churn_reads_the_spread_of_each_pair() {
        let flat = grid(&[0, 0, 0, 0], 2);
        let dots = grid(&[1, 0, 0, 1], 2);
        assert_eq!(spread(flat.types(), dots.types()), 0.5);
        assert_eq!(churn(&[flat, dots]), 0.5);
    }
}
