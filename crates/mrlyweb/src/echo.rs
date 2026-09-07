use crate::Fault;
use mrlycore::json;
use mrlynum::design;
use wasm_bindgen::prelude::*;

const SAMPLES: usize = 4096;
const ELEMENT_CAP: u128 = 1 << 18;
const VALUE_CAP: u128 = 1 << 27;
const ECHO_CAP: u128 = 1 << 24;
const FLOOR: usize = 101;
const BAND: (f64, f64) = (4.0, 60.0);
const THRESHOLD: f64 = 8.0;
const TOP: usize = 10;
const LOW: usize = 3;

/// The design Mobius meter read at one base, digit set and depth: the meter drawn against log x, its density echo and residual, and the spectrum they carry.
#[wasm_bindgen(getter_with_clone)]
pub struct Echo {
    /// The log of x at every sample of the meter.
    pub logx: Vec<f32>,
    /// The meter M_F(x) over x to the alpha over two at every sample.
    pub meter: Vec<f32>,
    /// The density echo at every sample, empty when the depth is past the sieve cap.
    pub echo: Vec<f32>,
    /// The residual, the meter less its echo, empty when the depth is past the sieve cap.
    pub rest: Vec<f32>,
    /// The ordinate gamma of every spectral bin.
    pub gamma: Vec<f32>,
    /// The power over its local median floor at every spectral bin.
    pub score: Vec<f32>,
    /// The reading beside the curves, as JSON.
    pub read: String,
}

fn digits_of(base: u32, mask: u32) -> Result<(u64, Vec<u64>), Fault> {
    if !(2..=10).contains(&base) {
        return Err(Fault::new(format!(
            "the base {base} is not between two and ten."
        )));
    }
    if mask >> base != 0 {
        return Err(Fault::new(format!(
            "the digit set names a digit at or above the base {base}."
        )));
    }
    let digits = design::digits_of(mask, u64::from(base));
    if digits.len() < 2 {
        return Err(Fault::new(
            "the design needs at least two digits.".to_string(),
        ));
    }
    if digits.iter().all(|&d| d == 0) {
        return Err(Fault::new(
            "the design needs a digit above zero.".to_string(),
        ));
    }
    Ok((u64::from(base), digits))
}

fn depths(base: u64, digits: &[u64]) -> (usize, usize) {
    let mut deepest = 0;
    let mut sieved = 0;
    for depth in 1..64 {
        let span = match u128::from(base).checked_pow(depth as u32) {
            Some(span) => span,
            None => break,
        };
        if span > VALUE_CAP || design::size(digits, depth) > ELEMENT_CAP {
            break;
        }
        deepest = depth;
        if span <= ECHO_CAP {
            sieved = depth;
        }
    }
    (deepest, sieved)
}

fn matched(peak: f64, list: &[f64], bin: f64) -> bool {
    design::nearest(peak, list) <= bin
}

/// Returns the digit set the mask names, its exponent alpha and the depths the caps allow, as JSON.
///
/// The deepest depth is the last one whose element count stays inside `2^18` and whose span
/// `q^L` stays inside `2^27`; the sieved depth is the last one whose span stays inside `2^24`,
/// past which the density echo needs a Mobius sieve this page will not run.
#[wasm_bindgen]
pub fn echo_caps(base: u32, mask: u32) -> Result<String, Fault> {
    let (base, digits) = digits_of(base, mask)?;
    let (deepest, sieved) = depths(base, &digits);
    if deepest < LOW {
        return Err(Fault::new(format!(
            "base {base} with {} digits reaches only depth {deepest}.",
            digits.len()
        )));
    }
    Ok(json!({
        "base": base,
        "digits": digits,
        "k": digits.len(),
        "alpha": (digits.len() as f64).ln() / (base as f64).ln(),
        "deepest": deepest,
        "sieved": sieved,
        "least": LOW,
        "samples": SAMPLES,
        "caps": { "elements": ELEMENT_CAP as f64, "span": VALUE_CAP as f64, "echo": ECHO_CAP as f64 },
    })
    .to_string())
}

