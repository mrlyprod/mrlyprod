use crate::prime::is_prime;
use crate::spiral::flags;

const ROOT3: f64 = 1.732_050_807_568_877_2;

/// The two rings of whole numbers in the plane, each a pair (a, b) on its own lattice.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ring {
    /// a + b i on the square lattice: norm a^2 + b^2, four units, the window a square.
    Gaussian,
    /// a + b omega on the hexagonal lattice, omega a cube root of one: norm a^2 - a b + b^2, six units, the window a hexagon.
    Eisenstein,
}

impl Ring {
    /// Reads a ring from its name.
    pub fn named(name: &str) -> Option<Ring> {
        match name {
            "gaussian" => Some(Ring::Gaussian),
            "eisenstein" => Some(Ring::Eisenstein),
            _ => None,
        }
    }
    /// Returns the norm of a point: its squared length.
    pub fn norm(self, a: i64, b: i64) -> u64 {
        match self {
            Ring::Gaussian => (a * a + b * b) as u64,
            Ring::Eisenstein => (a * a - a * b + b * b) as u64,
        }
    }
    /// Returns the product of two points.
    pub fn mul(self, (a, b): (i64, i64), (c, d): (i64, i64)) -> (i64, i64) {
        match self {
            Ring::Gaussian => (a * c - b * d, a * d + b * c),
            Ring::Eisenstein => (a * c - b * d, a * d + b * c - b * d),
        }
    }
    /// Returns the point turned anticlockwise by one unit: a quarter turn or a sixth.
    pub fn turn(self, a: i64, b: i64) -> (i64, i64) {
        match self {
            Ring::Gaussian => (-b, a),
            Ring::Eisenstein => (a - b, a),
        }
    }
    /// Returns the count of units: 4 or 6.
    pub fn units(self) -> usize {
        match self {
            Ring::Gaussian => 4,
            Ring::Eisenstein => 6,
        }
    }
    /// Returns the order of the symmetry of the picture, the units and the mirror: 8 or 12.
    pub fn symmetry(self) -> usize {
        2 * self.units()
    }
    /// Returns the unit multiples of a point, the point first, turning anticlockwise.
    pub fn associates(self, a: i64, b: i64) -> Vec<(i64, i64)> {
        let mut out = Vec::with_capacity(self.units());
        let mut at = (a, b);
        for _ in 0..self.units() {
            out.push(at);
            at = self.turn(at.0, at.1);
        }
        out
    }
    /// Returns the conjugate: the mirror image in the real axis.
    pub fn conjugate(self, a: i64, b: i64) -> (i64, i64) {
        match self {
            Ring::Gaussian => (a, -b),
            Ring::Eisenstein => (a - b, -b),
        }
    }
    /// Returns the whole number an associate of the point lies on, when one lies on the positive real axis.
    pub fn whole(self, a: i64, b: i64) -> Option<u64> {
        self.associates(a, b)
            .into_iter()
            .find(|&(x, y)| y == 0 && x > 0)
            .map(|(x, _)| x as u64)
    }
    /// Returns the one rational prime that ramifies: 2 or 3.
    pub fn ramified(self) -> u64 {
        match self {
            Ring::Gaussian => 2,
            Ring::Eisenstein => 3,
        }
    }
    /// Returns whether a rational prime stays prime in the ring: 3 mod 4, or 2 mod 3.
    pub fn inert(self, p: u64) -> bool {
        match self {
            Ring::Gaussian => p % 4 == 3,
            Ring::Eisenstein => p % 3 == 2,
        }
    }
    /// Returns the fate of a whole number as a prime of the ring: split, inert or ramified, unit for one, zero for zero, composite otherwise.
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
    /// Returns the reach of a point: the ring of the window it sits on, the Chebyshev distance or the hex distance.
    pub fn reach(self, a: i64, b: i64) -> u64 {
        match self {
            Ring::Gaussian => a.abs().max(b.abs()) as u64,
            Ring::Eisenstein => a.abs().max(b.abs()).max((a - b).abs()) as u64,
        }
    }
    /// Returns the count of points within the reach: the square or the hexagon.
    pub fn count(self, radius: u64) -> usize {
        let r = radius as usize;
        match self {
            Ring::Gaussian => (2 * r + 1) * (2 * r + 1),
            Ring::Eisenstein => 3 * r * r + 3 * r + 1,
        }
    }
    /// Returns the largest norm within the reach: 2 r^2 at the square's corner, r^2 at the hexagon's.
    pub fn top(self, radius: u64) -> u64 {
        match self {
            Ring::Gaussian => 2 * radius * radius,
            Ring::Eisenstein => radius * radius,
        }
    }
    /// Returns the place of a point in the plane, x right and y up, one unit between neighbours.
    pub fn place(self, a: i64, b: i64) -> (f64, f64) {
        let (a, b) = (a as f64, b as f64);
        match self {
            Ring::Gaussian => (a, b),
            Ring::Eisenstein => (a - b / 2.0, b * ROOT3 / 2.0),
        }
    }
    /// Returns the point nearest a place in the plane.
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

/// What a point of the ring is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    /// The origin.
    Zero,
    /// A unit: norm one.
    Unit,
    /// A prime over the one rational prime that ramifies, 2 or 3.
    Ramified,
    /// A prime whose norm is a rational prime that splits into it and its conjugate.
    Split,
    /// A rational prime that stays prime in the ring, times a unit.
    Inert,
    /// A product of two points of norm above one.
    Composite,
}

