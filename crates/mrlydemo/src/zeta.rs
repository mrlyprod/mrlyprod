use crate::Fault;
use mrlynum::zeta::{self, Line, JOIN};
use wasm_bindgen::prelude::*;

const REACH: f64 = 250.0;
const POINTS: usize = 20_000;
const ZEROS: usize = 100;
const STAIR: usize = 1_000;
const SAMPLES: usize = 2_000;

fn checked(t: f64) -> Result<f64, Fault> {
    if !(0.0..=REACH).contains(&t) {
        return Err(Fault::new(format!("the line runs from t = 0 to {REACH}.")));
    }
    Ok(t)
}

fn stones(x: f64, gammas: &[f64]) -> Result<(), Fault> {
    if !(2.0..=STAIR as f64).contains(&x) {
        return Err(Fault::new(format!("the staircase runs from 2 to {STAIR}.")));
    }
    if gammas.len() > ZEROS {
        return Err(Fault::new(format!(
            "the formula takes at most {ZEROS} zeros."
        )));
    }
    Ok(())
}

/// Walks the critical line from one t to another in the given count of steps: for every point t, the real and the imaginary part of zeta at one half plus i t, and Z(t), as flat quadruples.
#[wasm_bindgen]
pub fn zeta_line(t0: f64, t1: f64, steps: usize) -> Result<Vec<f64>, Fault> {
    checked(t0)?;
    checked(t1)?;
    if steps == 0 || steps > POINTS {
        return Err(Fault::new(format!("the walk takes 1 to {POINTS} steps.")));
    }
    let line = Line::new();
    let mut out = Vec::with_capacity(4 * (steps + 1));
    for k in 0..=steps {
        let t = t0 + (t1 - t0) * k as f64 / steps as f64;
        let (value, z) = line.point(t);
        out.extend([t, value.re, value.im, z]);
    }
    Ok(out)
}

/// Reads one point of the critical line: the real and the imaginary part of zeta, Z(t) and theta(t).
#[wasm_bindgen]
pub fn zeta_at(t: f64) -> Result<Vec<f64>, Fault> {
    let line = Line::new();
    let (value, z) = line.point(checked(t)?);
    Ok(vec![value.re, value.im, z, line.theta(t)])
}

/// Returns the first zeros of zeta on the critical line, at most a hundred: sign changes of Z between Gram points, refined by bisection to a billionth.
#[wasm_bindgen]
pub fn zeta_zeros(count: usize) -> Result<Vec<f64>, Fault> {
    if count > ZEROS {
        return Err(Fault::new(format!("the list holds {ZEROS} zeros.")));
    }
    Ok(Line::new().zeros(count))
}

/// Counts the zeros on the critical line below t.
#[wasm_bindgen]
pub fn zeta_count(t: f64) -> Result<u32, Fault> {
    Ok(Line::new().count(checked(t)?) as u32)
}

/// Returns the join where Riemann-Siegel takes over from Euler-Maclaurin, and the largest gap between the two up to the given t on a grid of the given steps.
#[wasm_bindgen]
pub fn zeta_seam(t1: f64, steps: usize) -> Result<Vec<f64>, Fault> {
    if steps == 0 || steps > POINTS {
        return Err(Fault::new(format!("the seam takes 1 to {POINTS} steps.")));
    }
    Ok(vec![
        JOIN,
        Line::new().seam(JOIN, checked(t1)?.max(JOIN), steps),
    ])
}

/// Returns the Chebyshev staircase psi at every whole number from one to x, at most a thousand.
#[wasm_bindgen]
pub fn psi_stair(x: usize) -> Result<Vec<f64>, Fault> {
    stones(x as f64, &[])?;
    Ok(zeta::psi_stair(x))
}

/// Samples the explicit formula over the given zero ordinates at evenly spaced points from two to x, as flat pairs of the point and the value.
#[wasm_bindgen]
pub fn psi_formula(x: f64, gammas: &[f64], samples: usize) -> Result<Vec<f64>, Fault> {
    stones(x, gammas)?;
    if !(2..=SAMPLES).contains(&samples) {
        return Err(Fault::new(format!(
            "the formula takes 2 to {SAMPLES} samples."
        )));
    }
    let mut out = Vec::with_capacity(2 * samples);
    for k in 0..samples {
        let u = 2.0 + (x - 2.0) * k as f64 / (samples - 1) as f64;
        out.extend([u, zeta::psi_formula(u, gammas)]);
    }
    Ok(out)
}

/// Returns psi(x) less the explicit formula over the given zeros at x.
#[wasm_bindgen]
pub fn psi_gap(x: usize, gammas: &[f64]) -> Result<f64, Fault> {
    stones(x as f64, gammas)?;
    Ok(zeta::psi_stair(x)[x - 1] - zeta::psi_formula(x as f64, gammas))
}
