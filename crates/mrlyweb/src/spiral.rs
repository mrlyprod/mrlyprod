use crate::ink::{BLUE, DEEP, FAINT, GOLD, ORANGE, PINK};
use crate::{Fault, Pixels};
use mrlycore::json;
use mrlynum::factor::factorize;
use mrlynum::spiral::{self, Diagonal, Lattice, Mark};
use wasm_bindgen::prelude::*;

const SIDE: usize = 401;
const SIZE: usize = 1024;
const REACH: i32 = 1_000_000;
const LABELS: usize = 4096;
const ROOT3: f64 = 1.732_050_807_568_877_2;

struct Sheet {
    lattice: Lattice,
    radius: i64,
    half: f64,
    scale: f64,
}

impl Sheet {
    fn new(lattice: &str, side: usize, size: usize) -> Result<Sheet, Fault> {
        let lattice =
            Lattice::named(lattice).ok_or_else(|| Fault::new("the lattice is square or hex."))?;
        if side.is_multiple_of(2) || side > SIDE {
            return Err(Fault::new(format!("the side is odd and at most {SIDE}.")));
        }
        if size == 0 || size > SIZE {
            return Err(Fault::new(format!("the sheet is 1 to {SIZE} pixels wide.")));
        }
        let radius = lattice.radius(side) as i64;
        let half = size as f64 / 2.0;
        let scale = match lattice {
            Lattice::Square => size as f64 / side as f64,
            Lattice::Hex => {
                let r = radius as f64;
                (size as f64 / (2.0 * ROOT3 * (r + 0.5))).min(size as f64 / (3.0 * r + 2.0))
            }
        };
        Ok(Sheet {
            lattice,
            radius,
            half,
            scale,
        })
    }
    fn center(&self, x: i64, y: i64) -> (f64, f64) {
        let (x, y) = (x as f64, y as f64);
        match self.lattice {
            Lattice::Square => (self.half + x * self.scale, self.half - y * self.scale),
            Lattice::Hex => (
                self.half + self.scale * ROOT3 * (x + y / 2.0),
                self.half + 1.5 * self.scale * y,
            ),
        }
    }
    fn span(&self) -> f64 {
        match self.lattice {
            Lattice::Square => self.scale,
            Lattice::Hex => self.scale * ROOT3,
        }
    }
    fn cell(&self, px: f64, py: f64) -> Option<(i64, i64)> {
        let (x, y) = (px - self.half, py - self.half);
        let cell = match self.lattice {
            Lattice::Square => (
                (x / self.scale + 0.5).floor() as i64,
                (-y / self.scale + 0.5).floor() as i64,
            ),
            Lattice::Hex => {
                let q = (ROOT3 / 3.0 * x - y / 3.0) / self.scale;
                let r = 2.0 / 3.0 * y / self.scale;
                let s = -q - r;
                let (mut rq, mut rr, rs) = (q.round(), r.round(), s.round());
                let (dq, dr, ds) = ((rq - q).abs(), (rr - r).abs(), (rs - s).abs());
                if dq > dr && dq > ds {
                    rq = -rr - rs;
                } else if dr > ds {
                    rr = -rq - rs;
                }
                (rq as i64, rr as i64)
            }
        };
        (self.lattice.ring_of(cell.0, cell.1) <= self.radius as u64).then_some(cell)
    }
    fn gap(&self, px: f64, py: f64, cell: (i64, i64)) -> bool {
        if self.scale < 6.0 {
            return false;
        }
        let (cx, cy) = self.center(cell.0, cell.1);
        let (x, y) = (px - cx, py - cy);
        match self.lattice {
            Lattice::Square => x + self.scale / 2.0 < 1.0 || y + self.scale / 2.0 < 1.0,
            Lattice::Hex => {
                let reach = x
                    .abs()
                    .max((x / 2.0 + y * ROOT3 / 2.0).abs())
                    .max((y * ROOT3 / 2.0 - x / 2.0).abs());
                reach > self.scale * ROOT3 / 2.0 - 1.0
            }
        }
    }
}

fn read(lattice: Lattice, side: usize, a: i32, b: i32, c: i32) -> Result<Diagonal, Fault> {
    if a < 1 {
        return Err(Fault::new("a quadratic needs a of at least 1."));
    }
    if b.abs() > REACH || c.abs() > REACH {
        return Err(Fault::new(format!("b and c stay within {REACH}.")));
    }
    Ok(spiral::diagonal(
        lattice,
        side,
        i64::from(a),
        i64::from(b),
        i64::from(c),
    ))
}

