use mrly_automaton::memory::{Rule, SWEEPS, TOLERANCE};
use mrly_error::{Result, overflow_error, value_error};
use serde::{Deserialize, Serialize};

fn power(base: u8, digits: u32) -> Result<u64> {
    if base < 2 {
        return value_error(format!("base {base} is under two."));
    }
    match u64::from(base).checked_pow(digits) {
        Some(count) => Ok(count),
        None => overflow_error(format!("{base}^{digits} runs past a u64.")),
    }
}

fn digits(dimension: u32, level: u32) -> Result<u32> {
    match dimension.checked_mul(level) {
        Some(count) => Ok(count),
        None => overflow_error(format!(
            "dimension {dimension} at level {level} reads more digits than a u32 counts."
        )),
    }
}

pub fn cells(base: u8, dimension: u8, level: u32) -> Result<u64> {
    power(base, digits(u32::from(dimension), level)?)
}

pub fn cell(point: &[f64], base: u8, level: u32) -> Result<u64> {
    let side = power(base, level)?;
    power(base, digits(point.len() as u32, level)?)?;
    let top = (side - 1) as f64;
    let mut index = 0u64;
    let mut weight = 1u64;
    for &x in point {
        let seat = (x * side as f64).floor().clamp(0.0, top) as u64;
        index += seat * weight;
        weight = weight.wrapping_mul(side);
    }
    Ok(index)
}

pub fn word_cell(word: &[u8], base: u8, dimension: u8) -> Result<u64> {
    if dimension < 1 {
        return value_error("dimension must be at least one.");
    }
    let corners = u64::from(base).saturating_pow(u32::from(dimension));
    for &c in word {
        if u64::from(c) >= corners || base < 2 {
            return value_error(format!(
                "corner {c} is out of range for base {base} at dimension {dimension}."
            ));
        }
    }
    let level = word.len() as u32;
    let side = power(base, level)?;
    power(base, digits(u32::from(dimension), level)?)?;
    let b = u64::from(base);
    let mut index = 0u64;
    let mut weight = 1u64;
    for axis in 0..u32::from(dimension) {
        let place = b.pow(axis);
        let coordinate = word
            .iter()
            .fold(0u64, |run, &c| run * b + (u64::from(c) / place) % b);
        index += coordinate * weight;
        weight = weight.wrapping_mul(side);
    }
    Ok(index)
}

pub const LAP: u64 = 1 << 24;

fn lyndon(t: usize, p: usize, k: usize, m: usize, word: &mut [usize], out: &mut Vec<u8>) {
    if t > k {
        if k.is_multiple_of(p) {
            out.extend(word[1..=p].iter().map(|&c| c as u8));
        }
        return;
    }
    word[t] = word[t - p];
    lyndon(t + 1, p, k, m, word, out);
    for c in word[t - p] + 1..m {
        word[t] = c;
        lyndon(t + 1, t, k, m, word, out);
    }
}

pub fn de_bruijn(m: u32, k: u32) -> Result<Vec<u8>> {
    if m == 0 || k == 0 {
        return value_error("a de Bruijn cycle needs at least one letter and order one.");
    }
    if m > 256 {
        return value_error(format!("{m} letters do not fit a byte."));
    }
    match u64::from(m).checked_pow(k) {
        Some(length) if length <= LAP => {}
        _ => {
            return overflow_error(format!("{m}^{k} letters run past the lap of {LAP}."));
        }
    }
    let mut word = vec![0usize; k as usize + 1];
    let mut out = Vec::new();
    lyndon(1, 1, k as usize, m as usize, &mut word, &mut out);
    Ok(out)
}

pub fn reverse(rule: &Rule) -> Rule {
    let mask = rule.letters() - 1;
    let mut code = 0u64;
    for w in 0..rule.windows() {
        if !rule.allowed(w) {
            continue;
        }
        let mut turned = 0usize;
        let mut rest = w;
        for _ in 0..rule.width {
            turned = (turned << rule.dimension) | (rest & mask);
            rest >>= rule.dimension;
        }
        code |= 1 << turned;
    }
    Rule {
        dimension: rule.dimension,
        width: rule.width,
        code,
    }
}

