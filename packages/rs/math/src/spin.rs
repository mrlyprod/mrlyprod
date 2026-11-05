use mrly_core::error::{Result, shape_error};
use mrly_num::factor::gcd;
use serde::{Deserialize, Serialize};
use std::f64::consts::{PI, SQRT_2};

fn centre(size: usize) -> f64 {
    size as f64 / 2.0
}

pub fn reach(size: usize) -> f64 {
    size as f64 / SQRT_2
}

pub fn arcs(data: &[f32], size: usize, radius: f64) -> Result<Vec<(f64, f64, f32)>> {
    if data.len() != size * size {
        return shape_error(format!(
            "a raster of side {size} needs {} samples, got {}.",
            size * size,
            data.len()
        ));
    }
    if size == 0 {
        return Ok(Vec::new());
    }
    let c = centre(size);
    let r = radius.max(1e-9);
    let mut cuts = vec![0.0, 2.0 * PI];
    let low = (c - r).floor().max(0.0) as usize;
    let high = ((c + r).ceil() as usize).min(size);
    for line in low..=high {
        let u = (line as f64 - c) / r;
        if u.abs() < 1.0 {
            let a = u.acos();
            cuts.push(a);
            cuts.push(2.0 * PI - a);
            let b = u.asin();
            cuts.push(b.rem_euclid(2.0 * PI));
            cuts.push((PI - b).rem_euclid(2.0 * PI));
        }
    }
    cuts.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mut out = Vec::with_capacity(cuts.len());
    for pair in cuts.windows(2) {
        let (a, b) = (pair[0], pair[1]);
        if b <= a {
            continue;
        }
        let mid = (a + b) / 2.0;
        let x = c + r * mid.cos();
        let y = c + r * mid.sin();
        let mut value = 0.0;
        if x >= 0.0 && y >= 0.0 {
            let (col, row) = (x as usize, y as usize);
            if col < size && row < size {
                value = data[row * size + col];
            }
        }
        out.push((a, b, value));
    }
    Ok(out)
}

pub fn ring(data: &[f32], size: usize, radius: f64) -> Result<f64> {
    Ok(arcs(data, size, radius)?
        .iter()
        .map(|&(a, b, v)| v as f64 * (b - a))
        .sum::<f64>()
        / (2.0 * PI))
}

pub fn harmonics(data: &[f32], size: usize, rings: usize, orders: usize) -> Result<Vec<f64>> {
    let rings = rings.max(2);
    let far = reach(size);
    let step = far / (rings - 1) as f64;
    let mut power = vec![0.0; orders + 1];
    for k in 0..rings {
        let r = k as f64 * step;
        let pieces = arcs(data, size, r)?;
        let mut re = vec![0.0; orders + 1];
        let mut im = vec![0.0; orders + 1];
        for &(a, b, v) in &pieces {
            if v == 0.0 {
                continue;
            }
            let v = v as f64;
            re[0] += v * (b - a);
            for m in 1..=orders {
                let f = m as f64;
                re[m] += v * ((f * b).sin() - (f * a).sin()) / f;
                im[m] += v * ((f * b).cos() - (f * a).cos()) / f;
            }
        }
        for m in 0..=orders {
            let (x, y) = (re[m] / (2.0 * PI), im[m] / (2.0 * PI));
            power[m] += (x * x + y * y) * 2.0 * PI * r * step;
        }
    }
    Ok(power)
}

pub fn turns(power: &[f64]) -> usize {
    let total: f64 = power.iter().sum();
    power
        .iter()
        .enumerate()
        .skip(1)
        .filter(|&(_, &p)| p > 1e-4 * total)
        .fold(0, |g, (m, _)| gcd(g as u128, m as u128) as usize)
}

