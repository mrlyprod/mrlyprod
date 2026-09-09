use crate::{code_of, Fault, Grid};
use mrlycore::json;
use mrlymath::six::star::{arm_law, chi8, constant, decay, width_law, Branch, Star};
use wasm_bindgen::prelude::*;

const LAYER_CAP: usize = 800;
const FIELD_CAP: usize = 128;
const SIZE_CAP: usize = 384;
const HALF_CAP: usize = 64;

// GUARDS

fn star_of(code: &str) -> Result<Star, Fault> {
    Ok(Star::new(code_of(code)?)?)
}

fn layers_of(layers: usize, cap: usize) -> Result<usize, Fault> {
    if !(2..=cap).contains(&layers) {
        return Err(Fault::new(format!(
            "the layer count must be between 2 and {cap}."
        )));
    }
    Ok(layers)
}

fn half_of(half: usize) -> Result<usize, Fault> {
    if half > HALF_CAP {
        return Err(Fault::new(format!(
            "the band half-width must be at most {HALF_CAP} cells."
        )));
    }
    Ok(half)
}

fn size_of(size: usize) -> Result<usize, Fault> {
    if !(16..=SIZE_CAP).contains(&size) {
        return Err(Fault::new(format!(
            "the raster size must be between 16 and {SIZE_CAP}."
        )));
    }
    Ok(size)
}

fn columns(size: usize, n: i64) -> Vec<i64> {
    (0..size)
        .map(|step| {
            let point = (step as f64 + 0.5) / size as f64;
            ((point * 4.0 * n as f64).floor() as i64).min(4 * n - 1)
        })
        .collect()
}

fn rows(size: usize, n: i64) -> Vec<i64> {
    (0..size)
        .map(|step| {
            let point = (step as f64 + 0.5) / size as f64;
            (2 * (point * 2.0 * n as f64).floor() as i64).min(4 * n - 2)
        })
        .collect()
}

// EXPORTS

/// Stacks the first `L` odd cut layers onto one square raster and hands back the mean ink at every sample, row-major, `NaN` outside the hexagon every layer shares.
///
/// This is the ideal frame: each layer is resampled onto the common grid, the row of a sample being
/// `z = 2 floor(2 n Z)` and its column `x = floor(4 n X)`, so the half-cell displacement between
/// layers is averaged away. The three bright lines through the centre are the ghost star, and they
/// fade as the layers pile up, since the star and the background both walk to one half.
#[wasm_bindgen]
pub fn star_field(code: &str, layers: usize, size: usize) -> Result<Vec<f32>, Fault> {
    let star = star_of(code)?;
    let layers = layers_of(layers, FIELD_CAP)?;
    let size = size_of(size)?;
    let mut total = vec![0f32; size * size];
    let mut whole = vec![true; size * size];
    for step in 0..layers {
        let number = 2 * step + 1;
        let n = number as i64;
        let across = columns(size, n);
        let down = rows(size, n);
        for (line, z) in down.iter().enumerate() {
            let base = line * size;
            for (slot, x) in across.iter().enumerate() {
                match star.cell(number, *x, *z) {
                    Some(fill) => total[base + slot] += f32::from(u8::from(fill)),
                    None => whole[base + slot] = false,
                }
            }
        }
    }
    Ok(total
        .iter()
        .zip(&whole)
        .map(
            |(sum, keep)| {
                if *keep {
                    sum / layers as f32
                } else {
                    f32::NAN
                }
            },
        )
        .collect())
}

/// Marks the same raster with the band the star is measured on: `0` the star, `1` the hexagon background, `2` outside.
///
/// The band is `|x - y| <= W` cells of the deepest layer, the widest and last of the `L` stacked, so
/// widening `W` widens the arms of the cross on the picture exactly as it widens the reading.
#[wasm_bindgen]
pub fn star_band(code: &str, layers: usize, size: usize, half: usize) -> Result<Grid, Fault> {
    let star = star_of(code)?;
    let layers = layers_of(layers, FIELD_CAP)?;
    let size = size_of(size)?;
    let half = half_of(half)? as i64;
    let mut types = vec![2u8; size * size];
    let deepest = 2 * layers - 1;
    let n = deepest as i64;
    let across = columns(size, n);
    let down = rows(size, n);
    for step in 0..layers {
        let number = 2 * step + 1;
        let inner = columns(size, number as i64);
        let heights = rows(size, number as i64);
        for (line, z) in heights.iter().enumerate() {
            let base = line * size;
            for (slot, x) in inner.iter().enumerate() {
                if star.cell(number, *x, *z).is_none() {
                    types[base + slot] = 3;
                }
            }
        }
    }
    for (line, z) in down.iter().enumerate() {
        let base = line * size;
        let y = |x: i64| 6 * n - 2 - x - z;
        for (slot, x) in across.iter().enumerate() {
            if types[base + slot] == 3 {
                continue;
            }
            let (x, y, z) = (*x, y(*x), *z);
            let near = (x - y).abs() <= half || (y - z).abs() <= half || (z - x).abs() <= half;
            types[base + slot] = u8::from(!near);
        }
    }
    for slot in types.iter_mut() {
        if *slot == 3 {
            *slot = 2;
        }
    }
    Ok(Grid {
        width: size as u32,
        height: size as u32,
        types,
    })
}

