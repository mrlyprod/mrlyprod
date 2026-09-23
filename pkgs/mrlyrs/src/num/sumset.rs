use super::design::elements;
use crate::core::error::{value_error, Result};

/// The deepest level a [`Sumset`] is built to: `S meet [0, 3^20]`, a bit array of `436` MB.
pub const DEEPEST: u32 = 20;

/// The widest level a [`Pair`] names in either base, so `3^k`, `4^m` and every difference string fit a signed 64-bit integer.
pub const WIDEST: u32 = 30;

const BLOCK: usize = 8;

// THE SUMSET

/// The sumset `S = A + B` of Erdos problem 125 up to `3^level`: `A` the integers whose base-3 digits are all `0` or `1`, `B` those whose base-4 digits are.
///
/// One bit per integer of `[0, 3^level]`: the members of `A` are set directly and each power `4^j <= 3^level` is folded in by one shift-or pass, `S |= S << 4^j`.
/// A running count every eight words makes `card(S meet [1, x])` one table read and at most eight popcounts.
pub struct Sumset {
    level: u32,
    top: u64,
    words: Vec<u64>,
    runs: Vec<u64>,
}

impl Sumset {
    /// Builds `S meet [0, 3^level]`.
    ///
    /// ```
    /// let s = mrlyrs::num::sumset::Sumset::new(4).unwrap();
    /// assert_eq!(s.count(81), Some(79));
    /// ```
    ///
    /// # Errors
    ///
    /// Errs when the level is zero or past [`DEEPEST`].
    pub fn new(level: u32) -> Result<Sumset> {
        if !(1..=DEEPEST).contains(&level) {
            return value_error(format!("the level runs from 1 to {DEEPEST}, not {level}."));
        }
        let top = 3u64.pow(level);
        let mut words = vec![0u64; (top / 64 + 1) as usize];
        for a in elements(3, &[0, 1], level as usize)
            .into_iter()
            .chain([0, top])
        {
            words[(a / 64) as usize] |= 1 << (a % 64);
        }
        let mut power = 1u64;
        while power <= top {
            shift_or(&mut words, power);
            power *= 4;
        }
        let spare = top % 64;
        if spare != 63 {
            let last = words.len() - 1;
            words[last] &= (1u64 << (spare + 1)) - 1;
        }
        let mut runs = Vec::with_capacity(words.len() / BLOCK + 2);
        let mut total = 0u64;
        for chunk in words.chunks(BLOCK) {
            runs.push(total);
            total += chunk.iter().map(|w| u64::from(w.count_ones())).sum::<u64>();
        }
        runs.push(total);
        Ok(Sumset {
            level,
            top,
            words,
            runs,
        })
    }

    /// The level the array was built to.
    pub fn level(&self) -> u32 {
        self.level
    }

    /// The largest integer the array holds, `3^level`.
    pub fn top(&self) -> u64 {
        self.top
    }

    /// Whether `x` is in `S`, or `None` past the top.
    pub fn contains(&self, x: u64) -> Option<bool> {
        (x <= self.top).then(|| self.words[(x / 64) as usize] >> (x % 64) & 1 == 1)
    }

    /// Counts `card(S meet [1, x])`, or `None` past the top.
    pub fn count(&self, x: u64) -> Option<u64> {
        (x <= self.top).then(|| self.upto(x) - 1)
    }

    /// Reads the density `D(x) = card(S meet [1, x])/x`, or `None` at zero and past the top.
    pub fn density(&self, x: u64) -> Option<f64> {
        (1..=self.top)
            .contains(&x)
            .then(|| (self.upto(x) - 1) as f64 / x as f64)
    }

    /// Reads the share of members in each of `cells` equal runs of the integers `[low, high)`, the strip of `S` a page draws.
    ///
    /// Run `i` is `[low + floor(i n/cells), low + floor((i + 1) n/cells))` with `n = high - low`.
    ///
    /// # Errors
    ///
    /// Errs when the window is empty or runs past the top, or when `cells` is zero or more than the integers in the window.
    pub fn fills(&self, low: u64, high: u64, cells: usize) -> Result<Vec<f64>> {
        if low >= high || high > self.top + 1 {
            return value_error(format!(
                "the window [{low}, {high}) must be a nonempty part of [0, {}].",
                self.top
            ));
        }
        let span = high - low;
        if cells == 0 || cells as u64 > span {
            return value_error(format!(
                "a window of {span} integers splits into 1 to {span} cells, not {cells}."
            ));
        }
        let edge = |i: u64| low + (u128::from(i) * u128::from(span) / cells as u128) as u64;
        Ok((0..cells as u64)
            .map(|i| {
                let (a, b) = (edge(i), edge(i + 1));
                (self.below(b) - self.below(a)) as f64 / (b - a) as f64
            })
            .collect())
    }

