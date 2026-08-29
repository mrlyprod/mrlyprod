use crate::{code_of, Fault};
use mrlycore::json;
use mrlymath::formulas::six as formulas;
use mrlymath::six::{self, Cell6d};
use mrlymath::three;
use wasm_bindgen::prelude::*;

/// Projects the cube the code names to a hexagon, iso, pro or cut, and renders it as SVG at the scale.
#[wasm_bindgen]
pub fn hex_svg(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    projection: &str,
    scale: usize,
) -> Result<String, Fault> {
    let code = code_of(code)?;
    let cell = match projection {
        "pro" => six::pro_design(code, number, level, base)?,
        "cut" => six::cut_design(code, number, level, base)?,
        _ => six::iso_design(code, number, level, base)?,
    };
    Ok(six::svg(&cell, scale, None, 0)?)
}

// SLICE

fn slice(code: &str, number: usize, level: usize, base: usize) -> Result<Cell6d, Fault> {
    Ok(six::cut(&three::create(
        code_of(code)?,
        number,
        level,
        base,
    )?)?)
}

/// Tallies the diagonal section of the cube the code names: the mesh, the fill, its pieces and holes, and the solid closed forms at that side, as JSON.
#[wasm_bindgen]
pub fn slice_census(code: &str, number: usize, level: usize, base: usize) -> Result<String, Fault> {
    let cell = slice(code, number, level, base)?;
    let tally = six::census(&cell, false);
    let side = number.pow(level as u32);
    Ok(json!({
        "side": side,
        "triangles": tally.triangles,
        "boundary": tally.boundary_edges,
        "edges": tally.edges,
        "interior": tally.interior_edges,
        "vertices": tally.vertices,
        "euler": tally.euler,
        "fills": tally.fills,
        "voids": tally.voids,
        "components": six::components(&cell)?,
        "holes": six::holes(&cell)?,
        "giant": six::giant(&cell)?,
        "closed": {
            "triangles": formulas::solid_slice_triangles(side)?.to_string(),
            "boundary": formulas::solid_slice_boundary(side)?.to_string(),
            "edges": formulas::solid_slice_edges(side)?.to_string(),
            "vertices": formulas::solid_slice_vertices(side)?.to_string(),
        },
    })
    .to_string())
}

/// Walks the level-one slice of the code at odd side `2k-1`, one row per `k`, as JSON.
#[wasm_bindgen]
pub fn slice_series(code: &str, max_k: usize) -> Result<String, Fault> {
    if !(1..=16).contains(&max_k) {
        return Err(Fault::new("max_k must be between 1 and 16."));
    }
    let code = code_of(code)?;
    let mut rows = Vec::new();
    for k in 1..=max_k {
        let number = 2 * k - 1;
        let cell = six::cut(&three::create(code, number, 1, 2)?)?;
        rows.push(json!({
            "k": k,
            "n": number,
            "fills": six::census(&cell, false).fills,
            "components": six::components(&cell)?,
            "holes": six::holes(&cell)?,
        }));
    }
    Ok(json!(rows).to_string())
}
