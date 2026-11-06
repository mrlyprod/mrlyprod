pub mod nodal;

use mrly_error::{Result, shape_error, value_error};
use serde::{Deserialize, Serialize};
use std::f64::consts::{PI, TAU};

const WAVE_CAP: usize = 64;
const SIZE_CAP: usize = 1024;
const GRAIN_CAP: usize = 400_000;
const STEP_CAP: usize = 64;
const KICK: f64 = 0.6;
const DRIFT: f64 = 0.3;
const FLOOR: f64 = 2e-4;
const FLAT_KICK: f64 = 0.01;
const FLAT: f64 = 1e-6;
const SILENT: f64 = 1e-12;
const TOP: f32 = 1.0 - f32::EPSILON / 2.0;
const TURNS: usize = 1024;

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Rung {
    pub n: usize,
    pub m: usize,
    pub sign: i32,
    pub pitch: f64,
}

pub fn plate(n: usize, m: usize, sign: i32, size: usize) -> Result<Vec<f64>> {
    if sign != 1 && sign != -1 {
        return value_error(format!("the sign {sign} is not 1 or -1."));
    }
    if size == 0 || size > SIZE_CAP {
        return value_error(format!("the size {size} is not 1 to {SIZE_CAP}."));
    }
    if n > WAVE_CAP || m > WAVE_CAP {
        return value_error(format!(
            "the wave numbers {n} and {m} are not both 0 to {WAVE_CAP}."
        ));
    }
    let wave = |k: usize| -> Vec<f64> {
        (0..size)
            .map(|i| (k as f64 * PI * (i as f64 + 0.5) / size as f64).cos())
            .collect()
    };
    let (along, across) = (wave(n), wave(m));
    let mut field = Vec::with_capacity(size * size);
    for row in 0..size {
        for col in 0..size {
            let one = along[col] * across[row];
            field.push(if n == m {
                one
            } else {
                one + f64::from(sign) * across[col] * along[row]
            });
        }
    }
    let top = field.iter().fold(0.0f64, |a, v| a.max(v.abs()));
    if top > SILENT {
        field.iter_mut().for_each(|v| *v /= top);
    } else {
        field.fill(0.0);
    }
    Ok(field)
}

pub fn ladder(cap: usize) -> Result<Vec<Rung>> {
    if cap > WAVE_CAP {
        return value_error(format!("the cap {cap} is over {WAVE_CAP}."));
    }
    let mut rungs = Vec::new();
    for m in 1..=cap {
        for n in 0..=m {
            let signs: &[i32] = if n == m { &[1] } else { &[1, -1] };
            for &sign in signs {
                rungs.push(Rung {
                    n,
                    m,
                    sign,
                    pitch: ((n * n + m * m) as f64).sqrt(),
                });
            }
        }
    }
    rungs.sort_by_key(|r| (r.n * r.n + r.m * r.m, r.n, -r.sign));
    Ok(rungs)
}

struct Board<'a> {
    field: &'a [f64],
    side: usize,
    grid: Vec<f64>,
}

