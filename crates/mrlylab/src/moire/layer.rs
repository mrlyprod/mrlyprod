use super::sample::{axes, membership};
use super::{Lattice, Spec};
use mrlycore::errors::{value_error, Result};

/// The recipe for one moire layer.
#[derive(Clone, Debug)]
pub struct Layer {
    /// The design to sample.
    pub spec: Spec,
    /// The side number of the residue grid.
    pub number: usize,
    /// The fractal depth.
    pub level: usize,
    /// The sampling lattice.
    pub lattice: Lattice,
    /// The output side in pixels.
    pub size: usize,
    /// The 0..1 positions fixing the axes beyond the first two.
    pub slices: Vec<f64>,
}

impl Layer {
    /// Builds a layer at level 1 on a 512-pixel square lattice.
    pub fn new(spec: Spec, number: usize) -> Layer {
        Layer {
            spec,
            number,
            level: 1,
            lattice: Lattice::Square,
            size: 512,
            slices: Vec::new(),
        }
    }
}

/// Samples a design over the pixel grid into a boolean mask, or an error below dimension 2.
pub fn layer(params: &Layer) -> Result<Vec<bool>> {
    let Spec {
        code,
        base: q,
        dimension: d,
    } = params.spec;
    if d < 2 {
        return value_error("moire needs dimension >= 2.");
    }
    let size = params.size;
    let n = params.number;
    let table = membership(code, q, d)?;
    let extra = d - 2;
    let mut mask = vec![true; size * size];
    for k in 0..params.level.max(1) {
        let s = (n * q.pow(k as u32)) as f64;
        let mut fixed = Vec::with_capacity(extra);
        for e in 0..extra {
            let pos = params.slices.get(e).copied().unwrap_or(0.0);
            let r = ((s * pos).floor() as i64).rem_euclid(q as i64) as usize;
            fixed.push(r);
        }
        for row in 0..size {
            let (a, b) = axes(size, params.lattice, row);
            let base_idx = row * size;
            for col in 0..size {
                let cell = base_idx + col;
                if !mask[cell] {
                    continue;
                }
                let ia = (s * a[col]).floor() as i64;
                let ib = (s * b[col]).floor() as i64;
                let ra = ia.rem_euclid(q as i64) as usize;
                let rb = ib.rem_euclid(q as i64) as usize;
                let mut idx = ra * q + rb;
                for &f in &fixed {
                    idx = idx * q + f;
                }
                if !table[idx] {
                    mask[cell] = false;
                }
            }
        }
    }
    Ok(mask)
}
