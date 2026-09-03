use crate::Fault;
use mrlycore::json;
use mrlynum::formulas;
use mrlynum::series::EULER;
use std::f64::consts::{E, PI};
use wasm_bindgen::prelude::*;

const FLOOR: usize = 2;
const REACH: usize = 2000;
const STOPS: usize = 400;

fn depth(n: usize) -> Result<usize, Fault> {
    if !(FLOOR..=REACH).contains(&n) {
        return Err(Fault::new(format!(
            "the depth must be between {FLOOR} and {REACH}."
        )));
    }
    Ok(n)
}

fn drift(value: f64, limit: f64) -> f64 {
    (value - limit).abs() / limit.abs()
}

fn reading(kind: &str, m: usize) -> Result<(f64, f64, f64), Fault> {
    let x = m as f64;
    Ok(match kind {
        "wallis" => {
            let v = formulas::wallis(m);
            (x, v, drift(v, PI / 2.0))
        }
        "leibniz" => {
            let v = formulas::leibniz(m);
            (x, v, drift(v, PI / 4.0))
        }
        "basel" => {
            let v = formulas::basel(m);
            (x, v, drift(v, PI * PI / 6.0))
        }
        "gamma" => {
            let v = formulas::euler_gamma_partial(m);
            (x, v, drift(v, EULER))
        }
        "e" => {
            let v = formulas::e_partial(m);
            (x, v, drift(v, E))
        }
        "primes" => {
            let v = formulas::prime_count(m) as f64;
            (x, v, drift(v, formulas::li(x)))
        }
        "goldbach" => {
            let v = formulas::goldbach(2 * m) as f64;
            (2.0 * x, v, 1.0 / v)
        }
        "mertens" => {
            let v = formulas::mertens(m) as f64;
            (x, v, v.abs() / x.sqrt())
        }
        _ => return Err(Fault::new(format!("no formula is named {kind:?}."))),
    })
}

/// Reads the eight elementary systems at the depth n: the constants pi, e and gamma the crate holds, then each system's partial, its target, the gap and the gap against the target, with the prime count beside li and n over ln n, the Goldbach partition count of two n beside the smallest partition count of any even number up to it, and the Mertens sum beside the square root of n, as JSON.
#[wasm_bindgen]
pub fn formulas_read(n: usize) -> Result<String, Fault> {
    let n = depth(n)?;
    let card = |value: f64, limit: f64| {
        json!({
            "value": value,
            "limit": limit,
            "error": (value - limit).abs(),
            "rel": drift(value, limit),
        })
    };
    let count = formulas::prime_count(n);
    let li = formulas::li(n as f64);
    let record = formulas::goldbach_record(2 * n);
    let pairs = record.last().copied().unwrap_or(0);
    let least = record.iter().copied().min().unwrap_or(0);
    let sum = formulas::mertens(n);
    let root = (n as f64).sqrt();
    Ok(json!({
        "n": n,
        "constants": { "pi": PI, "e": E, "gamma": EULER },
        "cards": {
            "wallis": card(formulas::wallis(n), PI / 2.0),
            "leibniz": card(formulas::leibniz(n), PI / 4.0),
            "basel": card(formulas::basel(n), PI * PI / 6.0),
            "gamma": card(formulas::euler_gamma_partial(n), EULER),
            "e": card(formulas::e_partial(n), E),
            "primes": {
                "value": count,
                "li": li,
                "ratio": n as f64 / (n as f64).ln(),
                "gauge": count as f64 / li,
                "rel": drift(count as f64, li),
            },
            "goldbach": {
                "even": 2 * n,
                "value": pairs,
                "floor": least,
                "rel": 1.0 / pairs as f64,
            },
            "mertens": {
                "value": sum,
                "root": root,
                "rel": (sum as f64).abs() / root,
            },
        },
    })
    .to_string())
}

/// Walks one of the eight systems from the floor up to n at the count of stops and returns each stop as its point on the number line, the partial there and the normalised gap it stands at, three numbers a stop, so the approach can be drawn.
#[wasm_bindgen]
pub fn formulas_walk(kind: &str, n: usize, stops: usize) -> Result<Vec<f64>, Fault> {
    let n = depth(n)?;
    if !(2..=STOPS).contains(&stops) {
        return Err(Fault::new(format!(
            "the stops must be between 2 and {STOPS}."
        )));
    }
    let stops = stops.min(n - FLOOR + 1).max(2);
    let mut out = Vec::with_capacity(stops * 3);
    for k in 0..stops {
        let at = FLOOR + (n - FLOOR) * k / (stops - 1);
        let (x, value, gauge) = reading(kind, at)?;
        out.push(x);
        out.push(value);
        out.push(gauge);
    }
    Ok(out)
}
