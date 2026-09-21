use crate::{Fault, Grid};
use mrlyrs::core::json;
use mrlyrs::num::memory::{self, Rule};
use wasm_bindgen::prelude::*;

/// The sheet budget: the most sites one drawing may carry.
pub const SITES: usize = 1 << 16;

fn sites(dimension: usize, level: usize) -> u64 {
    let side = 1u64 << level;
    if dimension == 1 {
        side * level as u64
    } else {
        side.pow(dimension as u32)
    }
}

/// Returns the span `k D` a rule code is held inside, the crate's own bound.
#[wasm_bindgen]
pub fn memory_span() -> usize {
    memory::SPAN
}

/// Returns the deepest level a sheet is drawn at in a dimension: the largest `L` whose sites are inside the sheet budget.
#[wasm_bindgen]
pub fn memory_cap(dimension: usize) -> usize {
    let mut level = 1;
    while level < 16 && sites(dimension, level + 1) <= SITES as u64 {
        level += 1;
    }
    level
}

fn rule(dimension: usize, width: usize, code: &str) -> Result<Rule, Fault> {
    let code: u64 = code
        .trim()
        .parse()
        .map_err(|_| Fault::new(format!("code {code:?} is not a whole number.")))?;
    Ok(Rule::new(dimension, width, code)?)
}

/// Reads a memory rule: its shape, its alphabet, its allowed windows, the accepted word counts to the level, the Perron root, the growth exponent and the memory number `kappa = log_2(card W) / k - log_2 rho`, as JSON.
#[wasm_bindgen]
pub fn memory_read(
    dimension: usize,
    width: usize,
    code: &str,
    levels: usize,
) -> Result<String, Fault> {
    let rule = rule(dimension, width, code)?;
    let rho = memory::perron(&rule);
    let kappa = memory::kappa(&rule);
    Ok(json!({
        "dimension": rule.dimension,
        "width": rule.width,
        "code": rule.code.to_string(),
        "codes": rule.codes().to_string(),
        "letters": rule.letters(),
        "windows": rule.windows(),
        "states": rule.states(),
        "alphabet": rule.alphabet(),
        "allowed": (0..rule.windows()).map(|w| rule.allowed(w)).collect::<Vec<bool>>(),
        "window_count": memory::allowed_windows(&rule),
        "counts": memory::counts(&rule, levels),
        "perron": rho,
        "exponent": if rho > 0.0 { rho.log2() } else { f64::NEG_INFINITY },
        "kappa": if kappa.is_finite() { kappa } else { f64::INFINITY },
    })
    .to_string())
}

/// Draws the accepted words of a memory rule as a byte grid, one byte a site.
///
/// At dimension one the sheet is the Cantor staircase: one row a level, the row of level `j` filled on the intervals its accepted words hold.
/// At dimension two the sheet is the `2^L` by `2^L` grid of the level's accepted cells, which at width one is the design of the same code cell for cell.
#[wasm_bindgen]
pub fn memory_sheet(
    dimension: usize,
    width: usize,
    code: &str,
    level: usize,
) -> Result<Grid, Fault> {
    let rule = rule(dimension, width, code)?;
    if !(1..=16).contains(&level) {
        return Err(Fault::new(format!("level {level} is out of range.")));
    }
    let side = 1usize << level;
    let count = sites(dimension, level);
    if count > SITES as u64 {
        return Err(Fault::new(format!(
            "level {level} at dimension {dimension} asks for {count} sites, over the sheet budget of {SITES}."
        )));
    }
    if dimension == 1 {
        let mut types = vec![0u8; side * level];
        for depth in 1..=level {
            let span = 1usize << (level - depth);
            for cell in memory::cells(&rule, depth) {
                let start = cell as usize * span;
                types[(depth - 1) * side + start..(depth - 1) * side + start + span].fill(1);
            }
        }
        return Ok(Grid {
            width: side as u32,
            height: level as u32,
            types,
        });
    }
    let mut types = vec![0u8; side.pow(dimension as u32)];
    for cell in memory::cells(&rule, level) {
        types[cell as usize] = 1;
    }
    Ok(Grid {
        width: side as u32,
        height: (types.len() / side) as u32,
        types,
    })
}
