use crate::{checked, code_of, Fault, Pixels};
use mrlycore::Colorizer;
use mrlylab::moire::{presets, render};
use mrlylab::press;
use wasm_bindgen::prelude::*;

/// Lists the first members of a design in the sequence press, each as a decimal string.
#[wasm_bindgen]
pub fn press_members(
    code: &str,
    dimension: usize,
    base: usize,
    count: usize,
) -> Result<Vec<String>, Fault> {
    let code = checked(code, dimension, base)?;
    Ok(press::members(code, dimension, base, count)
        .iter()
        .map(|m| m.to_string())
        .collect())
}

/// Counts the members of a design below the limit, as a decimal string.
#[wasm_bindgen]
pub fn press_count_below(
    code: &str,
    dimension: usize,
    base: usize,
    limit: &str,
) -> Result<String, Fault> {
    let code = checked(code, dimension, base)?;
    Ok(press::count_below(code, dimension, base, code_of(limit)?).to_string())
}

/// Names the moire presets.
#[wasm_bindgen]
pub fn moire_names() -> Vec<String> {
    presets::all(1).iter().map(|p| p.name.to_string()).collect()
}

/// Samples a named moire preset up to the scale limit into a square of the size and renders it through the fire, heat or diverge ramp into pixels.
#[wasm_bindgen]
pub fn moire(
    name: &str,
    limit: usize,
    size: usize,
    ramp: &str,
    levels: usize,
    invert: bool,
) -> Result<Pixels, Fault> {
    let field = presets::named(name, limit)?.field(size)?;
    let colorizer = match ramp {
        "heat" => Colorizer::heat(),
        "diverge" => Colorizer::diverge(),
        _ => Colorizer::fire(),
    };
    let png = render(&field, &colorizer, levels, false, invert, 1)?;
    let (width, height, colors) = mrlycore::unpng(&png)?;
    Ok(Pixels::of(width, height, colors))
}

/// Lists the odd scales a moire stack samples up to the limit.
#[wasm_bindgen]
pub fn odd_scales(limit: usize) -> Vec<u32> {
    (1..=limit.max(1)).step_by(2).map(|n| n as u32).collect()
}
