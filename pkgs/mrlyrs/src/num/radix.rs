use crate::core::error::{overflow_error, shape_error, value_error, Result};
use crate::num::gauss::Ring;
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
    if t < 0.0 {
        t + TAU
    } else {
        t
    }
}

/// The base of a radix design: a ring and an element of norm at least two, the scale every word is read against.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Base {
    ring: Ring,
    value: (i64, i64),
}

impl Base {
    /// Fixes a base in a ring, or an error below norm two.
    pub fn new(ring: Ring, value: (i64, i64)) -> Result<Base> {
        let norm = ring.norm(value.0, value.1);
        if norm < 2 {
            return value_error(format!("a base needs norm two or more, not {norm}."));
        }
        Ok(Base { ring, value })
    }
    /// Returns the ring.
    pub fn ring(self) -> Ring {
        self.ring
    }
    /// Returns the base element.
    pub fn value(self) -> (i64, i64) {
        self.value
    }
    /// Returns the norm `q` of the base: the count of residue classes and the square of the scale.
    pub fn norm(self) -> u64 {
        self.ring.norm(self.value.0, self.value.1)
    }
    /// Returns the base raised to a level.
    pub fn power(self, level: usize) -> (i64, i64) {
        let mut out = (1, 0);
        for _ in 0..level {
            out = self.ring.mul(out, self.value);
        }
        out
    }
    /// Returns whether two points are congruent modulo the base.
    pub fn congruent(self, z: (i64, i64), w: (i64, i64)) -> bool {
        divides(self.ring, (z.0 - w.0, z.1 - w.1), self.value)
    }
    /// Returns the canonical complete residue system modulo the base: the `q` representatives of least norm, ties broken by argument in `[0, 2 pi)`.
    ///
    /// The system is built greedily over every point of norm at most `q` sorted by norm and then by argument, which reaches every class because the covering radius of the lattice `b R` is `|b| / sqrt(2)` on the square lattice and `|b| / sqrt(3)` on the hexagonal.
    ///
    /// ```
    /// use mrlyrs::num::gauss::Ring;
    /// use mrlyrs::num::radix::Base;
    /// assert_eq!(Base::new(Ring::Gaussian, (1, 1)).unwrap().residues().unwrap(), vec![(0, 0), (1, 0)]);
    /// ```
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
    /// Returns the index in the canonical residue system of the class of a point, or an error when the system is incomplete.
    pub fn class(self, z: (i64, i64)) -> Result<usize> {
        match self.residues()?.iter().position(|&w| self.congruent(z, w)) {
            Some(index) => Ok(index),
            None => value_error(format!("the point {z:?} lies in no residue class.")),
        }
    }
    /// Returns whether the conjugate of the base is an associate of the base, which is when the mirror joins the symmetry group.
    pub fn mirrored(self) -> bool {
        let c = self.ring.conjugate(self.value.0, self.value.1);
        self.ring
            .associates(self.value.0, self.value.1)
            .contains(&c)
    }
    /// Returns the symmetry group of the base as permutations of the canonical residue indices: every unit multiplication, and every unit times conjugation when the conjugate of the base is an associate of the base.
    ///
    /// Multiplying every digit by a unit `v` carries the attractor of a design to `v` times that attractor over the same base, so the unit group acts; conjugation carries the base to its conjugate and joins the group exactly when that is an associate.
    ///
    /// The list is the Burnside multiset, one entry per abstract group element, and not the order of the permutation group it induces: the action need not be faithful, so entries repeat, and the acting image is the deduplicated list.
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
                            ))
                        }
                    }
                }
                out.push(map);
            }
        }
        Ok(out)
    }
}

/// A radix design: a digit set inside one ring, placed by a base with a unit twist per digit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Radix {
    base: Base,
    digits: Vec<(i64, i64)>,
    twists: Vec<(i64, i64)>,
}

