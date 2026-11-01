//! The chaos game of a design: the shift register that writes its digits backwards, the de Bruijn schedules that paint every cell once, the restricted games that draw the memory dial's sets, and the games on a graph that draw a mosaic or a slice.
//!
//! A design at base `b` has one map per digit, `phi_d(x) = (x + d) / b`, and the game plays `x_n = phi_(d_n)(x_(n-1))` from a start in `[0, 1)^D`.
//! Each draw becomes the new leading digit, so for `n >= k` the level-`k` cell of `x_n` is the word of the last `k` draws read newest first.
//! A corner is the integer `c = sum_i d_i b^i`, coordinate `0` its least significant digit, the `c = x + b y` of the plane designs, and a cell index is row major, `y side + x` at `side = b^k`, as [`crate::memory::cells`] reads it.
//! A restricted game is a rule of [`crate::memory`] on the draws in time order, the oldest draw the most significant digit of the window, and the set it draws is the dial's set of the reversed rule.
//! A game on a graph keeps a state, draws an edge into it, plays the edge's digit and moves to the edge's source: the mosaic game on its slots, the slice game on its carry heights.

use crate::memory::{Rule, SWEEPS, TOLERANCE};
use mrly_core::error::{Result, overflow_error, value_error};
use serde::{Deserialize, Serialize};

// THE SHIFT REGISTER

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

/// Returns the count of level-`k` cells of the cube, `b^(D k)`.
///
/// ```
/// assert_eq!(mrly_math::game::cells(2, 2, 3).unwrap(), 64);
/// ```
///
/// # Errors
///
/// Errs under base two, and when the count runs past a `u64`.
pub fn cells(base: u8, dimension: u8, level: u32) -> Result<u64> {
    power(base, digits(u32::from(dimension), level)?)
}

/// Returns the index of the level-`k` cell holding a point of `[0, 1)^D`: `floor(x b^k) + floor(y b^k) b^k + ...`, row major with coordinate `0` least significant, the dimension read off the point.
///
/// At level one the index is the corner the point sits in. A coordinate outside the cube is clamped to its nearest cell.
///
/// ```
/// assert_eq!(mrly_math::game::cell(&[0.3, 0.6], 2, 1).unwrap(), 2);
/// assert_eq!(mrly_math::game::cell(&[0.3, 0.6], 2, 2).unwrap(), 9);
/// ```
///
/// # Errors
///
/// Errs under base two, and when the cell count runs past a `u64`.
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

/// Returns the cell of a word of corners read coarsest digit first, the cell the game lands in after playing the word's digits last letter first.
///
/// ```
/// assert_eq!(mrly_math::game::word_cell(&[1, 2], 2, 2).unwrap(), 6);
/// ```
///
/// # Errors
///
/// Errs under base two or dimension one, at a corner past `b^D`, and when the cell count runs past a `u64`.
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

// PERFECT PAINTING

/// The longest lap a de Bruijn cycle may run, in letters.
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

/// Returns a de Bruijn cycle of order `k` over `m` letters: `m^k` letters in which every word of length `k` occurs exactly once as a cyclic window, so played as the draw schedule of an `m`-digit design it paints every level-`k` cell once per lap, and from a cold start once in `m^k + k - 1` draws.
///
/// The cycle concatenates the Lyndon words over `{0, ..., m - 1}` whose length divides `k`, in lexicographic order, the construction of Fredricksen, Kessler and Maiorana.
///
/// ```
/// assert_eq!(mrly_math::game::de_bruijn(2, 3).unwrap(), vec![0, 0, 0, 1, 0, 1, 1, 1]);
/// ```
///
/// # Errors
///
/// Errs when `m` or `k` is zero, when `m` passes the `256` letters a byte holds, and when `m^k` passes [`LAP`].
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

// RESTRICTED GAMES

/// Returns the rule whose windows are the rule's windows read backwards.
///
/// A restricted game allows a window of draws in time order, oldest first, and the digits of its point are the draws newest first, so the set a game draws is the dial's set of the reversed rule: [`crate::memory::cells`] of the reverse are the game's cells, and the two share their Perron root, their transfer matrices being transposes.
///
/// ```
/// use mrly_math::memory::Rule;
/// let anticlockwise = Rule::new(2, 2, 16770).unwrap();
/// assert_eq!(mrly_math::game::reverse(&anticlockwise).code, 10260);
/// assert_eq!(mrly_math::game::reverse(&mrly_math::game::reverse(&anticlockwise)), anticlockwise);
/// ```
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

