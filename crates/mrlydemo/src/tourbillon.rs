use crate::Fault;
use mrlycore::{json, Json};
use mrlynum::tourbillon as spun;
use wasm_bindgen::prelude::*;

const EYE_CAP: usize = 60;

/// Spins the odd parity carpets at the scales one, three, five up to the top into one stack on a square of the size, every layer turned about the centre by its own angle and masked to the inscribed disc, so every pixel sees every layer.
///
/// The schedule is unspun, degrees, golden, primes, random or gaussian, the layer set is odd, primes, squarefree or prime powers, the weights are plain, mobius or harmonic, the render mode is cells, edges or corners and the blend is mean, sum, union, meet, parity or difference. The weights are scaled to average magnitude one and apply only under the linear blends, so mean is paper coverage and sum is the inked layer count. Sites outside the disc are NaN.
#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn tourbillon(
    top: usize,
    size: usize,
    schedule: &str,
    increment: f64,
    set: &str,
    weights: &str,
    mode: &str,
    blend: &str,
    seed: u32,
) -> Result<Vec<f32>, Fault> {
    Ok(spun::field(
        top, size, schedule, increment, set, weights, mode, blend, seed,
    )?)
}

/// Reads a spun stack: the layer count, the first eight scales and angles, the mean and RMS contrast over the disc, that contrast times the root of the layer count, the exact centre value, whether the blend carries the weights, the span the raster covers and the brightest three sites, as JSON.
#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn tourbillon_stats(
    field: &[f32],
    size: usize,
    top: usize,
    schedule: &str,
    increment: f64,
    set: &str,
    weights: &str,
    blend: &str,
    seed: u32,
) -> Result<String, Fault> {
    let read = spun::stats(
        field, size, top, schedule, increment, set, weights, blend, seed,
    )?;
    Ok(json!({
        "layers": read.layers,
        "scales": read.scales,
        "angles": read.angles,
        "mean": read.mean,
        "rms": read.rms,
        "faded": read.faded,
        "centre": read.centre,
        "weighted": read.weighted,
        "low": read.low,
        "high": read.high,
        "inside": read.inside,
        "peaks": read.peaks,
        "period": read.period,
        "classes": read.classes,
        "pairs": read.pairs,
    })
    .to_string())
}

/// Lists the angles a quarter turn shares with itself up to the denominator cap: each one's degrees, its numerator over ninety times a and its denominator, sorted, as JSON.
#[wasm_bindgen]
pub fn tourbillon_eyes(qmax: usize) -> Result<String, Fault> {
    if !(1..=EYE_CAP).contains(&qmax) {
        return Err(Fault::new(format!(
            "the denominator cap must be between 1 and {EYE_CAP}."
        )));
    }
    let list: Vec<Json> = spun::eyes(qmax)
        .iter()
        .map(|eye| json!([eye.angle, eye.numer, eye.denom]))
        .collect();
    Ok(Json::Array(list).to_string())
}
