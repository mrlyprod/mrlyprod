use crate::{checked, Fault};
use mrlymath::two;
use wasm_bindgen::prelude::*;

const REACH: usize = 729;
const FLOOR: f64 = 3.0;

// TRANSFORM

fn lower(f: &[f64], d: &mut [f64], hull: &mut [usize], edge: &mut [f64]) {
    let n = f.len();
    let mut k = 0;
    hull[0] = 0;
    edge[0] = f64::NEG_INFINITY;
    edge[1] = f64::INFINITY;
    for q in 1..n {
        let mut cut;
        loop {
            let p = hull[k];
            cut = ((f[q] + (q * q) as f64) - (f[p] + (p * p) as f64)) / (2 * (q - p)) as f64;
            if k > 0 && cut <= edge[k] {
                k -= 1;
            } else {
                break;
            }
        }
        k += 1;
        hull[k] = q;
        edge[k] = cut;
        edge[k + 1] = f64::INFINITY;
    }
    k = 0;
    for (q, slot) in d.iter_mut().enumerate() {
        while edge[k + 1] < q as f64 {
            k += 1;
        }
        let gap = q as f64 - hull[k] as f64;
        *slot = gap * gap + f[hull[k]];
    }
}

fn transform(types: &[u8], side: usize) -> Vec<f64> {
    let far = (4 * side * side) as f64;
    let mut square: Vec<f64> = types
        .iter()
        .map(|&b| if b != 0 { 0.0 } else { far })
        .collect();
    let mut lane = vec![0.0; side];
    let mut out = vec![0.0; side];
    let mut hull = vec![0usize; side];
    let mut edge = vec![0.0; side + 1];
    for col in 0..side {
        for row in 0..side {
            lane[row] = square[row * side + col];
        }
        lower(&lane, &mut out, &mut hull, &mut edge);
        for row in 0..side {
            square[row * side + col] = out[row];
        }
    }
    for row in 0..side {
        lane.copy_from_slice(&square[row * side..(row + 1) * side]);
        lower(&lane, &mut out, &mut hull, &mut edge);
        square[row * side..(row + 1) * side].copy_from_slice(&out);
    }
    square.iter().map(|v| v.sqrt()).collect()
}

// FIELD

struct Field {
    side: usize,
    number: usize,
    dimension: f64,
    dist: Vec<f64>,
}

impl Field {
    fn read(code: &str, number: usize, level: usize, base: usize) -> Result<Field, Fault> {
        if number < 2 {
            return Err(Fault::new("the side must be at least two."));
        }
        if level < 1 {
            return Err(Fault::new("the level must be at least one."));
        }
        let side = number
            .checked_pow(level as u32)
            .filter(|&side| side <= REACH)
            .ok_or_else(|| {
                Fault::new(format!(
                    "side {number} at level {level} passes the {REACH} cells a side the tube allows."
                ))
            })?;
        let code = checked(code, 2, base)?;
        let tile = two::create(code, number, 1, 0, base)?;
        let digits = tile.types().bytes().iter().filter(|&&b| b != 0).count();
        if digits == 0 {
            return Err(Fault::new("the empty design has no tube."));
        }
        let cell = two::create(code, number, level, 0, base)?;
        if cell.width() != side || cell.height() != side {
            return Err(Fault::new("the design is not a square grid."));
        }
        let dist = transform(cell.types().bytes(), side);
        Ok(Field {
            side,
            number,
            dimension: (digits as f64).ln() / (number as f64).ln(),
            dist,
        })
    }

    fn volume(&self, eps: f64) -> f64 {
        self.dist
            .iter()
            .map(|&d| (eps - d + 1.0).clamp(0.0, 1.0))
            .sum()
    }
}

// EXPORTS

/// Reads the distance field of the design's level-L grid: the Euclidean distance in cell widths from every cell centre to the nearest filled cell centre, row-major, zero on the filled cells.
///
/// The transform is exact, the two-pass lower envelope of parabolas, so a cell inside a square
/// hole of side `s` reads the whole number of cells to the hole's wall. The grid is refused past
/// 729 cells a side.
#[wasm_bindgen]
pub fn tube_distance(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
) -> Result<Vec<f32>, Fault> {
    let field = Field::read(code, number, level, base)?;
    Ok(field.dist.iter().map(|&v| v as f32).collect())
}

