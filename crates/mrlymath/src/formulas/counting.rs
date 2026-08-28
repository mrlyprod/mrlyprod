use crate::bang::factory;
use crate::bang::universe::Code;
use mrlycore::errors::{value_error, Result};
use mrlynum::classics::reduce;

/// Counts the indices below number that equal residue modulo base.
pub fn positions(residue: usize, number: usize, base: usize) -> u128 {
    if residue >= number {
        return 0;
    }
    (number - residue).div_ceil(base) as u128
}

/// Returns the total cells of the grid, number to the dimension, to the level.
pub fn grid(number: usize, dimension: usize, level: u32) -> u128 {
    (number as u128).pow(dimension as u32).pow(level)
}

/// Sums each corner's position products into a base fill and raises it to the level.
pub fn fill_from_corners(
    filled: &[Vec<u8>],
    number: usize,
    _dimension: usize,
    level: u32,
    base: usize,
) -> u128 {
    let base_fill: u128 = filled
        .iter()
        .map(|corner| {
            corner
                .iter()
                .map(|&r| positions(r as usize, number, base))
                .product::<u128>()
        })
        .sum();
    base_fill.pow(level)
}

/// Returns the filled cell count of the code's fractal at the given level, without rendering it.
pub fn fill(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> Result<u128> {
    let filled = factory::code_to_corners(code, dimension, base)?;
    Ok(fill_from_corners(&filled, number, dimension, level, base))
}

/// Returns the empty cell count, grid minus fill.
pub fn void(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> Result<u128> {
    Ok(grid(number, dimension, level) - fill(code, number, dimension, level, base)?)
}

/// Returns the filled fraction of the grid, or 0.0 for an empty grid.
pub fn ratio(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> Result<f64> {
    let total = grid(number, dimension, level);
    if total == 0 {
        return Ok(0.0);
    }
    Ok(fill(code, number, dimension, level, base)? as f64 / total as f64)
}

/// Returns the exact filled fraction as a fraction of fill over grid, reduced.
///
/// ```
/// assert_eq!(mrlymath::formulas::rational(7, 3, 2, 2, 2).unwrap(), (64, 81));
/// ```
pub fn rational(
    code: Code,
    number: usize,
    dimension: usize,
    level: u32,
    base: usize,
) -> Result<(u128, u128)> {
    let total = grid(number, dimension, level);
    if total == 0 {
        return Ok((0, 1));
    }
    Ok(reduce(fill(code, number, dimension, level, base)?, total))
}

/// Returns the fill ratio the code walks toward as the side number grows, reduced.
///
/// The ratio is the corner count over the corner slots, so it is always rational: the
/// carpet holds three corners of four and the sponge four of eight.
///
/// ```
/// assert_eq!(mrlymath::formulas::limit(7, 2, 1, 2).unwrap(), (3, 4));
/// assert_eq!(mrlymath::formulas::limit(7, 2, 2, 2).unwrap(), (9, 16));
/// ```
pub fn limit(code: Code, dimension: usize, level: u32, base: usize) -> Result<(u128, u128)> {
    let corners = factory::code_to_corners(code, dimension, base)?.len() as u128;
    let slots = (base as u128).pow(dimension as u32);
    let overflow = || value_error("fill ratio limit overflows at this level.");
    let (Some(top), Some(bottom)) = (corners.checked_pow(level), slots.checked_pow(level)) else {
        return overflow();
    };
    Ok(reduce(top, bottom))
}

/// Returns the code's fractal dimension, the log of its one-level fill over the log of number.
pub fn dimension(code: Code, number: usize, base_dimension: usize, base: usize) -> Result<f64> {
    if number == 1 {
        return Ok(base_dimension as f64);
    }
    let f = fill(code, number, base_dimension, 1, base)?;
    if f == 0 {
        return Ok(0.0);
    }
    Ok((f as f64).ln() / (number as f64).ln())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bang::factory;
    #[test]
    fn fill_matches_rendered_sum() {
        for dimension in 2..=3usize {
            for code in [0u128, 1, 7, 23, 100] {
                if code >= factory::total_codes(dimension, 2) {
                    continue;
                }
                for number in 1..6 {
                    for level in 1..3 {
                        let rendered =
                            factory::create(code, number, dimension, 2, level as usize).unwrap();
                        assert_eq!(
                            fill(code, number, dimension, level, 2).unwrap(),
                            rendered.sum() as u128,
                            "code={code} d={dimension} n={number} l={level}"
                        );
                    }
                }
            }
        }
    }
    #[test]
    fn menger_dimension() {
        let d = dimension(23, 3, 3, 2).unwrap();
        assert!((d - 2.7268).abs() < 0.001);
    }
    #[test]
    fn rational_reduces_the_exact_fraction() {
        assert_eq!(rational(7, 3, 2, 1, 2).unwrap(), (8, 9));
        assert_eq!(rational(7, 3, 2, 3, 2).unwrap(), (512, 729));
        assert_eq!(rational(15, 5, 2, 2, 2).unwrap(), (1, 1));
        assert_eq!(rational(0, 5, 2, 2, 2).unwrap(), (0, 1));
        assert_eq!(rational(7, 0, 2, 1, 2).unwrap(), (0, 1));
        for number in 1..6usize {
            let (top, bottom) = rational(23, number, 3, 2, 2).unwrap();
            let exact = fill(23, number, 3, 2, 2).unwrap() as f64 / grid(number, 3, 2) as f64;
            assert!(
                (top as f64 / bottom as f64 - exact).abs() < 1e-12,
                "n={number}"
            );
        }
    }
    #[test]
    fn the_carpet_and_sponge_limits_stay_rational() {
        assert_eq!(limit(7, 2, 1, 2).unwrap(), (3, 4));
        let sponge: Vec<Vec<u8>> = factory::residue_corners(3, 2)
            .into_iter()
            .filter(|corner| corner.iter().filter(|&&r| r == 1).count() <= 1)
            .collect();
        let code = factory::corners_to_code(&sponge, 3, 2);
        assert_eq!(limit(code, 3, 1, 2).unwrap(), (1, 2));
        assert_eq!(limit(code, 3, 3, 2).unwrap(), (1, 8));
        assert_eq!(limit(0, 2, 1, 2).unwrap(), (0, 1));
        assert_eq!(limit(15, 2, 4, 2).unwrap(), (1, 1));
        assert!(limit(7, 2, 1000, 2).is_err());
    }
    #[test]
    fn wide_grids_walk_toward_the_limit() {
        let (top, bottom) = limit(7, 2, 1, 2).unwrap();
        let target = top as f64 / bottom as f64;
        let mut last = f64::MAX;
        for number in [3usize, 9, 27, 81, 243] {
            let gap = (ratio(7, number, 2, 1, 2).unwrap() - target).abs();
            assert!(gap < last, "n={number} gap {gap} did not shrink");
            last = gap;
        }
        assert!(last < 0.01);
    }
    #[test]
    fn fill_plus_void_is_grid() {
        for code in 0..16u128 {
            let f = fill(code, 4, 2, 2, 2).unwrap();
            let v = void(code, 4, 2, 2, 2).unwrap();
            assert_eq!(f + v, grid(4, 2, 2));
        }
    }
}