/// Paints the numbers from one wound on the lattice over a sheet the odd side wide: marked cells gold, a Mobius minus one pink, the quadratic a k^2 + b k + c orange on a prime and blue otherwise, the rest faint or dark.
#[wasm_bindgen]
#[allow(clippy::too_many_arguments)]
pub fn spiral_pixels(
    lattice: &str,
    side: usize,
    a: i32,
    b: i32,
    c: i32,
    mark: &str,
    faint: bool,
    size: usize,
) -> Result<Pixels, Fault> {
    let sheet = Sheet::new(lattice, side, size)?;
    let mark = Mark::named(mark)
        .ok_or_else(|| Fault::new("the mark is prime, twin, squarefree or mobius."))?;
    let quadratic = read(sheet.lattice, side, a, b, c)?;
    let mut look: Vec<[u8; 4]> = spiral::marks(mark, quadratic.top)
        .iter()
        .map(|&m| match m {
            1 => GOLD,
            -1 => PINK,
            _ if faint => FAINT,
            _ => DEEP,
        })
        .collect();
    for (&value, &hit) in quadratic.values.iter().zip(&quadratic.hit) {
        look[value as usize] = if hit { ORANGE } else { BLUE };
    }
    let mut colors = Vec::with_capacity(size * size);
    for py in 0..size {
        for px in 0..size {
            let (fx, fy) = (px as f64 + 0.5, py as f64 + 0.5);
            colors.push(match sheet.cell(fx, fy) {
                Some(cell) if !sheet.gap(fx, fy, cell) => {
                    look[sheet.lattice.n(cell.0, cell.1) as usize]
                }
                _ => DEEP,
            });
        }
    }
    Ok(Pixels::of(size, size, colors))
}

/// Returns the cell of a number and its ring: x right and y up on the square, axial q and r on the hexagon.
#[wasm_bindgen]
pub fn spiral_xy(lattice: &str, n: u32) -> Result<Vec<i32>, Fault> {
    let lattice =
        Lattice::named(lattice).ok_or_else(|| Fault::new("the lattice is square or hex."))?;
    let (x, y) = lattice.xy(u64::from(n));
    Ok(vec![x as i32, y as i32, lattice.ring(u64::from(n)) as i32])
}

/// Reads the cell under a pixel of the sheet: its number, cell, pixel centre and width, ring, primality and factors, as JSON.
#[wasm_bindgen]
pub fn spiral_at(lattice: &str, side: usize, x: f64, y: f64, size: usize) -> Result<String, Fault> {
    let sheet = Sheet::new(lattice, side, size)?;
    let (cx, cy) = sheet
        .cell(x, y)
        .ok_or_else(|| Fault::new("the click missed the spiral."))?;
    let n = sheet.lattice.n(cx, cy);
    let factors = factorize(n as usize);
    let (px, py) = sheet.center(cx, cy);
    Ok(json!({
        "n": n,
        "x": cx,
        "y": cy,
        "px": px,
        "py": py,
        "span": sheet.span(),
        "ring": sheet.lattice.ring(n),
        "prime": factors.len() == 1 && factors[0].1 == 1,
        "factors": factors,
    })
    .to_string())
}

/// Reads the quadratic a k^2 + b k + c over the sheet: the count of numbers, of primes and their density, the values inside with their cells and prime hits, the hit count, its share and the opening streak, as JSON.
#[wasm_bindgen]
pub fn spiral_polynomial(
    lattice: &str,
    side: usize,
    a: i32,
    b: i32,
    c: i32,
) -> Result<String, Fault> {
    let sheet = Sheet::new(lattice, side, 1)?;
    let quadratic = read(sheet.lattice, side, a, b, c)?;
    Ok(json!({
        "top": quadratic.top,
        "primes": quadratic.primes,
        "density": quadratic.density,
        "count": quadratic.values.len(),
        "hits": quadratic.hits,
        "share": quadratic.share,
        "streak": quadratic.streak,
        "values": quadratic.values,
        "hit": quadratic.hit,
        "cells": quadratic.cells,
    })
    .to_string())
}

/// Returns the pixel centre of every number from one across the sheet, x then y, for sheets of at most a few thousand cells.
#[wasm_bindgen]
pub fn spiral_centers(lattice: &str, side: usize, size: usize) -> Result<Vec<f32>, Fault> {
    let sheet = Sheet::new(lattice, side, size)?;
    let top = sheet.lattice.count(side);
    if top > LABELS {
        return Err(Fault::new(format!("the labels stop at {LABELS} cells.")));
    }
    let mut out = Vec::with_capacity(2 * top);
    for n in 1..=top as u64 {
        let (x, y) = sheet.lattice.xy(n);
        let (px, py) = sheet.center(x, y);
        out.push(px as f32);
        out.push(py as f32);
    }
    Ok(out)
}
