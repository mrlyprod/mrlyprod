use mrly_error::{Result, value_error};
use mrly_tensor::Tensor;
use std::collections::BTreeMap;

pub const WIDEST: usize = 1 << 18;

fn histogram(tile: &Tensor) -> BTreeMap<usize, u128> {
    let shape = &tile.shape;
    let mut out = BTreeMap::new();
    for flat in 0..tile.size() {
        if tile.at(flat) == 0 {
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
