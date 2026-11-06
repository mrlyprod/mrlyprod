use super::code::Code;
use crate::name::Bang;
use crate::rules;
use mrly_error::{Result, overflow_error, value_error};
use mrly_name::Named;
use mrly_rand::Rng;
use mrly_tensor::Tensor;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MagicLayer {
    pub design: Bang,
    pub number: usize,
}

impl MagicLayer {
    pub fn new(design: Bang, number: usize) -> MagicLayer {
        MagicLayer { design, number }
    }
}

pub fn residue_corners(dimension: usize, base: usize) -> Vec<Vec<u8>> {
    let count = base.pow(dimension as u32);
    (0..count)
        .map(|i| {
            (0..dimension)
                .map(|j| ((i / base.pow((dimension - 1 - j) as u32)) % base) as u8)
                .collect()
        })
        .collect()
}

pub fn total_codes(dimension: usize, base: usize) -> Result<Code> {
    let cells = base.pow(dimension as u32);
    if cells >= 128 {
        return overflow_error(format!(
            "dimension {dimension} base {base} has {cells} corners, past the 127 a u128 code holds."
        ));
    }
    Ok(Code(1 << cells))
}

pub fn code_to_corners(code: Code, dimension: usize, base: usize) -> Result<Vec<Vec<u8>>> {
    let cells = residue_corners(dimension, base);
    let code = code.get();
    if cells.len() >= 128 {
        return overflow_error(format!(
            "dimension {dimension} base {base} has {} corners, past the 127 a u128 code holds.",
            cells.len()
        ));
    }
    if code >= (1 << cells.len()) {
        return value_error(format!(
            "code {code} out of range for dimension {dimension} base {base} (0..{}).",
            (1u128 << cells.len()) - 1
        ));
    }
    Ok(cells
        .into_iter()
        .enumerate()
        .filter(|(i, _)| (code >> i) & 1 == 1)
        .map(|(_, c)| c)
        .collect())
}

pub fn levels_code(dimension: usize, base: usize, levels: &[usize]) -> Code {
    let filled: Vec<Vec<u8>> = residue_corners(dimension, base)
        .into_iter()
        .filter(|corner| levels.contains(&corner.iter().map(|&b| b as usize).sum()))
        .collect();
    corners_to_code(&filled, dimension, base)
}

pub fn corners_to_code(filled: &[Vec<u8>], dimension: usize, base: usize) -> Code {
    let cells = residue_corners(dimension, base);
    let wanted: HashSet<&Vec<u8>> = filled.iter().collect();
    Code(
        cells
            .iter()
            .enumerate()
            .filter(|(_, c)| wanted.contains(c))
            .map(|(i, _)| 1 << i)
            .sum(),
    )
}

fn render(
    filled: &[Vec<u8>],
    number: usize,
    dimension: usize,
    base: usize,
    level: usize,
) -> Result<Tensor> {
    if level < 1 {
        return value_error("level must be at least 1.");
    }
    let tile = rules::render(filled, number, dimension, base)?;
    Ok(tile.fractal(level))
}

pub fn create(
    code: Code,
    number: usize,
    dimension: usize,
    base: usize,
    level: usize,
) -> Result<Tensor> {
    let filled = code_to_corners(code, dimension, base)?;
    render(&filled, number, dimension, base, level)
}

pub fn create_named(spec: &str, number: usize, level: usize) -> Result<Tensor> {
    let bang = Bang::from_json(spec)?;
    create(Code::from(bang.code), number, bang.dim, bang.base, level)
}

pub fn magic(layers: &[MagicLayer]) -> Result<Tensor> {
    if layers.len() < 2 {
        return value_error("magic needs at least two layers.");
    }
    let dimension = layers[0].design.dim;
    if layers.iter().any(|l| l.design.dim != dimension) {
        return value_error("magic layers must share one dimension.");
    }
    let mut out: Option<Tensor> = None;
    for layer in layers {
        let next = create(
            Code::from(layer.design.code),
            layer.number,
            dimension,
            layer.design.base,
            1,
        )?;
        out = Some(match out {
            Some(tile) => tile.kron(&next),
            None => next,
        });
    }
    Ok(out.expect("two or more layers leave a tile"))
}

pub fn magic_named(layers: &[(&str, usize)]) -> Result<Tensor> {
    let parsed: Result<Vec<MagicLayer>> = layers
        .iter()
        .map(|(spec, number)| Ok(MagicLayer::new(Bang::from_json(spec)?, *number)))
        .collect();
    magic(&parsed?)
}

pub fn create_from_corners(
    filled: &[Vec<u8>],
    number: usize,
    dimension: usize,
    base: usize,
    level: usize,
) -> Result<Tensor> {
    render(filled, number, dimension, base, level)
}

pub fn level_cap(number: usize, dimension: usize, budget: usize) -> usize {
    if number < 2 || dimension == 0 {
        return 1;
    }
    let mut level = 1;
    while dimension
        .checked_mul(level + 1)
        .and_then(|power| u32::try_from(power).ok())
        .and_then(|power| (number as u128).checked_pow(power))
        .is_some_and(|cells| cells <= budget as u128)
    {
        level += 1;
    }
    level
}

pub fn random_code(dimension: usize, base: usize, rng: &mut Rng, min_fills: usize) -> Result<Code> {
    total_codes(dimension, base)?;
    let corners = base.pow(dimension as u32);
    let least = min_fills.max(1);
    if least > corners {
        return value_error(format!(
            "dimension {dimension} base {base} has {corners} corners, fewer than {least} fills."
        ));
    }
    let mut row = vec![0u128; corners + 1];
    row[0] = 1;
    for n in 1..=corners {
        for k in (1..=n).rev() {
            row[k] += row[k - 1];
        }
    }
    let total: u128 = row[least..].iter().sum();
    let width = 128 - (total - 1).leading_zeros();
    let mut draw = loop {
        let draw = (0..width).fold(0u128, |draw, i| draw | u128::from(rng.boolean()) << i);
        if draw < total {
            break draw;
        }
    };
    let mut fills = least;
    while draw >= row[fills] {
        draw -= row[fills];
        fills += 1;
    }
    Ok(Code(
        rng.sample_indices(corners, fills)
            .into_iter()
            .fold(0, |code, i| code | 1 << i),
    ))
}
