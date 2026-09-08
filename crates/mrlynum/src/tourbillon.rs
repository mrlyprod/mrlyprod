use crate::classics::primes;
use crate::factor::{factorize, gcd, mobius, squarefree};
use crate::prime::{is_prime, squares};
use crate::spin::Blend;
use mrlycore::errors::{value_error, Result};
use mrlycore::Rng;

const SCALE_CAP: usize = 199;
const SIZE_FLOOR: usize = 16;
const SIZE_CAP: usize = 1024;
const SHOWN: usize = 8;
const PEAKS: usize = 3;
const LADDER: usize = 1000;
const PLANE_CAP: usize = 64_000_000;
const PERIOD_CAP: usize = 360;
const TIGHT: f64 = 1e-9;

// THE LAYERS

/// One carpet of a stack: the odd scale it is drawn at, the weight the linear blends carry it at and the turn it takes about the centre, in degrees.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Layer {
    /// The odd scale, the number of cells across the carpet.
    pub scale: usize,
    /// The weight, scaled so the magnitudes average one.
    pub weight: f64,
    /// The turn about the centre, in degrees.
    pub degrees: f64,
}

/// The readings of a spun stack: its layers, the first eight scales and angles, the mean and RMS contrast over the disc, that contrast times the root of the layer count, the exact centre value, whether the blend carries the weights, the span the raster covers, the sites inside the disc and the brightest three.
#[derive(Clone, Debug, PartialEq)]
pub struct Stats {
    /// The count of layers in the stack.
    pub layers: usize,
    /// The first eight scales.
    pub scales: Vec<usize>,
    /// The first eight angles, in degrees.
    pub angles: Vec<f64>,
    /// The mean over the disc.
    pub mean: f64,
    /// The RMS contrast over the disc.
    pub rms: f64,
    /// The RMS contrast times the root of the layer count.
    pub faded: f64,
    /// The exact value at the centre, computed and never sampled.
    pub centre: f64,
    /// Whether the blend carries the weights.
    pub weighted: bool,
    /// The smallest value over the disc.
    pub low: f64,
    /// The largest value over the disc.
    pub high: f64,
    /// The count of sites inside the disc.
    pub inside: usize,
    /// The brightest three sites, each as its unit coordinates and its value.
    pub peaks: Vec<[f64; 3]>,
    /// The least whole number of increments that closes a quarter turn, none past the cap.
    pub period: Option<usize>,
    /// The count of distinct angle classes the layers fall in, read a quarter turn apart.
    pub classes: usize,
    /// The count of layer pairs sharing an angle class.
    pub pairs: usize,
}

/// One angle of the quarter-turn lattice: the turn in degrees and the ninety a over q that names it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Eye {
    /// The turn in degrees.
    pub angle: f64,
    /// The numerator, ninety times a.
    pub numer: usize,
    /// The denominator q.
    pub denom: usize,
}

fn golden() -> f64 {
    180.0 * (3.0 - 5f64.sqrt())
}

// THE QUARTER TURN

/// The angles a quarter turn shares with itself: ninety a over q for every q up to the cap and every a from zero to four q coprime to it, sorted by angle.
///
/// The odd carpet is unchanged by a quarter turn, so two layers whose angles agree modulo ninety stand on one exact lattice of nodes and the stack stops moving there.
///
/// ```
/// let list = mrlynum::tourbillon::eyes(2);
/// assert_eq!(list.len(), 9);
/// assert_eq!(list[1].angle, 45.0);
/// ```
pub fn eyes(qmax: usize) -> Vec<Eye> {
    let mut out = Vec::new();
    for denom in 1..=qmax {
        for step in 0..=4 * denom {
            if gcd(step, denom) != 1 {
                continue;
            }
            out.push(Eye {
                angle: 90.0 * step as f64 / denom as f64,
                numer: 90 * step,
                denom,
            });
        }
    }
    out.sort_by(|a, b| a.angle.total_cmp(&b.angle));
    out
}

/// The least whole number of increments that closes a quarter turn, none once the count passes the cap.
///
/// ```
/// assert_eq!(mrlynum::tourbillon::period(18.0), Some(5));
/// assert_eq!(mrlynum::tourbillon::period(1.0), Some(90));
/// ```
pub fn period(increment: f64) -> Option<usize> {
    (1..=PERIOD_CAP).find(|count| {
        let turns = *count as f64 * increment / 90.0;
        (turns - turns.round()).abs() <= TIGHT
    })
}

