use crate::{code_of, Fault};
use mrlymath::six;
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