impl<'a> Board<'a> {
    fn new(field: &'a [f64], side: usize) -> Result<Board<'a>> {
        if side == 0 {
            return value_error("the side is 0.");
        }
        if side.checked_mul(side) != Some(field.len()) {
            return shape_error(format!(
                "the field holds {} and its side is {side}.",
                field.len()
            ));
        }
        if field.iter().any(|v| v.is_infinite()) {
            return value_error("the field holds an infinity.");
        }
        let span = side + 2;
        let mut grid = vec![f64::NAN; span * span];
        for row in 0..side {
            let at = (row + 1) * span + 1;
            grid[at..at + side].copy_from_slice(&field[row * side..(row + 1) * side]);
        }
        for _ in 0..2 {
            let known = grid.clone();
            for row in 0..span {
                for col in 0..span {
                    if !known[row * span + col].is_nan() {
                        continue;
                    }
                    let mut near = Vec::with_capacity(4);
                    if row > 0 {
                        near.push(known[(row - 1) * span + col]);
                    }
                    if row + 1 < span {
                        near.push(known[(row + 1) * span + col]);
                    }
                    if col > 0 {
                        near.push(known[row * span + col - 1]);
                    }
                    if col + 1 < span {
                        near.push(known[row * span + col + 1]);
                    }
                    let held: Vec<f64> = near.into_iter().filter(|v| !v.is_nan()).collect();
                    if !held.is_empty() {
                        grid[row * span + col] = held.iter().sum::<f64>() / held.len() as f64;
                    }
                }
            }
        }
        Ok(Board { field, side, grid })
    }

    fn span(&self) -> usize {
        self.side + 2
    }

    fn solid(&self, row: usize, col: usize) -> bool {
        row < self.side && col < self.side && !self.field[row * self.side + col].is_nan()
    }

    fn cell(&self, at: f32) -> usize {
        ((f64::from(at) * self.side as f64) as usize).min(self.side - 1)
    }

    fn holds(&self, x: f32, y: f32) -> bool {
        self.solid(self.cell(y), self.cell(x))
    }

    fn sample(&self, x: f64, y: f64) -> (f64, f64, f64) {
        let side = self.side as f64;
        let span = self.span();
        let (pu, pv) = (x * side + 0.5, y * side + 0.5);
        let col = (pu.floor().max(0.0) as usize).min(self.side);
        let row = (pv.floor().max(0.0) as usize).min(self.side);
        let (u, v) = (pu - col as f64, pv - row as f64);
        let a = self.grid[row * span + col];
        let b = self.grid[row * span + col + 1];
        let c = self.grid[(row + 1) * span + col];
        let d = self.grid[(row + 1) * span + col + 1];
        let value = (1.0 - u) * (1.0 - v) * a + u * (1.0 - v) * b + (1.0 - u) * v * c + u * v * d;
        let dx = ((1.0 - v) * (b - a) + v * (d - c)) * side;
        let dy = ((1.0 - u) * (c - a) + u * (d - b)) * side;
        (value, dx, dy)
    }

    fn steepest(&self) -> f64 {
        let side = self.side;
        let at = |row: usize, col: usize| self.field[row * side + col];
        let slope = |here: f64, next: Option<f64>, back: Option<f64>| match (next, back) {
            (Some(next), _) => next - here,
            (None, Some(back)) => here - back,
            _ => 0.0,
        };
        let mut top = 0.0f64;
        for row in 0..side {
            for col in 0..side {
                if !self.solid(row, col) {
                    continue;
                }
                let here = at(row, col);
                let pick = |r: usize, c: usize| self.solid(r, c).then(|| at(r, c));
                let gx = slope(
                    here,
                    pick(row, col + 1),
                    col.checked_sub(1).and_then(|c| pick(row, c)),
                );
                let gy = slope(
                    here,
                    pick(row + 1, col),
                    row.checked_sub(1).and_then(|r| pick(r, col)),
                );
                top = top.max(gx.hypot(gy));
            }
        }
        top * side as f64
    }

    fn peak(&self) -> f64 {
        self.field
            .iter()
            .filter(|v| !v.is_nan())
            .fold(0.0f64, |a, v| a.max(v.abs()))
    }
}

fn mix(mut z: u64) -> u64 {
    z = z.wrapping_add(0x9e37_79b9_7f4a_7c15);
    z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    z ^ (z >> 31)
}

fn turn(seed: u64, step: u64, grain: u64) -> usize {
    (mix(mix(mix(seed) ^ step) ^ grain) % TURNS as u64) as usize
}

fn fold(at: f64) -> f32 {
    if (0.0..1.0).contains(&at) {
        return (at as f32).min(TOP);
    }
    let t = at.rem_euclid(2.0);
    let t = if t >= 1.0 { 2.0 - t } else { t };
    (t as f32).min(TOP)
}

pub fn shake(
    field: &[f64],
    side: usize,
    grains: &[f32],
    from: u64,
    steps: usize,
    seed: u64,
) -> Result<Vec<f32>> {
    let board = Board::new(field, side)?;
    if !grains.len().is_multiple_of(2) {
        return shape_error(format!("the grains hold {}, an odd count.", grains.len()));
    }
    if grains.len() / 2 > GRAIN_CAP {
        return value_error(format!(
            "the grains hold {} pairs, over {GRAIN_CAP}.",
            grains.len() / 2
        ));
    }
    if steps > STEP_CAP {
        return value_error(format!("the steps {steps} are over {STEP_CAP}."));
    }
    let steep = board.steepest();
    let (kick, pull) = if steep > FLAT * board.peak() * side as f64 {
        (KICK / steep, DRIFT / (steep * steep))
    } else {
        (FLAT_KICK, 0.0)
    };
    let turns: Vec<(f64, f64)> = (0..TURNS)
        .map(|k| (TAU * k as f64 / TURNS as f64).sin_cos())
        .collect();
    let mut out = grains.to_vec();
    for (index, pair) in out.chunks_exact_mut(2).enumerate() {
        if !(pair[0].is_finite() && pair[1].is_finite()) {
            pair.fill(f32::NAN);
            continue;
        }
        let (mut x, mut y) = (fold(f64::from(pair[0])), fold(f64::from(pair[1])));
        let mut fallen = !board.holds(x, y);
        for step in 0..steps {
            if fallen {
                break;
            }
            let (px, py) = (f64::from(x), f64::from(y));
            let (value, dx, dy) = board.sample(px, py);
            let (sin, cos) = turns[turn(seed, from.wrapping_add(step as u64), index as u64)];
            let reach = kick * value.abs() + FLOOR;
            x = fold(px + reach * cos - pull * value * dx);
            y = fold(py + reach * sin - pull * value * dy);
            fallen = !board.holds(x, y);
        }
        if fallen {
            pair.fill(f32::NAN);
        } else {
            pair[0] = x;
            pair[1] = y;
        }
    }
    Ok(out)
}

fn pieces(from: (f64, f64), to: (f64, f64), solid: impl Fn(usize, usize) -> bool) -> Vec<[f64; 4]> {
    let at = |t: f64| (from.0 + t * (to.0 - from.0), from.1 + t * (to.1 - from.1));
    let mut cuts = vec![0.0, 1.0];
    for (p, q) in [(from.0, to.0), (from.1, to.1)] {
        if (p - 0.5) * (q - 0.5) < 0.0 {
            cuts.push((0.5 - p) / (q - p));
        }
    }
    cuts.sort_by(f64::total_cmp);
    let halves = |v: f64| -> &[usize] {
        if v < 0.5 {
            &[0]
        } else if v > 0.5 {
            &[1]
        } else {
            &[0, 1]
        }
    };
    let mut out = Vec::new();
    for span in cuts.windows(2) {
        if span[1] - span[0] <= 1e-12 {
            continue;
        }
        let (mu, mv) = at((span[0] + span[1]) / 2.0);
        let kept = halves(mv)
            .iter()
            .any(|&r| halves(mu).iter().any(|&c| solid(r, c)));
        if kept {
            let ((u0, v0), (u1, v1)) = (at(span[0]), at(span[1]));
            out.push([u0, v0, u1, v1]);
        }
    }
    out
}

pub fn lines(field: &[f64], side: usize) -> Result<Vec<f64>> {
    let board = Board::new(field, side)?;
    let span = board.span();
    let size = side as f64;
    let mut out = Vec::new();
    for row in 0..span - 1 {
        for col in 0..span - 1 {
            let corner = |r: usize, c: usize| board.grid[(row + r) * span + col + c];
            let (a, b, c, d) = (corner(0, 0), corner(0, 1), corner(1, 1), corner(1, 0));
            if a.is_nan() || b.is_nan() || c.is_nan() || d.is_nan() {
                continue;
            }
            let cut = |p: f64, q: f64| p / (p - q);
            let up = |v: f64| v > 0.0;
            let mut crossings = Vec::with_capacity(4);
            if up(a) != up(b) {
                crossings.push((cut(a, b), 0.0));
            }
            if up(b) != up(c) {
                crossings.push((1.0, cut(b, c)));
            }
            if up(d) != up(c) {
                crossings.push((cut(d, c), 1.0));
            }
            if up(a) != up(d) {
                crossings.push((0.0, cut(a, d)));
            }
            let joins = match crossings.len() {
                2 => vec![(crossings[0], crossings[1])],
                4 if up((a + b + c + d) / 4.0) == up(a) => {
                    vec![(crossings[0], crossings[1]), (crossings[2], crossings[3])]
                }
                4 => vec![(crossings[0], crossings[3]), (crossings[1], crossings[2])],
                _ => Vec::new(),
            };
            let solid = |r: usize, c: usize| {
                (row + r)
                    .checked_sub(1)
                    .zip((col + c).checked_sub(1))
                    .is_some_and(|(r, c)| board.solid(r, c))
            };
            let whole = solid(0, 0) && solid(0, 1) && solid(1, 0) && solid(1, 1);
            for (from, to) in joins {
                let kept = if whole {
                    vec![[from.0, from.1, to.0, to.1]]
                } else {
                    pieces(from, to, solid)
                };
                for [u0, v0, u1, v1] in kept {
                    out.extend(
                        [
                            (col as f64 + u0 - 0.5) / size,
                            (row as f64 + v0 - 0.5) / size,
                            (col as f64 + u1 - 0.5) / size,
                            (row as f64 + v1 - 0.5) / size,
                        ]
                        .map(|v| v.clamp(0.0, 1.0)),
                    );
                }
            }
        }
    }
    Ok(out)
}