    /// Reads the least and the greatest `D(x)` over each window `[edges[i], edges[i + 1])`, `None` for an empty window.
    ///
    /// # Errors
    ///
    /// Errs when the edges decrease, start at zero or run past `top + 1`.
    pub fn extremes(&self, edges: &[u64]) -> Result<Vec<Option<(f64, f64)>>> {
        if edges.first() == Some(&0)
            || edges.last().is_some_and(|&e| e > self.top + 1)
            || edges.windows(2).any(|w| w[0] > w[1])
        {
            return value_error(format!(
                "the edges must climb inside [1, {}].",
                self.top + 1
            ));
        }
        Ok(edges
            .windows(2)
            .map(|w| {
                let (a, b) = (w[0], w[1]);
                if a == b {
                    return None;
                }
                let mut members = self.below(a);
                let (mut low, mut high) = (f64::INFINITY, f64::NEG_INFINITY);
                for x in a..b {
                    members += self.words[(x / 64) as usize] >> (x % 64) & 1;
                    let d = (members - 1) as f64 / x as f64;
                    low = low.min(d);
                    high = high.max(d);
                }
                Some((low, high))
            })
            .collect())
    }

    fn upto(&self, x: u64) -> u64 {
        self.below(x + 1)
    }

    fn below(&self, x: u64) -> u64 {
        let word = (x / 64) as usize;
        let block = word / BLOCK;
        let whole: u64 = self.words[block * BLOCK..word]
            .iter()
            .map(|w| u64::from(w.count_ones()))
            .sum();
        let part = match (x % 64, self.words.get(word)) {
            (0, _) | (_, None) => 0,
            (bits, Some(w)) => u64::from((w & ((1u64 << bits) - 1)).count_ones()),
        };
        self.runs[block] + whole + part
    }
}

fn shift_or(words: &mut [u64], shift: u64) {
    let (jump, bits) = ((shift / 64) as usize, (shift % 64) as u32);
    for i in (jump..words.len()).rev() {
        let from = i - jump;
        let mut v = words[from] << bits;
        if bits > 0 && from > 0 {
            v |= words[from - 1] >> (64 - bits);
        }
        words[i] |= v;
    }
}

// THE PAIRS

/// A pair of levels: the base-3 level `A_k = A meet [0, 3^k)` against the base-4 level `B_m = B meet [0, 4^m)`, `k` the field `three` and `m` the field `four`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pair {
    /// The base-3 level `k`.
    pub three: u32,
    /// The base-4 level `m`.
    pub four: u32,
}

impl Pair {
    /// Names the pair `(k, m)`.
    ///
    /// # Errors
    ///
    /// Errs when either level is zero or past [`WIDEST`].
    pub fn new(three: u32, four: u32) -> Result<Pair> {
        if !(1..=WIDEST).contains(&three) || !(1..=WIDEST).contains(&four) {
            return value_error(format!(
                "both levels run from 1 to {WIDEST}, not ({three}, {four})."
            ));
        }
        Ok(Pair { three, four })
    }

    fn powers(&self) -> (u64, u64) {
        (3u64.pow(self.three), 4u64.pow(self.four))
    }

    /// The largest element `d(k, m) = (3^k - 1)/2 + (4^m - 1)/3` of `A_k + B_m`.
    pub fn largest(&self) -> u64 {
        let (p, q) = self.powers();
        (p - 1) / 2 + (q - 1) / 3
    }

    /// The first and the last integer of the open interval `(d(k, m), min(3^k, 4^m))`, which `S` misses, or `None` when it holds none.
    ///
    /// A sum with `a < 3^k` and `b < 4^m` is at most `d(k, m)`, and a sum with `a >= 3^k` or `b >= 4^m` is at least `min(3^k, 4^m)`.
    pub fn gap(&self) -> Option<(u64, u64)> {
        let (p, q) = self.powers();
        let d = self.largest();
        (p.min(q) > d + 1).then(|| (d + 1, p.min(q) - 1))
    }

    /// Whether the pair is clean, `3^k > d(k, m)` and `4^m > d(k, m)`, so that `S meet [0, d] = A_k + B_m`.
    pub fn clean(&self) -> bool {
        let (p, q) = self.powers();
        p.min(q) > self.largest()
    }