impl Class {
    /// Returns the class as a word.
    pub fn word(self) -> &'static str {
        match self {
            Class::Zero => "zero",
            Class::Unit => "unit",
            Class::Ramified => "ramified",
            Class::Split => "split",
            Class::Inert => "inert",
            Class::Composite => "composite",
        }
    }
    /// Returns whether the class is prime.
    pub fn prime(self) -> bool {
        matches!(self, Class::Ramified | Class::Split | Class::Inert)
    }
}

/// The tallies of a window: every class counted and the share of primes.
#[derive(Clone, Debug, PartialEq)]
pub struct Census {
    /// The count of points.
    pub points: usize,
    /// The count of primes.
    pub primes: usize,
    /// The split primes.
    pub split: usize,
    /// The inert primes.
    pub inert: usize,
    /// The ramified primes.
    pub ramified: usize,
    /// The units.
    pub units: usize,
    /// The composites.
    pub composites: usize,
    /// The primes over the points.
    pub density: f64,
}

/// The symmetric window of one ring: every point within a reach, with the norms sieved once.
#[derive(Clone, Debug)]
pub struct Window {
    ring: Ring,
    radius: u64,
    prime: Vec<bool>,
}

impl Window {
    /// Opens the window of a ring out to a reach, sieving every norm inside it.
    pub fn new(ring: Ring, radius: u64) -> Window {
        Window {
            ring,
            radius,
            prime: flags(ring.top(radius) as usize),
        }
    }
    /// Returns the ring.
    pub fn ring(&self) -> Ring {
        self.ring
    }
    /// Returns the reach.
    pub fn radius(&self) -> u64 {
        self.radius
    }
    /// Returns whether a point lies inside.
    pub fn holds(&self, a: i64, b: i64) -> bool {
        self.ring.reach(a, b) <= self.radius
    }
    fn is_prime(&self, n: u64) -> bool {
        match self.prime.get(n as usize) {
            Some(&p) => p,
            None => is_prime(n as usize),
        }
    }
    /// Classifies a point: prime when its norm is a rational prime, or when it is a unit times a rational prime that stays prime.
    ///
    /// ```
    /// use mrlynum::gauss::{Class, Ring, Window};
    /// let window = Window::new(Ring::Gaussian, 3);
    /// assert_eq!(window.class(2, 1), Class::Split);
    /// assert_eq!(window.class(0, -3), Class::Inert);
    /// ```
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
    /// Lists every point inside, row by row from the bottom left of the bounding square.
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
    /// Counts every class inside.
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

/// Counts the points of every norm from zero through the limit, by enumeration: the ring weights of the lattice.
///
/// ```
/// assert_eq!(mrlynum::gauss::shells(mrlynum::gauss::Ring::Gaussian, 5), vec![1, 4, 4, 0, 4, 8]);
/// ```
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

/// Returns the norm from one through the limit with the most points and that count, the earliest on a tie.
///
/// ```
/// assert_eq!(mrlynum::gauss::peak(mrlynum::gauss::Ring::Gaussian, 60), (25, 12));
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::factor::divisors;

