use crate::Grid;
use mrlycore::{json, Json, Rng, Tensor};
use mrlymath::life::elementary;
use wasm_bindgen::prelude::*;

fn diagram(cells: Tensor) -> Grid {
    Grid {
        width: cells.shape[1] as u32,
        height: cells.shape[0] as u32,
        types: cells.bytes().to_vec(),
    }
}

/// Advances one row of the elementary rule one generation, a constant-0 boundary unless the edges wrap.
#[wasm_bindgen]
pub fn eca_next(row: &[u8], rule: u8, wrap: bool) -> Vec<u8> {
    elementary::step(row, rule, wrap)
}

/// Draws the space-time diagram of a seed row: row 0 the seed, then one row per generation.
#[wasm_bindgen]
pub fn eca_history(row: &[u8], rule: u8, steps: usize, wrap: bool) -> Grid {
    diagram(elementary::history(row, rule, steps, wrap))
}

/// Draws the single-seed diagram: one live cell run the given generations on a padded line, cropped back to the 2 steps + 1 window.
#[wasm_bindgen]
pub fn eca_seed(rule: u8, steps: usize) -> Grid {
    diagram(elementary::single_seed(rule, steps))
}

/// Reads one rule's card: its name, corners, popcount, lambda, degree, genus, affine, surjective and reversible flags, outer-totalistic counts, cube class, Wolfram class, NPN representative and gasket, as JSON.
#[wasm_bindgen]
pub fn eca_card(rule: u8) -> String {
    let cube = elementary::cube_orbit(rule);
    let class = elementary::wolfram_class(rule);
    let totalistic = match elementary::outer_totalistic(rule) {
        Some((birth, survive)) => json!({"birth": birth, "survive": survive}),
        None => Json::Null,
    };
    json!({
        "rule": rule,
        "name": elementary::rule_name(rule),
        "corners": elementary::corner_bits(rule),
        "popcount": elementary::popcount(rule),
        "lambda": elementary::lambda(rule),
        "degree": elementary::rule_degree(rule),
        "genus": elementary::genus(rule),
        "affine": elementary::affine(rule),
        "surjective": elementary::surjective(rule),
        "reversible": elementary::reversible(rule),
        "outer_totalistic": totalistic,
        "b3_rep": cube[0],
        "b3_orbit": cube,
        "wolfram_class": class,
        "wolfram_rep": class[0],
        "npn_rep": elementary::npn_class(rule)[0],
        "gasket": elementary::gasket(rule),
    })
    .to_string()
}

/// Draws a seeded row whose sites fill with the given chance.
#[wasm_bindgen]
pub fn eca_soup(width: usize, density: f64, seed: u32) -> Vec<u8> {
    let mut rng = Rng::new(seed as u64);
    (0..width).map(|_| u8::from(rng.chance(density))).collect()
}
