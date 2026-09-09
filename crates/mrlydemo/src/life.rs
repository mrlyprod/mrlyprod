#![allow(clippy::too_many_arguments)]

use crate::{code_of, Fault, Grid};
use mrlycore::{json, Rng, Tensor};
use mrlymath::life::{self, Boundary, Config, Sequence};
use mrlymath::two::Cell2d;
use wasm_bindgen::prelude::*;

fn grid(types: &[u8], width: usize, height: usize) -> Result<Cell2d, Fault> {
    if types.len() != width * height {
        return Err(Fault::new(
            "the grid bytes do not match width times height.",
        ));
    }
    Ok(Cell2d::new(Tensor::of(types.to_vec(), vec![height, width])))
}

fn counts(list: &[u32]) -> Vec<usize> {
    list.iter().map(|&v| v as usize).collect()
}

fn boundary(wrap: bool) -> Boundary {
    if wrap {
        Boundary::Wrap
    } else {
        Boundary::Constant
    }
}

/// Advances a grid one generation under the birth and survive counts on the Moore neighborhood, wrapping the edges on request.
#[wasm_bindgen]
pub fn life_next(
    types: &[u8],
    width: usize,
    height: usize,
    birth: &[u32],
    survive: &[u32],
    wrap: bool,
) -> Result<Vec<u8>, Fault> {
    let mask = life::moore().types().clone();
    let next = life::next_grid(
        &grid(types, width, height)?,
        &counts(birth),
        &counts(survive),
        &mask,
        boundary(wrap),
    )?;
    Ok(next.types().bytes().to_vec())
}

/// Runs a seed until it fixes, loops or times out: the fate, the generation count and the loop length, as JSON.
#[wasm_bindgen]
pub fn life_run(
    types: &[u8],
    width: usize,
    height: usize,
    birth: &[u32],
    survive: &[u32],
    wrap: bool,
    max_generations: usize,
) -> Result<String, Fault> {
    let config = Config {
        boundary: boundary(wrap),
        max_generations,
        ..Config::new(life::moore(), counts(birth), counts(survive))
    };
    let run = life::animate(&grid(types, width, height)?, &config)?;
    Ok(json!({
        "fate": run.fate.name(),
        "count": run.count,
        "loop": run.loop_length,
    })
    .to_string())
}

/// Lays down the values a named sequence gives up to the limit.
#[wasm_bindgen]
pub fn life_sequence(name: &str, limit: usize) -> Result<Vec<u32>, Fault> {
    let values = life::sequence::sequence(Sequence::parse(name)?, limit)?;
    Ok(values.iter().map(|&v| v as u32).collect())
}

/// Draws a seeded grid whose sites fill with the given chance.
#[wasm_bindgen]
pub fn life_noise(width: usize, height: usize, density: f64, seed: u32) -> Vec<u8> {
    let mut rng = Rng::new(seed as u64);
    (0..width * height)
        .map(|_| u8::from(rng.chance(density)))
        .collect()
}

/// Names every fixed sequence.
#[wasm_bindgen]
pub fn life_sequences() -> Vec<String> {
    Sequence::all().iter().map(|s| s.name()).collect()
}

fn mask_grid(mask: &[u8], width: usize, height: usize) -> Result<Tensor, Fault> {
    if mask.len() != width * height {
        return Err(Fault::new(
            "the mask bytes do not match width times height.",
        ));
    }
    Ok(Tensor::of(mask.to_vec(), vec![height, width]))
}

/// Builds the base-2 design mask a code names at an odd side grown to the given Kronecker level, its centre popped; dimension 1 gives a grid of height one.
#[wasm_bindgen]
pub fn life_mask(dimension: usize, code: &str, number: usize, level: usize) -> Result<Grid, Fault> {
    let mask = life::design_mask(dimension, code_of(code)?, number, level)?;
    let width = *mask.shape.last().expect("a mask carries a shape");
    Ok(Grid {
        width: width as u32,
        height: (mask.size() / width) as u32,
        types: mask.bytes().to_vec(),
    })
}

/// Reads the index of the lattice a mask's offsets generate together with its centre, zero when they do not span; height one reads a line.
#[wasm_bindgen]
pub fn life_mask_index(mask: &[u8], width: usize, height: usize) -> Result<u32, Fault> {
    let grid = mask_grid(mask, width, height)?;
    let flat = if height == 1 {
        Tensor::of(mask.to_vec(), vec![width])
    } else {
        grid
    };
    Ok(life::lattice_index(&flat) as u32)
}

/// Advances a grid one generation under the birth and survive counts on the given mask, wrapping the edges on request; height one steps a line.
#[wasm_bindgen]
pub fn life_next_masked(
    types: &[u8],
    width: usize,
    height: usize,
    birth: &[u32],
    survive: &[u32],
    mask: &[u8],
    mask_width: usize,
    mask_height: usize,
    wrap: bool,
) -> Result<Vec<u8>, Fault> {
    let next = life::next_grid(
        &grid(types, width, height)?,
        &counts(birth),
        &counts(survive),
        &mask_grid(mask, mask_width, mask_height)?,
        boundary(wrap),
    )?;
    Ok(next.types().bytes().to_vec())
}

/// Runs a seed on the given mask until it fixes, loops or times out: the fate, the generation count and the loop length, as JSON.
#[wasm_bindgen]
pub fn life_run_masked(
    types: &[u8],
    width: usize,
    height: usize,
    birth: &[u32],
    survive: &[u32],
    mask: &[u8],
    mask_width: usize,
    mask_height: usize,
    wrap: bool,
    max_generations: usize,
) -> Result<String, Fault> {
    let shape = Cell2d::new(mask_grid(mask, mask_width, mask_height)?);
    let config = Config {
        boundary: boundary(wrap),
        max_generations,
        ..Config::new(shape, counts(birth), counts(survive))
    };
    let run = life::animate(&grid(types, width, height)?, &config)?;
    Ok(json!({
        "fate": run.fate.name(),
        "count": run.count,
        "loop": run.loop_length,
    })
    .to_string())
}