fn quarter(degrees: f64) -> f64 {
    let angle = degrees.rem_euclid(90.0);
    if 90.0 - angle <= TIGHT {
        0.0
    } else {
        angle
    }
}

/// The angle classes of a stack read a quarter turn apart: how many the layers fall in, and how many layer pairs share one.
pub fn sharing(list: &[Layer]) -> (usize, usize) {
    let mut angles: Vec<f64> = list.iter().map(|layer| quarter(layer.degrees)).collect();
    angles.sort_by(f64::total_cmp);
    let mut counts: Vec<usize> = Vec::new();
    let mut held = f64::NEG_INFINITY;
    for angle in angles {
        match counts.last_mut() {
            Some(count) if angle - held <= TIGHT => *count += 1,
            _ => {
                counts.push(1);
                held = angle;
            }
        }
    }
    let pairs = counts.iter().map(|count| count * (count - 1) / 2).sum();
    (counts.len(), pairs)
}

fn odd_top(top: usize) -> Result<usize> {
    if !(3..=SCALE_CAP).contains(&top) || top.is_multiple_of(2) {
        return value_error(format!(
            "the top scale must be odd between 3 and {SCALE_CAP}."
        ));
    }
    Ok(top)
}

fn side(size: usize) -> Result<usize> {
    if !(SIZE_FLOOR..=SIZE_CAP).contains(&size) {
        return value_error(format!(
            "the raster must be between {SIZE_FLOOR} and {SIZE_CAP} pixels."
        ));
    }
    Ok(size)
}

fn blend_of(name: &str) -> Result<Blend> {
    match Blend::named(name) {
        Some(blend) => Ok(blend),
        None => value_error(format!(
            "blend {name:?} is not mean, sum, union, meet, parity or difference."
        )),
    }
}

fn linear(blend: Blend) -> bool {
    matches!(blend, Blend::Mean | Blend::Sum)
}

fn chosen(scale: usize, set: &str) -> Result<bool> {
    match set {
        "odd" => Ok(true),
        "primes" => Ok(is_prime(scale)),
        "squarefree" => Ok(squarefree(scale)),
        "prime powers" => Ok(factorize(scale).len() == 1),
        _ => value_error(format!(
            "layer set {set:?} is not odd, primes, squarefree or prime powers."
        )),
    }
}

fn weight_of(scale: usize, weights: &str) -> Result<f64> {
    match weights {
        "plain" => Ok(1.0),
        "mobius" => Ok(f64::from(mobius(scale))),
        "harmonic" => Ok(1.0 / scale as f64),
        _ => value_error(format!(
            "weights {weights:?} are not plain, mobius or harmonic."
        )),
    }
}

/// The layers of a stack: every scale one, three, five up to the top the set keeps, each with its weight and its angle.
///
/// The schedule is unspun, degrees, golden, primes, random or gaussian, the layer set is odd, primes, squarefree or prime powers and the weights are plain, mobius or harmonic. The weights are rescaled so their magnitudes sum to the layer count, so the mean blend stays paper coverage and the sum blend stays the inked layer count.
pub fn layers(
    top: usize,
    schedule: &str,
    increment: f64,
    set: &str,
    weights: &str,
    seed: u32,
) -> Result<Vec<Layer>> {
    let top = odd_top(top)?;
    let ladder = primes(LADDER);
    let mut rng = Rng::new(u64::from(seed));
    let mut out: Vec<Layer> = Vec::new();
    let mut index = 0usize;
    for scale in (1..=top).step_by(2) {
        if !chosen(scale, set)? {
            continue;
        }
        index += 1;
        let degrees = match schedule {
            "unspun" => 0.0,
            "degrees" => index as f64 * increment,
            "golden" => index as f64 * golden(),
            "primes" => match index {
                1 => 0.0,
                _ => ladder.get(index - 2).copied().unwrap_or(0) as f64,
            },
            "random" => rng.unit() * 360.0,
            "gaussian" => {
                squares(scale).map_or(0.0, |(a, b)| (b as f64).atan2(a as f64).to_degrees())
            }
            _ => {
                return value_error(format!(
                "schedule {schedule:?} is not unspun, degrees, golden, primes, random or gaussian."
            ))
            }
        };
        out.push(Layer {
            scale,
            weight: weight_of(scale, weights)?,
            degrees,
        });
    }
    let mass: f64 = out.iter().map(|layer| layer.weight.abs()).sum();
    if mass > 0.0 {
        let factor = out.len() as f64 / mass;
        for layer in &mut out {
            layer.weight *= factor;
        }
    }
    Ok(out)
}

