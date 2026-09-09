#![allow(clippy::too_many_arguments)]

use crate::{code_of, Fault, Grid};
use mrlycore::json;
use mrlymath::six::{self, Cell6d};
use mrlymath::three::{self, Cell3d};
use mrlymath::two::{self, Cell2d};
use wasm_bindgen::prelude::*;

const PLANE_CELLS: usize = 262_144;
const SOLID_CELLS: usize = 1_000_000;
const SOLID_FILLS: usize = 150_000;
const HEX_TRIANGLES: usize = 200_000;
const HEX_SIDE: usize = 81;
const WALK_CELLS: usize = 150_000;
const WALK_TRIANGLES: usize = 80_000;

fn rep(value: u32) -> Result<usize, Fault> {
    match (1..=32).contains(&value) {
        true => Ok(value as usize),
        false => Err(Fault::new(
            "a repetition count runs from one to thirty two.",
        )),
    }
}

fn reps_of(reps: &[u32], axes: usize) -> Result<Vec<usize>, Fault> {
    if reps.len() != axes {
        return Err(Fault::new(format!(
            "a {axes}-axis tiling wants {axes} repetition counts."
        )));
    }
    reps.iter().map(|&value| rep(value)).collect()
}

fn budget(count: usize, limit: usize, what: &str) -> Result<(), Fault> {
    match count > limit {
        true => Err(Fault::new(format!(
            "{count} {what} is more than this page draws; lower the level or the repeats."
        ))),
        false => Ok(()),
    }
}

fn side_of(number: usize, level: usize) -> Result<usize, Fault> {
    number
        .checked_pow(level as u32)
        .ok_or_else(|| Fault::new("that side is past what a page draws."))
}

// PLANE

fn plane(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    reps: &[usize],
) -> Result<(Cell2d, Cell2d), Fault> {
    let cell = two::create(code_of(code)?, number, level, 0, base)?;
    budget(
        cell.width() * reps[0] * cell.height() * reps[1],
        PLANE_CELLS,
        "cells",
    )?;
    let sheet = cell.clone().tile(reps[0], reps[1]);
    Ok((cell, sheet))
}

/// Builds the flat design the code names repeated into a wide-by-high array of copies, as a byte grid.
#[wasm_bindgen]
pub fn tile_grid(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    wide: u32,
    high: u32,
) -> Result<Grid, Fault> {
    let reps = reps_of(&[wide, high], 2)?;
    let (_, sheet) = plane(code, number, level, base, &reps)?;
    Ok(Grid {
        width: sheet.width() as u32,
        height: sheet.height() as u32,
        types: sheet.types().bytes().to_vec(),
    })
}

// SOLID

fn solid(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    reps: &[usize],
) -> Result<(Cell3d, Cell3d), Fault> {
    let cell = three::create(code_of(code)?, number, level, base)?;
    budget(
        cell.width() * reps[0] * cell.height() * reps[1] * cell.depth() * reps[2],
        SOLID_CELLS,
        "cells",
    )?;
    let sheet = cell.clone().tile(reps[0], reps[1], reps[2]);
    Ok((cell, sheet))
}

/// Lists the filled sites of the cube design repeated into a wide-by-high-by-deep array of copies, as x, y, z triples.
#[wasm_bindgen]
pub fn tile_cells(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    wide: u32,
    high: u32,
    deep: u32,
) -> Result<Vec<u32>, Fault> {
    let reps = reps_of(&[wide, high, deep], 3)?;
    let (_, sheet) = solid(code, number, level, base, &reps)?;
    let grid = sheet.types();
    let (cols, deep) = (grid.shape[1], grid.shape[2]);
    budget(three::fills(&sheet), SOLID_FILLS, "cubes")?;
    let mut out = Vec::new();
    for (flat, &site) in grid.bytes().iter().enumerate() {
        if site != 0 {
            out.extend([
                (flat / (cols * deep)) as u32,
                (flat / deep % cols) as u32,
                (flat % deep) as u32,
            ]);
        }
    }
    Ok(out)
}

// HEXAGON

fn hexagon(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    projection: &str,
) -> Result<Cell6d, Fault> {
    let code = code_of(code)?;
    budget(side_of(number, level)?, HEX_SIDE, "cells to a side")?;
    Ok(match projection {
        "pro" => six::pro_design(code, number, level, base)?,
        "cut" => six::cut_design(code, number, level, base)?,
        _ => six::iso_design(code, number, level, base)?,
    })
}

fn hex_sheet(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    projection: &str,
    reps: &[usize],
    crop: bool,
) -> Result<(Cell6d, Cell6d), Fault> {
    let hex = hexagon(code, number, level, base, projection)?;
    budget(
        hex.width() * reps[0] * hex.height() * reps[1],
        HEX_TRIANGLES,
        "triangles",
    )?;
    if crop && (reps[0] < 2 || reps[1] < 2) {
        return Err(Fault::new(
            "the interlocking crop eats a sheet under two copies on an axis.",
        ));
    }
    let sheet = six::tile_cell(&hex, reps[0], reps[1], crop)?;
    if six::census(&six::skin(&sheet), false).triangles == 0 {
        return Err(Fault::new("that crop leaves no triangle to draw."));
    }
    Ok((hex, sheet))
}

/// Renders the hexagonal projection of the design tessellated into an interlocking sheet of copies, as SVG.
#[wasm_bindgen]
pub fn tile_svg(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    projection: &str,
    wide: u32,
    high: u32,
    crop: bool,
    scale: usize,
) -> Result<String, Fault> {
    let reps = reps_of(&[wide, high], 2)?;
    let (_, sheet) = hex_sheet(code, number, level, base, projection, &reps, crop)?;
    Ok(six::svg(&six::framed(&sheet), scale, None, 0)?)
}

