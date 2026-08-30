use crate::ink::{BLUE, DEEP, FAINT, GOLD, GREEN, ORANGE, PINK};
use crate::spin::ramp_of;
use crate::{Fault, Pixels};
use mrlycore::json;
use mrlynum::factor::factorize_wide;
use mrlynum::gauss::{peak, shells, Class, Ring, Window};
use wasm_bindgen::prelude::*;

const RADIUS: u32 = 200;
const SIZE: usize = 768;
const LIMIT: usize = 10_000;
const ROOT3: f64 = 1.732_050_807_568_877_2;

struct Sheet {
    window: Window,
    half: f64,
    scale: f64,
}

impl Sheet {
    fn new(ring: &str, radius: u32, size: usize) -> Result<Sheet, Fault> {
        let ring =
            Ring::named(ring).ok_or_else(|| Fault::new("the ring is gaussian or eisenstein."))?;
        if radius == 0 || radius > RADIUS {
            return Err(Fault::new(format!("the radius is 1 to {RADIUS}.")));
        }
        if size == 0 || size > SIZE {
            return Err(Fault::new(format!("the sheet is 1 to {SIZE} pixels wide.")));
        }
        Ok(Sheet {
            window: Window::new(ring, u64::from(radius)),
            half: size as f64 / 2.0,
            scale: size as f64 / (2 * radius + 1) as f64,
        })
    }
    fn ring(&self) -> Ring {
        self.window.ring()
    }
    fn center(&self, a: i64, b: i64) -> (f64, f64) {
        let (x, y) = self.ring().place(a, b);
        (self.half + x * self.scale, self.half - y * self.scale)
    }
    fn cell(&self, px: f64, py: f64) -> Option<(i64, i64)> {
        let (a, b) = self
            .ring()
            .nearest((px - self.half) / self.scale, (self.half - py) / self.scale);
        self.window.holds(a, b).then_some((a, b))
    }
    fn gap(&self, px: f64, py: f64, cell: (i64, i64)) -> bool {
        if self.scale < 6.0 {
            return false;
        }
        let (cx, cy) = self.center(cell.0, cell.1);
        let (x, y) = (px - cx, py - cy);
        match self.ring() {
            Ring::Gaussian => x + self.scale / 2.0 < 1.0 || y + self.scale / 2.0 < 1.0,
            Ring::Eisenstein => {
                let reach = x
                    .abs()
                    .max((x / 2.0 + y * ROOT3 / 2.0).abs())
                    .max((y * ROOT3 / 2.0 - x / 2.0).abs());
                reach > self.scale / 2.0 - 1.0
            }
        }
    }
    fn spot(&self, (a, b): (i64, i64)) -> [f64; 4] {
        let (px, py) = self.center(a, b);
        [a as f64, b as f64, px, py]
    }
}

/// Paints the window of a ring over a square sheet: primes by class, split blue, inert orange, ramified pink and the units green, or by norm through the fire ramp, or plain gold; the composites faint or dark.
#[wasm_bindgen]
pub fn ring_pixels(
    ring: &str,
    radius: u32,
    colour: &str,
    faint: bool,
    size: usize,
) -> Result<Pixels, Fault> {
    let sheet = Sheet::new(ring, radius, size)?;
    if !["class", "norm", "plain"].contains(&colour) {
        return Err(Fault::new("the colour is class, norm or plain."));
    }
    let fire = ramp_of("fire");
    let top = sheet.ring().top(u64::from(radius)) as usize;
    let side = (2 * radius + 1) as usize;
    let r = radius as i64;
    let mut look = vec![DEEP; side * side];
    for b in -r..=r {
        for a in -r..=r {
            if !sheet.window.holds(a, b) {
                continue;
            }
            let class = sheet.window.class(a, b);
            look[(b + r) as usize * side + (a + r) as usize] = match (colour, class) {
                (_, Class::Zero) => DEEP,
                (_, Class::Unit) => GREEN,
                (_, Class::Composite) if faint => FAINT,
                (_, Class::Composite) => DEEP,
                ("class", Class::Split) => BLUE,
                ("class", Class::Inert) => ORANGE,
                ("class", Class::Ramified) => PINK,
                ("norm", _) => {
                    let c = fire.color(sheet.ring().norm(a, b) as usize, top);
                    [c.r, c.g, c.b, 255]
                }
                _ => GOLD,
            };
        }
    }
    let mut colors = Vec::with_capacity(size * size);
    for py in 0..size {
        for px in 0..size {
            let (fx, fy) = (px as f64 + 0.5, py as f64 + 0.5);
            colors.push(match sheet.cell(fx, fy) {
                Some(cell) if !sheet.gap(fx, fy, cell) => {
                    look[(cell.1 + r) as usize * side + (cell.0 + r) as usize]
                }
                _ => DEEP,
            });
        }
    }
    Ok(Pixels::of(size, size, colors))
}

