use crate::{checked, code_of, Fault};
use mrlycore::{json, Json, Rng};
use mrlymath::bang::{self, baseq, code_to_corners, counting};
use mrlymath::formulas;
use mrlymath::name::{Bang, Named};
use wasm_bindgen::prelude::*;

fn strings(values: Vec<u128>) -> Vec<String> {
    values.iter().map(|v| v.to_string()).collect()
}

fn stream(seed: u32, lane: u64) -> Rng {
    Rng::new(u64::from(seed) | (lane << 32))
}

fn draw(rng: &mut Rng, dimension: usize, base: usize) -> Result<u128, Fault> {
    let cells = base
        .checked_pow(dimension as u32)
        .filter(|&cells| cells < 128)
        .ok_or_else(|| Fault::new("too many corners to draw a code."))?;
    loop {
        let code = (0..cells).fold(0u128, |code, bit| code | (u128::from(rng.boolean()) << bit));
        if code.count_ones() >= 2 && code_to_corners(code, dimension, base).is_ok() {
            return Ok(code);
        }
    }
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

/// Counts the fill classes, the popcount profiles of the base-2 designs, for dimensions one through the limit, each as a decimal string.
#[wasm_bindgen]
pub fn classes_sequence(max_dimension: usize) -> Vec<String> {
    strings(counting::class_sequence(max_dimension))
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

/// Draws one design code of the dimension and base from the seed, uniform over the codes that fill a corner.
#[wasm_bindgen]
pub fn random_code(dimension: usize, base: usize, seed: u32) -> Result<String, Fault> {
    Ok(draw(&mut stream(seed, 0), dimension, base)?.to_string())
}

/// Draws a run of design codes of the dimension and base from the seed, each uniform over the codes that fill a corner.
#[wasm_bindgen]
pub fn random_codes(
    dimension: usize,
    base: usize,
    seed: u32,
    count: usize,
) -> Result<Vec<String>, Fault> {
    let mut rng = stream(seed, 0);
    (0..count)
        .map(|_| Ok(draw(&mut rng, dimension, base)?.to_string()))
        .collect()
}

/// Draws one whole number between each low and high inclusive from the seed's second lane, so a page's extra draws never echo its code.
#[wasm_bindgen]
pub fn random_between(seed: u32, lows: &[i32], highs: &[i32]) -> Vec<i32> {
    let mut rng = stream(seed, 1);
    lows.iter()
        .zip(highs)
        .map(|(&low, &high)| rng.range(i64::from(low), i64::from(high)) as i32)
        .collect()
}

/// Returns the largest level, at least one, at which a grid of the number and dimension holds at most the budget of cells.
#[wasm_bindgen]
pub fn level_cap(number: usize, dimension: usize, budget: usize) -> usize {
    if number < 2 {
        return 1;
    }
    let cells = |level: u32| (number as u128).checked_pow(dimension as u32 * level);
    let mut level = 1;
    while cells(level + 1).is_some_and(|count| count <= budget as u128) {
        level += 1;
    }
    level as usize
}

/// Returns the largest level, at least one, at which the code's fractal fills at most the budget of sites.
#[wasm_bindgen]
pub fn fill_cap(
    code: &str,
    number: usize,
    dimension: usize,
    base: usize,
    budget: usize,
) -> Result<usize, Fault> {
    let code = checked(code, dimension, base)?;
    let fits = |level: u32| {
        formulas::fill(code, number, dimension, level, base)
            .is_ok_and(|count| count <= budget as u128)
    };
    let mut level = 1;
    while level < 40 && fits(level + 1) {
        level += 1;
    }
    Ok(level as usize)
}

/// Counts the cells of a grid of the number and dimension at the level, as a decimal string.
#[wasm_bindgen]
pub fn grid_total(number: usize, dimension: usize, level: usize) -> Result<String, Fault> {
    (number as u128)
        .checked_pow((dimension * level) as u32)
        .map(|total| total.to_string())
        .ok_or_else(|| Fault::new("that grid holds more cells than a u128 counts."))
}