fn lit(scale: usize) -> f32 {
    f32::from(u8::from(((scale as f64 * 0.5).floor() as i64) & 1 == 1))
}

fn sample(list: &[Layer], blend: Blend) -> Vec<f32> {
    list.iter()
        .map(|layer| {
            let ink = lit(layer.scale);
            if linear(blend) {
                layer.weight as f32 * ink
            } else {
                ink
            }
        })
        .collect()
}

// THE RASTER

fn disc(size: usize) -> Vec<bool> {
    let mut out = vec![false; size * size];
    for row in 0..size {
        let v = (row as f64 + 0.5) / size as f64 - 0.5;
        for column in 0..size {
            let u = (column as f64 + 0.5) / size as f64 - 0.5;
            out[row * size + column] = u * u + v * v <= 0.25;
        }
    }
    out
}

fn cells(layer: &Layer, size: usize, inside: &[bool], ink: &mut [u8]) {
    let (sin, cos) = layer.degrees.to_radians().sin_cos();
    let scale = layer.scale as f64;
    let step = 1.0 / size as f64;
    let (dx, dy) = (cos * step, -sin * step);
    let edge = 0.5 * step - 0.5;
    for row in 0..size {
        let v = (row as f64 + 0.5) * step - 0.5;
        let mut x = cos * edge + sin * v + 0.5;
        let mut y = cos * v - sin * edge + 0.5;
        for column in 0..size {
            let at = row * size + column;
            let held = inside[at] && (0.0..1.0).contains(&x) && (0.0..1.0).contains(&y);
            ink[at] = u8::from(
                held && (scale * x).floor() as i64 & 1 == 1 && (scale * y).floor() as i64 & 1 == 1,
            );
            x += dx;
            y += dy;
        }
    }
}

fn edges(size: usize, inside: &[bool], ink: &[u8], mark: &mut [u8]) {
    for row in 0..size {
        for column in 0..size {
            let at = row * size + column;
            if !inside[at] {
                mark[at] = 0;
                continue;
            }
            let right = column + 1 < size && inside[at + 1] && ink[at + 1] != ink[at];
            let below = row + 1 < size && inside[at + size] && ink[at + size] != ink[at];
            mark[at] = u8::from(right || below);
        }
    }
}

fn corners(layer: &Layer, size: usize, inside: &[bool], mark: &mut [u8]) {
    mark.fill(0);
    let (sin, cos) = layer.degrees.to_radians().sin_cos();
    let scale = layer.scale;
    for down in (1..scale).step_by(2) {
        for across in (1..scale).step_by(2) {
            for (vx, vy) in [
                (across, down),
                (across + 1, down),
                (across, down + 1),
                (across + 1, down + 1),
            ] {
                let x = vx as f64 / scale as f64 - 0.5;
                let y = vy as f64 / scale as f64 - 0.5;
                let u = cos * x - sin * y;
                let v = sin * x + cos * y;
                if u * u + v * v > 0.25 {
                    continue;
                }
                let column = ((u + 0.5) * size as f64 - 0.5).round() as i64;
                let row = ((v + 0.5) * size as f64 - 0.5).round() as i64;
                for dy in -1..=1i64 {
                    for dx in -1..=1i64 {
                        let (a, b) = (column + dx, row + dy);
                        if a < 0 || b < 0 || a >= size as i64 || b >= size as i64 {
                            continue;
                        }
                        let at = b as usize * size + a as usize;
                        if inside[at] {
                            mark[at] = 1;
                        }
                    }
                }
            }
        }
    }
}

