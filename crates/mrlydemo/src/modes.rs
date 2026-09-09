use crate::{checked, Fault};
use mrlymath::two;
use std::f64::consts::TAU;
use wasm_bindgen::prelude::*;

const REACH: usize = 256;

// MASK

struct Mask {
    side: usize,
    level: usize,
    span: usize,
    digits: Vec<(usize, usize)>,
    wheel: Vec<(f64, f64)>,
}

impl Mask {
    fn read(code: &str, number: usize, level: usize, base: usize) -> Result<Mask, Fault> {
        if number < 2 {
            return Err(Fault::new("the side must be at least two."));
        }
        if level < 1 {
            return Err(Fault::new("the level must be at least one."));
        }
        let span = number
            .checked_pow(level as u32)
            .filter(|&span| span <= REACH)
            .ok_or_else(|| {
                Fault::new(format!(
                    "side {number} at level {level} passes the {REACH} sites a side the torus allows."
                ))
            })?;
        let tile = two::create(checked(code, 2, base)?, number, 1, 0, base)?;
        let types = tile.types().bytes().to_vec();
        let digits: Vec<(usize, usize)> = (0..number * number)
            .filter(|&at| types[at] != 0)
            .map(|at| (at / number, at % number))
            .collect();
        if digits.is_empty() {
            return Err(Fault::new("the empty design carries no modes."));
        }
        let wheel = (0..span)
            .map(|step| {
                let angle = TAU * step as f64 / span as f64;
                (angle.cos(), angle.sin())
            })
            .collect();
        Ok(Mask {
            side: number,
            level,
            span,
            digits,
            wheel,
        })
    }

    fn mass(&self) -> f64 {
        (self.digits.len() as f64).powi(self.level as i32)
    }

    fn value(&self, t1: usize, t2: usize) -> (f64, f64) {
        let (mut re, mut im) = (1.0, 0.0);
        let mut scale = 1usize;
        for _ in 0..self.level {
            let (mut fr, mut fi) = (0.0, 0.0);
            for &(a, b) in &self.digits {
                let dot = (a * t1 + b * t2) % self.span;
                let (c, s) = self.wheel[(dot * scale) % self.span];
                fr += c;
                fi += s;
            }
            let next = (re * fr - im * fi, re * fi + im * fr);
            re = next.0;
            im = next.1;
            scale = (scale * self.side) % self.span;
        }
        (re, im)
    }
}

// EXPORTS

/// Reads the eigenvalue field of a design's level-L mask: `|lambda(t)| / k^L` row-major on the `q^L` by `q^L` frequency torus, so `t = 0` reads one.
///
/// The design is the plane code at side `q` and residue base, whose level-one filled cells are
/// the digit set `F` of `k = |F|` offsets. The mask operator lays `S_L` over every site, its
/// eigenvalue at the character `e(<t, x> / q^L)` is the product of `L` rescaled copies of the
/// tile's own transform, and the side is refused past 256.
#[wasm_bindgen]
pub fn modes_field(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
) -> Result<Vec<f32>, Fault> {
    let mask = Mask::read(code, number, level, base)?;
    let mass = mask.mass();
    let mut field = Vec::with_capacity(mask.span * mask.span);
    for t1 in 0..mask.span {
        for t2 in 0..mask.span {
            let (re, im) = mask.value(t1, t2);
            field.push((re.hypot(im) / mass) as f32);
        }
    }
    Ok(field)
}

/// Reads the eigenvalue at one frequency as the three numbers `re`, `im` and `|lambda| / k^L`.
///
/// The frequency is taken on the torus, so `t1` and `t2` are read modulo `q^L`.
#[wasm_bindgen]
pub fn modes_value(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    t1: usize,
    t2: usize,
) -> Result<Vec<f64>, Fault> {
    let mask = Mask::read(code, number, level, base)?;
    let (re, im) = mask.value(t1 % mask.span, t2 % mask.span);
    Ok(vec![re, im, re.hypot(im) / mask.mass()])
}

/// Counts the frequencies whose eigenvalue reaches the threshold, `|lambda(t)| >= threshold * k^L`, the large-values set of the mask.
///
/// A threshold of zero counts the whole torus and a threshold of one counts the frequencies the
/// mask leaves untouched, `t = 0` among them.
#[wasm_bindgen]
pub fn modes_large(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    threshold: f64,
) -> Result<usize, Fault> {
    let mask = Mask::read(code, number, level, base)?;
    let bar = threshold * mask.mass();
    let mut count = 0;
    for t1 in 0..mask.span {
        for t2 in 0..mask.span {
            let (re, im) = mask.value(t1, t2);
            if re.hypot(im) >= bar {
                count += 1;
            }
        }
    }
    Ok(count)
}

/// Draws the real standing pattern of one frequency, `cos(2 pi <t, x> / q^L)` row-major over the torus, every value in minus one to one.
#[wasm_bindgen]
pub fn modes_pattern(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    t1: usize,
    t2: usize,
) -> Result<Vec<f32>, Fault> {
    let mask = Mask::read(code, number, level, base)?;
    let (t1, t2) = (t1 % mask.span, t2 % mask.span);
    let mut pattern = Vec::with_capacity(mask.span * mask.span);
    for x1 in 0..mask.span {
        for x2 in 0..mask.span {
            let dot = (t1 * x1 + t2 * x2) % mask.span;
            pattern.push(mask.wheel[dot].0 as f32);
        }
    }
    Ok(pattern)
}

/// Counts the filled cells of the design's level-one tile, the digit count `k` the mass `k^L` is built from.
#[wasm_bindgen]
pub fn modes_digits(code: &str, number: usize, base: usize) -> Result<usize, Fault> {
    Ok(Mask::read(code, number, 1, base)?.digits.len())
}
