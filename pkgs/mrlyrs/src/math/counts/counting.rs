use crate::core::error::{value_error, Result};
use crate::math::bang::factory;
use crate::math::bang::Code;
use crate::num::factor::reduce;

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
///
/// ```
/// use mrlyrs::math::bang::Code;
/// assert_eq!(mrlyrs::math::counts::fill(Code::from(7u64), 3, 2, 2, 2).unwrap(), 64);
/// ```
///
/// # Errors
///
/// Errors when the code is out of range for the dimension and base.
pub fn fill(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> Result<u128> {
    let filled = factory::code_to_corners(code, dimension, base)?;
    Ok(fill_from_corners(&filled, number, dimension, level, base))
}

/// Returns the empty cell count, grid minus fill.
///
/// # Errors
///
/// Errors when the code is out of range for the dimension and base.
pub fn void(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> Result<u128> {
    Ok(grid(number, dimension, level) - fill(code, number, dimension, level, base)?)
}

/// Returns the filled fraction of the grid, or 0.0 for an empty grid.
///
/// # Errors
///
/// Errors when the code is out of range for the dimension and base.
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
/// use mrlyrs::math::bang::Code;
/// assert_eq!(mrlyrs::math::counts::rational(Code::from(7u64), 3, 2, 2, 2).unwrap(), (64, 81));
/// ```
///
/// # Errors
///
/// Errors when the code is out of range for the dimension and base.
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
/// use mrlyrs::math::bang::Code;
/// assert_eq!(mrlyrs::math::counts::limit(Code::from(7u64), 2, 1, 2).unwrap(), (3, 4));
/// assert_eq!(mrlyrs::math::counts::limit(Code::from(7u64), 2, 2, 2).unwrap(), (9, 16));
/// ```
///
/// # Errors
///
/// Errors when the code is out of range, or the limit overflows at this level.
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
///
/// # Errors
///
/// Errors when the code is out of range for the dimension and base.
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
    use crate::math::bang::factory;
    #[test]
    fn fill_matches_rendered_sum() {
        for dimension in 2..=3usize {
            for code in [Code(0), Code(1), Code(7), Code(23), Code(100)] {
                if code >= factory::total_codes(dimension, 2).unwrap() {
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
        let d = dimension(Code(23), 3, 3, 2).unwrap();
        assert!((d - 2.7268).abs() < 0.001);
    }
    #[test]
    fn rational_reduces_the_exact_fraction() {
        assert_eq!(rational(Code(7), 3, 2, 1, 2).unwrap(), (8, 9));
        assert_eq!(rational(Code(7), 3, 2, 3, 2).unwrap(), (512, 729));
        assert_eq!(rational(Code(15), 5, 2, 2, 2).unwrap(), (1, 1));
        assert_eq!(rational(Code(0), 5, 2, 2, 2).unwrap(), (0, 1));
        assert_eq!(rational(Code(7), 0, 2, 1, 2).unwrap(), (0, 1));
        for number in 1..6usize {
            let (top, bottom) = rational(Code(23), number, 3, 2, 2).unwrap();
            let exact = fill(Code(23), number, 3, 2, 2).unwrap() as f64 / grid(number, 3, 2) as f64;
            assert!(
                (top as f64 / bottom as f64 - exact).abs() < 1e-12,
                "n={number}"
            );
        }
    }
    #[test]
    fn the_carpet_and_sponge_limits_stay_rational() {
        assert_eq!(limit(Code(7), 2, 1, 2).unwrap(), (3, 4));
        let sponge: Vec<Vec<u8>> = factory::residue_corners(3, 2)
            .into_iter()
            .filter(|corner| corner.iter().filter(|&&r| r == 1).count() <= 1)
            .collect();
        let code = factory::corners_to_code(&sponge, 3, 2);
        assert_eq!(limit(code, 3, 1, 2).unwrap(), (1, 2));
        assert_eq!(limit(code, 3, 3, 2).unwrap(), (1, 8));
        assert_eq!(limit(Code(0), 2, 1, 2).unwrap(), (0, 1));
        assert_eq!(limit(Code(15), 2, 4, 2).unwrap(), (1, 1));
    }
    #[test]
    fn refuses_a_code_past_its_range_and_a_level_past_a_u128() {
        assert!(fill(Code(16), 3, 2, 1, 2).is_err());
        assert!(void(Code(16), 3, 2, 1, 2).is_err());
        assert!(ratio(Code(16), 3, 2, 1, 2).is_err());
        assert!(rational(Code(16), 3, 2, 1, 2).is_err());
        assert!(dimension(Code(16), 3, 2, 2).is_err());
        assert!(fill(Code(1), 3, 7, 1, 2).is_err());
        assert!(limit(Code(16), 2, 1, 2).is_err());
        assert!(limit(Code(7), 2, 1000, 2).is_err());
    }
    #[test]
    fn wide_grids_walk_toward_the_limit() {
        let (top, bottom) = limit(Code(7), 2, 1, 2).unwrap();
        let target = top as f64 / bottom as f64;
        let mut last = f64::MAX;
        for number in [3usize, 9, 27, 81, 243] {
            let gap = (ratio(Code(7), number, 2, 1, 2).unwrap() - target).abs();
            assert!(gap < last, "n={number} gap {gap} did not shrink");
            last = gap;
        }
        assert!(last < 0.01);
    }
    #[test]
    fn fill_plus_void_is_grid() {
        for bits in 0..16u128 {
            let code = Code(bits);
            let f = fill(code, 4, 2, 2, 2).unwrap();
            let v = void(code, 4, 2, 2, 2).unwrap();
            assert_eq!(f + v, grid(4, 2, 2));
        }
    }
}