/// Measures the inner tube at radius `eps_cells`: the area within that distance of the design, in cell widths squared, the filled cells counted whole.
///
/// A cell carries the share of itself the radius reaches, `eps - dist + 1` clamped to the unit
/// interval, which is exact at whole radii on a design whose holes are squares: the carpet at
/// level 6 and radius 21 cells reads `5912/6561` of the unit square.
#[wasm_bindgen]
pub fn tube_volume(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    eps_cells: f64,
) -> Result<f64, Fault> {
    Ok(Field::read(code, number, level, base)?.volume(eps_cells))
}

/// Walks the Minkowski profile across the radii the grid resolves: the pairs `u = ln(1/eps)` and `M = eps^(d-2) V(eps)` in unit-square units, flat, two numbers a sample.
///
/// The radius runs from `1/q` down to three cells, evenly in `u`, and the pairs come back empty
/// when the grid is too coarse to hold that range. `d` is `log_q k`, the design's fractal
/// dimension, so `M` is the Minkowski content reading whose limit the closed profile is.
#[wasm_bindgen]
pub fn tube_profile(
    code: &str,
    number: usize,
    level: usize,
    base: usize,
    samples: usize,
) -> Result<Vec<f32>, Fault> {
    let field = Field::read(code, number, level, base)?;
    let side = field.side as f64;
    let top = side / field.number as f64;
    if top <= FLOOR || samples < 2 {
        return Ok(Vec::new());
    }
    let (low, high) = ((side / top).ln(), (side / FLOOR).ln());
    let mut pairs = Vec::with_capacity(2 * samples);
    for step in 0..samples {
        let u = low + (high - low) * step as f64 / (samples - 1) as f64;
        let cells = side * (-u).exp();
        let eps = cells / side;
        let volume = field.volume(cells) / (side * side);
        pairs.push(u as f32);
        pairs.push((eps.powf(field.dimension - 2.0) * volume) as f32);
    }
    Ok(pairs)
}

/// Reads the closed limit profile `G(t)` of the interior-hole class at base `q` with `k` filled cells, the hole sum in closed form.
///
/// Every hole of such a design is an isolated open square whose boundary lies in the design, so
/// the inner tube is the exact sum over the levels, `V(eps) = sum k^(m-1) h(q^-m, eps)`, and
/// `M(eps) = eps^(d-2) V(eps)` runs onto `G`, log-periodic with period `ln q`. The carpet is
/// `q = 3`, `k = 8`, where `G(1/3) = G(1) = 379/280` and the swing above zero is the whole proof
/// that the design is not Minkowski measurable.
#[wasm_bindgen]
pub fn tube_closed(q: usize, k: usize, t: f64) -> Result<f64, Fault> {
    if q < 2 || k <= q || k >= q * q {
        return Err(Fault::new(format!(
            "the class needs {q} < k < {}, not {k}.",
            q * q
        )));
    }
    if !(t > 0.0 && t <= 1.0) {
        return Err(Fault::new("the phase must lie in zero to one."));
    }
    let (base, mass) = (q as f64, k as f64);
    let mut step = 0i32;
    while t * base.powi(step) < 0.5 && step < 512 {
        step += 1;
    }
    while step > -512 && t * base.powi(step - 1) >= 0.5 {
        step -= 1;
    }
    let tail = (mass / (base * base)).powi(step - 1);
    let phase = t * base.powi(step) / base;
    let dimension = mass.ln() / base.ln();
    let sum = 1.0 / (base * base - mass) + 4.0 * phase / (mass - base)
        - 4.0 * phase * phase / (mass - 1.0);
    Ok(t.powf(dimension - 2.0) * tail * sum)
}

/// Decides whether the design's holes are the isolated interior squares the closed profile needs: every empty cell of the level-one tile off the border, and no two of them touching, even at a corner.
#[wasm_bindgen]
pub fn tube_class(code: &str, number: usize, base: usize) -> Result<bool, Fault> {
    if number < 3 {
        return Ok(false);
    }
    let tile = two::create(checked(code, 2, base)?, number, 1, 0, base)?;
    let types = tile.types().bytes().to_vec();
    if types.len() != number * number {
        return Ok(false);
    }
    let holes: Vec<(usize, usize)> = (0..types.len())
        .filter(|&at| types[at] == 0)
        .map(|at| (at / number, at % number))
        .collect();
    if holes.is_empty() {
        return Ok(false);
    }
    if holes
        .iter()
        .any(|&(row, col)| row == 0 || col == 0 || row + 1 == number || col + 1 == number)
    {
        return Ok(false);
    }
    for (at, &(row, col)) in holes.iter().enumerate() {
        for &(other, next) in &holes[at + 1..] {
            if row.abs_diff(other) <= 1 && col.abs_diff(next) <= 1 {
                return Ok(false);
            }
        }
    }
    Ok(true)
}
