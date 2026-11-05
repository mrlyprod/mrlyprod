use mrly_core::error::{Result, overflow_error, value_error};
use mrly_core::tensor::Tensor;
use mrly_math::bang::Code;
use mrly_math::bang::factory;
use mrly_math::two;
use mrly_num::factor::gcd;

pub fn design_mask(dimension: usize, code: Code, number: usize, level: usize) -> Result<Tensor> {
    if !(1..=2).contains(&dimension) {
        return value_error("a mask lives in dimension 1 or 2.");
    }
    if number < 1 || number.is_multiple_of(2) {
        return value_error("the mask side must be odd.");
    }
    if level < 1 {
        return value_error("level must be at least 1.");
    }
    let Some(side) = number.checked_pow(level as u32) else {
        return overflow_error(format!("the mask side {number}^{level} overflows usize."));
    };
    let mut mask = if dimension == 1 {
        factory::create(code, number, 1, 2, level)?
    } else {
        two::create(code, number, level, 0, 2)?.types().clone()
    };
    let centre = (side - 1) / 2;
    mask.set(&vec![centre; dimension], 0)?;
    Ok(mask)
}

fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a.abs(), if a < 0 { -1 } else { 1 }, 0)
    } else {
        let (g, s, t) = egcd(b, a % b);
        (g, t, s - (a / b) * t)
    }
}

pub fn mask_offsets(mask: &Tensor) -> Vec<Vec<i64>> {
    let dimension = mask.shape.len();
    let centre: Vec<i64> = mask.shape.iter().map(|&n| (n as i64 - 1) / 2).collect();
    let mut out = Vec::new();
    for flat in 0..mask.size() {
        if mask.at(flat) != 1 {
            continue;
        }
        let mut rest = flat;
        let mut offset = vec![0i64; dimension];
        for axis in (0..dimension).rev() {
            offset[axis] = (rest % mask.shape[axis]) as i64 - centre[axis];
            rest /= mask.shape[axis];
        }
        if offset.iter().any(|&v| v != 0) {
            out.push(offset);
        }
    }
    out
}

pub fn lattice_index(mask: &Tensor) -> usize {
    let offsets = mask_offsets(mask);
    if mask.shape.len() == 1 {
        return offsets
            .iter()
            .fold(0u128, |g, v| gcd(g, v[0].unsigned_abs().into())) as usize;
    }
    let (mut pivot, mut shear) = (0i64, 0i64);
    for offset in &offsets {
        let (g, s, t) = egcd(pivot, offset[0]);
        shear = s * shear + t * offset[1];
        pivot = g;
    }
    if pivot == 0 {
        return 0;
    }
    let mut rise = 0u128;
    for offset in &offsets {
        let step = offset[1] - (offset[0] / pivot) * shear;
        rise = gcd(rise, step.unsigned_abs().into());
    }
    if rise == 0 {
        return 0;
    }
    (u128::from(pivot.unsigned_abs()) * rise) as usize
}
