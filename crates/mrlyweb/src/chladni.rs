#![allow(clippy::too_many_arguments)]

use crate::life::life_noise;
use crate::{code_of, Fault, Grid};
use mrlycore::json;
use mrlymath::life;
use mrlynum::fft::{
    convolve_with, embed_kernel, log_spectrum, peak_ring, radial_profile, transform,
};
use wasm_bindgen::prelude::*;

fn checked_size(size: usize) -> Result<(), Fault> {
    if size == 0 || !size.is_power_of_two() {
        return Err(Fault::new("the field size must be a power of two."));
    }
    Ok(())
}

fn field_of(types: &[u8], size: usize) -> Result<Vec<f64>, Fault> {
    checked_size(size)?;
    if types.len() != size * size {
        return Err(Fault::new("the grid bytes do not match size squared."));
    }
    Ok(types
        .iter()
        .map(|&t| if t != 0 { 1.0 } else { 0.0 })
        .collect())
}

struct Mask {
    span: usize,
    cells: Vec<u8>,
    budget: usize,
}

fn mask_of(code: &str, side: usize, level: usize, size: usize) -> Result<Mask, Fault> {
    checked_size(size)?;
    let mask = life::design_mask(2, code_of(code)?, side, level)?;
    let span = mask.shape[0];
    if span > size {
        return Err(Fault::new("the mask does not fit the field."));
    }
    let budget = mask.sum() as usize;
    if budget == 0 {
        return Err(Fault::new("the mask is empty."));
    }
    Ok(Mask {
        span,
        cells: mask.bytes().to_vec(),
        budget,
    })
}

fn window(lo: f64, hi: f64, budget: usize) -> Vec<bool> {
    (0..=budget)
        .map(|k| {
            let share = k as f64 / budget as f64;
            lo <= share && share <= hi
        })
        .collect()
}

struct Engine {
    size: usize,
    budget: usize,
    kernel_re: Vec<f64>,
    kernel_im: Vec<f64>,
    born: Vec<bool>,
    kept: Vec<bool>,
    field: Vec<f64>,
}

impl Engine {
    fn new(
        code: &str,
        side: usize,
        level: usize,
        b_lo: f64,
        b_hi: f64,
        s_lo: f64,
        s_hi: f64,
        size: usize,
    ) -> Result<Engine, Fault> {
        let mask = mask_of(code, side, level, size)?;
        let (kernel_re, kernel_im) = transform(&embed_kernel(&mask.cells, mask.span, size), size);
        Ok(Engine {
            size,
            budget: mask.budget,
            kernel_re,
            kernel_im,
            born: window(b_lo, b_hi, mask.budget),
            kept: window(s_lo, s_hi, mask.budget),
            field: vec![0.0; size * size],
        })
    }
    fn step(&mut self, types: &mut [u8]) {
        for (slot, &t) in self.field.iter_mut().zip(types.iter()) {
            *slot = if t != 0 { 1.0 } else { 0.0 };
        }
        let counts = convolve_with(&self.field, &self.kernel_re, &self.kernel_im, self.size);
        for (slot, &count) in types.iter_mut().zip(&counts) {
            let n = (count.round().max(0.0) as usize).min(self.budget);
            let lives = if *slot != 0 {
                self.kept[n]
            } else {
                self.born[n]
            };
            *slot = u8::from(lives);
        }
    }
}

/// Runs a seeded soup on a design mask for the given steps on a wrapped power-of-two torus: a dead cell is born and a live one kept when its neighbour count over the mask budget lies in the closed birth or survive window, the count read by FFT convolution.
#[wasm_bindgen]
pub fn chladni_run(
    code: &str,
    side: usize,
    level: usize,
    b_lo: f64,
    b_hi: f64,
    s_lo: f64,
    s_hi: f64,
    size: usize,
    steps: usize,
    density: f64,
    seed: u32,
) -> Result<Grid, Fault> {
    let mut engine = Engine::new(code, side, level, b_lo, b_hi, s_lo, s_hi, size)?;
    let mut types = life_noise(size, size, density, seed);
    for _ in 0..steps {
        engine.step(&mut types);
    }
    Ok(Grid {
        width: size as u32,
        height: size as u32,
        types,
    })
}

/// Advances a size-square grid one generation on a design mask under the closed birth and survive windows, wrapping the edges.
#[wasm_bindgen]
pub fn chladni_next(
    types: &[u8],
    size: usize,
    code: &str,
    side: usize,
    level: usize,
    b_lo: f64,
    b_hi: f64,
    s_lo: f64,
    s_hi: f64,
) -> Result<Vec<u8>, Fault> {
    field_of(types, size)?;
    let mut engine = Engine::new(code, side, level, b_lo, b_hi, s_lo, s_hi, size)?;
    let mut next = types.to_vec();
    engine.step(&mut next);
    Ok(next)
}

/// Draws a design mask centred in a size-square picture, the mask centre at size over two.
#[wasm_bindgen]
pub fn chladni_kernel(code: &str, side: usize, level: usize, size: usize) -> Result<Grid, Fault> {
    let mask = mask_of(code, side, level, size)?;
    let start = size / 2 - mask.span / 2;
    let mut types = vec![0u8; size * size];
    for r in 0..mask.span {
        for c in 0..mask.span {
            types[(start + r) * size + start + c] = mask.cells[r * mask.span + c];
        }
    }
    Ok(Grid {
        width: size as u32,
        height: size as u32,
        types,
    })
}

/// Reads the centred log magnitude spectrum of a size-square 0/1 field, scaled to at most one by its peak.
#[wasm_bindgen]
pub fn chladni_spectrum(types: &[u8], size: usize) -> Result<Vec<f32>, Fault> {
    let spectrum = log_spectrum(&field_of(types, size)?, size);
    let top = spectrum.iter().cloned().fold(0.0f64, f64::max);
    let scale = if top > 0.0 { 1.0 / top } else { 0.0 };
    Ok(spectrum.iter().map(|&v| (v * scale) as f32).collect())
}

/// Reads the ring means of a size-square 0/1 field's log spectrum, the peak ring past the centre and its wavelength in cells, as JSON.
#[wasm_bindgen]
pub fn chladni_profile(types: &[u8], size: usize) -> Result<String, Fault> {
    let profile = radial_profile(&log_spectrum(&field_of(types, size)?, size), size);
    let ring = peak_ring(&profile);
    let wavelength = if ring == 0 {
        0.0
    } else {
        size as f64 / ring as f64
    };
    Ok(json!({
        "profile": profile,
        "peak_ring": ring,
        "wavelength": wavelength,
    })
    .to_string())
}
