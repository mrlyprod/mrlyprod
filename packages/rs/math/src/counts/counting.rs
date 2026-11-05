use crate::bang::Code;
use crate::bang::factory;
use mrly_core::error::{Result, value_error};
use mrly_num::factor::reduce;

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

pub fn limit(code: Code, dimension: usize, level: u32, base: usize) -> Result<(u128, u128)> {
    let corners = factory::code_to_corners(code, dimension, base)?.len() as u128;
    let slots = (base as u128).pow(dimension as u32);
    let overflow = || value_error("fill ratio limit overflows at this level.");
    let (Some(top), Some(bottom)) = (corners.checked_pow(level), slots.checked_pow(level)) else {
        return overflow();
    };
    Ok(reduce(top, bottom))
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
