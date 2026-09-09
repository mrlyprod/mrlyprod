use crate::Fault;
use mrlycore::{json, Json};
use mrlymath::dim::carry;
use mrlynum::blend;
use wasm_bindgen::prelude::*;

const LEVELS: usize = 32;
const WIDEST: usize = 60;

// SPELLING

fn spell(value: f64) -> String {
    if !value.is_finite() {
        return "none".to_string();
    }
    let text = format!("{value:.4}");
    let text = text.trim_end_matches('0').trim_end_matches('.');
    if text == "-0" {
        "0".to_string()
    } else {
        text.to_string()
    }
}

fn ratios(terms: &[i128]) -> Vec<String> {
    terms
        .windows(2)
        .map(|pair| spell(pair[1] as f64 / pair[0] as f64))
        .collect()
}

fn polynomial_text(poly: &[i128]) -> String {
    let top = poly.len().saturating_sub(1);
    let mut text = String::new();
    for (index, &weight) in poly.iter().enumerate() {
        if weight == 0 {
            continue;
        }
        let power = top - index;
        if text.is_empty() {
            if weight < 0 {
                text.push('-');
            }
        } else {
            text.push_str(if weight < 0 { " - " } else { " + " });
        }
        let shown = weight.abs() != 1 || power == 0;
        if shown {
            text.push_str(&weight.abs().to_string());
        }
        if power > 0 {
            if shown {
                text.push(' ');
            }
            text.push('x');
            if power > 1 {
                text.push_str(&format!("^{power}"));
            }
        }
    }
    if text.is_empty() {
        text.push('0');
    }
    text
}

fn decimals(terms: &[i128]) -> Vec<String> {
    terms.iter().map(|term| term.to_string()).collect()
}

// READINGS

fn logarithm(value: f64, base: usize) -> f64 {
    value.ln() / (base as f64).ln()
}

fn reading(base: usize, dimension: usize) -> Result<Json, Fault> {
    let block = carry::even_block(base, dimension)?;
    let root = carry::perron(&block)?;
    let full = carry::fill(base, dimension)? as f64;
    Ok(json!({
        "sign": carry::sign(base, dimension)?,
        "root": root,
        "gap": root - full / base as f64,
        "log_root": logarithm(root, base),
        "log_fill": logarithm(full, base) - 1.0,
    }))
}

/// The widest dimension the exact carry arithmetic reaches at the base.
#[wasm_bindgen]
pub fn carry_cap(base: usize) -> Result<usize, Fault> {
    Ok(carry::cap(base)?)
}

/// Reads the base-`q` slice carry automaton in dimension `D`, as JSON.
///
/// The digit polynomial, the reflection-even carry block with its characteristic polynomial, trace,
/// determinant and Perron root, the fill, the two exponents the sign law compares and the sign it
/// reads, the ladder of central diagonal counts with the ratios of its terms, and the smallest
/// linear recurrence those terms exhibit against the proved order `ceil(D/2)`.
#[wasm_bindgen]
pub fn carry_block(base: usize, dimension: usize, levels: usize) -> Result<String, Fault> {
    if !(1..=LEVELS).contains(&levels) {
        return Err(Fault::new(format!(
            "levels must be between 1 and {LEVELS}."
        )));
    }
    let top = carry::cap(base)?;
    if !(2..=top).contains(&dimension) {
        return Err(Fault::new(format!(
            "base {base} carries the dimensions 2 to {top} in exact integers."
        )));
    }
    let block = carry::even_block(base, dimension)?;
    let poly = carry::characteristic(&block)?;
    let terms = carry::ladder(base, dimension, levels)?;
    let order = dimension.div_ceil(2);
    let rule = blend::recurrence(&terms);
    let found = rule.as_ref().map(|rule| rule.len());
    Ok(json!({
        "base": base,
        "dimension": dimension,
        "cap": top,
        "order": order,
        "digits": carry::digit_polynomial(base, dimension)?,
        "block": block,
        "characteristic": decimals(&poly),
        "polynomial": polynomial_text(&poly),
        "trace": carry::trace(&block).to_string(),
        "determinant": carry::determinant(&block)?.to_string(),
        "fill": carry::fill(base, dimension)?.to_string(),
        "read": reading(base, dimension)?,
        "law": if dimension.is_multiple_of(2) { -1 } else { 1 },
        "open": dimension % 2 == 1 && dimension % 3 == 1,
        "terms": decimals(&terms),
        "ratios": ratios(&terms),
        "levels": terms.len() - 1,
        "capped": terms.len() <= levels,
        "found": json!(found),
        "fits": found == Some(order),
        "spectral": json!(carry::spectral_ratio(base, dimension)?),
    })
    .to_string())
}

/// Walks the slice sign law over the dimensions two to the top at both bases, as JSON.
///
/// Each row carries the parity the law predicts, whether the dimension sits in the odd residue
/// class the shelf leaves open, and the sign the exact integers read at base three and base five,
/// null where the dimension passes that base's exact cap.
#[wasm_bindgen]
pub fn carry_signs(top: usize) -> Result<String, Fault> {
    if !(2..=WIDEST).contains(&top) {
        return Err(Fault::new(format!(
            "the top must be between 2 and {WIDEST}."
        )));
    }
    let rows: Vec<Json> = (2..=top)
        .map(|dimension| {
            let read = |base: usize| reading(base, dimension).unwrap_or(Json::Null);
            json!({
                "dimension": dimension,
                "order": dimension.div_ceil(2),
                "law": if dimension.is_multiple_of(2) { -1 } else { 1 },
                "open": dimension % 2 == 1 && dimension % 3 == 1,
                "three": read(3),
                "five": read(5),
            })
        })
        .collect();
    Ok(json!(rows).to_string())
}

/// Walks the even block's spectral ratio over the dimensions four to the top, as JSON.
///
/// The ratio of the Perron root to the second eigenvalue's modulus against the free bound
/// `(D + 2)/(D - 2)` it falls to, so the vanishing spectral gap is a drawing and not a claim.
#[wasm_bindgen]
pub fn carry_ratios(base: usize, top: usize) -> Result<String, Fault> {
    if !(4..=WIDEST).contains(&top) {
        return Err(Fault::new(format!(
            "the top must be between 4 and {WIDEST}."
        )));
    }
    let rows: Vec<Json> = (4..=top)
        .map(|dimension| {
            json!({
                "dimension": dimension,
                "ratio": json!(carry::spectral_ratio(base, dimension).ok().flatten()),
                "free": (dimension as f64 + 2.0) / (dimension as f64 - 2.0),
            })
        })
        .collect();
    Ok(json!(rows).to_string())
}