/// Returns the letters a restricted game may draw next: the corners `c` for which some allowed window, read in time order, ends in the last `k - 1` draws of the history followed by `c`.
///
/// A history shorter than `k - 1` fixes only the end of the window, so the letters consistent with some allowed window are returned, and the empty history at width one reads the rule's alphabet.
/// A history letter past the rule's corners matches no window.
///
/// ```
/// use mrly_math::memory::Rule;
/// let no_repeat = Rule::new(2, 2, 31710).unwrap();
/// assert_eq!(mrly_math::game::successors(&no_repeat, &[0]), vec![1, 2, 3]);
/// ```
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

/// A restricted game of the base-2 square with a name: a rule of [`crate::memory`] in time order, oldest draw most significant.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NamedGame {
    /// The name the game is known by.
    pub name: String,
    /// The dimension of the rule, two on the square.
    pub dimension: usize,
    /// The window width `k`, one more than the memory.
    pub width: usize,
    /// The window code, bit `w` set when the time-ordered window `w` is allowed; a `u128` so a width-3 code crosses the bridge as a decimal string.
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

/// Returns the named games of the square, corner `c = x + 2 y`, cyclic order `0, 1, 3, 2` anticlockwise with `y` up: the seven one-step games and the width-3 game that forbids a neighbour after a repeat.
///
/// ```
/// let games = mrly_math::game::named_games();
/// assert_eq!((games[0].name.as_str(), games[0].code), ("no repeat", 31710));
/// ```
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

// GAMES ON A GRAPH

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

/// Returns the Perron root of a square nonnegative integer matrix, the growth per level of the paths its graph counts.
///
/// The graph is split into its strongly connected components; a component carrying no cycle contributes nothing, and a matrix with no cycle reads exactly `0`.
/// On each cyclic component `I + B` is primitive, so the power iteration on it converges to the Perron vector, and the Collatz-Wielandt ratios of that vector bracket the root; the answer is the largest component root.
///
/// ```
/// let golden = vec![vec![1, 1], vec![1, 0]];
/// assert!((mrly_math::game::perron(&golden).unwrap() - 1.618_033_988_749_895).abs() < 1e-12);
/// ```
///
/// # Errors
///
/// Errs when the matrix is not square.
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

/// Returns the path counts from a state, `e_start M^L 1` for `L = 1 ..= levels`: the filled cells of a mosaic slot or of a slice at each level, and stops early on the level whose count overruns a `u64`.
///
/// ```
/// let golden = vec![vec![1, 1], vec![1, 0]];
/// assert_eq!(mrly_math::game::counts(&golden, 0, 5).unwrap(), vec![2, 3, 5, 8, 13]);
/// ```
///
/// # Errors
///
/// Errs when the matrix is not square, and when the start is not one of its states.
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

/// A mosaic: one mask per palette slot, each filled corner of a slot's mask naming the slot it sends to.
///
/// Slot `t` at level `L + 1` is slot `t`'s mask with every filled corner `(d, u)` replaced by slot `u` at level `L`, so the filled cells of slot `t` at level `L` count `e_t S^L 1` with `S` the substitution matrix.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mosaic {
    /// The base `b`, the digits per axis.
    pub base: u8,
    /// The dimension `D` of the cube.
    pub dimension: u8,
    /// The masks, one per slot: the filled corners `c = sum_i d_i b^i` of the slot, each with the slot it sends to.
    pub masks: Vec<Vec<(u8, u8)>>,
}

