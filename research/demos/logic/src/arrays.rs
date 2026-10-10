use crate::Fault;
use mrlyrs::core::json;
use mrlyrs::num::arrays::{essential, fractal, holes, law, lost, paired, weights};
use wasm_bindgen::prelude::*;

const SENSORS: u64 = 4096;
const REACH: u64 = 1 << 19;
const DIGITS: usize = 8;
const SPAN: u64 = 40;

struct Array {
    generator: Vec<u64>,
    base: u64,
    level: u32,
    sensors: Vec<u64>,
}

fn generator_of(text: &str) -> Result<Vec<u64>, Fault> {
    let mut digits = text
        .split(|c: char| !c.is_ascii_digit())
        .filter(|word| !word.is_empty())
        .map(|word| {
            word.parse::<u64>()
                .map_err(|_| Fault::new(format!("{word} is not a whole number.")))
        })
        .collect::<Result<Vec<u64>, Fault>>()?;
    digits.sort_unstable();
    digits.dedup();
    let least = digits.first().copied().unwrap_or(0);
    let digits: Vec<u64> = digits.iter().map(|d| d - least).collect();
    if !(2..=DIGITS).contains(&digits.len()) || digits.last().is_some_and(|&a| a > SPAN) {
        return Err(Fault::new(format!(
            "a generator holds 2 to {DIGITS} sensors spanning at most {SPAN}."
        )));
    }
    Ok(digits)
}

fn reach(span: u64, base: u64, level: u32) -> u64 {
    (0..level).fold(0, |acc, _| acc.saturating_mul(base).saturating_add(span))
}

fn cap(generator: &[u64], base: u64) -> u32 {
    let span = generator.last().copied().unwrap_or(0);
    let size = generator.len() as u64;
    (2..)
        .take_while(|&r| size.saturating_pow(r) <= SENSORS && reach(span, base, r) <= REACH)
        .last()
        .unwrap_or(1)
}

fn built(text: &str, base: u32, level: u32) -> Result<Array, Fault> {
    let generator = generator_of(text)?;
    let span = generator.last().copied().unwrap_or(0);
    let base = u64::from(base);
    if base <= span || base > 2 * span + 1 {
        return Err(Fault::new(format!(
            "the base runs from {} to {}, not {base}.",
            span + 1,
            2 * span + 1
        )));
    }
    let top = cap(&generator, base);
    if !(1..=top).contains(&level) {
        return Err(Fault::new(format!(
            "the level runs from 1 to {top} at base {base}, not {level}."
        )));
    }
    let sensors = fractal(&generator, base, level)?;
    Ok(Array {
        generator,
        base,
        level,
        sensors,
    })
}

/// Reads a typed generator as JSON: its sensors shifted to start at 0, its size `L`, its span `a` and the full base `2a + 1`.
#[wasm_bindgen]
pub fn arrays_generator(text: &str) -> Result<String, Fault> {
    let generator = generator_of(text)?;
    let span = generator.last().copied().unwrap_or(0);
    Ok(json!({
        "generator": generator,
        "size": generator.len(),
        "span": span,
        "full": 2 * span + 1,
    })
    .to_string())
}

/// Returns the deepest level, at least one, this page builds for the generator at the base: at most 4096 sensors and a coarray reach of `2^19`.
#[wasm_bindgen]
pub fn arrays_cap(text: &str, base: u32) -> Result<u32, Fault> {
    Ok(cap(&generator_of(text)?, u64::from(base)))
}

/// Reads the fractal array of the generator at the base and level as JSON: its sensors and which are essential, the counted fragility, the coarray reach and holes, the paired digits `U(G)` and essential digits `E(G)`, the law `(u^r, L^r)` of Theorem A at the full base, and the two published bounds.
#[wasm_bindgen]
pub fn arrays_read(text: &str, base: u32, level: u32) -> Result<String, Fault> {
    let array = built(text, base, level)?;
    let g = &array.generator;
    let size = g.len() as u64;
    let span = g.last().copied().unwrap_or(0);
    let flags = essential(&array.sensors);
    let count = flags.iter().filter(|&&e| e).count() as u64;
    let total = array.sensors.len() as u64;
    let kernel: Vec<u64> = g
        .iter()
        .zip(essential(g))
        .filter_map(|(&d, e)| e.then_some(d))
        .collect();
    let full = array.base == 2 * span + 1;
    let law = law(g, array.level).filter(|_| full);
    let k = kernel.len() as u64;
    Ok(json!({
        "generator": g,
        "size": size,
        "span": span,
        "full": 2 * span + 1,
        "base": array.base,
        "level": array.level,
        "sensors": array.sensors,
        "essential": flags,
        "count": count,
        "total": total,
        "fragility": count as f64 / total as f64,
        "reach": array.sensors.last(),
        "holes": holes(&array.sensors).len(),
        "seed": holes(g).len(),
        "paired": paired(g),
        "kernel": kernel,
        "law": law.map(|(count, total)| json!({
            "count": count,
            "total": total,
            "fragility": count as f64 / total as f64,
        })),
        "bounds": {
            "cohen": [k, size],
            "yang": [k.pow(array.level), size.pow(array.level)],
        },
    })
    .to_string())
}

/// Returns the coarray weights of the array with one sensor knocked out, by its index, or of the whole array when the index is past the last: `w(t)` for every lag `t` from `-A` to `A` of the whole array, at index `t + A`.
#[wasm_bindgen]
pub fn arrays_weights(text: &str, base: u32, level: u32, out: u32) -> Result<Vec<u32>, Fault> {
    let array = built(text, base, level)?;
    let reach = array.sensors.last().copied().unwrap_or(0) as usize;
    let rest: Vec<u64> = array
        .sensors
        .iter()
        .enumerate()
        .filter_map(|(i, &s)| (i != out as usize).then_some(s))
        .collect();
    let inner = weights(&rest);
    let mut all = vec![0u32; 2 * reach + 1];
    let shift = reach - inner.len() / 2;
    for (i, w) in inner.into_iter().enumerate() {
        all[shift + i] = w as u32;
    }
    Ok(all)
}

/// Reads one knocked-out sensor, by its index, as JSON: its position, its base digits lowest first, whether every digit is paired, and the lags its loss deletes from the coarray.
#[wasm_bindgen]
pub fn arrays_knock(text: &str, base: u32, level: u32, out: u32) -> Result<String, Fault> {
    let array = built(text, base, level)?;
    let Some(&sensor) = array.sensors.get(out as usize) else {
        return Err(Fault::new(format!(
            "the array holds {} sensors, not {}.",
            array.sensors.len(),
            out + 1
        )));
    };
    let mut rest = sensor;
    let digits: Vec<u64> = (0..array.level)
        .map(|_| {
            let d = rest % array.base;
            rest /= array.base;
            d
        })
        .collect();
    let paired = paired(&array.generator);
    let gone = lost(&array.sensors, sensor);
    Ok(json!({
        "sensor": sensor,
        "digits": digits,
        "paired": digits.iter().all(|d| paired.contains(d)),
        "essential": !gone.is_empty(),
        "lost": gone,
    })
    .to_string())
}
