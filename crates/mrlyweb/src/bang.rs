use crate::{checked, code_of, Fault};
use mrlycore::{json, Json};
use mrlymath::bang::{self, baseq, counting};
use mrlymath::formulas;
use mrlymath::name::{Bang, Named};
use wasm_bindgen::prelude::*;

fn strings(values: Vec<u128>) -> Vec<String> {
    values.iter().map(|v| v.to_string()).collect()
}

/// Enumerates a universe of dimension one to three: its code total, its distinct count and every canonical design with its orbit size, name, degree, normal form and corners, as JSON.
#[wasm_bindgen]
pub fn universe(dimension: usize) -> Result<String, Fault> {
    if !(1..=3).contains(&dimension) {
        return Err(Fault::new(
            "the universe gallery runs from dimension 1 to 3.",
        ));
    }
    let universe = bang::bang(dimension);
    let designs: Vec<Json> = universe
        .canonical()
        .iter()
        .map(|design| {
            json!({
                "code": design.i.to_string(),
                "orbit": design.orbit_size,
                "name": design.name(),
                "degree": design.degree(),
                "anf": design.anf(),
                "corners": design.rule(),
            })
        })
        .collect();
    Ok(json!({
        "dimension": dimension,
        "total": universe.total,
        "distinct": universe.distinct(),
        "designs": designs,
    })
    .to_string())
}

/// Counts the designs distinct under symmetry for dimensions one through the limit, each as a decimal string.
#[wasm_bindgen]
pub fn counting_sequence(max_dimension: usize) -> Result<Vec<String>, Fault> {
    Ok(strings(counting::sequence(max_dimension)?))
}

/// Counts the base-q designs distinct under symmetry for dimensions one through the limit, each as a decimal string.
#[wasm_bindgen]
pub fn baseq_sequence(base: usize, max_dimension: usize) -> Result<Vec<String>, Fault> {
    Ok(strings(baseq::sequence(base, max_dimension)?))
}

/// Counts the filled sites of the code's fractal at the level in closed form, as a decimal string.
#[wasm_bindgen]
pub fn fills(
    code: &str,
    number: usize,
    dimension: usize,
    level: u32,
    base: usize,
) -> Result<String, Fault> {
    Ok(formulas::fill(code_of(code)?, number, dimension, level, base)?.to_string())
}

/// Counts the empty sites of the code's fractal at the level in closed form, as a decimal string.
#[wasm_bindgen]
pub fn voids(
    code: &str,
    number: usize,
    dimension: usize,
    level: u32,
    base: usize,
) -> Result<String, Fault> {
    Ok(formulas::void(code_of(code)?, number, dimension, level, base)?.to_string())
}

/// Returns the filled fraction of the code's fractal at the level.
#[wasm_bindgen]
pub fn ratio(
    code: &str,
    number: usize,
    dimension: usize,
    level: u32,
    base: usize,
) -> Result<f64, Fault> {
    Ok(formulas::ratio(
        code_of(code)?,
        number,
        dimension,
        level,
        base,
    )?)
}

/// Returns the code's fractal dimension at the side number.
#[wasm_bindgen]
pub fn dimension(
    code: &str,
    number: usize,
    base_dimension: usize,
    base: usize,
) -> Result<f64, Fault> {
    Ok(formulas::dimension(
        code_of(code)?,
        number,
        base_dimension,
        base,
    )?)
}

/// Prints the canonical name of a design code at its dimension and base.
#[wasm_bindgen]
pub fn name_of(code: &str, dimension: usize, base: usize) -> Result<String, Fault> {
    Ok(Bang::new(checked(code, dimension, base)?, dimension, base).to_str())
}

/// Parses a canonical design name into its code, dimension and base, as JSON.
#[wasm_bindgen]
pub fn name_parse(text: &str) -> Result<String, Fault> {
    let bang = Bang::from_str(text)?;
    Ok(json!({
        "code": bang.code.to_string(),
        "dimension": bang.dimension,
        "base": bang.base,
    })
    .to_string())
}