    /// Whether the pair is a gap copy, `2 4^m < 3^k + 5`: `A_k + B_m` is then two disjoint translates of `A_(k-1) + B_m` and its energy is twice theirs.
    pub fn copy(&self) -> bool {
        let (p, q) = self.powers();
        2 * q < p + 5
    }

    /// The scaling `tau = 4^m/3^k`.
    pub fn scale(&self) -> f64 {
        let (p, q) = self.powers();
        q as f64 / p as f64
    }

    /// The additive energy `E(k, m) = sum_x r(x)^2`, `r(x)` the number of ways `x = a + b` with `a` in `A_k` and `b` in `B_m`.
    ///
    /// Summed as `sum_t 2^(z_3(t) + z_4(t))` over the `3^m` integers `t` with an `m`-digit base-4 string in `{-1, 0, 1}`, `z_4(t)` its zero digits and `z_3(t)` the zero digits of the `k`-digit balanced ternary expansion of `t`, a `t` past `(3^k - 1)/2` adding nothing: a difference of two digit strings from `{0, 1}` is a digit string from `{-1, 0, 1}`, which fixes `t` in either base, and each zero difference arises twice. The cost is `3^m` strings.
    pub fn energy(&self) -> u128 {
        let powers: Vec<i64> = (0..self.four).map(|i| 4i64.pow(i)).collect();
        strings(&powers, 0, 0, 0, self.three)
    }

    /// The energy ratio `Q(k, m) = E(k, m) (d + 1)/4^(k+m)` of the energy [`Pair::energy`] returns, the energy against its flat value, at least `1`; `card(A_k + B_m) >= (d + 1)/Q` by Cauchy-Schwarz.
    pub fn ratio(&self, energy: u128) -> f64 {
        let flat = 4f64.powi((self.three + self.four) as i32);
        energy as f64 * (self.largest() + 1) as f64 / flat
    }
}

fn strings(powers: &[i64], at: usize, t: i64, zeros: u32, three: u32) -> u128 {
    if at == powers.len() {
        return balanced_zeros(t, three).map_or(0, |z| 1u128 << (z + zeros));
    }
    (-1..=1)
        .map(|digit| {
            strings(
                powers,
                at + 1,
                t + digit * powers[at],
                zeros + u32::from(digit == 0),
                three,
            )
        })
        .sum()
}

fn balanced_zeros(mut t: i64, digits: u32) -> Option<u32> {
    let mut zeros = 0;
    for _ in 0..digits {
        let r = t.rem_euclid(3);
        zeros += u32::from(r == 0);
        t = (t - r) / 3 + i64::from(r == 2);
    }
    (t == 0).then_some(zeros)
}

