use crate::prime::flags;
use crate::prime::is_prime;
use serde::{Deserialize, Serialize};

const ROOT3: f64 = 1.732_050_807_568_877_2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Ring {
    Gaussian,
    Eisenstein,
}

impl Ring {
    pub fn named(name: &str) -> Option<Ring> {
        match name {
            "gaussian" => Some(Ring::Gaussian),
            "eisenstein" => Some(Ring::Eisenstein),
            _ => None,
        }
    }
    pub fn norm(self, a: i64, b: i64) -> u64 {
        match self {
            Ring::Gaussian => (a * a + b * b) as u64,
            Ring::Eisenstein => (a * a - a * b + b * b) as u64,
        }
    }
    pub fn mul(self, (a, b): (i64, i64), (c, d): (i64, i64)) -> (i64, i64) {
        match self {
            Ring::Gaussian => (a * c - b * d, a * d + b * c),
            Ring::Eisenstein => (a * c - b * d, a * d + b * c - b * d),
        }
    }
    pub fn turn(self, a: i64, b: i64) -> (i64, i64) {
        match self {
            Ring::Gaussian => (-b, a),
            Ring::Eisenstein => (a - b, a),
        }
    }
    pub fn units(self) -> usize {
        match self {
            Ring::Gaussian => 4,
            Ring::Eisenstein => 6,
        }
    }
    pub fn symmetry(self) -> usize {
        2 * self.units()
    }
    pub fn associates(self, a: i64, b: i64) -> Vec<(i64, i64)> {
        let mut out = Vec::with_capacity(self.units());
        let mut at = (a, b);
        for _ in 0..self.units() {
            out.push(at);
            at = self.turn(at.0, at.1);
        }
        out
    }
    pub fn conjugate(self, a: i64, b: i64) -> (i64, i64) {
        match self {
            Ring::Gaussian => (a, -b),
            Ring::Eisenstein => (a - b, -b),
        }
    }
    pub fn div_rem(self, z: (i64, i64), w: (i64, i64)) -> ((i64, i64), (i64, i64)) {
        let n = self.norm(w.0, w.1) as i64;
        let p = self.mul(z, self.conjugate(w.0, w.1));
        let q = (
            (2 * p.0 + n).div_euclid(2 * n),
            (2 * p.1 + n).div_euclid(2 * n),
        );
        let s = self.mul(q, w);
        (q, (z.0 - s.0, z.1 - s.1))
    }
    pub fn canon(self, a: i64, b: i64) -> (i64, i64) {
        self.associates(a, b)
            .into_iter()
            .find(|&(x, y)| match self {
                Ring::Gaussian => x > 0 && y >= 0,
                Ring::Eisenstein => x > 0 && y >= 0 && y < x,
            })
            .unwrap_or((0, 0))
    }
    pub fn gaussian_gcd(self, z: (i64, i64), w: (i64, i64)) -> (i64, i64) {
        let (mut z, mut w) = (z, w);
        while w != (0, 0) {
            let (_, r) = self.div_rem(z, w);
            z = w;
            w = r;
        }
        self.canon(z.0, z.1)
    }
    pub fn whole(self, a: i64, b: i64) -> Option<u64> {
        self.associates(a, b)
            .into_iter()
            .find(|&(x, y)| y == 0 && x > 0)
            .map(|(x, _)| x as u64)
    }
    pub fn ramified(self) -> u64 {
        match self {
            Ring::Gaussian => 2,
            Ring::Eisenstein => 3,
        }
    }
    pub fn inert(self, p: u64) -> bool {
        match self {
            Ring::Gaussian => p % 4 == 3,
            Ring::Eisenstein => p % 3 == 2,
        }
    }
    pub fn fate(self, n: u64) -> Class {
        match n {
            0 => Class::Zero,
            1 => Class::Unit,
            _ if !is_prime(n as usize) => Class::Composite,
            _ if n == self.ramified() => Class::Ramified,
            _ if self.inert(n) => Class::Inert,
            _ => Class::Split,
        }
    }
    pub fn reach(self, a: i64, b: i64) -> u64 {
        match self {
            Ring::Gaussian => a.abs().max(b.abs()) as u64,
            Ring::Eisenstein => a.abs().max(b.abs()).max((a - b).abs()) as u64,
        }
    }
    pub fn count(self, radius: u64) -> usize {
        let r = radius as usize;
        match self {
            Ring::Gaussian => (2 * r + 1) * (2 * r + 1),
            Ring::Eisenstein => 3 * r * r + 3 * r + 1,
        }
    }
    pub fn top(self, radius: u64) -> u64 {
        match self {
            Ring::Gaussian => 2 * radius * radius,
            Ring::Eisenstein => radius * radius,
        }
    }
    pub fn place(self, a: i64, b: i64) -> (f64, f64) {
        let (a, b) = (a as f64, b as f64);
        match self {
            Ring::Gaussian => (a, b),
            Ring::Eisenstein => (a - b / 2.0, b * ROOT3 / 2.0),
        }
    }
    pub fn nearest(self, x: f64, y: f64) -> (i64, i64) {
        match self {
            Ring::Gaussian => (x.round() as i64, y.round() as i64),
            Ring::Eisenstein => {
                let v = -2.0 * y / ROOT3;
                let u = x - v / 2.0;
                let w = -u - v;
                let (mut ru, mut rv, rw) = (u.round(), v.round(), w.round());
                let (du, dv, dw) = ((ru - u).abs(), (rv - v).abs(), (rw - w).abs());
                if du > dv && du > dw {
                    ru = -rv - rw;
                } else if dv > dw {
                    rv = -ru - rw;
                }
                (ru as i64, -rv as i64)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Class {
    Zero,
    Unit,
    Ramified,
    Split,
    Inert,
    Composite,
}

impl Class {
    pub fn word(self) -> String {
        match self {
            Class::Zero => "zero",
            Class::Unit => "unit",
            Class::Ramified => "ramified",
            Class::Split => "split",
            Class::Inert => "inert",
            Class::Composite => "composite",
        }
        .to_string()
    }
    pub fn prime(self) -> bool {
        matches!(self, Class::Ramified | Class::Split | Class::Inert)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Census {
    pub points: usize,
    pub primes: usize,
    pub split: usize,
    pub inert: usize,
    pub ramified: usize,
    pub units: usize,
    pub composites: usize,
    pub density: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Window {
    ring: Ring,
    radius: u64,
    prime: Vec<bool>,
}

impl Window {
    pub fn new(ring: Ring, radius: u64) -> Window {
        Window {
            ring,
            radius,
            prime: flags(ring.top(radius) as usize),
        }
    }
    pub fn ring(&self) -> Ring {
        self.ring
    }
    pub fn radius(&self) -> u64 {
        self.radius
    }
    pub fn holds(&self, a: i64, b: i64) -> bool {
        self.ring.reach(a, b) <= self.radius
    }
    fn is_prime(&self, n: u64) -> bool {
        match self.prime.get(n as usize) {
            Some(&p) => p,
            None => is_prime(n as usize),
        }
    }
    pub fn class(&self, a: i64, b: i64) -> Class {
        let n = self.ring.norm(a, b);
        if n == 0 {
            return Class::Zero;
        }
        if n == 1 {
            return Class::Unit;
        }
        if self.is_prime(n) {
            return if n == self.ring.ramified() {
                Class::Ramified
            } else {
                Class::Split
            };
        }
        match self.ring.whole(a, b) {
            Some(p) if self.ring.inert(p) && self.is_prime(p) => Class::Inert,
            _ => Class::Composite,
        }
    }
    pub fn points(&self) -> Vec<(i64, i64)> {
        let r = self.radius as i64;
        let mut out = Vec::with_capacity(self.ring.count(self.radius));
        for b in -r..=r {
            for a in -r..=r {
                if self.holds(a, b) {
                    out.push((a, b));
                }
            }
        }
        out
    }
    pub fn census(&self) -> Census {
        let mut census = Census {
            points: 0,
            primes: 0,
            split: 0,
            inert: 0,
            ramified: 0,
            units: 0,
            composites: 0,
            density: 0.0,
        };
        for (a, b) in self.points() {
            census.points += 1;
            match self.class(a, b) {
                Class::Split => census.split += 1,
                Class::Inert => census.inert += 1,
                Class::Ramified => census.ramified += 1,
                Class::Unit => census.units += 1,
                Class::Composite => census.composites += 1,
                Class::Zero => {}
            }
        }
        census.primes = census.split + census.inert + census.ramified;
        census.density = census.primes as f64 / census.points as f64;
        census
    }
}

pub fn shells(ring: Ring, limit: usize) -> Vec<u32> {
    let mut out = vec![0u32; limit + 1];
    let reach = (4 * limit / 3).isqrt() as i64 + 1;
    for a in -reach..=reach {
        for b in -reach..=reach {
            let n = ring.norm(a, b) as usize;
            if n <= limit {
                out[n] += 1;
            }
        }
    }
    out
}

pub fn peak(ring: Ring, limit: usize) -> (usize, u32) {
    shells(ring, limit)
        .into_iter()
        .enumerate()
        .skip(1)
        .fold(
            (0, 0),
            |best, (n, count)| if count > best.1 { (n, count) } else { best },
        )
}

pub fn classes(ring: Ring, bound: u64) -> Vec<(i64, i64)> {
    let reach = (4 * bound / 3).isqrt() as i64 + 1;
    let mut out = Vec::new();
    for a in 1..=reach {
        let wide = match ring {
            Ring::Gaussian => reach,
            Ring::Eisenstein => a - 1,
        };
        for b in 0..=wide {
            if ring.norm(a, b) <= bound {
                out.push((a, b));
            }
        }
    }
    out.sort_by_key(|&(a, b)| (ring.norm(a, b), a, b));
    out
}
