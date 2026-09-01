use crate::moire::{layer, Layer, Spec};
use mrlycore::errors::{value_error, Result};
use mrlynum::factor::lcm;

fn odd_blocks(span: u64, block: u64) -> u64 {
    block * (span / (2 * block)) + (span % (2 * block)).saturating_sub(block)
}

fn overlap(m: u64, n: u64) -> (u64, u64) {
    let grid = lcm(m as usize, n as usize) as u64;
    let (a, b) = (grid / m, grid / n);
    let both = (1..n)
        .step_by(2)
        .map(|k| odd_blocks((k + 1) * b, a) - odd_blocks(k * b, a))
        .sum();
    (both, grid)
}

/// Returns the exact Pearson correlation of the flat carpet layers at two scales, area-weighted on their lcm grid.
///
/// A layer at scale n is lit where the row and the column of the n by n grid are not both odd.
/// The correlation is exactly zero when the two odd scales are coprime, and zero by convention below scale two, where a layer is constant.
///
/// ```
/// assert_eq!(mrlylab::moire::pairs::correlation(3, 5), 0.0);
/// assert!(mrlylab::moire::pairs::correlation(3, 9) > 0.0);
/// ```
pub fn correlation(m: usize, n: usize) -> f64 {
    let (m, n) = (m as u64, n as u64);
    let (hm, hn) = (m / 2, n / 2);
    if hm == 0 || hn == 0 {
        return 0.0;
    }
    let (both, grid) = overlap(m, n);
    let cross = (both * m * n) as i128;
    let solo = (hm * hn * grid) as i128;
    let gap = cross - solo;
    if gap == 0 {
        return 0.0;
    }
    let scale = (grid * m * n) as f64;
    let covariance = (gap as f64 / scale) * ((cross + solo) as f64 / scale);
    let variance = |half: u64, side: u64| {
        let share = (half * half) as f64 / (side * side) as f64;
        share * (1.0 - share)
    };
    covariance / (variance(hm, m) * variance(hn, n)).sqrt()
}

/// The witness row of an odd scale: its correlation with every earlier odd scale from three, and the verdict the row gives.
#[derive(Clone, Debug, PartialEq)]
pub struct Witness {
    /// The scale on trial.
    pub scale: usize,
    /// The earlier odd scales, three up to the scale less two.
    pub scales: Vec<usize>,
    /// The exact correlation with each earlier scale.
    pub row: Vec<f64>,
    /// The largest correlation in the row, zero for an empty row.
    pub max: f64,
    /// The earlier scale carrying the largest correlation, zero when the row is clear.
    pub at: usize,
    /// Whether the row is exactly clear, which is the scale being prime.
    pub prime: bool,
}

/// Puts an odd scale of three or more on trial against every earlier odd scale, or an error for another scale.
///
/// ```
/// let trial = mrlylab::moire::pairs::witness(9).unwrap();
/// assert_eq!((trial.scales, trial.at, trial.prime), (vec![3, 5, 7], 3, false));
/// ```
pub fn witness(scale: usize) -> Result<Witness> {
    if scale < 3 || scale.is_multiple_of(2) {
        return value_error("the stack has odd scales from three.");
    }
    let scales: Vec<usize> = (3..scale).step_by(2).collect();
    let row: Vec<f64> = scales.iter().map(|&m| correlation(m, scale)).collect();
    let (mut max, mut at) = (0.0, 0);
    for (&m, &r) in scales.iter().zip(&row) {
        if r > max {
            (max, at) = (r, m);
        }
    }
    let prime = row.iter().all(|&r| r == 0.0);
    Ok(Witness {
        scale,
        scales,
        row,
        max,
        at,
        prime,
    })
}

/// Returns the Pearson correlation of two rendered carpet layers on their lcm grid, sampled rather than integrated.
pub fn sampled(m: usize, n: usize) -> f64 {
    let size = lcm(m, n);
    let mask = |number| {
        let params = Layer {
            size,
            ..Layer::new(Spec::new(7, 2, 2), number)
        };
        layer(&params).unwrap()
    };
    let (a, b) = (mask(m), mask(n));
    let mean = |v: &[bool]| v.iter().filter(|&&x| x).count() as f64 / v.len() as f64;
    let (ea, eb) = (mean(&a), mean(&b));
    let eab = a.iter().zip(&b).filter(|(&x, &y)| x && y).count() as f64 / a.len() as f64;
    (eab - ea * eb) / (ea * (1.0 - ea) * eb * (1.0 - eb)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn brute(m: u64, n: u64) -> (u64, u64) {
        let grid = lcm(m as usize, n as usize) as u64;
        let (a, b) = (grid / m, grid / n);
        let both = (0..grid)
            .filter(|&j| !(j / a).is_multiple_of(2) && !(j / b).is_multiple_of(2))
            .count() as u64;
        (both, grid)
    }

    #[test]
    fn the_closed_count_matches_the_lcm_grid() {
        for m in 2..30u64 {
            for n in 2..30u64 {
                assert_eq!(overlap(m, n), brute(m, n), "{m} {n}");
            }
        }
    }
}