/// The chaos game of a mosaic: the graph-directed game on its slots.
///
/// The game keeps a slot `v`, draws an edge `(u, d, v)` into it, plays `x -> (x + d) / b` and moves to slot `u`; its limit set from slot `v` is the slot-`v` set of the mosaic.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct MosaicGame {
    /// The substitution matrix: entry `(u, v)` counts the corners of mask `u` sending to slot `v`.
    pub matrix: Vec<Vec<u64>>,
    /// The Perron root of the substitution matrix.
    pub rho: f64,
    /// The Hausdorff dimension `log_b rho` on a strongly connected slot graph, `-inf` when nothing grows.
    pub dimension: f64,
    /// The edges `(from slot, corner, to slot)`, one per filled corner of every mask.
    pub edges: Vec<(u8, u8, u8)>,
}

/// Reads the game of a mosaic: its substitution matrix, Perron root, dimension and edges.
///
/// ```
/// let mosaic = mrly_math::game::Mosaic {
///     base: 2,
///     dimension: 2,
///     masks: vec![vec![(0, 0), (1, 1), (2, 0)], vec![(0, 1), (3, 0)]],
/// };
/// let game = mrly_math::game::mosaic_game(&mosaic).unwrap();
/// assert_eq!(game.matrix, vec![vec![2, 1], vec![1, 1]]);
/// assert_eq!(format!("{:.6}", game.dimension), "1.388484");
/// ```
///
/// # Errors
///
/// Errs under base two or dimension one, with no mask or more than `256`, at a corner past `b^D` or repeated in its mask, and at a slot past the masks.
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

/// The chaos game of a plane design cut by the line `x + y = p / q`: the graph-directed game on its carry heights.
///
/// A corner `d` carries the weight `w(d) = d_x + d_y`, and the slice of the set at height `h` is the union over the corners of `phi_d` of the slice at height `b h - w(d)`, so the heights reachable from `p / q` inside the hull `[min w, max w] / (b - 1)` are the states, finitely many at a rational height, and the heights with no way on are dropped.
/// The game keeps a height, draws an edge into it, plays the edge's corner and moves to the edge's source height; its limit set from the start is the slice.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SliceGame {
    /// The live heights as reduced fractions `(numerator, denominator)`, in increasing order.
    pub states: Vec<(i64, i64)>,
    /// The index of the start height `p / q` in the states, `0` on an empty game.
    pub start: usize,
    /// The edges `(from state, corner, to state)`, one per corner that carries a live height to a live height.
    pub edges: Vec<(usize, u8, usize)>,
    /// The carry matrix: entry `(s, t)` counts the corners carrying state `s` to state `t`.
    pub matrix: Vec<Vec<u64>>,
    /// The Perron root of the carry matrix.
    pub rho: f64,
    /// The Hausdorff dimension `log_b rho` of the slice, `-inf` when it is empty.
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