// CENSUS

fn readings(
    fills: u128,
    voids: u128,
    exposed: u128,
    tile_fills: u128,
    tile_exposed: u128,
    copies: u128,
) -> mrlycore::Json {
    json!({
        "copies": copies.to_string(),
        "fills": fills.to_string(),
        "voids": voids.to_string(),
        "exposed": exposed.to_string(),
        "tile_fills": tile_fills.to_string(),
        "tile_exposed": tile_exposed.to_string(),
        "buried": (copies * tile_exposed - exposed).to_string(),
        "ratio": fills as f64 / (fills + voids).max(1) as f64,
    })
}

fn plane_census(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    reps: &[usize],
) -> Result<mrlycore::Json, Fault> {
    let (cell, sheet) = plane(code, number, level, base, reps)?;
    let copies = (reps[0] * reps[1]) as u128;
    let fills = two::census::fills(&sheet) as u128;
    let voids = two::census::voids(&sheet) as u128;
    let exposed = two::census::perimeter(&sheet);
    let mut out = readings(
        fills,
        voids,
        exposed,
        two::census::fills(&cell) as u128,
        two::census::perimeter(&cell),
        copies,
    );
    out["tile"] = json!([cell.width(), cell.height()]);
    out["sheet"] = json!([sheet.width(), sheet.height()]);
    out["cells"] = json!((sheet.width() * sheet.height()).to_string());
    if sheet.width() * sheet.height() <= WALK_CELLS {
        let tally = two::census::census(&sheet)?;
        out["vertices"] = json!(tally.vertices);
        out["edges"] = json!(tally.edges);
        out["euler"] = json!(tally.euler);
        out["walked"] = json!(true);
    }
    Ok(out)
}

fn solid_census(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    reps: &[usize],
) -> Result<mrlycore::Json, Fault> {
    let (cell, sheet) = solid(code, number, level, base, reps)?;
    let copies = (reps[0] * reps[1] * reps[2]) as u128;
    let fills = three::census::fills(&sheet) as u128;
    let voids = three::census::voids(&sheet) as u128;
    let exposed = three::census::surface(&sheet);
    let mut out = readings(
        fills,
        voids,
        exposed,
        three::census::fills(&cell) as u128,
        three::census::surface(&cell),
        copies,
    );
    out["tile"] = json!([cell.width(), cell.height(), cell.depth()]);
    out["sheet"] = json!([sheet.width(), sheet.height(), sheet.depth()]);
    out["cells"] = json!((sheet.width() * sheet.height() * sheet.depth()).to_string());
    if sheet.width() * sheet.height() * sheet.depth() <= WALK_CELLS {
        let tally = three::census::census(&sheet)?;
        out["vertices"] = json!(tally.vertices);
        out["edges"] = json!(tally.edges);
        out["faces"] = json!(tally.faces);
        out["euler"] = json!(tally.euler);
        out["walked"] = json!(true);
    }
    Ok(out)
}

fn hex_census(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    projection: &str,
    reps: &[usize],
    crop: bool,
) -> Result<mrlycore::Json, Fault> {
    let (hex, sheet) = hex_sheet(code, number, level, base, projection, reps, crop)?;
    let (hex, sheet) = (six::skin(&hex), six::skin(&sheet));
    let one = six::census(&hex, false);
    let tally = six::census(&sheet, false);
    let rim = six::census::fills_only(&sheet);
    let mut out = readings(
        tally.fills as u128,
        tally.voids as u128,
        rim.boundary_edges as u128,
        one.fills as u128,
        six::census::fills_only(&hex).boundary_edges as u128,
        (reps[0] * reps[1]) as u128,
    );
    out["tile"] = json!([hex.width(), hex.height()]);
    out["sheet"] = json!([sheet.width(), sheet.height()]);
    out["cells"] = json!(tally.triangles.to_string());
    out["triangles"] = json!(tally.triangles);
    out["boundary"] = json!(tally.boundary_edges);
    out["projection"] = json!(projection);
    if tally.triangles <= WALK_TRIANGLES {
        out["vertices"] = json!(tally.vertices);
        out["edges"] = json!(tally.edges);
        out["euler"] = json!(tally.euler);
        out["walked"] = json!(true);
    }
    Ok(out)
}

/// Tallies a tiled design in the plane, the cube or the hexagon: the tile and sheet shapes, the copies, the fills, the voids, the exposed faces of the sheet and of one copy, and the faces the tiling buries, as JSON.
///
/// The exposed count is the perimeter in the plane, the surface in the cube and the boundary edges
/// of the filled sub-mesh on the hexagon. Corners, edges and the Euler number ride along under the
/// walk budget, flagged by `walked`.
#[wasm_bindgen]
pub fn tile_census(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    dimension: usize,
    projection: &str,
    reps: Vec<u32>,
    crop: bool,
) -> Result<String, Fault> {
    let mut out = match dimension {
        2 => plane_census(code, number, level, base, &reps_of(&reps, 2)?)?,
        3 => solid_census(code, number, level, base, &reps_of(&reps, 3)?)?,
        6 => hex_census(
            code,
            number,
            level,
            base,
            projection,
            &reps_of(&reps, 2)?,
            crop,
        )?,
        _ => return Err(Fault::new("a design tiles in dimension 2, 3 or 6.")),
    };
    out["dimension"] = json!(dimension);
    out["side"] = json!(side_of(number, level)?);
    out["reps"] = json!(reps.iter().map(|&r| r as usize).collect::<Vec<usize>>());
    out["crop"] = json!(crop);
    Ok(out.to_string())
}