/// Reads the design Mobius meter at the base, digit set and depth: the meter resampled uniformly in log x and scaled by x to the alpha over two, its density echo and residual when the span is inside the sieve cap, and the power spectrum of the meter or of the residual against its local median floor.
///
/// The elements of `S_F` are enumerated in order and their Mobius values taken by trial
/// division; the echo is `sum of mu(n) A_F(n)/n` over every whole number up to x, which needs
/// a Mobius sieve to the span and is refused past `2^24`. The spectrum is mean-removed,
/// Hann-windowed and read as `gamma = 2 pi j` over the log range, and a peak counts as landing
/// on a list when it sits within one bin of one of its entries.
#[wasm_bindgen]
pub fn echo_read(base: u32, mask: u32, depth: usize, subtract: bool) -> Result<Echo, Fault> {
    let (base, digits) = digits_of(base, mask)?;
    let (deepest, sieved) = depths(base, &digits);
    if !(LOW..=deepest).contains(&depth) {
        return Err(Fault::new(format!(
            "the depth must be between {LOW} and {deepest} at this base and digit set."
        )));
    }
    let values = design::elements(base, &digits, depth);
    let mu = design::mobius_of(&values);
    let running = design::meter(&mu);
    let k = digits.len() as f64;
    let alpha = k.ln() / (base as f64).ln();
    let exponent = alpha / 2.0;
    let logx = design::log_grid(&values, SAMPLES);
    let meter = design::resample(&values, &running, exponent, &logx);
    let sieve = depth <= sieved;
    let echo = if sieve {
        design::echo_series(&values, &logx, exponent)
    } else {
        Vec::new()
    };
    let rest: Vec<f64> = meter
        .iter()
        .zip(echo.iter())
        .map(|(whole, part)| whole - part)
        .collect();
    let taken = if subtract && sieve { &rest } else { &meter };
    let (gamma, power) = design::spectrum(&logx, taken);
    let score = design::score(&power, FLOOR);
    let found = design::peaks(&gamma, &score, BAND, THRESHOLD);
    let bin = gamma.get(1).copied().unwrap_or(0.0);
    let zeros: Vec<f64> = design::ZETA_ORDINATES
        .iter()
        .copied()
        .filter(|g| *g > BAND.0 && *g < BAND.1)
        .collect();
    let lattice: Vec<f64> = design::pole_lattice(base, BAND.1)
        .into_iter()
        .filter(|g| *g > BAND.0)
        .collect();
    let head: Vec<usize> = found.iter().copied().take(TOP).collect();
    let rows: Vec<mrlycore::Json> = head
        .iter()
        .map(|&at| {
            json!({
                "gamma": gamma[at],
                "score": score[at],
                "zeta": design::nearest(gamma[at], &zeros),
                "lattice": design::nearest(gamma[at], &lattice),
            })
        })
        .collect();
    let last = running.last().copied().unwrap_or(0);
    let peak = running.iter().map(|v| v.abs()).max().unwrap_or(0);
    let scale = design::upper_rms(&meter);
    let width = BAND.1 - BAND.0;
    let chance = |list: &[f64]| (2.0 * bin * list.len() as f64 / width).min(1.0);
    let read = json!({
        "base": base,
        "digits": digits,
        "k": digits.len(),
        "depth": depth,
        "deepest": deepest,
        "sieved": sieved,
        "sieve": sieve,
        "alpha": alpha,
        "count": values.len(),
        "last": last,
        "peak": peak,
        "theta": if values.len() > 1 { (peak.max(1) as f64).ln() / (values.len() as f64).ln() } else { 0.0 },
        "span": logx.last().copied().unwrap_or(0.0) - logx.first().copied().unwrap_or(0.0),
        "bin": bin,
        "samples": SAMPLES,
        "band": [BAND.0, BAND.1],
        "threshold": THRESHOLD,
        "zeros": zeros,
        "lattice": lattice,
        "peaks": rows,
        "found": found.len(),
        "hits": head.iter().filter(|&&at| matched(gamma[at], &zeros, bin)).count(),
        "lines": head.iter().filter(|&&at| matched(gamma[at], &lattice, bin)).count(),
        "share": if sieve && scale > 0.0 { json!(design::upper_rms(&echo) / scale) } else { json!(null) },
        "residual": if sieve && scale > 0.0 { json!(design::upper_rms(&rest) / scale) } else { json!(null) },
        "chance": chance(&zeros),
        "chanceLines": chance(&lattice),
        "rate": -alpha.min(1.0 - alpha) / 2.0,
        "caps": { "elements": ELEMENT_CAP as f64, "span": VALUE_CAP as f64, "echo": ECHO_CAP as f64 },
    })
    .to_string();
    let thin = |series: &[f64]| series.iter().map(|&v| v as f32).collect::<Vec<f32>>();
    Ok(Echo {
        logx: thin(&logx),
        meter: thin(&meter),
        echo: thin(&echo),
        rest: thin(&rest),
        gamma: thin(&gamma),
        score: thin(&score),
        read,
    })
}