/// Reads one row per odd layer: the band's exact ink as a fraction, the closed form `1/2 + chi_8(n)/(2n)` beside it, the hexagon's own ink and the excess of one over the other.
///
/// At `W = 0` the band is the arm `x = y` itself, a diameter of `2n` cells with no width to choose,
/// and `exact` says the counted fraction is the closed form cell for cell. `W = 1` is the same point
/// set, since `x - y` is even on the cut, so it is exact too and an odd width is never a new band.
/// The background is the whole hexagon's ink at that layer, so `excess` is the per-layer ghost the
/// stack averages.
#[wasm_bindgen]
pub fn star_layers(code: &str, layers: usize, half: usize) -> Result<String, Fault> {
    let star = star_of(code)?;
    let layers = layers_of(layers, LAYER_CAP)?;
    let half = half_of(half)?;
    let mut out = Vec::with_capacity(layers);
    let mut exact = 0;
    for step in 0..layers {
        let number = 2 * step + 1;
        let band = star.arm(number, half)?;
        let hexagon = star.hexagon(number)?;
        let law = arm_law(number)?;
        let (numer, denom) = band.reduced();
        let (top, bottom) = law.reduced();
        let held = band == law;
        exact += usize::from(held);
        out.push(json!({
            "n": number,
            "inked": band.inked,
            "cells": band.cells,
            "numer": numer,
            "denom": denom,
            "ink": band.value(),
            "lawNumer": top,
            "lawDenom": bottom,
            "law": law.value(),
            "chi": chi8(number),
            "hex": hexagon.value(),
            "excess": band.value() - hexagon.value(),
            "exact": held,
        }));
    }
    Ok(json!({
        "layers": layers,
        "half": half,
        "exact": exact,
        "rows": out,
    })
    .to_string())
}

/// Reads the decay in the cell frame: the `L`-layer excess scaled by `L`, the `(ln L)/4` it hides, the constant it settles on and the slope against `ln L` the coefficient is.
///
/// Nothing is resampled here: the star is the exact arm band of each layer and the background that
/// layer's own hexagon, so the reading is a ratio of cell counts. At `W = 0` and even `L` the law is
/// `L * excess_L = -(ln L)/4 + C + O(1/L^2)` with `C` in closed form, the slope settles on `-1/4`,
/// and the `1/L^2` term reads `L` mod 4: `-23/192` at `L = 0 mod 4` and `+25/192` at `L = 2 mod 4`.
/// Odd `L` shifts the constant by `1/8` and leaves `O(1/L)` behind. A wider band walks the slope off
/// `-1/4` onto `-(K + b)/(4(2K + 1))`, which is why the star has no frame-free coefficient. The
/// slope is absent unless `L` is divisible by four, the one window that cancels the character term
/// at both of its ends.
#[wasm_bindgen]
pub fn star_decay(code: &str, layers: usize, half: usize) -> Result<String, Fault> {
    let star = star_of(code)?;
    let layers = layers_of(layers, LAYER_CAP)?;
    let half = half_of(half)?;
    let excesses = star.excesses(layers, half)?;
    let read = decay(&excesses, layers)?;
    let branch = Branch::of(layers);
    let mut rungs = vec![layers];
    while rungs[rungs.len() - 1] % 4 == 0 && rungs[rungs.len() - 1] / 2 >= 4 {
        rungs.push(rungs[rungs.len() - 1] / 2);
    }
    let mut ladder = Vec::new();
    for count in rungs {
        let rung = decay(&excesses, count)?;
        let class = Branch::of(count);
        ladder.push(json!({
            "layers": count,
            "excess": rung.excess,
            "scaled": rung.scaled,
            "logged": rung.logged,
            "miss": rung.miss,
            "linear": rung.linear,
            "residual": rung.residual,
            "slope": rung.slope,
            "branch": class.name(),
            "predicted": class.residual(),
        }));
    }
    ladder.reverse();
    let mut walk = Vec::new();
    let mut count = 4;
    while count <= layers {
        let rung = decay(&excesses, count)?;
        walk.push(json!([count, rung.scaled]));
        count += 2;
    }
    Ok(json!({
        "layers": layers,
        "half": half,
        "deepest": 2 * layers - 1,
        "excess": read.excess,
        "scaled": read.scaled,
        "logged": read.logged,
        "miss": read.miss,
        "linear": read.linear,
        "residual": read.residual,
        "slope": read.slope,
        "target": width_law(half),
        "arm": half == 0,
        "constant": constant(),
        "branch": branch.name(),
        "branchConstant": branch.constant(),
        "predicted": branch.residual(),
        "rows": ladder,
        "walk": walk,
    })
    .to_string())
}
