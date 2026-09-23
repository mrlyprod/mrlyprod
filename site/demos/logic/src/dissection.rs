use crate::Fault;
use mrlyrs::core::json;
use mrlyrs::num::dissection::{self as cut, Region};
use wasm_bindgen::prelude::*;

const LEAST: u32 = 3;
const MOST: u32 = 128;
const SPAN: u64 = 1 << 24;
const MASS: u64 = 1 << 21;
const GRID: u64 = 1 << 16;
const SAMPLES: usize = 720;
const CHAIN: u64 = 4096;

fn set(base: u32, digit: u32) -> Result<Vec<u64>, Fault> {
    if !(LEAST..=MOST).contains(&base) {
        return Err(Fault::new(format!(
            "the base runs from {LEAST} to {MOST}, not {base}."
        )));
    }
    if digit >= base {
        return Err(Fault::new(format!(
            "the missing digit sits below the base {base}, not at {digit}."
        )));
    }
    Ok((0..u64::from(base))
        .filter(|&d| d != u64::from(digit))
        .collect())
}

fn deepest(base: u32, span: u64) -> u32 {
    let mut level = 0;
    while u64::from(base).pow(level + 1) <= span {
        level += 1;
    }
    level
}

fn index(region: Region) -> usize {
    match region {
        Region::A => 0,
        Region::B => 1,
        Region::C1 => 2,
        Region::C2 => 3,
    }
}

/// Reads the set missing one digit as JSON: its fill, dimension, `kappa_F`, the consecutive pair, the unshifted masses to the deepest level inside `2^21` frequencies and the exponent they read, the chain certificate at the base, the bars `1/5` and `1/4`, the walls and how the theorem reaches the set, and the depths the other reads allow.
///
/// The base runs from 3 to 128; the masses come from `mrlyrs::num::dissection::masses`, the chain
/// from `chain_exponent` and `chain_margin`, and `reach` from `mrlyrs::num::dissection::reach`:
/// `proof` from the chain's wall 584, `certificate` from base 115 and at the certified sets below
/// it, `none` elsewhere.
#[wasm_bindgen]
pub fn dissection_read(base: u32, digit: u32) -> Result<String, Fault> {
    let digits = set(base, digit)?;
    let q = u64::from(base);
    let fill = digits.len();
    let level = deepest(base, MASS);
    let masses = cut::masses(q, &digits, level);
    let (top, bottom) = cut::kappa(q, &digits);
    let wall = cut::chain_wall();
    let reach = cut::reach(q, u64::from(digit));
    Ok(json!({
        "base": base,
        "digit": digit,
        "digits": digits,
        "fill": fill,
        "alpha": (fill as f64).ln() / (q as f64).ln(),
        "consecutive": cut::consecutive(&digits),
        "kappa": [top, bottom],
        "kappaValue": top as f64 / bottom as f64,
        "level": level,
        "masses": masses,
        "reading": cut::reading(q, fill, &masses),
        "chain": cut::chain_exponent(q),
        "margin": cut::chain_margin(q),
        "bars": [cut::BAR_A, cut::BAR_B],
        "walls": { "chain": wall, "window": cut::WINDOW_WALL, "digit": cut::DIGIT_WALL, "first": cut::FIRST_BELOW },
        "reach": reach,
        "depths": [LEAST, deepest(base, SPAN)],
        "grids": [1, deepest(base, GRID)],
    })
    .to_string())
}

/// The set's own sums on a log grid of `x`: the meter against its two yardsticks and the prime count against its main term.
#[wasm_bindgen(getter_with_clone)]
pub struct Mertens {
    /// The log of `x` at every sample.
    pub logx: Vec<f32>,
    /// `M_F(x)/A_F(x)` at every sample.
    pub mass: Vec<f32>,
    /// `M_F(x)/A_F(x)^(1/2)` at every sample.
    pub root: Vec<f32>,
    /// `psi_F(x)/(kappa_F A_F(x))` at every sample.
    pub primes: Vec<f32>,
    /// The reading at the last sample, as JSON.
    pub read: String,
}