impl Radix {
    /// Builds a design from a base, a digit list and a unit twist per digit, or an error on a length mismatch, a twist whose norm is not one, or two digits congruent modulo the base.
    ///
    /// Pairwise incongruent digits are the hypothesis of the untwisted fill law: they are what recovers the last digit from the word read modulo the base, so a repeated class is refused here rather than silently gluing words.
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
    /// Builds an untwisted design from a code over the canonical residue system, bit `i` of the code selecting residue `i`, or an error when the class count or the code overruns a u128.
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
    /// Returns the design with the twists named by their index in the unit list, the units in turning order from one, or an error at an index past the unit list.
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
                    ))
                }
            }
        }
        Radix::new(self.base, self.digits, twists)
    }
    /// Returns the base.
    pub fn base(&self) -> Base {
        self.base
    }
    /// Returns the ring.
    pub fn ring(&self) -> Ring {
        self.base.ring()
    }
    /// Returns the digits.
    pub fn digits(&self) -> &[(i64, i64)] {
        &self.digits
    }
    /// Returns the twists.
    pub fn twists(&self) -> &[(i64, i64)] {
        &self.twists
    }
    /// Returns the digit count `|F|`.
    pub fn size(&self) -> usize {
        self.digits.len()
    }
    /// Returns the code of the classes the digits occupy, which names the design only when the digits are the canonical representatives.
    pub fn code(&self) -> Result<u128> {
        let mut out = 0u128;
        for &d in &self.digits {
            out |= 1 << self.base.class(d)?;
        }
        Ok(out)
    }
    /// Returns whether every digit is the canonical representative of its class.
    pub fn canonical(&self) -> Result<bool> {
        let residues = self.base.residues()?;
        Ok(self.digits.iter().all(|d| residues.contains(d)))
    }
    /// Returns the count of words of a level, `|F|^L`.
    pub fn fill(&self, level: usize) -> u128 {
        (self.size() as u128).pow(level as u32)
    }
    /// Returns the similarity dimension `log |F| / log sqrt(q)`, the ratio of the digit count to the scale of the base.
    pub fn dimension(&self) -> f64 {
        (self.size() as f64).ln() / (self.base.norm() as f64).ln() * 2.0
    }
    /// Returns the level-`L` points in exact ring coordinates scaled by `b^L`.
    ///
    /// The place map of digit `d` is `phi_d(x) = (u_d x + d) / b`, a word `d_1 ... d_L` lands on `phi_(d_1)(... phi_(d_L)(0))`, and that point times `b^L` is `sum_(j=1..L) (prod_(i<j) u_(d_i)) d_j b^(L-j)`, which stays in the ring. Words come in digit-lexicographic order, the first digit slowest.
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
    /// Returns the level-`L` points in the plane, the scaled words divided by `b^L`.
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
    /// Returns the count of distinct level-`L` points: the glue count, which is the fill exactly when no two words name one point.
    pub fn distinct(&self, level: usize) -> usize {
        self.words(level).into_iter().collect::<HashSet<_>>().len()
    }
}

/// Returns the Koch curve as a radix design: base `3` on the hexagonal lattice, digits `0, 1, 2 + omega, 2`, twists `1, e^(i pi/3), e^(-i pi/3), 1`.
///
/// The digits are not the canonical residues: `2` and `-1` share a class and the canonical system holds `-1`, so the code alone does not name this design.
pub fn koch() -> Result<Radix> {
    let base = Base::new(Ring::Eisenstein, (3, 0))?;
    let digits = vec![(0, 0), (1, 0), (2, 1), (2, 0)];
    Radix::new(base, digits, vec![(1, 0); 4])?.with_twists(&[0, 1, 5, 0])
}

/// Returns the Sierpinski gasket as a radix design: base `2` on the hexagonal lattice, three of the four residues, code `7`.
pub fn gasket() -> Result<Radix> {
    Radix::from_code(Base::new(Ring::Eisenstein, (2, 0))?, 7)
}

/// Returns the twindragon as a radix design: base `1 + i` on the square lattice, the full residue system, code `3`.
pub fn twindragon() -> Result<Radix> {
    Radix::from_code(Base::new(Ring::Gaussian, (1, 1))?, 3)
}

/// Returns the terdragon as a radix design: base `2 + omega` on the hexagonal lattice, the full residue system, code `7`, twisted by `1, omega, 1`.
///
/// The untwisted code misses the curve: reading the L-system `F -> F + F - F` at `120` degrees as a turtle, three segments to a level, and normalising by the endpoint gives the segment starts word for word only under this twist.
pub fn terdragon() -> Result<Radix> {
    Radix::from_code(Base::new(Ring::Eisenstein, (2, 1))?, 7)?.with_twists(&[0, 2, 0])
}

