use crate::factor::mobius_sieve;
use crate::prime::{is_prime, Sieve};

const HEX: [(i64, i64); 6] = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];

/// The two lattices a spiral of the whole numbers is wound on, one at the centre and two to its right.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lattice {
    /// Unit squares turning anticlockwise with y up: ring k holds 8k cells and ends at the odd square (2k + 1)^2 on the diagonal below right.
    Square,
    /// Hexagons in axial coordinates q and r, r growing downward: ring r holds 6r cells and ends at the centered hexagonal number 3r^2 + 3r + 1 below right of the centre.
    Hex,
}

impl Lattice {
    /// Reads a lattice from its name.
    pub fn named(name: &str) -> Option<Lattice> {
        match name {
            "square" => Some(Lattice::Square),
            "hex" => Some(Lattice::Hex),
            _ => None,
        }
    }
    /// Returns the outermost ring of a sheet the odd side wide, half the side rounded down.
    pub fn radius(self, side: usize) -> usize {
        side.saturating_sub(1) / 2
    }
    /// Returns the count of numbers a sheet the odd side wide holds: the side squared, or the hexagon of that many cells across.
    pub fn count(self, side: usize) -> usize {
        let r = self.radius(side);
        match self {
            Lattice::Square => (2 * r + 1) * (2 * r + 1),
            Lattice::Hex => 3 * r * r + 3 * r + 1,
        }
    }
    /// Returns the ring a number sits on, zero for one.
    pub fn ring(self, n: u64) -> u64 {
        if n < 2 {
            return 0;
        }
        match self {
            Lattice::Square => (n - 1).isqrt().div_ceil(2),
            Lattice::Hex => {
                let mut r = ((12 * n - 3).isqrt() - 3) / 6;
                while 3 * r * r + 3 * r + 1 < n {
                    r += 1;
                }
                r
            }
        }
    }
    /// Returns the ring of a cell: the larger of the coordinates on the square, the hex distance on the hexagon.
    pub fn ring_of(self, x: i64, y: i64) -> u64 {
        match self {
            Lattice::Square => x.abs().max(y.abs()) as u64,
            Lattice::Hex => x.abs().max(y.abs()).max((x + y).abs()) as u64,
        }
    }
    /// Returns the cell of a number: x right and y up on the square, axial q and r on the hexagon.
    ///
    /// ```
    /// use mrlynum::spiral::Lattice;
    /// assert_eq!(Lattice::Square.xy(10), (2, -1));
    /// assert_eq!(Lattice::Hex.xy(8), (1, 1));
    /// ```
    pub fn xy(self, n: u64) -> (i64, i64) {
        let k = self.ring(n) as i64;
        if k == 0 {
            return (0, 0);
        }
        let n = n as i64;
        match self {
            Lattice::Square => {
                let m = (2 * k + 1) * (2 * k + 1);
                if n >= m - 2 * k {
                    (k - (m - n), -k)
                } else if n >= m - 4 * k {
                    (-k, -k + (m - 2 * k - n))
                } else if n >= m - 6 * k {
                    (-k + (m - 4 * k - n), k)
                } else {
                    (k, k - (m - 6 * k - n))
                }
            }
            Lattice::Hex => {
                let i = n - (3 * k * k - 3 * k + 1) - 1;
                let (side, step) = ((i / k) as usize, i % k + 1);
                let (cq, cr) = HEX[(side + 5) % 6];
                let (dq, dr) = HEX[(side + 1) % 6];
                (k * cq + step * dq, k * cr + step * dr)
            }
        }
    }
    /// Returns the number at a cell, one at the origin.
    ///
    /// ```
    /// use mrlynum::spiral::Lattice;
    /// assert_eq!(Lattice::Square.n(2, -2), 25);
    /// assert_eq!(Lattice::Hex.n(0, 2), 19);
    /// ```
    pub fn n(self, x: i64, y: i64) -> u64 {
        let k = self.ring_of(x, y) as i64;
        if k == 0 {
            return 1;
        }
        let n = match self {
            Lattice::Square => {
                let m = (2 * k + 1) * (2 * k + 1);
                if y == -k {
                    m - k + x
                } else if x == -k {
                    m - 3 * k - y
                } else if y == k {
                    m - 5 * k - x
                } else {
                    m - 7 * k + y
                }
            }
            Lattice::Hex => {
                let base = 3 * k * k - 3 * k + 1;
                let (side, step) = if x > 0 && y >= 0 && x + y == k {
                    (0, x)
                } else if x == k {
                    (1, -y)
                } else if y == -k && x >= 0 {
                    (2, k - x)
                } else if x + y == -k && x < 0 {
                    (3, -x)
                } else if x == -k {
                    (4, y)
                } else {
                    (5, x + k)
                };
                base + side * k + step
            }
        };
        n as u64
    }
}