/// Reads the game of a plane design, corners `c = x + b y`, cut by the line `x + y = p / q`: its carry states, edges, matrix, Perron root and dimension.
///
/// ```
/// let game = mrly_math::game::slice_game(2, &[0, 1, 3], 2, 3).unwrap();
/// assert_eq!(game.states, vec![(1, 3), (2, 3), (4, 3), (5, 3)]);
/// assert_eq!(format!("{:.6}", game.dimension), "0.694242");
/// ```
///
/// # Errors
///
/// Errs under base two, at a corner past `b^2` or repeated, at a zero `q`, and when a height overruns an `i64`.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory;
    use crate::round_trip;
    use std::collections::BTreeSet;

    const PHI: f64 = 1.618_033_988_749_895;

    fn corner_vector(c: u8, base: u8, dimension: u8) -> Vec<u8> {
        (0..dimension)
            .map(|i| (c / base.pow(u32::from(i))) % base)
            .collect()
    }

    fn play(start: &[f64], draws: &[u8], base: u8) -> Vec<f64> {
        let mut point = start.to_vec();
        for &d in draws {
            let digits = corner_vector(d, base, point.len() as u8);
            for (x, &digit) in point.iter_mut().zip(&digits) {
                *x = (*x + f64::from(digit)) / f64::from(base);
            }
        }
        point
    }

    #[test]
    fn the_cell_of_a_point_is_the_reversed_last_draws() {
        let starts = [0.37, 0.21, 0.13];
        for (base, dimension) in [(2u8, 1u8), (2, 2), (3, 2), (2, 3)] {
            let corners = base.pow(u32::from(dimension));
            let start = &starts[..usize::from(dimension)];
            let mut words: Vec<Vec<u8>> = vec![Vec::new()];
            for _ in 0..4 {
                words = words
                    .iter()
                    .flat_map(|w| {
                        (0..corners).map(move |c| {
                            let mut next = w.clone();
                            next.push(c);
                            next
                        })
                    })
                    .collect();
                for draws in &words {
                    let point = play(start, draws, base);
                    for k in 1..=draws.len() {
                        let reversed: Vec<u8> =
                            draws[draws.len() - k..].iter().rev().copied().collect();
                        assert_eq!(
                            cell(&point, base, k as u32).unwrap(),
                            word_cell(&reversed, base, dimension).unwrap(),
                            "base {base} dim {dimension} draws {draws:?} level {k}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn word_cells_are_the_memory_dials_cells() {
        for code in [7u64, 11, 13, 14, 9] {
            let rule = Rule::new(2, 1, code).unwrap();
            let corners: Vec<u8> = (0..4u8).filter(|&c| (code >> c) & 1 == 1).collect();
            for level in 1..=4u32 {
                let mut words: Vec<Vec<u8>> = vec![Vec::new()];
                for _ in 0..level {
                    words = words
                        .iter()
                        .flat_map(|w| {
                            corners.iter().map(move |&c| {
                                let mut next = w.clone();
                                next.push(c);
                                next
                            })
                        })
                        .collect();
                }
                let mut mine: Vec<u64> =
                    words.iter().map(|w| word_cell(w, 2, 2).unwrap()).collect();
                mine.sort_unstable();
                let mut theirs = memory::cells(&rule, level as usize);
                theirs.sort_unstable();
                assert_eq!(mine, theirs, "code {code} level {level}");
            }
        }
    }

    #[test]
    fn cells_count_the_grid_and_refuse_an_overrun() {
        assert_eq!(cells(2, 2, 3).unwrap(), 64);
        assert_eq!(cells(3, 2, 2).unwrap(), 81);
        assert_eq!(cells(2, 3, 21).unwrap(), 1 << 63);
        assert!(cells(2, 3, 22).is_err());
        assert!(cells(1, 2, 1).is_err());
        assert!(cell(&[0.5, 0.5], 2, 32).is_err());
        assert_eq!(cell(&[-1.0, 2.0], 2, 2).unwrap(), 12);
        assert!(word_cell(&[4], 2, 2).is_err());
        assert!(word_cell(&[0], 2, 0).is_err());
    }

    #[test]
    fn de_bruijn_cycles_hold_every_window_once() {
        for (m, k) in [
            (2u32, 1u32),
            (2, 2),
            (2, 3),
            (2, 4),
            (3, 1),
            (3, 2),
            (3, 3),
            (4, 2),
            (5, 2),
        ] {
            let cycle = de_bruijn(m, k).unwrap();
            let length = m.pow(k) as usize;
            assert_eq!(cycle.len(), length, "m {m} k {k}");
            assert!(cycle.iter().all(|&c| u32::from(c) < m));
            let windows: BTreeSet<Vec<u8>> = (0..length)
                .map(|i| (0..k as usize).map(|j| cycle[(i + j) % length]).collect())
                .collect();
            assert_eq!(windows.len(), length, "m {m} k {k}");
        }
    }

    #[test]
    fn de_bruijn_refuses_a_cycle_out_of_range() {
        assert!(de_bruijn(0, 2).is_err());
        assert!(de_bruijn(2, 0).is_err());
        assert!(de_bruijn(257, 1).is_err());
        assert!(de_bruijn(2, 25).is_err());
        assert_eq!(de_bruijn(256, 1).unwrap().len(), 256);
    }

    #[test]
    fn the_named_games_read_the_census() {
        let games = named_games();
        let codes: Vec<u64> = games.iter().map(|g| g.code as u64).collect();
        assert_eq!(
            codes[..7],
            [31710, 48765, 60855, 27030, 38505, 50115, 16770]
        );
        let want = [
            (12, 3.0),
            (12, 3.0),
            (12, 3.0),
            (8, 2.0),
            (8, 2.0),
            (8, 2.0),
            (4, 1.0),
            (56, 2.0 + 2f64.sqrt()),
        ];
        for (game, (windows, rho)) in games.iter().zip(want) {
            let rule = Rule::new(game.dimension, game.width, game.code as u64).unwrap();
            assert_eq!(memory::allowed_windows(&rule), windows, "{}", game.name);
            let root = memory::perron(&reverse(&rule));
            assert!((root - rho).abs() < 1e-12, "{} reads {root}", game.name);
            assert!((memory::perron(&rule) - rho).abs() < 1e-12, "{}", game.name);
        }
    }

    #[test]
    fn successors_keep_the_newest_window_allowed() {
        let games = named_games();
        let rule = |i: usize| Rule::new(2, games[i].width, games[i].code as u64).unwrap();
        for a in 0..4u8 {
            let others: Vec<u8> = (0..4u8).filter(|&b| b != a).collect();
            assert_eq!(successors(&rule(0), &[3, a]), others);
            assert_eq!(successors(&rule(6), &[a]), vec![ahead(a, 1)]);
            let shifted: Vec<u8> = (0..4u8).filter(|&b| b >> 1 == a & 1).collect();
            assert_eq!(successors(&rule(5), &[a]), shifted);
            let mut after_repeat = vec![a, ahead(a, 2)];
            after_repeat.sort_unstable();
            assert_eq!(successors(&rule(7), &[a, a]), after_repeat);
            assert_eq!(successors(&rule(7), &[ahead(a, 1), a]), vec![0, 1, 2, 3]);
            assert_eq!(successors(&rule(7), &[a]), vec![0, 1, 2, 3]);
        }
        assert_eq!(successors(&rule(7), &[]), vec![0, 1, 2, 3]);
        assert_eq!(successors(&Rule::new(2, 1, 7).unwrap(), &[]), vec![0, 1, 2]);
        assert!(successors(&rule(0), &[4]).is_empty());
    }

    #[test]
    fn the_reversed_shift_rule_draws_the_doubling_graph() {
        let shift = Rule::new(2, 2, 50115).unwrap();
        let dial = reverse(&shift);
        assert_eq!(reverse(&dial), shift);
        for level in 1..=5usize {
            let side = 1u64 << level;
            let mut want: Vec<u64> = (0..side)
                .flat_map(|x| {
                    let y = (2 * x) % side;
                    [y * side + x, (y + 1) * side + x]
                })
                .collect();
            want.sort_unstable();
            let mut drawn = memory::cells(&dial, level);
            drawn.sort_unstable();
            assert_eq!(drawn, want, "level {level}");
        }
    }

    #[test]
    fn perron_reads_the_largest_component_root() {
        assert!((perron(&[vec![0, 1], vec![2, 0]]).unwrap() - 2f64.sqrt()).abs() < 1e-12);
        assert!((perron(&[vec![1, 0], vec![1, 2]]).unwrap() - 2.0).abs() < 1e-12);
        assert!(perron(&[vec![0, 1], vec![0, 0]]).unwrap().abs() < 1e-12);
        assert!((perron(&[vec![3]]).unwrap() - 3.0).abs() < 1e-12);
        assert_eq!(perron(&[]).unwrap(), 0.0);
        assert!(perron(&[vec![1, 1]]).is_err());
    }

    #[test]
    fn counts_stop_on_an_overrun() {
        let doubling = vec![vec![1u64 << 40]];
        assert_eq!(counts(&doubling, 0, 3).unwrap(), vec![1 << 40]);
        assert!(counts(&doubling, 1, 3).is_err());
        assert_eq!(
            counts(&[vec![0, 1], vec![2, 0]], 1, 4).unwrap(),
            vec![2, 2, 4, 4]
        );
    }

    #[test]
    fn the_two_slot_mosaic_grows_by_the_golden_square() {
        let mosaic = Mosaic {
            base: 2,
            dimension: 2,
            masks: vec![vec![(0, 0), (1, 1), (2, 0)], vec![(0, 1), (3, 0)]],
        };
        let game = mosaic_game(&mosaic).unwrap();
        assert_eq!(game.matrix, vec![vec![2, 1], vec![1, 1]]);
        assert!((game.rho - PHI * PHI).abs() < 1e-9);
        assert_eq!(format!("{:.6}", game.dimension), "1.388484");
        assert_eq!(
            counts(&game.matrix, 0, 8).unwrap(),
            vec![3, 8, 21, 55, 144, 377, 987, 2584]
        );
        assert_eq!(
            game.edges,
            vec![(0, 0, 0), (0, 1, 1), (0, 2, 0), (1, 0, 1), (1, 3, 0)]
        );
    }

    #[test]
    fn mosaic_game_refuses_a_bad_mask() {
        let good = Mosaic {
            base: 2,
            dimension: 2,
            masks: vec![vec![(0, 0), (3, 0)]],
        };
        assert!(mosaic_game(&good).is_ok());
        assert!(
            mosaic_game(&Mosaic {
                base: 1,
                ..good.clone()
            })
            .is_err()
        );
        assert!(
            mosaic_game(&Mosaic {
                dimension: 0,
                ..good.clone()
            })
            .is_err()
        );
        assert!(
            mosaic_game(&Mosaic {
                masks: vec![],
                ..good.clone()
            })
            .is_err()
        );
        assert!(
            mosaic_game(&Mosaic {
                masks: vec![vec![(4, 0)]],
                ..good.clone()
            })
            .is_err()
        );
        assert!(
            mosaic_game(&Mosaic {
                masks: vec![vec![(0, 1)]],
                ..good.clone()
            })
            .is_err()
        );
        assert!(
            mosaic_game(&Mosaic {
                masks: vec![vec![(0, 0), (0, 0)]],
                ..good
            })
            .is_err()
        );
    }

    #[test]
    fn the_gasket_slices_at_two_thirds_carry_the_notes_states() {
        let light = slice_game(2, &[0, 1, 2], 2, 3).unwrap();
        assert_eq!(light.states, vec![(1, 3), (2, 3)]);
        assert_eq!(light.start, 1);
        assert_eq!(light.matrix, vec![vec![0, 1], vec![2, 0]]);
        assert!((light.rho - 2f64.sqrt()).abs() < 1e-12);
        assert!((light.dimension - 0.5).abs() < 1e-12);
        assert_eq!(light.edges, vec![(1, 1, 0), (1, 2, 0), (0, 0, 1)]);
        let heavy = slice_game(2, &[0, 1, 3], 2, 3).unwrap();
        assert_eq!(heavy.states, vec![(1, 3), (2, 3), (4, 3), (5, 3)]);
        assert_eq!(heavy.start, 1);
        assert_eq!(
            heavy.matrix,
            vec![
                vec![0, 1, 0, 0],
                vec![1, 0, 1, 0],
                vec![0, 1, 0, 1],
                vec![0, 0, 1, 0]
            ]
        );
        assert!((heavy.rho - PHI).abs() < 1e-12);
        assert_eq!(format!("{:.6}", heavy.dimension), "0.694242");
        assert_eq!(
            counts(&heavy.matrix, heavy.start, 12).unwrap(),
            vec![2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377]
        );
    }

    #[test]
    fn slice_game_refuses_a_bad_cut_and_empties_outside_the_hull() {
        assert!(slice_game(1, &[0], 1, 2).is_err());
        assert!(slice_game(2, &[4], 1, 2).is_err());
        assert!(slice_game(2, &[1, 1], 1, 2).is_err());
        assert!(slice_game(2, &[0, 1], 1, 0).is_err());
        let outside = slice_game(2, &[0, 1, 2], 3, 2).unwrap();
        assert!(outside.states.is_empty());
        assert_eq!(outside.rho, 0.0);
        let turned = slice_game(2, &[0, 1, 3], -2, -3).unwrap();
        assert_eq!(turned.states, vec![(1, 3), (2, 3), (4, 3), (5, 3)]);
    }

    #[test]
    fn serde_round_trips() {
        round_trip(named_games());
        let mosaic = Mosaic {
            base: 2,
            dimension: 2,
            masks: vec![vec![(0, 0), (1, 1), (2, 0)], vec![(0, 1), (3, 0)]],
        };
        round_trip(mosaic_game(&mosaic).unwrap());
        round_trip(mosaic);
        round_trip(slice_game(2, &[0, 1, 3], 2, 3).unwrap());
    }
}
