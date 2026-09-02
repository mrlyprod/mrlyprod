use crate::{code_of, Fault};
use mrlycore::json::parse;
use mrlycore::{json, Json, Map};
use mrlylab::ledger::{self, Axis, Key, Measure};
use mrlynum::blend;
use wasm_bindgen::prelude::*;

// TERMS

fn whole(text: &str) -> Result<i128, Fault> {
    text.trim()
        .parse()
        .map_err(|_| Fault::new(format!("term {text:?} is not a whole number.")))
}

fn read(terms: &[String]) -> Result<Vec<i128>, Fault> {
    terms.iter().map(|term| whole(term)).collect()
}

fn numbers(value: &Json) -> Result<Vec<i128>, Fault> {
    value
        .as_array()
        .ok_or_else(|| Fault::new("the terms are a JSON array of decimal strings."))?
        .iter()
        .map(|term| whole(term.as_str().unwrap_or_default()))
        .collect()
}

fn pairs(text: &str) -> Result<Vec<(i128, i128)>, Fault> {
    let value = parse(text)?;
    let rows = value
        .as_array()
        .ok_or_else(|| Fault::new("the coefficients are a JSON array of pairs."))?;
    rows.iter()
        .map(|row| match (row[0].as_i64(), row[1].as_i64()) {
            (Some(num), Some(den)) if den > 0 => Ok((num as i128, den as i128)),
            _ => Err(Fault::new(
                "a coefficient is a numerator and a denominator above zero.",
            )),
        })
        .collect()
}

fn spread(rule: &[(i128, i128)]) -> Json {
    json!(rule
        .iter()
        .map(|&(num, den)| json!([num as i64, den as i64]))
        .collect::<Vec<Json>>())
}

// SPELLING

fn size(num: i128, den: i128) -> String {
    if den == 1 {
        num.abs().to_string()
    } else {
        format!("{}/{den}", num.abs())
    }
}

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

fn recurrence_text(rule: &[(i128, i128)]) -> String {
    let mut text = String::from("a(n) =");
    for (index, &(num, den)) in rule.iter().enumerate() {
        if num == 0 {
            continue;
        }
        text.push_str(if num < 0 {
            " - "
        } else if text.ends_with('=') {
            " "
        } else {
            " + "
        });
        let weight = size(num, den);
        if weight != "1" {
            text.push_str(&weight);
            text.push(' ');
        }
        text.push_str(&format!("a(n-{})", index + 1));
    }
    if text.ends_with('=') {
        text.push_str(" 0");
    }
    text
}