/// What a cell is painted for.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mark {
    /// The primes.
    Prime,
    /// The primes with a prime two away.
    Twin,
    /// The numbers no prime squares into.
    Squarefree,
    /// The Mobius value: one, minus one, or zero for a squared factor.
    Mobius,
}

impl Mark {
    /// Reads a mark from its name.
    pub fn named(name: &str) -> Option<Mark> {
        match name {
            "prime" => Some(Mark::Prime),
            "twin" => Some(Mark::Twin),
            "squarefree" => Some(Mark::Squarefree),
            "mobius" => Some(Mark::Mobius),
            _ => None,
        }
    }
}

/// Returns whether every number from zero through the limit is prime, by one sieve.
pub fn flags(limit: usize) -> Vec<bool> {
    let mut sieve = Sieve::new(limit);
    sieve.finish();
    sieve.types().iter().map(|&t| t == 1).collect()
}

/// Marks every number from zero through the limit: one when marked, minus one for a Mobius value of minus one, else zero.
///
/// ```
/// assert_eq!(mrlynum::spiral::marks(mrlynum::spiral::Mark::Mobius, 6), vec![0, 1, -1, -1, 0, -1, 1]);
/// ```
pub fn marks(mark: Mark, limit: usize) -> Vec<i8> {
    match mark {
        Mark::Prime => flags(limit).iter().map(|&p| i8::from(p)).collect(),
        Mark::Twin => {
            let prime = flags(limit);
            (0..=limit)
                .map(|n| {
                    let twin = (n >= 2 && prime[n - 2]) || is_prime(n + 2);
                    i8::from(prime[n] && twin)
                })
                .collect()
        }
        Mark::Squarefree => mobius_sieve(limit)
            .iter()
            .map(|&m| i8::from(m != 0))
            .collect(),
        Mark::Mobius => mobius_sieve(limit),
    }
}

/// The readout of one quadratic a k^2 + b k + c across a sheet: where it lands and how often on a prime.
#[derive(Clone, Debug, PartialEq)]
pub struct Diagonal {
    /// The count of numbers the sheet holds.
    pub top: usize,
    /// The count of primes among them.
    pub primes: usize,
    /// The primes as a share of the numbers.
    pub density: f64,
    /// The values of the quadratic inside the sheet, k counting up from zero.
    pub values: Vec<u64>,
    /// The cell of each value.
    pub cells: Vec<(i64, i64)>,
    /// Whether each value is prime.
    pub hit: Vec<bool>,
    /// The count of values that are prime.
    pub hits: usize,
    /// The count of primes before the first composite.
    pub streak: usize,
    /// The hits as a share of the values, zero when the quadratic misses the sheet.
    pub share: f64,
}