/// Lists the pairs of the census: every `(k, m)` with `4^m` within a factor `3` of `3^k` and `d(k, m) <= 3^level`, by `d`.
///
/// # Errors
///
/// Errs when the level is zero or past [`WIDEST`].
pub fn pairs(level: u32) -> Result<Vec<Pair>> {
    if !(1..=WIDEST).contains(&level) {
        return value_error(format!("the level runs from 1 to {WIDEST}, not {level}."));
    }
    let top = 3u64.pow(level);
    let mut out = Vec::new();
    for three in 1..=level {
        for four in 1..=WIDEST {
            let pair = Pair { three, four };
            let (p, q) = pair.powers();
            if q >= 3 * p {
                break;
            }
            if p < 3 * q && pair.largest() <= top {
                out.push(pair);
            }
        }
    }
    out.sort_by_key(Pair::largest);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn level(k: u32) -> Vec<u64> {
        let mut out = elements(3, &[0, 1], k as usize);
        out.push(0);
        out
    }

    fn quarter(m: u32) -> Vec<u64> {
        let mut out = elements(4, &[0, 1], m as usize);
        out.push(0);
        out
    }

    fn histogram(pair: Pair) -> u128 {
        let mut r = vec![0u128; pair.largest() as usize + 1];
        for a in level(pair.three) {
            for b in quarter(pair.four) {
                r[(a + b) as usize] += 1;
            }
        }
        r.iter().map(|v| v * v).sum()
    }

    fn up(pair: Pair) -> u128 {
        let flat = 4u128.pow(pair.three + pair.four);
        (pair.energy() * u128::from(pair.largest() + 1) * 1_000_000).div_ceil(flat)
    }

    #[test]
    fn the_array_is_the_double_loop() {
        let s = Sumset::new(8).unwrap();
        let mut sums = vec![false; s.top() as usize + 1];
        for a in level(9) {
            for b in quarter(7) {
                if a + b <= s.top() {
                    sums[(a + b) as usize] = true;
                }
            }
        }
        for (x, &member) in sums.iter().enumerate() {
            assert_eq!(s.contains(x as u64), Some(member), "x = {x}");
        }
        assert_eq!(
            s.count(s.top()),
            Some(sums.iter().filter(|&&v| v).count() as u64 - 1)
        );
        assert_eq!(s.contains(s.top() + 1), None);
    }

    #[test]
    fn the_first_non_members_are_a367090() {
        let s = Sumset::new(6).unwrap();
        let missed: Vec<u64> = (0..=480)
            .filter(|&x| s.contains(x) == Some(false))
            .collect();
        let mut want = vec![62, 63, 143, 144];
        want.extend(207..=242);
        want.extend(463..=480);
        assert_eq!(missed, want);
    }

    #[test]
    fn the_densities_at_the_powers_of_three_read_as_printed() {
        let s = Sumset::new(10).unwrap();
        let read: Vec<u64> = (4..=10)
            .map(|k| {
                let x = 3u64.pow(k);
                s.count(x).unwrap() * 1_000_000 / x
            })
            .collect();
        assert_eq!(
            read,
            [975308, 835390, 858710, 887517, 908855, 864959, 778472]
        );
    }

    #[test]
    fn the_window_extremes_read_as_printed() {
        let s = Sumset::new(10).unwrap();
        let edges: Vec<u64> = (5..=10).map(|k| 3u64.pow(k)).collect();
        let six = |v: f64| (v * 1e6).floor() as u64;
        let read = s.extremes(&edges).unwrap();
        let lows: Vec<u64> = read.iter().map(|w| six(w.unwrap().0)).collect();
        let highs: Vec<u64> = read.iter().map(|w| six(w.unwrap().1)).collect();
        assert_eq!(lows, [835390, 852729, 887517, 858945, 778468]);
        assert_eq!(highs, [913419, 903768, 912038, 931596, 913781]);
    }

    #[test]
    fn the_strip_is_the_share_of_members() {
        let s = Sumset::new(6).unwrap();
        let strip = s.fills(200, 250, 5).unwrap();
        assert_eq!(strip, [0.7, 0.0, 0.0, 0.0, 0.7]);
        assert!(s.fills(0, 731, 5).is_err() && s.fills(10, 12, 3).is_err());
    }

    #[test]
    fn the_energy_is_the_representation_histogram() {
        for (k, m) in [(3, 2), (4, 3), (5, 4), (6, 5), (7, 5), (9, 7)] {
            let pair = Pair::new(k, m).unwrap();
            assert_eq!(pair.energy(), histogram(pair), "({k}, {m})");
        }
    }

    #[test]
    fn the_energy_ratios_read_as_printed() {
        let read: Vec<u128> = pairs(22)
            .unwrap()
            .into_iter()
            .filter(|p| p.three >= 6)
            .take(4)
            .map(up)
            .collect();
        assert_eq!(read, [1467705, 1638125, 1664808, 1724517]);
    }

    #[test]
    fn the_census_holds_twenty_seven_pairs() {
        let census: Vec<Pair> = pairs(22)
            .unwrap()
            .into_iter()
            .filter(|p| p.three >= 6)
            .collect();
        assert_eq!(census.len(), 27);
        assert!(!census.contains(&Pair::new(22, 18).unwrap()));
    }

    #[test]
    fn a_gap_holds_no_member() {
        let s = Sumset::new(12).unwrap();
        let mut seen = 0;
        for pair in pairs(12).unwrap() {
            if let Some((a, b)) = pair.gap() {
                assert!(pair.clean());
                seen += 1;
                assert!((a..=b).all(|x| s.contains(x) == Some(false)));
                assert_eq!(s.contains(a - 1), Some(true));
            }
        }
        assert!(seen >= 4);
    }

    #[test]
    fn a_gap_copy_doubles_the_energy() {
        for (k, m) in [(6, 4), (7, 5), (11, 8), (12, 9)] {
            let pair = Pair::new(k, m).unwrap();
            assert!(pair.copy());
            assert_eq!(pair.energy(), 2 * Pair::new(k - 1, m).unwrap().energy());
        }
    }
}
