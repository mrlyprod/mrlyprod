use crate::{checked, Fault, Grid};
use mrlyrs::core::json;
use mrlyrs::math::arcs::{self, Arcs};
use mrlyrs::math::bang::Code;
use wasm_bindgen::prelude::*;

const CELLS: usize = 1 << 16;

fn drawn(code: &str, base: usize, level: usize) -> Result<Arcs, Fault> {
    let code = checked(code, 2, base)?;
    if level > arcs_cap(base) {
        return Err(Fault::new(format!(
            "level {level} at base {base} is past the {CELLS} cells this page draws."
        )));
    }
    Ok(arcs::draw(Code::from(code), base, level)?)
}

/// Returns the deepest level, at least one, whose arcs this page draws at the base: at most 65536 cells.
#[wasm_bindgen]
pub fn arcs_cap(base: usize) -> usize {
    crate::bang::level_cap(base, 2, CELLS)
}

/// Draws the level in Truchet arcs as a byte grid, row 0 at the bottom: bit 0 a filled cell, bit 1 its lower arc on a loop, bit 2 its upper arc on a loop.
#[wasm_bindgen]
pub fn arcs_grid(code: &str, base: usize, level: usize) -> Result<Grid, Fault> {
    let arcs = drawn(code, base, level)?;
    Ok(Grid {
        width: arcs.side as u32,
        height: arcs.side as u32,
        types: arcs.cells,
    })
}

/// Reads the level in Truchet arcs: its side, filled cells, loops and strands, and the proved loop law at that level or null, as JSON.
#[wasm_bindgen]
pub fn arcs_read(code: &str, base: usize, level: usize) -> Result<String, Fault> {
    let arcs = drawn(code, base, level)?;
    let law = arcs::law(Code::from(checked(code, 2, base)?), base, level)?;
    let filled = arcs.cells.iter().filter(|&&byte| byte & 1 == 1).count();
    Ok(json!({
        "side": arcs.side,
        "cells": arcs.side * arcs.side,
        "filled": filled,
        "loops": arcs.loops,
        "strands": arcs.strands,
        "law": law.map(|law| json!({ "formula": law.formula, "loops": law.loops })),
    })
    .to_string())
}
