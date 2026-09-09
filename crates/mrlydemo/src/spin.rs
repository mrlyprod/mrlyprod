use crate::{code_of, Fault, Grid, Pixels};
use mrlycore::{json, Colorizer};
use mrlylab::moire::{presets, render, Field};
use mrlymath::six;
use mrlynum::spin;
use wasm_bindgen::prelude::*;

pub(crate) fn ramp_of(ramp: &str) -> Colorizer {
    match ramp {
        "heat" => Colorizer::heat(),
        "diverge" => Colorizer::diverge(),
        _ => Colorizer::fire(),
    }
}

fn slice_raster(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    size: usize,
) -> Result<Vec<f32>, Fault> {
    let cell = six::cut_design(code_of(code)?, number, level, base)?;
    Ok(six::raster(&cell, size)?)
}

/// Spins a square field about its centre: the exact circle means at the steps radii from the centre to the corner.
#[wasm_bindgen]
pub fn profile(field: &[f32], size: usize, steps: usize) -> Result<Vec<f32>, Fault> {
    square(field, size)?;
    Ok(spin::profile(field, size, steps))
}

/// Rasterizes the diagonal slice of the cube the code names on a square of the size: one byte per pixel, one on a fill.
#[wasm_bindgen]
pub fn slice_grid(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    size: usize,
) -> Result<Grid, Fault> {
    let data = slice_raster(code, number, level, base, size)?;
    Ok(Grid {
        width: size as u32,
        height: size as u32,
        types: data.iter().map(|&v| v as u8).collect(),
    })
}

fn painted(
    data: Vec<f32>,
    size: usize,
    ramp: &str,
    levels: usize,
    invert: bool,
) -> Result<Pixels, Fault> {
    if size == 0 || data.len() != size * size {
        return Err(Fault::new(
            "the field must be size by size with size at least 1.",
        ));
    }
    let field = Field::from_data(data, size);
    let png = render(&field, &ramp_of(ramp), levels, false, invert, 1)?;
    let (width, height, colors) = mrlycore::unpng(&png)?;
    Ok(Pixels::of(width, height, colors))
}

/// Paints a ring profile back over a square of the size, quantized into levels through the fire, heat or diverge ramp.
#[wasm_bindgen]
pub fn wheel(
    profile: &[f32],
    size: usize,
    ramp: &str,
    levels: usize,
    invert: bool,
) -> Result<Pixels, Fault> {
    if size == 0 {
        return Err(Fault::new("size must be at least 1."));
    }
    painted(spin::wheel(profile, size), size, ramp, levels, invert)
}

/// Paints a square field, quantized into levels through the fire, heat or diverge ramp.
#[wasm_bindgen]
pub fn sheet(
    field: &[f32],
    size: usize,
    ramp: &str,
    levels: usize,
    invert: bool,
) -> Result<Pixels, Fault> {
    painted(field.to_vec(), size, ramp, levels, invert)
}

/// Samples a moire preset up to the scale limit on a square of the size: the raw field, row by row.
#[wasm_bindgen]
pub fn moire_field(name: &str, limit: usize, size: usize) -> Result<Vec<f32>, Fault> {
    Ok(presets::named(name, limit)?.field(size)?.data)
}

fn square(field: &[f32], size: usize) -> Result<(), Fault> {
    if size == 0 || field.len() != size * size {
        return Err(Fault::new(
            "the field must be size by size with size at least 1.",
        ));
    }
    Ok(())
}

/// Stacks a square field radially: copies turned by multiples of the step in degrees about the centre, merged by the named blend, on an output square of the out side whose inscribed circle is the field's corner circle, each pixel the mean of samples squared points.
#[wasm_bindgen]
pub fn radial(
    field: &[f32],
    size: usize,
    out: usize,
    copies: usize,
    step: f64,
    blend: &str,
    samples: usize,
) -> Result<Vec<f32>, Fault> {
    square(field, size)?;
    if out == 0 {
        return Err(Fault::new("out must be at least 1."));
    }
    let blend = spin::Blend::named(blend).ok_or_else(|| {
        Fault::new(format!(
            "blend {blend:?} is not mean, sum, union, meet, parity or difference."
        ))
    })?;
    Ok(spin::radial(
        field,
        size,
        out,
        copies,
        step / 360.0,
        blend,
        samples,
    ))
}

/// The circular-harmonic power of a square field over rings radii: one energy per order from zero to the last, each ring's coefficients exact from its arcs.
#[wasm_bindgen]
pub fn harmonics(
    field: &[f32],
    size: usize,
    rings: usize,
    orders: usize,
) -> Result<Vec<f64>, Fault> {
    square(field, size)?;
    Ok(spin::harmonics(field, size, rings, orders))
}

/// The rotation order a harmonic power spectrum reveals: the gcd of the live orders, zero when none lives.
#[wasm_bindgen]
pub fn turns(power: &[f64]) -> usize {
    spin::turns(power)
}

/// The share of the harmonic power order zero carries, in percent.
#[wasm_bindgen]
pub fn radial_share(power: &[f64]) -> f64 {
    let total: f64 = power.iter().sum();
    if total > 0.0 {
        power[0] / total * 100.0
    } else {
        0.0
    }
}

/// The step in degrees that shares one full turn over the copies.
#[wasm_bindgen]
pub fn full_turn(copies: usize) -> f64 {
    360.0 / copies.max(1) as f64
}

/// The degrees a turntable at the rpm turns between two frames at the frame rate.
#[wasm_bindgen]
pub fn frame_step(rpm: f64, fps: f64) -> f64 {
    rpm * 6.0 / fps.max(1e-9)
}

/// The petals a full radial stack of the copies shows on a design of the rotation order: their least common multiple.
#[wasm_bindgen]
pub fn petals(copies: usize, order: usize) -> usize {
    spin::petals(copies, order)
}

/// Reads a ring profile against the raster side it came from: the mass `2 pi r F(r)` integrates to, the reach of the last radius, the radius of the inscribed circle, the radius the first ring opens at and the brightest mean, as JSON.
#[wasm_bindgen]
pub fn spin_stats(profile: &[f32], size: usize) -> String {
    let last = profile.len().saturating_sub(1).max(1) as f64;
    let reach = spin::reach(size);
    let disc = profile
        .iter()
        .position(|&v| v > 0.0)
        .map(|k| k as f64 / last * reach)
        .unwrap_or(reach);
    let peak = profile.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    json!({
        "mass": spin::mass(profile, size),
        "reach": reach,
        "inner": size as f64 / 2.0,
        "disc": disc,
        "peak": peak as f64,
    })
    .to_string()
}
