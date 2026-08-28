use crate::{code_of, Fault};
use mrlycore::json;
use mrlymath::formulas;
use mrlymath::space::Pack;
use mrlymath::three::{self, Cell3d};
use wasm_bindgen::prelude::*;

fn cell(code: &str, number: usize, level: usize, base: usize) -> Result<Cell3d, Fault> {
    Ok(three::create(code_of(code)?, number, level, base)?)
}

/// Packs the exposed faces of the cube the code names: two section lengths, then six floats per vertex, position and normal, in the unit box.
#[wasm_bindgen]
pub fn three_faces(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
) -> Result<Vec<f32>, Fault> {
    let mut pack = Pack::new();
    for quad in three::quads(&cell(code, number, level, base)?) {
        pack.quad(quad.verts, quad.normal);
    }
    Ok(pack.buffer())
}

/// Lists the filled sites of the cube the code names as x, y, z triples.
#[wasm_bindgen]
pub fn three_cells(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
) -> Result<Vec<u32>, Fault> {
    let cell = cell(code, number, level, base)?;
    let grid = cell.types();
    let mut out = Vec::new();
    for (flat, &site) in grid.bytes().iter().enumerate() {
        if site != 0 {
            let (i, rest) = (
                flat / (grid.shape[1] * grid.shape[2]),
                flat % (grid.shape[1] * grid.shape[2]),
            );
            out.extend([
                i as u32,
                (rest / grid.shape[2]) as u32,
                (rest % grid.shape[2]) as u32,
            ]);
        }
    }
    Ok(out)
}

/// Tallies the cube: fills, voids, surface, vertices, edges, faces and Euler number, as JSON.
#[wasm_bindgen]
pub fn three_census(code: &str, number: usize, level: usize, base: usize) -> Result<String, Fault> {
    let tally = three::census(&cell(code, number, level, base)?)?;
    Ok(json!({
        "fills": tally.fills,
        "voids": tally.voids,
        "surface": tally.surface.to_string(),
        "vertices": tally.vertices,
        "edges": tally.edges,
        "faces": tally.faces,
        "euler": tally.euler,
    })
    .to_string())
}

/// Counts the exposed faces of the cube at the level by exact recurrence, without building it.
#[wasm_bindgen]
pub fn three_surface(code: &str, number: usize, level: u32, base: usize) -> Result<String, Fault> {
    Ok(formulas::surface(code_of(code)?, number, level, base)?.to_string())
}