pub fn successors(rule: &Rule, history: &[u8]) -> Vec<u8> {
    let letters = rule.letters();
    let tail_len = history.len().min(rule.width - 1);
    let tail = &history[history.len() - tail_len..];
    if tail.iter().any(|&c| usize::from(c) >= letters) {
        return Vec::new();
    }
    let suffix = tail
        .iter()
        .fold(0usize, |run, &c| (run << rule.dimension) | usize::from(c));
    let free = rule.width - 1 - tail_len;
    let heads = 1usize << (free * rule.dimension);
    let shift = (tail_len + 1) * rule.dimension;
    (0..letters)
        .filter(|&c| {
            let end = (suffix << rule.dimension) | c;
            (0..heads).any(|head| rule.allowed((head << shift) | end))
        })
        .map(|c| c as u8)
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamedGame {
    pub name: String,
    pub dimension: usize,
    pub width: usize,
    pub code: u128,
}

const CYCLE: [u8; 4] = [0, 1, 3, 2];

fn ahead(corner: u8, places: usize) -> u8 {
    let at = CYCLE.iter().position(|&c| c == corner).unwrap_or(0);
    CYCLE[(at + places) % 4]
}

fn neighbours(a: u8, b: u8) -> bool {
    b == ahead(a, 1) || b == ahead(a, 3)
}

fn square(width: usize, keep: impl Fn(&[u8]) -> bool) -> u64 {
    let mut code = 0u64;
    for w in 0..1usize << (2 * width) {
        let window: Vec<u8> = (0..width)
            .rev()
            .map(|j| ((w >> (2 * j)) & 3) as u8)
            .collect();
        if keep(&window) {
            code |= 1 << w;
        }
    }
    code
}

pub fn named_games() -> Vec<NamedGame> {
    let games: [(&str, usize, u64); 8] = [
        ("no repeat", 2, square(2, |w| w[1] != w[0])),
        (
            "not one place anticlockwise",
            2,
            square(2, |w| w[1] != ahead(w[0], 1)),
        ),
        (
            "not the opposite corner",
            2,
            square(2, |w| w[1] != ahead(w[0], 2)),
        ),
        ("only a neighbour", 2, square(2, |w| neighbours(w[0], w[1]))),
        (
            "not a neighbour",
            2,
            square(2, |w| w[1] == w[0] || w[1] == ahead(w[0], 2)),
        ),
        ("the shift rule", 2, square(2, |w| w[1] >> 1 == w[0] & 1)),
        (
            "only anticlockwise",
            2,
            square(2, |w| w[1] == ahead(w[0], 1)),
        ),
        (
            "no neighbour after a repeat",
            3,
            square(3, |w| !(w[0] == w[1] && neighbours(w[1], w[2]))),
        ),
    ];
    games
        .iter()
        .map(|&(name, width, code)| NamedGame {
            name: name.to_string(),
            dimension: 2,
            width,
            code: code as u128,
        })
        .collect()
}

fn order(matrix: &[Vec<u64>]) -> Result<usize> {
    let n = matrix.len();
    if matrix.iter().any(|row| row.len() != n) {
        return value_error(format!("the matrix of {n} rows is not square."));
    }
    Ok(n)
}

fn reachability(matrix: &[Vec<u64>]) -> Vec<Vec<bool>> {
    let n = matrix.len();
    let mut reach: Vec<Vec<bool>> = matrix
        .iter()
        .map(|row| row.iter().map(|&entry| entry > 0).collect())
        .collect();
    for t in 0..n {
        for s in 0..n {
            if !reach[s][t] {
                continue;
            }
            let via = reach[t].clone();
            for (seat, &on) in reach[s].iter_mut().zip(&via) {
                *seat |= on;
            }
        }
    }
    reach
}

fn component_root(block: &[Vec<f64>]) -> f64 {
    let n = block.len();
    let mut vector = vec![1.0 / n as f64; n];
    let mut settled = 0usize;
    for _ in 0..SWEEPS {
        let mut next: Vec<f64> = block
            .iter()
            .zip(&vector)
            .map(|(row, &own)| {
                own + row
                    .iter()
                    .zip(&vector)
                    .map(|(entry, x)| entry * x)
                    .sum::<f64>()
            })
            .collect();
        let mass: f64 = next.iter().sum();
        for seat in next.iter_mut() {
            *seat /= mass;
        }
        let moved: f64 = next.iter().zip(&vector).map(|(a, b)| (a - b).abs()).sum();
        vector = next;
        if moved <= TOLERANCE {
            settled += 1;
            if settled >= 3 {
                break;
            }
        } else {
            settled = 0;
        }
    }
    let mut low = f64::INFINITY;
    let mut high = 0.0f64;
    for (row, &own) in block.iter().zip(&vector) {
        let image: f64 = row.iter().zip(&vector).map(|(entry, x)| entry * x).sum();
        let ratio = image / own;
        low = low.min(ratio);
        high = high.max(ratio);
    }
    0.5 * (low + high)
}

pub fn perron(matrix: &[Vec<u64>]) -> Result<f64> {
    let n = order(matrix)?;
    let reach = reachability(matrix);
    let mut seen = vec![false; n];
    let mut best = 0.0f64;
    for s in 0..n {
        if seen[s] || !reach[s][s] {
            continue;
        }
        let part: Vec<usize> = (0..n)
            .filter(|&t| t == s || (reach[s][t] && reach[t][s]))
            .collect();
        for &t in &part {
            seen[t] = true;
        }
        let block: Vec<Vec<f64>> = part
            .iter()
            .map(|&i| part.iter().map(|&j| matrix[i][j] as f64).collect())
            .collect();
        best = best.max(component_root(&block));
    }
    Ok(best)
}

pub fn counts(matrix: &[Vec<u64>], start: usize, levels: usize) -> Result<Vec<u64>> {
    let n = order(matrix)?;
    if start >= n {
        return value_error(format!("state {start} is not one of the {n} states."));
    }
    let mut vector = vec![1u64; n];
    let mut out = Vec::with_capacity(levels);
    for _ in 0..levels {
        let next: Option<Vec<u64>> = matrix
            .iter()
            .map(|row| {
                row.iter().zip(&vector).try_fold(0u64, |run, (&entry, &x)| {
                    run.checked_add(entry.checked_mul(x)?)
                })
            })
            .collect();
        match next {
            Some(next) => vector = next,
            None => break,
        }
        out.push(vector[start]);
    }
    Ok(out)
}

fn exponent(rho: f64, base: u8) -> f64 {
    if rho <= 0.0 {
        return f64::NEG_INFINITY;
    }
    rho.log(f64::from(base))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mosaic {
    pub base: u8,
    pub dimension: u8,
    pub masks: Vec<Vec<(u8, u8)>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicGame {
    pub matrix: Vec<Vec<u64>>,
    pub rho: f64,
    pub dimension: f64,
    pub edges: Vec<(u8, u8, u8)>,
}

pub fn mosaic_game(mosaic: &Mosaic) -> Result<MosaicGame> {
    if mosaic.base < 2 {
        return value_error(format!("base {} is under two.", mosaic.base));
    }
    if mosaic.dimension < 1 {
        return value_error("dimension must be at least one.");
    }
    let slots = mosaic.masks.len();
    if slots == 0 || slots > 256 {
        return value_error(format!("{slots} masks are not between one and 256."));
    }
    let corners = u64::from(mosaic.base).saturating_pow(u32::from(mosaic.dimension));
    let mut matrix = vec![vec![0u64; slots]; slots];
    let mut edges = Vec::new();
    for (u, mask) in mosaic.masks.iter().enumerate() {
        for (i, &(digit, slot)) in mask.iter().enumerate() {
            if u64::from(digit) >= corners {
                return value_error(format!(
                    "corner {digit} of mask {u} is out of range for base {} at dimension {}.",
                    mosaic.base, mosaic.dimension
                ));
            }
            if usize::from(slot) >= slots {
                return value_error(format!(
                    "corner {digit} of mask {u} sends to slot {slot}, past the {slots} masks."
                ));
            }
            if mask[..i].iter().any(|&(d, _)| d == digit) {
                return value_error(format!("corner {digit} repeats in mask {u}."));
            }
            matrix[u][usize::from(slot)] += 1;
            edges.push((u as u8, digit, slot));
        }
    }
    let rho = perron(&matrix)?;
    Ok(MosaicGame {
        matrix,
        rho,
        dimension: exponent(rho, mosaic.base),
        edges,
    })
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SliceGame {
    pub states: Vec<(i64, i64)>,
    pub start: usize,
    pub edges: Vec<(usize, u8, usize)>,
    pub matrix: Vec<Vec<u64>>,
    pub rho: f64,
    pub dimension: f64,
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a.max(1) } else { gcd(b, a % b) }
}

fn empty_slice() -> SliceGame {
    SliceGame {
        states: Vec::new(),
        start: 0,
        edges: Vec::new(),
        matrix: Vec::new(),
        rho: 0.0,
        dimension: f64::NEG_INFINITY,
    }
}

pub fn slice_game(base: u8, digits: &[u8], p: i64, q: i64) -> Result<SliceGame> {
    if base < 2 {
        return value_error(format!("base {base} is under two."));
    }
    let corners = u64::from(base) * u64::from(base);
    for (i, &d) in digits.iter().enumerate() {
        if u64::from(d) >= corners {
            return value_error(format!("corner {d} is out of range for base {base}."));
        }
        if digits[..i].contains(&d) {
            return value_error(format!("corner {d} repeats."));
        }
    }
    if q == 0 {
        return value_error("the height p / q needs a non-zero q.");
    }
    let (Some(p), Some(q)) = (
        if q < 0 { p.checked_neg() } else { Some(p) },
        if q < 0 { q.checked_neg() } else { Some(q) },
    ) else {
        return overflow_error("the height overruns an i64.");
    };
    let g = gcd(p.unsigned_abs(), q.unsigned_abs()) as i64;
    let (p, q) = (p / g, q / g);
    if digits.is_empty() {
        return Ok(empty_slice());
    }
    let b = i64::from(base);
    let weight = |d: u8| i64::from(d % base) + i64::from(d / base);
    let lo = digits.iter().map(|&d| weight(d)).min().unwrap_or(0);
    let hi = digits.iter().map(|&d| weight(d)).max().unwrap_or(0);
    let (Some(low), Some(high)) = (lo.checked_mul(q), hi.checked_mul(q)) else {
        return overflow_error("the hull of the cut overruns an i64.");
    };
    let mut states = vec![p];
    let mut edges: Vec<(i64, u8, i64)> = Vec::new();
    let mut todo = vec![p];
    while let Some(h) = todo.pop() {
        for &d in digits {
            let next = b
                .checked_mul(h)
                .and_then(|v| v.checked_sub(weight(d).checked_mul(q)?));
            let scaled = next.and_then(|v| (b - 1).checked_mul(v));
            let (Some(next), Some(scaled)) = (next, scaled) else {
                return overflow_error("a carry height overruns an i64.");
            };
            if !(low..=high).contains(&scaled) {
                continue;
            }
            edges.push((h, d, next));
            if !states.contains(&next) {
                states.push(next);
                todo.push(next);
            }
        }
    }
    let mut live = states;
    loop {
        let next: Vec<i64> = live
            .iter()
            .copied()
            .filter(|&h| edges.iter().any(|&(a, _, c)| a == h && live.contains(&c)))
            .collect();
        if next.len() == live.len() {
            break;
        }
        live = next;
    }
    if !live.contains(&p) {
        return Ok(empty_slice());
    }
    live.sort_unstable();
    let index = |h: i64| live.iter().position(|&x| x == h);
    let mut matrix = vec![vec![0u64; live.len()]; live.len()];
    let mut out = Vec::new();
    for &(a, d, c) in &edges {
        let (Some(s), Some(t)) = (index(a), index(c)) else {
            continue;
        };
        matrix[s][t] += 1;
        out.push((s, d, t));
    }
    let rho = perron(&matrix)?;
    Ok(SliceGame {
        states: live
            .iter()
            .map(|&h| {
                let g = gcd(h.unsigned_abs(), q.unsigned_abs()) as i64;
                (h / g, q / g)
            })
            .collect(),
        start: index(p).unwrap_or(0),
        edges: out,
        matrix,
        rho,
        dimension: exponent(rho, base),
    })
}
