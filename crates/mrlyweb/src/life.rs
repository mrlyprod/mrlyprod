use crate::Fault;
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
