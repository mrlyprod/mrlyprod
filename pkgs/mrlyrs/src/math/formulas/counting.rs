use crate::core::errors::Result;
use crate::math::bang::factory;
use crate::math::bang::universe::Code;

pub fn positions(residue: usize, number: usize, base: usize) -> u128 {
    if residue >= number {
        return 0;
    }
    (number - residue).div_ceil(base) as u128
}

pub fn grid(number: usize, dimension: usize, level: u32) -> u128 {
    (number as u128).pow(dimension as u32).pow(level)
}

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

pub fn fill(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> Result<u128> {
    let filled = factory::code_to_corners(code, dimension, base)?;
    Ok(fill_from_corners(&filled, number, dimension, level, base))
}

pub fn void(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> Result<u128> {
    Ok(grid(number, dimension, level) - fill(code, number, dimension, level, base)?)
}

pub fn ratio(code: Code, number: usize, dimension: usize, level: u32, base: usize) -> Result<f64> {
    let total = grid(number, dimension, level);
    if total == 0 {
        return Ok(0.0);
    }
    Ok(fill(code, number, dimension, level, base)? as f64 / total as f64)
}

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
    fn fill_plus_void_is_grid() {
        for code in 0..16u128 {
            let f = fill(code, 4, 2, 2, 2).unwrap();
            let v = void(code, 4, 2, 2, 2).unwrap();
            assert_eq!(f + v, grid(4, 2, 2));
        }
    }
}