/// Reads the quadratic a k^2 + b k + c, a at least one, over the sheet the odd side wide: every value from one through the top, its cell, the prime hits and the opening streak.
///
/// ```
/// let read = mrlynum::spiral::diagonal(mrlynum::spiral::Lattice::Square, 201, 4, -2, 41);
/// assert_eq!((read.top, read.streak), (40401, 21));
/// ```
pub fn diagonal(lattice: Lattice, side: usize, a: i64, b: i64, c: i64) -> Diagonal {
    let top = lattice.count(side);
    let prime = flags(top);
    let mut values = Vec::new();
    let mut k = 0i64;
    while k <= b.abs() + side as i64 + 2 {
        let v = a * k * k + b * k + c;
        if v >= 1 && v <= top as i64 {
            values.push(v as u64);
        }
        if v > top as i64 && 2 * a * k + b >= 0 {
            break;
        }
        k += 1;
    }
    let cells = values.iter().map(|&v| lattice.xy(v)).collect();
    let hit: Vec<bool> = values.iter().map(|&v| prime[v as usize]).collect();
    let primes = prime.iter().filter(|&&p| p).count();
    let hits = hit.iter().filter(|&&h| h).count();
    Diagonal {
        top,
        primes,
        density: primes as f64 / top.max(1) as f64,
        share: if hit.is_empty() {
            0.0
        } else {
            hits as f64 / hit.len() as f64
        },
        hits,
        streak: hit.iter().take_while(|&&h| h).count(),
        values,
        cells,
        hit,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classics::primes;
    use crate::factor::{mobius, squarefree};

    #[test]
    fn the_square_spiral_pins_the_first_rings() {
        let first: Vec<(i64, i64)> = (1..=10).map(|n| Lattice::Square.xy(n)).collect();
        assert_eq!(
            first,
            vec![
                (0, 0),
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
                (2, -1)
            ]
        );
        assert_eq!(Lattice::Square.xy(25), (2, -2));
        assert_eq!(Lattice::Square.xy(0), (0, 0));
        for k in 1..=30u64 {
            let odd = (2 * k + 1) * (2 * k + 1);
            assert_eq!(Lattice::Square.xy(odd), (k as i64, -(k as i64)));
            assert_eq!(Lattice::Square.ring(odd), k);
            assert_eq!(Lattice::Square.ring(odd + 1), k + 1);
        }
    }

    #[test]
    fn the_hex_spiral_pins_the_first_rings() {
        let first: Vec<(i64, i64)> = (1..=8).map(|n| Lattice::Hex.xy(n)).collect();
        assert_eq!(
            first,
            vec![
                (0, 0),
                (1, 0),
                (1, -1),
                (0, -1),
                (-1, 0),
                (-1, 1),
                (0, 1),
                (1, 1)
            ]
        );
        assert_eq!(Lattice::Hex.ring(8), 2);
        assert_eq!((Lattice::Hex.xy(19), Lattice::Hex.ring(19)), ((0, 2), 2));
        assert_eq!((Lattice::Hex.xy(20), Lattice::Hex.ring(20)), ((1, 2), 3));
        for r in 1..=30u64 {
            let last = 3 * r * r + 3 * r + 1;
            assert_eq!(Lattice::Hex.xy(last), (0, r as i64));
            assert_eq!(Lattice::Hex.ring(last), r);
            assert_eq!(Lattice::Hex.ring(last + 1), r + 1);
        }
    }

    #[test]
    fn both_spirals_walk_neighbours_and_map_back() {
        for lattice in [Lattice::Square, Lattice::Hex] {
            let mut last = (0, 0);
            let mut on_ring = vec![0u64; 200];
            for n in 1..=100_000u64 {
                let (x, y) = lattice.xy(n);
                assert_eq!(lattice.n(x, y), n, "{lattice:?} {n}");
                let ring = lattice.ring(n);
                assert_eq!(lattice.ring_of(x, y), ring, "{lattice:?} {n}");
                on_ring[ring as usize] += 1;
                if n > 1 {
                    let step = (x - last.0, y - last.1);
                    let near = match lattice {
                        Lattice::Square => step.0.abs() + step.1.abs() == 1,
                        Lattice::Hex => HEX.contains(&step),
                    };
                    assert!(near, "{lattice:?} {n}");
                }
                last = (x, y);
            }
            let per = match lattice {
                Lattice::Square => 8,
                Lattice::Hex => 6,
            };
            for (r, &count) in on_ring.iter().enumerate().take(51).skip(1) {
                assert_eq!(count, per * r as u64, "{lattice:?} ring {r}");
            }
        }
    }

    #[test]
    fn the_sheet_counts_agree_with_the_rings() {
        assert_eq!(Lattice::Square.count(201), 40401);
        assert_eq!(Lattice::Square.count(401), 160801);
        assert_eq!(Lattice::Hex.count(401), 120601);
        assert_eq!(Lattice::Hex.radius(401), 200);
        assert_eq!((Lattice::Hex.count(1), Lattice::Hex.count(0)), (1, 1));
        for side in (1..=101).step_by(2) {
            for lattice in [Lattice::Square, Lattice::Hex] {
                let top = lattice.count(side) as u64;
                assert_eq!(lattice.ring(top), lattice.radius(side) as u64);
                assert_eq!(lattice.ring(top + 1), lattice.radius(side) as u64 + 1);
            }
        }
        assert_eq!(Lattice::named("hex"), Some(Lattice::Hex));
        assert_eq!(Lattice::named("cube"), None);
        assert_eq!(Mark::named("twin"), Some(Mark::Twin));
        assert_eq!(Mark::named("odd"), None);
    }

    #[test]
    fn the_marks_agree_with_the_single_tests() {
        let limit = 3_000;
        let prime = marks(Mark::Prime, limit);
        let twin = marks(Mark::Twin, limit);
        let free = marks(Mark::Squarefree, limit);
        let mu = marks(Mark::Mobius, limit);
        for n in 0..=limit {
            assert_eq!(prime[n] == 1, is_prime(n), "{n}");
            let pair = is_prime(n) && (is_prime(n + 2) || (n >= 2 && is_prime(n - 2)));
            assert_eq!(twin[n] == 1, pair, "{n}");
            assert_eq!(free[n] == 1, squarefree(n), "{n}");
            assert_eq!(mu[n], mobius(n), "{n}");
        }
        assert_eq!(
            marks(Mark::Prime, 40_000)
                .iter()
                .filter(|&&m| m == 1)
                .count(),
            4203
        );
        assert_eq!(
            marks(Mark::Prime, 40_401)
                .iter()
                .filter(|&&m| m == 1)
                .count(),
            primes(40_401).len()
        );
        assert_eq!(marks(Mark::Twin, 30)[19], 1);
        assert_eq!(marks(Mark::Twin, 30)[23], 0);
    }

    #[test]
    fn eulers_quadratic_opens_with_twenty_one_primes_on_one_line() {
        let read = diagonal(Lattice::Square, 201, 4, -2, 41);
        assert_eq!((read.top, read.primes), (40401, primes(40401).len()));
        assert_eq!(read.values.len(), 101);
        assert_eq!(read.streak, 21);
        for (k, &v) in read.values.iter().enumerate() {
            let m = 2 * k as u64;
            assert_eq!(v, m * m - m + 41);
        }
        assert_eq!(read.values[21], 1763);
        assert!(!is_prime(1763));
        let direct = read
            .values
            .iter()
            .filter(|&&v| is_prime(v as usize))
            .count();
        assert_eq!(read.hits, direct);
        assert_eq!(read.hits, 80);
        assert_eq!(read.hit.iter().filter(|&&h| h).count(), 80);
        assert!(!read.hit[21]);
        assert!((read.density - read.primes as f64 / 40401.0).abs() < 1e-12);
        assert!((read.share - 80.0 / 101.0).abs() < 1e-12);
        assert_eq!(diagonal(Lattice::Square, 21, 1, 0, 500).share, 0.0);
        for k in 20..=100 {
            assert_eq!(read.cells[k], (k as i64 - 40, k as i64), "{k}");
        }
        let spoke = diagonal(Lattice::Hex, 41, 3, 3, 1);
        assert_eq!(spoke.values.len(), 21);
        for (k, &cell) in spoke.cells.iter().enumerate() {
            assert_eq!(cell, (0, k as i64));
        }
        let dip = diagonal(Lattice::Square, 21, 1, -30, 2);
        assert_eq!(
            dip.values,
            vec![2, 2, 33, 66, 101, 138, 177, 218, 261, 306, 353, 402]
        );
        assert!(diagonal(Lattice::Square, 21, 1, 0, 500).values.is_empty());
    }
}