pub fn petals(copies: usize, order: usize) -> usize {
    if copies == 0 || order == 0 {
        return 0;
    }
    copies / gcd(copies as u128, order as u128) as usize * order
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Blend {
    Mean,
    Sum,
    Union,
    Meet,
    Parity,
    Difference,
}

impl Blend {
    pub fn named(name: &str) -> Option<Blend> {
        match name {
            "mean" => Some(Blend::Mean),
            "sum" => Some(Blend::Sum),
            "union" => Some(Blend::Union),
            "meet" => Some(Blend::Meet),
            "parity" => Some(Blend::Parity),
            "difference" => Some(Blend::Difference),
            _ => None,
        }
    }

    pub fn fold(self, values: &[f32]) -> f32 {
        let sum: f32 = values.iter().sum();
        let max = values.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        match self {
            Blend::Mean => sum / values.len() as f32,
            Blend::Sum => sum,
            Blend::Union => max,
            Blend::Meet => values.iter().cloned().fold(f32::INFINITY, f32::min),
            Blend::Parity => 1.0 - (sum.rem_euclid(2.0) - 1.0).abs(),
            Blend::Difference => {
                let rest = values[1..].iter().cloned().fold(0.0, f32::max);
                (values[0] - rest).max(0.0)
            }
        }
    }
}

pub fn radial(
    data: &[f32],
    size: usize,
    out: usize,
    copies: usize,
    step: f64,
    blend: Blend,
    samples: usize,
) -> Result<Vec<f32>> {
    if data.len() != size * size {
        return shape_error(format!(
            "a raster of side {size} needs {} samples, got {}.",
            size * size,
            data.len()
        ));
    }
    let copies = copies.max(1);
    let samples = samples.max(1);
    let c = centre(size);
    let scale = 2.0 * reach(size) / out as f64;
    let turns: Vec<(f64, f64)> = (0..copies)
        .map(|k| {
            let angle = 2.0 * PI * step * k as f64;
            (angle.cos(), angle.sin())
        })
        .collect();
    let mut values = vec![0.0f32; copies];
    let mut field = Vec::with_capacity(out * out);
    for i in 0..out {
        for j in 0..out {
            let mut total = 0.0;
            for a in 0..samples {
                for b in 0..samples {
                    let px =
                        (j as f64 + (b as f64 + 0.5) / samples as f64 - out as f64 / 2.0) * scale;
                    let py =
                        (i as f64 + (a as f64 + 0.5) / samples as f64 - out as f64 / 2.0) * scale;
                    for (k, &(cos, sin)) in turns.iter().enumerate() {
                        let x = c + px * cos + py * sin;
                        let y = c - px * sin + py * cos;
                        values[k] = if x >= 0.0 && y >= 0.0 && x < size as f64 && y < size as f64 {
                            data[y as usize * size + x as usize]
                        } else {
                            0.0
                        };
                    }
                    total += blend.fold(&values);
                }
            }
            field.push(total / (samples * samples) as f32);
        }
    }
    Ok(field)
}

pub fn profile(data: &[f32], size: usize, steps: usize) -> Result<Vec<f32>> {
    let steps = steps.max(2);
    let far = reach(size);
    (0..steps)
        .map(|k| Ok(ring(data, size, far * k as f64 / (steps - 1) as f64)? as f32))
        .collect()
}

pub fn wheel(profile: &[f32], size: usize) -> Vec<f32> {
    let last = profile.len().saturating_sub(1);
    if last == 0 {
        return vec![profile.first().copied().unwrap_or(0.0); size * size];
    }
    let c = centre(size);
    let scale = 2.0 * last as f64 / size as f64;
    let mut out = Vec::with_capacity(size * size);
    for row in 0..size {
        for col in 0..size {
            let (dx, dy) = (col as f64 + 0.5 - c, row as f64 + 0.5 - c);
            let t = (dx * dx + dy * dy).sqrt() * scale;
            let i = (t.floor() as usize).min(last);
            let f = (t - i as f64) as f32;
            let value = if i == last {
                profile[last]
            } else {
                profile[i] * (1.0 - f) + profile[i + 1] * f
            };
            out.push(value);
        }
    }
    out
}

pub fn mass(profile: &[f32], size: usize) -> f64 {
    let last = profile.len().saturating_sub(1);
    if last == 0 {
        return 0.0;
    }
    let step = reach(size) / last as f64;
    let weight = |k: usize| 2.0 * PI * (k as f64 * step) * profile[k] as f64;
    let inner: f64 = (1..last).map(weight).sum();
    step * (inner + (weight(0) + weight(last)) / 2.0)
}

pub fn mass_within(profile: &[f32], size: usize, radius: f64) -> f64 {
    let last = profile.len().saturating_sub(1);
    if last == 0 || radius <= 0.0 {
        return 0.0;
    }
    let step = reach(size) / last as f64;
    let weight = |k: usize| 2.0 * PI * (k as f64 * step) * profile[k] as f64;
    let full = (radius / step).floor().min(last as f64) as usize;
    let mut total = 0.0;
    for k in 1..=full {
        total += (weight(k - 1) + weight(k)) / 2.0 * step;
    }
    if full < last {
        let rest = radius - full as f64 * step;
        let share = rest / step;
        let edge = weight(full) + (weight(full + 1) - weight(full)) * share;
        total += (weight(full) + edge) / 2.0 * rest;
    }
    total
}
