use crate::{checked, ink, Fault, Pixels};
use mrlymath::two;
use wasm_bindgen::prelude::*;

const REACH: usize = 1024;
const SPREAD: f64 = 30.0;

// OBJECT

struct Weighted {
    number: usize,
    corners: Vec<(usize, usize)>,
    weights: Vec<f64>,
    logs: Vec<f64>,
    rung: f64,
}

fn corners_of(code: &str, number: usize, base: usize) -> Result<Vec<(usize, usize)>, Fault> {
    if number < 2 {
        return Err(Fault::new("the side must be at least two."));
    }
    let tile = two::create(checked(code, 2, base)?, number, 1, 0, base)?;
    let types = tile.types().bytes().to_vec();
    let corners: Vec<(usize, usize)> = (0..number * number)
        .filter(|&at| types[at] != 0)
        .map(|at| (at / number, at % number))
        .collect();
    if corners.is_empty() {
        return Err(Fault::new("the empty design carries no mass."));
    }
    Ok(corners)
}

fn ramp(t: f64) -> [u8; 4] {
    let stops = [ink::DEEP, ink::BLUE, ink::GOLD];
    let reach = t.clamp(0.0, 1.0) * (stops.len() - 1) as f64;
    let low = (reach.floor() as usize).min(stops.len() - 2);
    let fade = reach - low as f64;
    let mix = |a: u8, b: u8| (f64::from(a) + (f64::from(b) - f64::from(a)) * fade).round() as u8;
    [
        mix(stops[low][0], stops[low + 1][0]),
        mix(stops[low][1], stops[low + 1][1]),
        mix(stops[low][2], stops[low + 1][2]),
        255,
    ]
}

impl Weighted {
    fn read(code: &str, number: usize, base: usize, weights: &[f64]) -> Result<Weighted, Fault> {
        let corners = corners_of(code, number, base)?;
        if weights.len() != corners.len() {
            return Err(Fault::new(format!(
                "the design fills {} corners and {} weights arrived.",
                corners.len(),
                weights.len()
            )));
        }
        if weights.iter().any(|w| !w.is_finite() || *w <= 0.0) {
            return Err(Fault::new("every weight must be finite and above zero."));
        }
        let total: f64 = weights.iter().sum();
        let weights: Vec<f64> = weights.iter().map(|w| w / total).collect();
        let logs = weights.iter().map(|w| w.ln()).collect();
        Ok(Weighted {
            number,
            corners,
            weights,
            logs,
            rung: (number as f64).ln(),
        })
    }

    fn tilt(&self, s: f64) -> (f64, f64) {
        let peak = self
            .logs
            .iter()
            .map(|l| s * l)
            .fold(f64::NEG_INFINITY, f64::max);
        let (mut sum, mut drift) = (0.0, 0.0);
        for l in &self.logs {
            let share = (s * l - peak).exp();
            sum += share;
            drift += share * l;
        }
        ((peak + sum.ln()) / self.rung, -drift / sum / self.rung)
    }

    fn span(&self, level: usize) -> Result<usize, Fault> {
        if level < 1 {
            return Err(Fault::new("the level must be at least one."));
        }
        self.number
            .checked_pow(level as u32)
            .filter(|span| *span <= REACH)
            .ok_or_else(|| {
                Fault::new(format!(
                    "side {} at level {level} passes the {REACH} cells a side the grid allows.",
                    self.number
                ))
            })
    }

    fn mass(&self, level: usize) -> Result<(usize, Vec<f64>), Fault> {
        let span = self.span(level)?;
        let mut wide = 1usize;
        let mut field = vec![1.0f64];
        for _ in 0..level {
            let next = wide * self.number;
            let mut grown = vec![0.0f64; next * next];
            for row in 0..wide {
                for col in 0..wide {
                    let held = field[row * wide + col];
                    if held == 0.0 {
                        continue;
                    }
                    for (at, &(a, b)) in self.corners.iter().enumerate() {
                        grown[(row * self.number + a) * next + col * self.number + b] =
                            held * self.weights[at];
                    }
                }
            }
            field = grown;
            wide = next;
        }
        Ok((span, field))
    }
}

// EXPORTS

/// Lists the filled corners of the design's level-one tile as row and column pairs, two numbers a corner, the order every weight vector is read in.
///
/// The corner count `k` is half the length: a weighted design puts a probability vector on these
/// `k` cells, and the level-`L` cell of the digit word `f_1 ... f_L` carries the product of their
/// weights.
#[wasm_bindgen]
pub fn weights_corners(code: &str, number: usize, base: usize) -> Result<Vec<usize>, Fault> {
    Ok(corners_of(code, number, base)?
        .into_iter()
        .flat_map(|(row, col)| [row, col])
        .collect())
}

/// Reads the four local dimensions of a weighted design: `alpha_min`, `alpha_max`, the information exponent `alpha(1)` and `tau(0)`.
///
/// `alpha_min = -log_q max_f w_f` and `alpha_max = -log_q min_f w_f` are the ends of the spectrum's
/// support, `alpha(1) = -sum_f w_f log_q w_f` is the exponent almost every point carries, and
/// `tau(0) = log_q k` is the box dimension of the support, which no weighting moves. The weights
/// are given per filled corner in the corner order and normalised to sum to one inside.
#[wasm_bindgen]
pub fn weights_dims(
    code: &str,
    number: usize,
    base: usize,
    weights: &[f64],
) -> Result<Vec<f64>, Fault> {
    let read = Weighted::read(code, number, base, weights)?;
    let peak = read.logs.iter().copied().fold(f64::MIN, f64::max);
    let least = read.logs.iter().copied().fold(f64::MAX, f64::min);
    let held: f64 = read
        .weights
        .iter()
        .zip(&read.logs)
        .map(|(w, l)| w * l)
        .sum();
    Ok(vec![
        -peak / read.rung,
        -least / read.rung,
        -held / read.rung,
        (read.corners.len() as f64).ln() / read.rung,
    ])
}

