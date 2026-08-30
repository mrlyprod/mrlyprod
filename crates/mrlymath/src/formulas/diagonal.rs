use mrlycore::errors::{value_error, Result};
use mrlycore::tensor::Tensor;
use std::collections::BTreeMap;

/// The widest side a profile spans.
pub const WIDEST: usize = 1 << 18;

fn histogram(tile: &Tensor) -> BTreeMap<usize, u128> {
    let shape = &tile.shape;
    let mut out = BTreeMap::new();
    for (flat, &cell) in tile.bytes().iter().enumerate() {
        if cell == 0 {
            continue;
        }
        let mut rest = flat;
        let mut weight = 0;
        for &side in shape.iter().rev() {
            weight += rest % side;
            rest /= side;
        }
        *out.entry(weight).or_insert(0) += 1;
    }
    out
}

/// Counts the filled cells of the tile's level-fold power on every diagonal plane `x_1 + ... + x_D = s`.
///
/// The count is the coefficient of the digit polynomial, the level-fold product of the tile's
/// weight sums, so no cell of the power is ever built; the tile must be a hypercube.
///
/// ```
/// let gasket = mrlymath::bang::factory::create(126, 2, 3, 2, 1).unwrap();
/// let counts = mrlymath::formulas::profile_of_tile(&gasket, 4).unwrap();
/// assert_eq!(counts[15..=30].iter().copied().collect::<Vec<u128>>(), vec![81u128; 16]);
/// ```
pub fn profile_of_tile(tile: &Tensor, level: u32) -> Result<Vec<u128>> {
    let dimension = tile.shape.len();
    let number = tile.shape.first().copied().unwrap_or(0);
    if tile.shape.iter().any(|&side| side != number) {
        return value_error("the profile wants a hypercube tile.");
    }
    if level < 1 {
        return value_error("level must be at least 1.");
    }
    let side = match number.checked_pow(level) {
        Some(side) if side <= WIDEST => side,
        _ => return value_error(format!("the side must stay at or below {WIDEST}.")),
    };
    let weights = histogram(tile);
    let span = dimension * (side - 1) + 1;
    let mut poly = vec![0u128; span];
    poly[0] = 1;
    let mut step = 1usize;
    for _ in 0..level {
        let mut next = vec![0u128; span];
        for (exponent, &count) in poly.iter().enumerate() {
            if count == 0 {
                continue;
            }
            for (&weight, &multiplicity) in &weights {
                let slot = exponent + step * weight;
                match count
                    .checked_mul(multiplicity)
                    .and_then(|added| next[slot].checked_add(added))
                {
                    Some(total) => next[slot] = total,
                    None => return value_error("the slice counts overflow a u128."),
                }
            }
        }
        poly = next;
        step *= number;
    }
    Ok(poly)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::bang::factory;
    #[test]
    fn the_profile_sums_to_the_fill_and_matches_a_rendered_count() {
        for code in [7u128, 9, 11] {
            let tile = factory::create(code, 3, 2, 2, 1).unwrap();
            let counts = profile_of_tile(&tile, 3).unwrap();
            let rendered = factory::create(code, 3, 2, 2, 3).unwrap();
            assert_eq!(counts.iter().sum::<u128>(), u128::from(rendered.sum()));
            let mut direct = vec![0u128; counts.len()];
            for (flat, &cell) in rendered.bytes().iter().enumerate() {
                if cell != 0 {
                    direct[flat / 27 + flat % 27] += 1;
                }
            }
            assert_eq!(counts, direct, "code={code}");
        }
        let tile = factory::create(23, 3, 3, 2, 1).unwrap();
        assert_eq!(profile_of_tile(&tile, 1).unwrap(), [1, 3, 3, 6, 3, 3, 1]);
        assert!(profile_of_tile(&tile, 0).is_err());
        assert!(profile_of_tile(&mrlycore::atoms::ones_3d(2), 20).is_err());
    }
}