/// Tallies the set below `base^level` on 720 points uniform in `log x`: the meter `M_F(x)` over the mass `A_F(x)` and over its square root, and `psi_F(x) = sum Lambda(n)` over `kappa_F A_F(x)`.
///
/// The Mobius values and the primes are sieved to the span, which stays inside `2^24`.
#[wasm_bindgen]
pub fn dissection_tally(base: u32, digit: u32, level: u32) -> Result<Mertens, Fault> {
    let digits = set(base, digit)?;
    let most = deepest(base, SPAN);
    if !(LEAST..=most).contains(&level) {
        return Err(Fault::new(format!(
            "the digits of x run from {LEAST} to {most} at base {base}."
        )));
    }
    let q = u64::from(base);
    let (top, bottom) = cut::kappa(q, &digits);
    let kappa = top as f64 / bottom as f64;
    let t = cut::tally(q, &digits, level as usize, SAMPLES);
    let mass: Vec<f64> = t
        .meter
        .iter()
        .zip(&t.count)
        .map(|(&m, &a)| m as f64 / a as f64)
        .collect();
    let root: Vec<f64> = t
        .meter
        .iter()
        .zip(&t.count)
        .map(|(&m, &a)| m as f64 / (a as f64).sqrt())
        .collect();
    let primes: Vec<f64> = t
        .primes
        .iter()
        .zip(&t.count)
        .map(|(&p, &a)| p / (kappa * a as f64))
        .collect();
    let last = t.count.len() - 1;
    let peak = root.iter().fold(0.0f64, |high, v| high.max(v.abs()));
    let read = json!({
        "top": q.pow(level),
        "count": t.count[last],
        "meter": t.meter[last],
        "psi": t.primes[last],
        "mass": mass[last],
        "root": root[last],
        "primes": primes[last],
        "peak": peak,
        "samples": SAMPLES,
    })
    .to_string();
    Ok(Mertens {
        logx: t.log_x.iter().map(|&v| v as f32).collect(),
        mass: mass.iter().map(|&v| v as f32).collect(),
        root: root.iter().map(|&v| v as f32).collect(),
        primes: primes.iter().map(|&v| v as f32).collect(),
        read,
    })
}

/// The frequency grid `a/base^level` cut into the four regions, each frequency with its weight.
#[wasm_bindgen(getter_with_clone)]
pub struct Regions {
    /// The region of every frequency: 0 for A, 1 for B, 2 for C1, 3 for C2.
    pub region: Vec<u8>,
    /// `|hat F_level(a/y)|/fill^level` at every frequency.
    pub weight: Vec<f32>,
    /// The counts, the `l^1` shares and the cut, as JSON.
    pub read: String,
}

/// Cuts the grid of `y = base^level` frequencies at `Q = y^(3/5)` and the given `Z` into the regions A, B, C1 and C2, weighs each by the digit transform, and reads the count and the share of the `l^1` mass in each region.
#[wasm_bindgen]
pub fn dissection_grid(base: u32, digit: u32, level: u32, z: u32) -> Result<Regions, Fault> {
    let digits = set(base, digit)?;
    let most = deepest(base, GRID);
    if !(1..=most).contains(&level) {
        return Err(Fault::new(format!(
            "the grid level runs from 1 to {most} at base {base}."
        )));
    }
    if z < 2 {
        return Err(Fault::new("the cut Z is at least 2."));
    }
    let q = u64::from(base);
    let y = q.pow(level);
    let regions = cut::regions(q, level, u64::from(z));
    let weights = cut::weights(q, &digits, level);
    let scale = (digits.len() as f64).powi(level as i32);
    let mut counts = [0u64; 4];
    let mut mass = [0.0f64; 4];
    for (region, weight) in regions.iter().zip(&weights) {
        counts[index(*region)] += 1;
        mass[index(*region)] += weight;
    }
    let total: f64 = mass.iter().sum();
    let shares: Vec<f64> = mass.iter().map(|m| m / total).collect();
    let read = json!({
        "y": y,
        "cap": cut::cap(y),
        "low": (y as f64).powf(0.4),
        "z": z,
        "counts": counts,
        "shares": shares,
        "mass": total,
        "reading": (total / scale).ln() / (y as f64).ln(),
    })
    .to_string();
    Ok(Regions {
        region: regions.iter().map(|&r| index(r) as u8).collect(),
        weight: weights.iter().map(|&w| (w / scale) as f32).collect(),
        read,
    })
}

/// Returns the chain's certificate exponent `alpha_1` at one missing digit for every base from 3 to 4096, in order.
#[wasm_bindgen]
pub fn dissection_chain() -> Vec<f32> {
    (3..=CHAIN).map(|q| cut::chain_exponent(q) as f32).collect()
}