fn polynomial_text(poly: &[(i128, i128)]) -> String {
    let top = poly.len().saturating_sub(1);
    let mut text = String::new();
    for (index, &(num, den)) in poly.iter().enumerate() {
        if num == 0 {
            continue;
        }
        let power = top - index;
        if text.is_empty() {
            if num < 0 {
                text.push('-');
            }
        } else {
            text.push_str(if num < 0 { " - " } else { " + " });
        }
        let weight = size(num, den);
        let shown = weight != "1" || power == 0;
        if shown {
            text.push_str(&weight);
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

// VIEWS

fn tenlog(term: i128) -> f64 {
    if term == 0 {
        f64::NAN
    } else {
        (term.unsigned_abs() as f64).log10()
    }
}

fn ratios(terms: &[i128]) -> Vec<String> {
    terms
        .windows(2)
        .map(|pair| spell(pair[1] as f64 / pair[0] as f64))
        .collect()
}

fn triangle(terms: &[i128], depth: usize) -> Vec<Vec<String>> {
    let mut rows = vec![terms.to_vec()];
    while rows.len() < depth.max(1) {
        match rows.last() {
            Some(row) if row.len() > 1 => rows.push(blend::delta(row)),
            _ => break,
        }
    }
    rows.iter()
        .map(|row| row.iter().map(|term| term.to_string()).collect())
        .collect()
}

fn slope(terms: &[i128]) -> f64 {
    let points: Vec<(f64, f64)> = terms
        .iter()
        .enumerate()
        .filter(|(_, &term)| term != 0)
        .map(|(index, &term)| (index as f64, tenlog(term)))
        .collect();
    let keep = points.len().div_ceil(2).max(4).min(points.len());
    let tail = &points[points.len() - keep..];
    let count = tail.len() as f64;
    let mean = |pick: fn(&(f64, f64)) -> f64| tail.iter().map(pick).sum::<f64>() / count;
    let (mx, my) = (mean(|point| point.0), mean(|point| point.1));
    let top: f64 = tail
        .iter()
        .map(|point| (point.0 - mx) * (point.1 - my))
        .sum();
    let bottom: f64 = tail
        .iter()
        .map(|point| (point.0 - mx) * (point.0 - mx))
        .sum();
    top / bottom
}

fn views(terms: &[i128], depth: usize) -> Map {
    let rule = blend::recurrence(terms);
    let root = rule
        .as_ref()
        .map(|rule| blend::growth(rule))
        .filter(|root| root.is_finite() && *root > 0.0);
    let growth = root.unwrap_or_else(|| 10f64.powf(slope(terms)));
    let body = json!({
        "terms": terms.iter().map(|term| term.to_string()).collect::<Vec<String>>(),
        "log10": terms.iter().map(|&term| json!(tenlog(term))).collect::<Vec<Json>>(),
        "ratios": ratios(terms),
        "differences": triangle(terms, depth),
        "order": rule.as_ref().map_or(Json::Null, |rule| json!(rule.len())),
        "coefficients": rule.as_ref().map_or(Json::Null, |rule| spread(rule)),
        "characteristic": rule
            .as_ref()
            .map_or(Json::Null, |rule| spread(&blend::characteristic(rule))),
        "recurrence": rule.as_ref().map_or(String::new(), |rule| recurrence_text(rule)),
        "polynomial": rule.as_ref().map_or(String::new(), |rule| polynomial_text(
            &blend::characteristic(rule)
        )),
        "root": json!(root),
        "growth": json!(growth),
        "growth_from": if root.is_some() {
            "the recurrence root"
        } else {
            "the least-squares slope of the log terms over the tail, a fit"
        },
        "exponent": json!(growth.log10()),
    });
    match body {
        Json::Object(map) => map,
        _ => Map::new(),
    }
}

// MIX

fn held(value: Option<i128>) -> Result<i128, Fault> {
    value.ok_or_else(|| Fault::new("the mix passes a hundred and twenty-eight bits."))
}

fn mixed(left: &[i128], right: &[i128], op: &str, argument: i32) -> Result<Vec<i128>, Fault> {
    let width = left.len().min(right.len());
    match op {
        "add" => {
            for index in 0..width {
                held(left[index].checked_add(right[index]))?;
            }
            Ok(blend::add(left, right))
        }
        "sub" => {
            for index in 0..width {
                held(left[index].checked_sub(right[index]))?;
            }
            Ok(blend::sub(left, right))
        }
        "hadamard" => {
            for index in 0..width {
                held(left[index].checked_mul(right[index]))?;
            }
            Ok(blend::hadamard(left, right))
        }
        "cauchy" => {
            for n in 0..width {
                let mut sum = 0i128;
                for i in 0..=n {
                    sum = held(
                        left[i]
                            .checked_mul(right[n - i])
                            .and_then(|part| sum.checked_add(part)),
                    )?;
                }
            }
            Ok(blend::cauchy(left, right))
        }
        "shift" => Ok(blend::shift(left, argument.max(0) as usize)),
        "decimate" => Ok(blend::decimate(left, argument.max(1) as usize, 0)),
        "delta" => Ok(blend::delta(left)),
        "sigma" => {
            let mut sum = 0i128;
            for &term in left {
                sum = held(sum.checked_add(term))?;
            }
            Ok(blend::sigma(left))
        }
        "scale" => {
            for &term in left {
                held(term.checked_mul(argument as i128))?;
            }
            Ok(blend::scale(left, argument as i128))
        }
        other => Err(Fault::new(format!("unknown blend op {other:?}."))),
    }
}

// EXPORTS

/// Names the term operations a mix takes, in page order.
#[wasm_bindgen]
pub fn blend_ops() -> Vec<String> {
    [
        "add", "sub", "hadamard", "cauchy", "shift", "decimate", "delta", "sigma", "scale",
    ]
    .iter()
    .map(|op| op.to_string())
    .collect()
}

/// Finds the smallest linear constant-coefficient recurrence every term satisfies, as JSON: null where none fits, else the order, the coefficients as numerator and denominator pairs newest term first, and the rule spelled out.
#[wasm_bindgen]
pub fn blend_recurrence(terms: Vec<String>) -> Result<String, Fault> {
    let terms = read(&terms)?;
    Ok(match blend::recurrence(&terms) {
        None => "null".to_string(),
        Some(rule) => json!({
            "order": rule.len(),
            "coefficients": spread(&rule),
            "recurrence": recurrence_text(&rule),
        })
        .to_string(),
    })
}

/// Returns the monic characteristic polynomial of a recurrence highest power first, as JSON pairs, from the coefficients as JSON pairs.
#[wasm_bindgen]
pub fn blend_characteristic(coefficients: &str) -> Result<String, Fault> {
    Ok(spread(&blend::characteristic(&pairs(coefficients)?)).to_string())
}

/// Returns the largest positive real root of a recurrence's characteristic polynomial, the growth rate, from the coefficients as JSON pairs.
#[wasm_bindgen]
pub fn blend_growth(coefficients: &str) -> Result<f64, Fault> {
    Ok(blend::growth(&pairs(coefficients)?))
}

/// Reads one registry sequence with every view the plot draws, as JSON: the catalog row, the terms as decimal strings, the base-ten logarithm of each, the ratio of each term to the one before, the difference triangle to the depth, the recurrence with its characteristic polynomial and largest real root, and the growth with the exponent and which reading gave it.
#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn blend_series(
    code: &str,
    dimension: usize,
    base: usize,
    measure: &str,
    axis: &str,
    count: usize,
    cells: &str,
    depth: usize,
) -> Result<String, Fault> {
    let text = crate::ledger::ledger_row(code, dimension, base, measure, axis, count, cells)?;
    let row = parse(&text)?;
    let terms = numbers(&row["terms"])?;
    let mut out = match row {
        Json::Object(map) => map,
        _ => Map::new(),
    };
    out.extend(views(&terms, depth));
    Ok(Json::Object(out).to_string())
}

/// Reads the first terms of one measure and axis for every design of a dimension and base, as JSON: the code, the sequence name, the terms as decimal strings and whether the budget cut them short.
#[wasm_bindgen]
pub fn blend_family(
    dimension: usize,
    base: usize,
    measure: &str,
    axis: &str,
    count: usize,
    cells: &str,
) -> Result<String, Fault> {
    let measure = Measure::parse(measure)?;
    let axis = Axis::parse(axis)?;
    let cells = code_of(cells)?;
    let rows: Vec<Json> = ledger::designs(dimension, base)?
        .iter()
        .filter_map(|&code| {
            let key = Key::new(code, dimension, base, measure, axis);
            let (terms, capped) = ledger::terms(&key, count, cells).ok()?;
            Some(json!({
                "code": code.to_string(),
                "name": key.name(),
                "capped": capped,
                "terms": terms.iter().map(|term| term.to_string()).collect::<Vec<String>>(),
            }))
        })
        .collect();
    Ok(json!(rows).to_string())
}

/// Mixes two term lists by a blend operation and returns the mixed terms with the same views, as JSON: add, sub, hadamard and cauchy take both lists, shift, decimate, delta, sigma and scale take the first, and the argument is the shift count, the decimate step or the scale factor.
#[wasm_bindgen]
pub fn blend_mix(
    left: Vec<String>,
    right: Vec<String>,
    op: &str,
    argument: i32,
    depth: usize,
) -> Result<String, Fault> {
    let terms = mixed(&read(&left)?, &read(&right)?, op, argument)?;
    let mut out = views(&terms, depth);
    out.insert("op".to_string(), json!(op));
    out.insert("argument".to_string(), json!(argument));
    Ok(Json::Object(out).to_string())
}