    fn divides(ring: Ring, z: (i64, i64), w: (i64, i64)) -> bool {
        let n = ring.norm(w.0, w.1) as i64;
        let (x, y) = ring.mul(z, ring.conjugate(w.0, w.1));
        x % n == 0 && y % n == 0
    }

    fn brute(ring: Ring, z: (i64, i64)) -> bool {
        let n = ring.norm(z.0, z.1);
        if n < 2 {
            return false;
        }
        let r = n.isqrt() as i64 + 1;
        for c in -r..=r {
            for d in -r..=r {
                let m = ring.norm(c, d);
                if m > 1 && m < n && divides(ring, z, (c, d)) {
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn the_gaussian_window_pins_its_primes() {
        let two = Window::new(Ring::Gaussian, 2).census();
        assert_eq!((two.points, two.primes), (25, 12));
        assert_eq!((two.ramified, two.split, two.inert), (4, 8, 0));
        let three = Window::new(Ring::Gaussian, 3).census();
        assert_eq!((three.points, three.primes, three.units), (49, 24, 4));
        assert_eq!((three.ramified, three.split, three.inert), (4, 16, 4));
        assert_eq!(three.composites, 49 - 24 - 4 - 1);
        assert!((three.density - 24.0 / 49.0).abs() < 1e-12);
    }

    #[test]
    fn the_eisenstein_window_pins_its_primes() {
        let two = Window::new(Ring::Eisenstein, 2).census();
        assert_eq!((two.points, two.primes, two.units), (19, 12, 6));
        assert_eq!((two.ramified, two.split, two.inert), (6, 0, 6));
        let three = Window::new(Ring::Eisenstein, 3).census();
        assert_eq!((three.points, three.primes), (37, 24));
        assert_eq!((three.ramified, three.split, three.inert), (6, 12, 6));
    }

    #[test]
    fn the_classes_follow_the_norm_rules() {
        let g = Window::new(Ring::Gaussian, 5);
        assert_eq!(g.class(0, 0), Class::Zero);
        assert_eq!(g.class(-1, 0), Class::Unit);
        assert_eq!(g.class(1, 1), Class::Ramified);
        assert_eq!(g.class(2, 1), Class::Split);
        assert_eq!(g.class(3, 0), Class::Inert);
        assert_eq!(g.class(0, -3), Class::Inert);
        assert_eq!(g.class(5, 0), Class::Composite);
        assert_eq!(g.class(1, 3), Class::Composite);
        let e = Window::new(Ring::Eisenstein, 5);
        assert_eq!(e.class(1, 1), Class::Unit);
        assert_eq!(e.class(1, -1), Class::Ramified);
        assert_eq!(e.class(2, 0), Class::Inert);
        assert_eq!(e.class(2, 2), Class::Inert);
        assert_eq!(e.class(3, 0), Class::Composite);
        assert_eq!(e.class(2, -1), Class::Split);
        assert_eq!(e.class(3, 1), Class::Split);
        assert_eq!(e.class(4, 1), Class::Split);
        for ring in [Ring::Gaussian, Ring::Eisenstein] {
            let window = Window::new(ring, 9);
            for (a, b) in window.points() {
                assert_eq!(
                    window.class(a, b).prime(),
                    brute(ring, (a, b)),
                    "{ring:?} {a} {b}"
                );
            }
            for n in 0..40 {
                let axis = window.class(n, 0);
                let fate = ring.fate(n as u64);
                let expect = match fate {
                    Class::Split | Class::Ramified => Class::Composite,
                    other => other,
                };
                assert_eq!(axis, expect, "{ring:?} {n}");
            }
        }
        assert_eq!(Ring::Gaussian.fate(5), Class::Split);
        assert_eq!(Ring::Gaussian.fate(7), Class::Inert);
        assert_eq!(Ring::Gaussian.fate(2), Class::Ramified);
        assert_eq!(Ring::Eisenstein.fate(7), Class::Split);
        assert_eq!(Ring::Eisenstein.fate(5), Class::Inert);
        assert_eq!(Ring::Eisenstein.fate(3), Class::Ramified);
    }

    #[test]
    fn the_units_and_the_mirror_keep_the_norm() {
        assert_eq!(
            Ring::Gaussian.associates(2, 1),
            vec![(2, 1), (-1, 2), (-2, -1), (1, -2)]
        );
        assert_eq!(
            Ring::Eisenstein.associates(2, 0),
            vec![(2, 0), (2, 2), (0, 2), (-2, 0), (-2, -2), (0, -2)]
        );
        assert_eq!(Ring::Gaussian.conjugate(2, 1), (2, -1));
        assert_eq!(Ring::Eisenstein.conjugate(2, -1), (3, 1));
        assert_eq!(Ring::Eisenstein.conjugate(0, 1), (-1, -1));
        for ring in [Ring::Gaussian, Ring::Eisenstein] {
            assert_eq!(ring.symmetry(), 2 * ring.units());
            for a in -4..=4 {
                for b in -4..=4 {
                    let n = ring.norm(a, b);
                    let (c, d) = ring.conjugate(a, b);
                    assert_eq!(ring.norm(c, d), n);
                    assert_eq!(ring.mul((a, b), (c, d)), (n as i64, 0));
                    for (x, y) in ring.associates(a, b) {
                        assert_eq!(ring.norm(x, y), n);
                        assert_eq!(ring.reach(x, y), ring.reach(a, b));
                    }
                    let (x, y) = ring.place(a, b);
                    assert!((x * x + y * y - n as f64).abs() < 1e-9);
                    assert_eq!(ring.nearest(x + 0.45, y), (a, b));
                    assert_eq!(ring.nearest(x, y - 0.45), (a, b));
                }
            }
        }
        assert_eq!(Ring::Eisenstein.reach(1, -1), 2);
        assert_eq!(Ring::Eisenstein.reach(2, -1), 3);
        for r in 0..6 {
            let window = Window::new(Ring::Eisenstein, r);
            assert_eq!(window.points().len(), Ring::Eisenstein.count(r));
            let peak = window
                .points()
                .iter()
                .map(|&(a, b)| Ring::Eisenstein.norm(a, b))
                .max()
                .unwrap();
            assert_eq!(peak, Ring::Eisenstein.top(r));
        }
    }

    #[test]
    fn the_shells_are_the_ring_weights() {
        let r2 = shells(Ring::Gaussian, 25);
        assert_eq!(
            (r2[0], r2[1], r2[2], r2[3], r2[5], r2[25]),
            (1, 4, 4, 0, 8, 12)
        );
        for (n, &count) in shells(Ring::Gaussian, 2000).iter().enumerate().skip(1) {
            let (d1, d3) = divisors(n).iter().fold((0, 0), |(d1, d3), &d| {
                (d1 + (d % 4 == 1) as u32, d3 + (d % 4 == 3) as u32)
            });
            assert_eq!(count, 4 * (d1 - d3), "{n}");
        }
        let hex = shells(Ring::Eisenstein, 7);
        assert_eq!(hex, vec![1, 6, 0, 6, 6, 0, 0, 12]);
        for (n, &count) in shells(Ring::Eisenstein, 300).iter().enumerate().skip(1) {
            let chi: i32 = divisors(n)
                .iter()
                .map(|&d| match d % 3 {
                    1 => 1,
                    2 => -1,
                    _ => 0,
                })
                .sum();
            assert_eq!(count as i32, 6 * chi, "{n}");
        }
    }

    #[test]
    fn the_peak_is_the_busiest_norm() {
        assert_eq!(peak(Ring::Gaussian, 60), (25, 12));
        assert_eq!(peak(Ring::Eisenstein, 60), (49, 18));
        assert_eq!(peak(Ring::Gaussian, 4), (1, 4));
    }
}