/// Builds the level-`L` mass field of a weighted design row-major on the `q^L` by `q^L` grid, the masses summing to one and the empty cells zero.
///
/// The support is the unweighted design's own: only the contraction ratios enter it, and they stay
/// `1/q` at every weighting. The side is refused past 1024 cells.
#[wasm_bindgen]
pub fn weights_mass(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    weights: &[f64],
) -> Result<Vec<f32>, Fault> {
    let read = Weighted::read(code, number, base, weights)?;
    Ok(read
        .mass(level)?
        .1
        .into_iter()
        .map(|mass| mass as f32)
        .collect())
}

/// Paints the level-`L` mass field through the ground-blue-gold ramp, every cell read at `(mass / peak)^gamma`, the empty cells left on the ground.
///
/// The gamma is display alone: it lifts the light cells against the heavy ones without touching a
/// number the page prints.
#[wasm_bindgen]
pub fn weights_pixels(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    weights: &[f64],
    gamma: f64,
) -> Result<Pixels, Fault> {
    if !(gamma.is_finite() && gamma > 0.0) {
        return Err(Fault::new("the gamma must be finite and above zero."));
    }
    let read = Weighted::read(code, number, base, weights)?;
    let (span, field) = read.mass(level)?;
    let peak = field
        .iter()
        .copied()
        .fold(f64::MIN, f64::max)
        .max(f64::MIN_POSITIVE);
    let colors = field
        .iter()
        .map(|mass| {
            if *mass <= 0.0 {
                ink::DEEP
            } else {
                ramp((mass / peak).powf(gamma))
            }
        })
        .collect();
    Ok(Pixels::of(span, span, colors))
}

/// Walks the pressure `tau(s) = log_q sum_f w_f^s` across the range, the pairs `s` and `tau(s)`, two numbers a sample.
///
/// The closed form holds under the open set condition, which every design satisfies with the unit
/// cell. `tau(0) = log_q k` and `tau(1) = 0` at every probability vector.
#[wasm_bindgen]
pub fn weights_pressure(
    code: &str,
    number: usize,
    base: usize,
    weights: &[f64],
    s_lo: f64,
    s_hi: f64,
    samples: usize,
) -> Result<Vec<f32>, Fault> {
    if samples < 2 {
        return Err(Fault::new("the pressure needs at least two samples."));
    }
    if !(s_lo.is_finite() && s_hi.is_finite() && s_lo < s_hi) {
        return Err(Fault::new(
            "the range must run from a low s to a higher one.",
        ));
    }
    let read = Weighted::read(code, number, base, weights)?;
    let mut out = Vec::with_capacity(2 * samples);
    for step in 0..samples {
        let s = s_lo + (s_hi - s_lo) * step as f64 / (samples - 1) as f64;
        out.push(s as f32);
        out.push(read.tilt(s).0 as f32);
    }
    Ok(out)
}

/// Reads the pressure at one `s` as the four numbers `s`, `tau(s)`, `alpha(s)` and `f(alpha(s))`.
///
/// `alpha(s) = -tau'(s) = -(sum_f w_f^s log_q w_f)/(sum_f w_f^s)` in closed form, and the Legendre
/// transform is attained there, `f(alpha(s)) = alpha(s) s + tau(s)`. At `s = 1` this is the
/// information exponent and its own spectrum value, since `tau(1) = 0`.
#[wasm_bindgen]
pub fn weights_point(
    code: &str,
    number: usize,
    base: usize,
    weights: &[f64],
    s: f64,
) -> Result<Vec<f64>, Fault> {
    if !s.is_finite() {
        return Err(Fault::new("the moment s must be finite."));
    }
    let (tau, alpha) = Weighted::read(code, number, base, weights)?.tilt(s);
    Ok(vec![s, tau, alpha, alpha * s + tau])
}

/// Walks the multifractal spectrum, the pairs `alpha(s)` and `f(alpha(s))` over the whole range `[alpha_min, alpha_max]`, two numbers a sample.
///
/// The moment is swept on a cubic warp of `s` in minus thirty to thirty, dense where the spectrum
/// turns and sparse on its two flat tails, so the samples land evenly along the curve. Equal
/// weights collapse the range to the single point `(log_q k, log_q k)`.
#[wasm_bindgen]
pub fn weights_spectrum(
    code: &str,
    number: usize,
    base: usize,
    weights: &[f64],
    samples: usize,
) -> Result<Vec<f32>, Fault> {
    if samples < 2 {
        return Err(Fault::new("the spectrum needs at least two samples."));
    }
    let read = Weighted::read(code, number, base, weights)?;
    let mut out = Vec::with_capacity(2 * samples);
    for step in 0..samples {
        let walk = 2.0 * step as f64 / (samples - 1) as f64 - 1.0;
        let s = SPREAD * walk * walk * walk;
        let (tau, alpha) = read.tilt(s);
        out.push(alpha as f32);
        out.push((alpha * s + tau) as f32);
    }
    Ok(out)
}
