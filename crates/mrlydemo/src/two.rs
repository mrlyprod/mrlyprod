use crate::{code_of, Fault, Grid};
use mrlyrs::core::json;
use mrlyrs::math::two::{self, Cell2d};
use wasm_bindgen::prelude::*;

fn cell(
    code: &str,
    number: usize,
    level: usize,
    rotation: usize,
    base: usize,
) -> Result<Cell2d, Fault> {
    Ok(two::create(code_of(code)?, number, level, rotation, base)?)
}

/// Builds the flat design the code names as a byte grid, one byte per site.
#[wasm_bindgen]
pub fn two_grid(
    code: &str,
    number: usize,
    level: usize,
    rotation: usize,
    base: usize,
) -> Result<Grid, Fault> {
    let cell = cell(code, number, level, rotation, base)?;
    Ok(Grid {
        width: cell.width() as u32,
        height: cell.height() as u32,
        types: cell.types().bytes().to_vec(),
    })
}

/// Tallies the flat design: fills, voids, perimeter, vertices, edges and Euler number, as JSON.
#[wasm_bindgen]
pub fn two_census(
    code: &str,
    number: usize,
    level: usize,
    rotation: usize,
    base: usize,
) -> Result<String, Fault> {
    let tally = two::census(&cell(code, number, level, rotation, base)?)?;
    Ok(json!({
        "fills": tally.fills,
        "voids": tally.voids,
        "perimeter": tally.perimeter.to_string(),
        "vertices": tally.vertices,
        "edges": tally.edges,
        "euler": tally.euler,
    })
    .to_string())
}
