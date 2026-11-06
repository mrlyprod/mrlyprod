use mrly_name::named_enum;
use mrly_num::factor::mobius_sieve;
use mrly_num::prime::{flags, is_prime};
use serde::{Deserialize, Serialize};

const HEX: [(i64, i64); 6] = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];

named_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Lattice {
        Square => "square",
        Hex => "hex",
    }
}

impl Lattice {
    pub fn radius(self, side: usize) -> usize {
        side.saturating_sub(1) / 2
    }
    pub fn count(self, side: usize) -> usize {
        let r = self.radius(side);
        match self {
            Lattice::Square => (2 * r + 1) * (2 * r + 1),
            Lattice::Hex => 3 * r * r + 3 * r + 1,
        }
    }
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
    pub fn ring_of(self, x: i64, y: i64) -> u64 {
        match self {
            Lattice::Square => x.abs().max(y.abs()) as u64,
            Lattice::Hex => x.abs().max(y.abs()).max((x + y).abs()) as u64,
        }
    }
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

named_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Mark {
        Prime => "prime",
        Twin => "twin",
        Squarefree => "squarefree",
        Mobius => "mobius",
    }
}

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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Diagonal {
    pub top: usize,
    pub primes: usize,
    pub density: f64,
    pub values: Vec<u64>,
    pub cells: Vec<(i64, i64)>,
    pub hit: Vec<bool>,
    pub hits: usize,
    pub streak: usize,
    pub share: f64,
}

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

named_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Growth {
        Prime => "prime",
        Every => "every",
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tile {
    pub n: u64,
    pub level: u32,
    pub side: u64,
    pub prime: bool,
    pub x: i64,
    pub y: i64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Snail {
    pub base: u64,
    pub tiles: Vec<Tile>,
    pub primes: usize,
    pub levels: Vec<usize>,
    pub area: u128,
    pub low: (i64, i64),
    pub high: (i64, i64),
}

pub fn level_of(n: u64, base: u64) -> u32 {
    let base = base.max(2);
    let (mut level, mut reach) = (0, base);
    while reach <= n {
        reach *= base;
        level += 1;
    }
    level
}

pub fn snail(base: u64, top: u64, growth: Growth) -> Snail {
    let base = base.max(2);
    let top = top.max(1);
    let prime = flags(top as usize);
    let mut tiles = Vec::with_capacity(top as usize);
    let mut levels = vec![0usize; level_of(top, base) as usize + 1];
    let (mut x, mut y) = (0i64, 0i64);
    let (mut low, mut high) = ((0i64, 0i64), (0i64, 0i64));
    let mut area = 0u128;
    let mut cell = (0i64, 0i64);
    let level_at = |n: u64| {
        if growth == Growth::Every || prime.get(n as usize).copied().unwrap_or(false) {
            level_of(n, base)
        } else {
            0
        }
    };
    for n in 1..=top {
        let is_prime = prime[n as usize];
        let level = level_at(n);
        let side = base.pow(level);
        levels[level as usize] += 1;
        area += u128::from(side) * u128::from(side);
        low = (low.0.min(x), low.1.min(y));
        high = (high.0.max(x + side as i64), high.1.max(y + side as i64));
        tiles.push(Tile {
            n,
            level,
            side,
            prime: is_prime,
            x,
            y,
        });
        let next = Lattice::Square.xy(n + 1);
        let ahead = base.pow(level_at(n + 1)) as i64;
        let (dx, dy) = (next.0 - cell.0, next.1 - cell.1);
        x += if dx > 0 { side as i64 } else { dx * ahead };
        y += if dy > 0 { side as i64 } else { dy * ahead };
        cell = next;
    }
    Snail {
        base,
        tiles,
        primes: prime.iter().filter(|&&p| p).count(),
        levels,
        area,
        low,
        high,
    }
}