/// Rasters the layers onto a square of the size, every one turned about the centre by its own angle and masked to the inscribed disc, then merged site by site.
///
/// The render mode is cells, the lit squares of the carpet, edges, the boundaries between its cells, or corners, the vertices of its lit squares. The weights ride on the layers under the linear blends only. Sites outside the disc are NaN.
pub fn stack(list: &[Layer], size: usize, mode: &str, blend: Blend) -> Result<Vec<f32>> {
    if !["cells", "edges", "corners"].contains(&mode) {
        return value_error(format!(
            "render mode {mode:?} is not cells, edges or corners."
        ));
    }
    if list.len() * size * size > PLANE_CAP {
        return value_error(format!(
            "{} layers on a {size} raster passes the {PLANE_CAP} site budget.",
            list.len()
        ));
    }
    let inside = disc(size);
    let mut out = vec![f32::NAN; size * size];
    if list.is_empty() {
        return Ok(out);
    }
    let mut ink = vec![0u8; size * size];
    let mut planes: Vec<Vec<u8>> = Vec::with_capacity(list.len());
    for layer in list {
        let mut mark = vec![0u8; size * size];
        match mode {
            "corners" => corners(layer, size, &inside, &mut mark),
            "edges" => {
                cells(layer, size, &inside, &mut ink);
                edges(size, &inside, &ink, &mut mark);
            }
            _ => cells(layer, size, &inside, &mut mark),
        }
        planes.push(mark);
    }
    let scaled: Vec<f32> = list
        .iter()
        .map(|layer| {
            if linear(blend) {
                layer.weight as f32
            } else {
                1.0
            }
        })
        .collect();
    let mut buffer = vec![0f32; list.len()];
    for at in 0..size * size {
        if !inside[at] {
            continue;
        }
        for (slot, plane) in planes.iter().enumerate() {
            buffer[slot] = scaled[slot] * f32::from(plane[at]);
        }
        out[at] = blend.fold(&buffer);
    }
    Ok(out)
}

fn peaks(field: &[f32], size: usize, top: usize) -> Vec<[f64; 3]> {
    let block = (size / top.max(1)).max(1);
    let wide = size.div_ceil(block);
    let mut best: Vec<(f64, usize, usize)> = Vec::with_capacity(wide * wide);
    for tile in 0..wide * wide {
        let (bx, by) = (tile % wide * block, tile / wide * block);
        let mut hit: Option<(f64, usize, usize)> = None;
        for row in by..(by + block).min(size) {
            for column in bx..(bx + block).min(size) {
                let value = field[row * size + column];
                if value.is_nan() {
                    continue;
                }
                let value = f64::from(value);
                if hit.is_none_or(|(seen, _, _)| value > seen) {
                    hit = Some((value, column, row));
                }
            }
        }
        if let Some(found) = hit {
            best.push(found);
        }
    }
    best.sort_by(|a, b| b.0.total_cmp(&a.0));
    let apart = block as f64 / size as f64;
    let mut out: Vec<[f64; 3]> = Vec::new();
    for (value, column, row) in best {
        let x = (column as f64 + 0.5) / size as f64;
        let y = (row as f64 + 0.5) / size as f64;
        if out
            .iter()
            .all(|site| (site[0] - x).hypot(site[1] - y) >= apart)
        {
            out.push([x, y, value]);
        }
        if out.len() == PEAKS {
            break;
        }
    }
    out
}

// THE STACK

/// Spins the odd parity carpets at the scales one, three, five up to the top into one stack on a square of the size, every layer turned about the centre by its own angle and masked to the inscribed disc, so every pixel sees every layer.
///
/// The schedule is unspun, degrees, golden, primes, random or gaussian, the layer set is odd, primes, squarefree or prime powers, the weights are plain, mobius or harmonic, the render mode is cells, edges or corners and the blend is mean, sum, union, meet, parity or difference. The weights are scaled to average magnitude one and apply only under the linear blends, so mean is paper coverage and sum is the inked layer count. Sites outside the disc are NaN.
#[allow(clippy::too_many_arguments)]
pub fn field(
    top: usize,
    size: usize,
    schedule: &str,
    increment: f64,
    set: &str,
    weights: &str,
    mode: &str,
    blend: &str,
    seed: u32,
) -> Result<Vec<f32>> {
    let size = side(size)?;
    let blend = blend_of(blend)?;
    let list = layers(top, schedule, increment, set, weights, seed)?;
    stack(&list, size, mode, blend)
}

