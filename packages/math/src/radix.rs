use mrly_core::error::{Result, overflow_error, shape_error, value_error};
use mrly_num::gauss::Ring;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::f64::consts::TAU;

fn divides(ring: Ring, z: (i64, i64), w: (i64, i64)) -> bool {
    let n = ring.norm(w.0, w.1) as i64;
    let (x, y) = ring.mul(z, ring.conjugate(w.0, w.1));
    x % n == 0 && y % n == 0
}

fn argument(ring: Ring, a: i64, b: i64) -> f64 {
    let (x, y) = ring.place(a, b);
    let t = y.atan2(x);
    if t < 0.0 { t + TAU } else { t }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Base {
    ring: Ring,
    value: (i64, i64),
}

impl Base {
    pub fn new(ring: Ring, value: (i64, i64)) -> Result<Base> {
        let norm = ring.norm(value.0, value.1);
        if norm < 2 {
            return value_error(format!("a base needs norm two or more, not {norm}."));
        }
        Ok(Base { ring, value })
    }
    pub fn ring(self) -> Ring {
        self.ring
    }
    pub fn value(self) -> (i64, i64) {
        self.value
    }
    pub fn norm(self) -> u64 {
        self.ring.norm(self.value.0, self.value.1)
    }
    pub fn power(self, level: usize) -> (i64, i64) {
        let mut out = (1, 0);
        for _ in 0..level {
            out = self.ring.mul(out, self.value);
        }
        out
    }
    pub fn congruent(self, z: (i64, i64), w: (i64, i64)) -> bool {
        divides(self.ring, (z.0 - w.0, z.1 - w.1), self.value)
    }
    pub fn residues(self) -> Result<Vec<(i64, i64)>> {
        let q = self.norm();
        let reach = (2 * q).isqrt() as i64 + 1;
        let mut pool = Vec::new();
        for a in -reach..=reach {
            for b in -reach..=reach {
                if self.ring.norm(a, b) <= q {
                    pool.push((a, b));
                }
            }
        }
        pool.sort_by(|&(a, b), &(c, d)| {
            let left = (self.ring.norm(a, b), argument(self.ring, a, b));
            let right = (self.ring.norm(c, d), argument(self.ring, c, d));
            left.0
                .cmp(&right.0)
                .then_with(|| left.1.total_cmp(&right.1))
        });
        let mut out: Vec<(i64, i64)> = Vec::with_capacity(q as usize);
        for z in pool {
            if out.len() == q as usize {
                break;
            }
            if !out.iter().any(|&w| self.congruent(z, w)) {
                out.push(z);
            }
        }
        if out.len() != q as usize {
            return value_error(format!(
                "the residue system of {:?} holds {} of {q} classes.",
                self.value,
                out.len()
            ));
        }
        Ok(out)
    }
    pub fn class(self, z: (i64, i64)) -> Result<usize> {
        match self.residues()?.iter().position(|&w| self.congruent(z, w)) {
            Some(index) => Ok(index),
            None => value_error(format!("the point {z:?} lies in no residue class.")),
        }
    }
    pub fn mirrored(self) -> bool {
        let c = self.ring.conjugate(self.value.0, self.value.1);
        self.ring
            .associates(self.value.0, self.value.1)
            .contains(&c)
    }
    pub fn group(self) -> Result<Vec<Vec<usize>>> {
        let residues = self.residues()?;
        let mirror = self.mirrored();
        let mut out = Vec::new();
        for unit in self.ring.associates(1, 0) {
            for flip in [false, true] {
                if flip && !mirror {
                    continue;
                }
                let mut map = Vec::with_capacity(residues.len());
                for &z in &residues {
                    let w = if flip {
                        self.ring.conjugate(z.0, z.1)
                    } else {
                        z
                    };
                    let image = self.ring.mul(unit, w);
                    match residues.iter().position(|&r| self.congruent(image, r)) {
                        Some(index) => map.push(index),
                        None => {
                            return value_error(format!(
                                "the unit multiple {image:?} lies in no residue class."
                            ));
                        }
                    }
                }
                out.push(map);
            }
        }
        Ok(out)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Radix {
    base: Base,
    digits: Vec<(i64, i64)>,
    twists: Vec<(i64, i64)>,
}

impl Radix {
    pub fn new(base: Base, digits: Vec<(i64, i64)>, twists: Vec<(i64, i64)>) -> Result<Radix> {
        if digits.len() != twists.len() {
            return shape_error(format!(
                "there are {} digits and {} twists.",
                digits.len(),
                twists.len()
            ));
        }
        for &(a, b) in &twists {
            let norm = base.ring().norm(a, b);
            if norm != 1 {
                return value_error(format!("the twist {:?} has norm {norm}, not one.", (a, b)));
            }
        }
        for (i, &z) in digits.iter().enumerate() {
            for &w in &digits[..i] {
                if base.congruent(z, w) {
                    return value_error(format!(
                        "the digits {w:?} and {z:?} are congruent modulo the base."
                    ));
                }
            }
        }
        Ok(Radix {
            base,
            digits,
            twists,
        })
    }
    pub fn from_code(base: Base, code: u128) -> Result<Radix> {
        let residues = base.residues()?;
        if residues.len() >= 128 {
            return overflow_error(format!(
                "the base holds {} classes and a code carries 128.",
                residues.len()
            ));
        }
        if code >> residues.len() != 0 {
            return value_error(format!(
                "the code {code} runs past the {} classes of the base.",
                residues.len()
            ));
        }
        let digits: Vec<(i64, i64)> = residues
            .into_iter()
            .enumerate()
            .filter(|(i, _)| (code >> i) & 1 == 1)
            .map(|(_, z)| z)
            .collect();
        let twists = vec![(1, 0); digits.len()];
        Radix::new(base, digits, twists)
    }
    pub fn with_twists(self, units: &[usize]) -> Result<Radix> {
        let list = self.base.ring().associates(1, 0);
        let mut twists = Vec::with_capacity(units.len());
        for &i in units {
            match list.get(i) {
                Some(&unit) => twists.push(unit),
                None => {
                    return value_error(format!(
                        "the unit index {i} runs past the {} units of the ring.",
                        list.len()
                    ));
                }
            }
        }
        Radix::new(self.base, self.digits, twists)
    }
    pub fn base(&self) -> Base {
        self.base
    }
    pub fn ring(&self) -> Ring {
        self.base.ring()
    }
    pub fn digits(&self) -> &[(i64, i64)] {
        &self.digits
    }
    pub fn twists(&self) -> &[(i64, i64)] {
        &self.twists
    }
    pub fn size(&self) -> usize {
        self.digits.len()
    }
    pub fn code(&self) -> Result<u128> {
        let mut out = 0u128;
        for &d in &self.digits {
            out |= 1 << self.base.class(d)?;
        }
        Ok(out)
    }
    pub fn canonical(&self) -> Result<bool> {
        let residues = self.base.residues()?;
        Ok(self.digits.iter().all(|d| residues.contains(d)))
    }
    pub fn fill(&self, level: usize) -> u128 {
        (self.size() as u128).pow(level as u32)
    }
    pub fn dimension(&self) -> f64 {
        (self.size() as f64).ln() / (self.base.norm() as f64).ln() * 2.0
    }
    pub fn words(&self, level: usize) -> Vec<(i64, i64)> {
        let ring = self.ring();
        let mut out = vec![(0i64, 0i64)];
        for step in 1..=level {
            let shift = self.base.power(step - 1);
            let mut next = Vec::with_capacity(out.len() * self.size());
            for (&d, &u) in self.digits.iter().zip(self.twists.iter()) {
                let head = ring.mul(d, shift);
                for &p in &out {
                    let t = ring.mul(u, p);
                    next.push((head.0 + t.0, head.1 + t.1));
                }
            }
            out = next;
        }
        out
    }
    pub fn plane(&self, level: usize) -> Vec<(f64, f64)> {
        let ring = self.ring();
        let p = self.base.power(level);
        let (px, py) = ring.place(p.0, p.1);
        let den = px * px + py * py;
        self.words(level)
            .into_iter()
            .map(|(a, b)| {
                let (x, y) = ring.place(a, b);
                ((x * px + y * py) / den, (y * px - x * py) / den)
            })
            .collect()
    }
    pub fn distinct(&self, level: usize) -> usize {
        self.words(level).into_iter().collect::<HashSet<_>>().len()
    }
}

pub fn koch() -> Result<Radix> {
    let base = Base::new(Ring::Eisenstein, (3, 0))?;
    let digits = vec![(0, 0), (1, 0), (2, 1), (2, 0)];
    Radix::new(base, digits, vec![(1, 0); 4])?.with_twists(&[0, 1, 5, 0])
}

pub fn gasket() -> Result<Radix> {
    Radix::from_code(Base::new(Ring::Eisenstein, (2, 0))?, 7)
}

pub fn twindragon() -> Result<Radix> {
    Radix::from_code(Base::new(Ring::Gaussian, (1, 1))?, 3)
}

pub fn terdragon() -> Result<Radix> {
    Radix::from_code(Base::new(Ring::Eisenstein, (2, 1))?, 7)?.with_twists(&[0, 2, 0])
}

pub fn flowsnake() -> Result<Radix> {
    Radix::from_code(Base::new(Ring::Eisenstein, (3, 1))?, 127)
}

pub fn tile(m: u64, code: u128) -> Result<Radix> {
    let base = Base::new(Ring::Gaussian, (m as i64, 0))?;
    let cells = (m * m) as usize;
    if cells >= 128 {
        return overflow_error(format!(
            "a {m} by {m} tile holds {cells} cells and a code carries 128."
        ));
    }
    if code >> cells != 0 {
        return value_error(format!("the code {code} runs past the {cells} cells."));
    }
    let digits: Vec<(i64, i64)> = (0..cells)
        .filter(|i| (code >> i) & 1 == 1)
        .map(|i| ((i as u64 % m) as i64, (i as u64 / m) as i64))
        .collect();
    let twists = vec![(1, 0); digits.len()];
    Radix::new(base, digits, twists)
}