/// Returns the flowsnake as a radix design: base `3 + omega` of norm seven on the hexagonal lattice, the full residue system, code `127`.
pub fn flowsnake() -> Result<Radix> {
    Radix::from_code(Base::new(Ring::Eisenstein, (3, 1))?, 127)
}

/// Returns the plane design of a cell code as a radix design: base the rational integer `m`, of norm `m^2`, on the square lattice, no twist, digits the box residues `{x + y i : 0 <= x, y < m}`.
///
/// Bit `y m + x` of the code is the cell at row `y` and column `x` of the `m` by `m` tile, the column the real part and the row the imaginary part, so the level-`L` words scaled by `m^L` are exactly the filled cells of the level-`L` tile read as `(column, row)`.
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

#[cfg(test)]
mod tests {
    use super::*;

    fn mul(z: (f64, f64), w: (f64, f64)) -> (f64, f64) {
        (z.0 * w.0 - z.1 * w.1, z.0 * w.1 + z.1 * w.0)
    }

    #[test]
    fn the_canonical_residues_are_a_complete_system() {
        let three = Base::new(Ring::Eisenstein, (3, 0)).unwrap();
        assert_eq!(
            three.residues().unwrap(),
            vec![
                (0, 0),
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 0),
                (-1, -1),
                (0, -1),
                (2, 1),
                (1, 2)
            ]
        );
        assert_eq!(
            Base::new(Ring::Gaussian, (1, 1))
                .unwrap()
                .residues()
                .unwrap(),
            vec![(0, 0), (1, 0)]
        );
        assert_eq!(
            Base::new(Ring::Gaussian, (2, 1))
                .unwrap()
                .residues()
                .unwrap(),
            vec![(0, 0), (1, 0), (0, 1), (-1, 0), (0, -1)]
        );
        let bases = [
            Base::new(Ring::Gaussian, (2, 0)).unwrap(),
            Base::new(Ring::Gaussian, (1, 1)).unwrap(),
            Base::new(Ring::Gaussian, (2, 1)).unwrap(),
            Base::new(Ring::Gaussian, (3, 0)).unwrap(),
            Base::new(Ring::Eisenstein, (2, 0)).unwrap(),
            Base::new(Ring::Eisenstein, (2, 1)).unwrap(),
            Base::new(Ring::Eisenstein, (3, 0)).unwrap(),
            Base::new(Ring::Eisenstein, (3, 1)).unwrap(),
        ];
        for base in bases {
            let residues = base.residues().unwrap();
            assert_eq!(residues.len(), base.norm() as usize);
            for (i, &z) in residues.iter().enumerate() {
                for &w in &residues[..i] {
                    assert!(!base.congruent(z, w), "{base:?} {z:?} {w:?}");
                }
            }
            for a in -9..=9 {
                for b in -9..=9 {
                    let hits = residues
                        .iter()
                        .filter(|&&r| base.congruent((a, b), r))
                        .count();
                    assert_eq!(hits, 1, "{base:?} {a} {b}");
                }
            }
            for map in base.group().unwrap() {
                let seen: HashSet<usize> = map.iter().copied().collect();
                assert_eq!(seen.len(), residues.len(), "{base:?}");
            }
            assert_eq!(
                base.group().unwrap().len(),
                base.ring().units() * if base.mirrored() { 2 } else { 1 }
            );
        }
        let multiset = Base::new(Ring::Gaussian, (1, 1)).unwrap().group().unwrap();
        let image: HashSet<&Vec<usize>> = multiset.iter().collect();
        assert_eq!(multiset.len(), 8);
        assert_eq!(image.len(), 1);
    }

    #[test]
    fn refuses_a_base_a_digit_list_and_a_code_out_of_range() {
        assert!(Base::new(Ring::Gaussian, (1, 0)).is_err());
        assert!(Base::new(Ring::Eisenstein, (0, 0)).is_err());
        assert!(Base::new(Ring::Gaussian, (1, 1)).is_ok());
        let base = Base::new(Ring::Eisenstein, (3, 0)).unwrap();
        assert!(base.congruent((2, 0), (-1, 0)));
        assert!(Radix::new(base, vec![(0, 0), (2, 0), (-1, 0)], vec![(1, 0); 3]).is_err());
        assert!(Radix::new(base, vec![(0, 0), (1, 0)], vec![(1, 0); 3]).is_err());
        assert!(Radix::new(base, vec![(0, 0)], vec![(2, 0)]).is_err());
        assert!(Radix::new(base, vec![(0, 0), (1, 0)], vec![(1, 0); 2]).is_ok());
        assert!(Radix::from_code(base, 1 << 9).is_err());
        assert!(Radix::from_code(base, (1 << 9) - 1).is_ok());
        assert!(twindragon().unwrap().with_twists(&[0, 9]).is_err());
        assert!(tile(3, 1 << 9).is_err());
        assert!(tile(12, 1).is_err());
        assert!(tile(3, (1 << 9) - 1).is_ok());
    }

    #[test]
    fn the_canonical_residues_build_a_design() {
        let bases = [
            Base::new(Ring::Gaussian, (1, 1)).unwrap(),
            Base::new(Ring::Gaussian, (2, 1)).unwrap(),
            Base::new(Ring::Eisenstein, (2, 0)).unwrap(),
            Base::new(Ring::Eisenstein, (3, 0)).unwrap(),
            Base::new(Ring::Eisenstein, (3, 1)).unwrap(),
        ];
        for base in bases {
            let digits = base.residues().unwrap();
            let twists = vec![(1, 0); digits.len()];
            let design = Radix::new(base, digits, twists).unwrap();
            assert_eq!(design.size(), base.norm() as usize);
            assert!(design.canonical().unwrap());
        }
        assert_eq!(koch().unwrap().size(), 4);
    }

    #[test]
    fn the_terdragon_is_the_twist_the_l_system_reads() {
        let design = terdragon().unwrap();
        assert_eq!(design.twists().to_vec(), vec![(1, 0), (0, 1), (1, 0)]);
        let level = 2;
        let mut word = b"F".to_vec();
        for _ in 0..level {
            let mut next = Vec::with_capacity(word.len() * 5);
            for &c in &word {
                if c == b'F' {
                    next.extend_from_slice(b"F+F-F");
                } else {
                    next.push(c);
                }
            }
            word = next;
        }
        let root = 3f64.sqrt();
        let step = [(1.0, 0.0), (-0.5, root / 2.0), (-0.5, -root / 2.0)];
        let mut heading = 0usize;
        let mut at = (0.0f64, 0.0f64);
        let mut starts = Vec::new();
        for c in word {
            match c {
                b'F' => {
                    starts.push(at);
                    at = (at.0 + step[heading].0, at.1 + step[heading].1);
                }
                b'+' => heading = (heading + 1) % 3,
                _ => heading = (heading + 2) % 3,
            }
        }
        let den = at.0 * at.0 + at.1 * at.1;
        let want: Vec<(f64, f64)> = starts
            .into_iter()
            .map(|p| {
                (
                    (p.0 * at.0 + p.1 * at.1) / den,
                    (p.1 * at.0 - p.0 * at.1) / den,
                )
            })
            .collect();
        let got = design.plane(level);
        assert_eq!(got.len(), 9);
        assert_eq!(want.len(), 9);
        let worst = got
            .iter()
            .zip(want.iter())
            .map(|(a, b)| ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt())
            .fold(0.0f64, f64::max);
        assert!(worst < 1e-12, "deviation {worst}");
    }

    #[test]
    fn the_twisted_koch_words_are_the_textbook_maps() {
        let koch = koch().unwrap();
        assert_eq!(koch.size(), 4);
        assert!(!koch.canonical().unwrap());
        assert_eq!(koch.base().norm(), 9);
        assert_eq!(koch.digits(), [(0, 0), (1, 0), (2, 1), (2, 0)]);
        assert_eq!(koch.fill(2), 16);
        assert_eq!(format!("{:.6}", koch.dimension()), "1.261860");
        assert_eq!(koch.plane(2)[0], (0.0, 0.0));
        let root = 3f64.sqrt();
        let maps: [((f64, f64), (f64, f64)); 4] = [
            ((1.0 / 3.0, 0.0), (0.0, 0.0)),
            ((1.0 / 6.0, root / 6.0), (1.0 / 3.0, 0.0)),
            ((1.0 / 6.0, -root / 6.0), (0.5, root / 6.0)),
            ((1.0 / 3.0, 0.0), (2.0 / 3.0, 0.0)),
        ];
        let mut truth = vec![(0.0f64, 0.0f64)];
        for level in 1..=2 {
            let mut next = Vec::new();
            for &(scale, shift) in &maps {
                for &p in &truth {
                    let t = mul(scale, p);
                    next.push((t.0 + shift.0, t.1 + shift.1));
                }
            }
            truth = next;
            let got = koch.plane(level);
            assert_eq!(got.len(), truth.len());
            assert_eq!(got.len(), 4usize.pow(level as u32));
            let worst = got
                .iter()
                .zip(truth.iter())
                .map(|(a, b)| ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt())
                .fold(0.0f64, f64::max);
            assert!(worst < 1e-12, "level {level} deviation {worst}");
        }
        assert_eq!(koch.distinct(2), 16);
    }

    #[test]
    fn the_real_base_with_no_twist_is_the_plane_cell() {
        let carpet = tile(3, 0b111101111).unwrap();
        assert_eq!(carpet.size(), 8);
        assert_eq!(carpet.ring(), Ring::Gaussian);
        assert_eq!(carpet.base().norm(), 9);
        assert!(!carpet.canonical().unwrap());
        assert!((carpet.dimension() - 8f64.ln() / 3f64.ln()).abs() < 1e-12);
        assert_eq!(format!("{:.6}", carpet.dimension()), "1.892789");
        assert_eq!(carpet.fill(2), 64);
        assert_eq!(carpet.distinct(2), 64);
        let got: HashSet<(i64, i64)> = carpet.words(2).into_iter().collect();
        let mut want = HashSet::new();
        for row in 0..9i64 {
            for col in 0..9i64 {
                let hole = |r: i64, c: i64| r == 1 && c == 1;
                if !hole(row / 3, col / 3) && !hole(row % 3, col % 3) {
                    want.insert((col, row));
                }
            }
        }
        assert_eq!(got.len(), 64);
        assert_eq!(got, want);
    }

    #[test]
    fn the_fill_law_counts_every_word_and_the_twist_only_moves_it() {
        for design in [
            gasket().unwrap(),
            twindragon().unwrap(),
            terdragon().unwrap(),
            flowsnake().unwrap(),
            koch().unwrap(),
        ] {
            for level in 0..=4 {
                assert_eq!(design.words(level).len() as u128, design.fill(level));
                assert!(design.distinct(level) <= design.words(level).len());
            }
            assert!(
                (design.dimension() * (design.base().norm() as f64).ln() / 2.0
                    - (design.size() as f64).ln())
                .abs()
                    < 1e-12
            );
        }
        let plain = twindragon().unwrap();
        let turned = plain.clone().with_twists(&[0, 1]).unwrap();
        assert_eq!(plain.words(4).len(), turned.words(4).len());
        assert_ne!(plain.words(4), turned.words(4));
        let plain_pair = Radix::from_code(Base::new(Ring::Gaussian, (2, 0)).unwrap(), 3).unwrap();
        assert_eq!(plain_pair.distinct(2), 4);
        let glue = plain_pair.with_twists(&[0, 2]).unwrap();
        assert_eq!(glue.words(2), vec![(0, 0), (1, 0), (2, 0), (1, 0)]);
        assert_eq!(glue.fill(2), 4);
        assert_eq!(glue.distinct(2), 3);
        assert_eq!(glue.code().unwrap(), 3);
        assert!(glue.canonical().unwrap());
        assert_eq!(gasket().unwrap().code().unwrap(), 7);
        assert_eq!(flowsnake().unwrap().code().unwrap(), 127);
        assert!((gasket().unwrap().dimension() - 3f64.ln() / 2f64.ln()).abs() < 1e-12);
        assert!((twindragon().unwrap().dimension() - 2.0).abs() < 1e-12);
        assert!((terdragon().unwrap().dimension() - 2.0).abs() < 1e-12);
        assert!((flowsnake().unwrap().dimension() - 2.0).abs() < 1e-12);
    }
}
