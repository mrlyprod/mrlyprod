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

// DIAGONAL

fn depth(level: usize) -> Result<(), Fault> {
    if !(1..=40).contains(&level) {
        return Err(Fault::new("level must be between 1 and 40."));
    }
    Ok(())
}

/// Profiles the diagonal cut of the cube: the support, its central pair of heights, the count on every height inside it, the extremes and whether the cut is constant, as JSON.
#[wasm_bindgen]
pub fn diagonal_profile(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
) -> Result<String, Fault> {
    depth(level)?;
    let counts = three::profile(code_of(code)?, number, level, base)?;
    let (low, high) = three::support(&counts)
        .ok_or_else(|| Fault::new(format!("code {code} fills no cell, so it has no cut.")))?;
    let span = &counts[low..=high];
    let live: Vec<u128> = span.iter().copied().filter(|&count| count > 0).collect();
    let least = *live.iter().min().unwrap();
    let most = *live.iter().max().unwrap();
    let mid = (low + high) / 2;
    Ok(json!({
        "side": number.pow(level as u32),
        "support": [low, high],
        "central": [mid, high.min(mid + 1)],
        "counts": span.iter().map(|count| count.to_string()).collect::<Vec<String>>(),
        "nonempty": live.len(),
        "heights": span.len(),
        "min": least.to_string(),
        "max": most.to_string(),
        "constant": live.len() == span.len() && least == most,
    })
    .to_string())
}

/// Counts the filled cells on one diagonal plane of the cube, without building it.
#[wasm_bindgen]
pub fn diagonal_count(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    height: usize,
) -> Result<String, Fault> {
    depth(level)?;
    let counts = three::profile(code_of(code)?, number, level, base)?;
    Ok(counts.get(height).copied().unwrap_or(0).to_string())
}

/// Spells the height's offset above the cut's support in binary, the digits that say which corners each scale may use.
#[wasm_bindgen]
pub fn diagonal_digits(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    height: usize,
) -> Result<String, Fault> {
    depth(level)?;
    let counts = three::profile(code_of(code)?, number, level, base)?;
    let (low, _) = three::support(&counts)
        .ok_or_else(|| Fault::new(format!("code {code} fills no cell, so it has no cut.")))?;
    Ok(format!("{:b}", height.saturating_sub(low)))
}

/// Counts the filled cells on the named diagonal planes together, one per circle the drawing holds, as a decimal string.
#[wasm_bindgen]
pub fn diagonal_total(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    heights: Vec<u32>,
) -> Result<String, Fault> {
    depth(level)?;
    let counts = three::profile(code_of(code)?, number, level, base)?;
    let total: u128 = heights
        .iter()
        .map(|&height| counts.get(height as usize).copied().unwrap_or(0))
        .sum();
    Ok(total.to_string())
}

/// Draws the named diagonal slices of the cube down the `(1,1,1)` axis, one circle per cell, as SVG.
#[wasm_bindgen]
pub fn diagonal_svg(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    heights: Vec<u32>,
    scale: usize,
) -> Result<String, Fault> {
    depth(level)?;
    let heights: Vec<usize> = heights.iter().map(|&height| height as usize).collect();
    Ok(three::diagonal_svg(
        code_of(code)?,
        number,
        level,
        base,
        &heights,
        scale,
    )?)
}