/// Reads a spun stack against the schedule that made it: the layer count, the first eight scales and angles, the mean and RMS contrast over the disc, that contrast times the root of the layer count, the exact centre value, whether the blend carries the weights, the span the raster covers and the brightest three sites.
///
/// The centre is computed from the layers at the exact half-half point, never sampled off the raster, so no turn of the layers moves it.
#[allow(clippy::too_many_arguments)]
pub fn stats(
    field: &[f32],
    size: usize,
    top: usize,
    schedule: &str,
    increment: f64,
    set: &str,
    weights: &str,
    blend: &str,
    seed: u32,
) -> Result<Stats> {
    let size = side(size)?;
    if field.len() != size * size {
        return value_error("the field must be size by size with size at least 1.");
    }
    let blend = blend_of(blend)?;
    let list = layers(top, schedule, increment, set, weights, seed)?;
    let seen: Vec<f64> = field
        .iter()
        .filter(|value| !value.is_nan())
        .map(|value| f64::from(*value))
        .collect();
    let count = seen.len().max(1) as f64;
    let mean = seen.iter().sum::<f64>() / count;
    let rms = (seen.iter().map(|value| (value - mean).powi(2)).sum::<f64>() / count).sqrt();
    let low = seen.iter().cloned().fold(f64::INFINITY, f64::min);
    let high = seen.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let centre = if list.is_empty() {
        0.0
    } else {
        f64::from(blend.fold(&sample(&list, blend)))
    };
    let (classes, pairs) = sharing(&list);
    Ok(Stats {
        layers: list.len(),
        scales: list.iter().take(SHOWN).map(|layer| layer.scale).collect(),
        angles: list.iter().take(SHOWN).map(|layer| layer.degrees).collect(),
        mean,
        rms,
        faded: rms * (list.len() as f64).sqrt(),
        centre,
        weighted: linear(blend),
        low: if low.is_finite() { low } else { 0.0 },
        high: if high.is_finite() { high } else { 0.0 },
        inside: seen.len(),
        peaks: peaks(field, size, top),
        period: period(increment),
        classes,
        pairs,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_schedule_keeps_the_odd_scales_and_turns_them_by_the_primes() {
        let odd = layers(55, "primes", 0.0, "odd", "plain", 1).unwrap();
        assert_eq!(odd.len(), 28);
        let scales: Vec<usize> = odd.iter().take(SHOWN).map(|layer| layer.scale).collect();
        assert_eq!(scales, vec![1, 3, 5, 7, 9, 11, 13, 15]);
        let angles: Vec<f64> = odd.iter().take(SHOWN).map(|layer| layer.degrees).collect();
        assert_eq!(angles, vec![0.0, 2.0, 3.0, 5.0, 7.0, 11.0, 13.0, 17.0]);
        assert_eq!(
            layers(199, "unspun", 0.0, "primes", "plain", 1)
                .unwrap()
                .len(),
            45
        );
        let counts: Vec<usize> = ["odd", "primes", "squarefree", "prime powers"]
            .iter()
            .map(|set| layers(55, "unspun", 0.0, set, "plain", 1).unwrap().len())
            .collect();
        assert_eq!(counts, vec![28, 15, 23, 19]);
        assert!(layers(54, "unspun", 0.0, "odd", "plain", 1).is_err());
        assert!(layers(55, "spiral", 0.0, "odd", "plain", 1).is_err());
    }

    #[test]
    fn the_unspun_stack_counts_eighteen_layers_at_its_brightest() {
        let list = layers(55, "unspun", 0.0, "odd", "plain", 1).unwrap();
        let raster = stack(&list, 512, "cells", Blend::Sum).unwrap();
        assert_eq!(raster.len(), 512 * 512);
        let high = raster
            .iter()
            .filter(|value| !value.is_nan())
            .cloned()
            .fold(f32::NEG_INFINITY, f32::max);
        assert_eq!(high, 18.0);
        assert!(raster[0].is_nan());
        assert!(stack(&list, 512, "dots", Blend::Sum).is_err());
    }

    #[test]
    fn the_field_and_the_stats_hold_the_centre_at_half_the_layers() {
        let raster = field(55, 512, "unspun", 0.0, "odd", "plain", "cells", "mean", 1).unwrap();
        let read = stats(&raster, 512, 55, "unspun", 0.0, "odd", "plain", "mean", 1).unwrap();
        assert_eq!(read.layers, 28);
        assert_eq!(read.centre, 14.0 / 28.0);
        assert_eq!(format!("{:.6}", read.high), format!("{:.6}", 18.0 / 28.0));
        assert!(read.weighted);
        let counted = stats(&raster, 512, 55, "unspun", 0.0, "odd", "plain", "sum", 1).unwrap();
        assert_eq!(counted.centre, 14.0);
        let folded = stats(&raster, 512, 55, "unspun", 0.0, "odd", "plain", "parity", 1).unwrap();
        assert_eq!(folded.centre, 0.0);
        assert!(!folded.weighted);
        assert!(field(55, 4096, "unspun", 0.0, "odd", "plain", "cells", "mean", 1).is_err());
        assert!(stats(&raster, 256, 55, "unspun", 0.0, "odd", "plain", "mean", 1).is_err());
    }
}