/// Counts the window of a ring: its points, primes, split, inert, ramified, units and composites, the prime density, the largest norm and the symmetry order, as JSON.
#[wasm_bindgen]
pub fn ring_census(ring: &str, radius: u32) -> Result<String, Fault> {
    let sheet = Sheet::new(ring, radius, 1)?;
    let census = sheet.window.census();
    Ok(json!({
        "points": census.points,
        "primes": census.primes,
        "split": census.split,
        "inert": census.inert,
        "ramified": census.ramified,
        "units": census.units,
        "composites": census.composites,
        "density": census.density,
        "top": sheet.ring().top(u64::from(radius)),
        "symmetry": sheet.ring().symmetry(),
    })
    .to_string())
}

/// Reads the point under a pixel of the sheet: its coordinates, norm and its factors, class and primality, its pixel centre and width, and the places of its unit multiples and its conjugate, as JSON.
#[wasm_bindgen]
pub fn ring_at(ring: &str, radius: u32, x: f64, y: f64, size: usize) -> Result<String, Fault> {
    let sheet = Sheet::new(ring, radius, size)?;
    let (a, b) = sheet
        .cell(x, y)
        .ok_or_else(|| Fault::new("the click missed the window."))?;
    let class = sheet.window.class(a, b);
    let norm = sheet.ring().norm(a, b);
    let (px, py) = sheet.center(a, b);
    let associates: Vec<[f64; 4]> = sheet
        .ring()
        .associates(a, b)
        .into_iter()
        .map(|point| sheet.spot(point))
        .collect();
    Ok(json!({
        "a": a,
        "b": b,
        "norm": norm,
        "factors": factorize_wide(norm),
        "class": class.word(),
        "prime": class.prime(),
        "px": px,
        "py": py,
        "span": sheet.scale,
        "associates": associates,
        "conjugate": sheet.spot(sheet.ring().conjugate(a, b)),
    })
    .to_string())
}

/// Counts the points of every norm from zero through the limit: the ring weights of the lattice.
#[wasm_bindgen]
pub fn ring_weights(ring: &str, limit: usize) -> Result<Vec<u32>, Fault> {
    let ring =
        Ring::named(ring).ok_or_else(|| Fault::new("the ring is gaussian or eisenstein."))?;
    if limit > LIMIT {
        return Err(Fault::new(format!("the weights stop at norm {LIMIT}.")));
    }
    Ok(shells(ring, limit))
}

/// Returns the norm from one through the limit with the most points and that count.
#[wasm_bindgen]
pub fn ring_peak(ring: &str, limit: usize) -> Result<Vec<u32>, Fault> {
    let ring =
        Ring::named(ring).ok_or_else(|| Fault::new("the ring is gaussian or eisenstein."))?;
    if limit > LIMIT {
        return Err(Fault::new(format!("the weights stop at norm {LIMIT}.")));
    }
    let (norm, count) = peak(ring, limit);
    Ok(vec![norm as u32, count])
}

/// Reads the fate of every whole number from zero through the limit as a prime of the ring: 0 when not prime, 1 split, 2 inert, 3 ramified.
#[wasm_bindgen]
pub fn ring_fates(ring: &str, limit: usize) -> Result<Vec<u8>, Fault> {
    let ring =
        Ring::named(ring).ok_or_else(|| Fault::new("the ring is gaussian or eisenstein."))?;
    if limit > LIMIT {
        return Err(Fault::new(format!("the fates stop at {LIMIT}.")));
    }
    Ok((0..=limit as u64)
        .map(|n| match ring.fate(n) {
            Class::Split => 1,
            Class::Inert => 2,
            Class::Ramified => 3,
            _ => 0,
        })
        .collect())
}
