use mrlycore::errors::{value_error, Result};

/// The largest digit span a rule may read, so its code fits a `u64`.
pub const SPAN: usize = 6;

/// A rule on `k` consecutive digits of a design word.
///
/// The alphabet is the `2^D` digit vectors `d` of `{0, 1}^D`, each read as the corner integer `c = sum_i d[i] 2^i`, with `x` bit `0`, `y` bit `1` and `z` bit `2`.
/// A window is `k` digits `(c_1, ..., c_k)` read as `w = sum_j c_j 2^(D (k - j))`, the first digit most significant, and the code has bit `w` set exactly when that window is allowed.
/// The code therefore lives in `[0, 2^(2^(k D)))`, which forces `k D <= SPAN`.
/// A word `d_1 d_2 ... d_L` is read coarsest digit first and is accepted when every window of `k` consecutive digits is allowed; for `L < k` there is no window, so every word is accepted.
/// Width 1 is the memoryless design of the same code: the cells of `bang dim <dim>, code <code>` at that level.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rule {
    /// The dimension `D`, one to three.
    pub dimension: usize,
    /// The window width `k`, at least one.
    pub width: usize,
    /// The window code, bit `w` set when window `w` is allowed.
    pub code: u64,
}

impl Rule {
    /// Builds a rule, or an error when the dimension, the width or the code is out of range.
    ///
    /// ```
    /// assert!(mrlynum::memory::Rule::new(1, 2, 7).is_ok());
    /// assert!(mrlynum::memory::Rule::new(2, 4, 0).is_err());
    /// ```
    pub fn new(dimension: usize, width: usize, code: u64) -> Result<Rule> {
        if !(1..=3).contains(&dimension) {
            return value_error(format!("dimension {dimension} is not one, two or three."));
        }
        if width < 1 {
            return value_error("width must be at least one.");
        }
        if width * dimension > SPAN {
            return value_error(format!(
                "width {width} at dimension {dimension} reads {} digit bits, over the span of {SPAN}.",
                width * dimension
            ));
        }
        let rule = Rule {
            dimension,
            width,
            code: 0,
        };
        let bound = rule.codes();
        if u128::from(code) >= bound {
            return value_error(format!(
                "code {code} is out of range for width {width} at dimension {dimension} (0..{}).",
                bound - 1
            ));
        }
        Ok(Rule {
            dimension,
            width,
            code,
        })
    }

    /// Returns the rule that allows every window.
    ///
    /// ```
    /// assert_eq!(mrlynum::memory::Rule::full(1, 2).code, 15);
    /// ```
    pub fn full(dimension: usize, width: usize) -> Rule {
        let rule = Rule::new(dimension, width, 0).expect("a zero code is always in range");
        Rule {
            code: (rule.codes() - 1) as u64,
            ..rule
        }
    }

    /// Returns the letter count `2^D`, the digit vectors of the cube's corners.
    pub fn letters(&self) -> usize {
        1 << self.dimension
    }

    /// Returns the window count `2^(k D)`.
    pub fn windows(&self) -> usize {
        1 << (self.width * self.dimension)
    }

    /// Returns the count of rules of this shape, `2^(2^(k D))`.
    pub fn codes(&self) -> u128 {
        1u128 << self.windows()
    }

    /// Returns the state count `2^((k - 1) D)`, the windows of one digit less that the transfer matrix runs on.
    pub fn states(&self) -> usize {
        1 << ((self.width - 1) * self.dimension)
    }

    /// Returns whether the window is allowed, and false for any window out of range.
    ///
    /// ```
    /// let golden = mrlynum::memory::Rule::new(1, 2, 7).unwrap();
    /// assert!(golden.allowed(2));
    /// assert!(!golden.allowed(3));
    /// ```
    pub fn allowed(&self, window: usize) -> bool {
        window < self.windows() && (self.code >> window) & 1 == 1
    }

    /// Returns whether a word, coarsest digit first, is accepted.
    ///
    /// ```
    /// let golden = mrlynum::memory::Rule::new(1, 2, 7).unwrap();
    /// assert!(golden.accepts(&[1, 0, 1]));
    /// assert!(!golden.accepts(&[1, 1, 0]));
    /// ```
    pub fn accepts(&self, word: &[usize]) -> bool {
        if word.len() < self.width {
            return word.iter().all(|&c| c < self.letters());
        }
        word.windows(self.width).all(|slice| {
            let mut w = 0;
            for &c in slice {
                w = (w << self.dimension) | c;
            }
            self.allowed(w)
        })
    }

    /// Returns the letters that stand in at least one allowed window.
    ///
    /// ```
    /// assert_eq!(mrlynum::memory::Rule::new(1, 2, 7).unwrap().alphabet(), vec![0, 1]);
    /// assert!(mrlynum::memory::Rule::new(1, 2, 0).unwrap().alphabet().is_empty());
    /// ```
    pub fn alphabet(&self) -> Vec<usize> {
        let mut seen = vec![false; self.letters()];
        for w in 0..self.windows() {
            if !self.allowed(w) {
                continue;
            }
            for j in 0..self.width {
                seen[(w >> (self.dimension * j)) & (self.letters() - 1)] = true;
            }
        }
        (0..self.letters()).filter(|&c| seen[c]).collect()
    }
}

// THE TRANSFER MATRIX

/// Returns the transfer matrix on the `(k - 1)`-windows: entry `(s, t)` is one when the window that overlaps state `s` onto state `t` is allowed.
///
/// At `k = 1` the one state is the empty window and the single entry counts the allowed letters.
///
/// ```
/// let golden = mrlynum::memory::Rule::new(1, 2, 7).unwrap();
/// assert_eq!(mrlynum::memory::transfer(&golden), vec![vec![1, 1], vec![1, 0]]);
/// ```
pub fn transfer(rule: &Rule) -> Vec<Vec<u64>> {
    let states = rule.states();
    let letters = rule.letters();
    let mut matrix = vec![vec![0u64; states]; states];
    if rule.width == 1 {
        matrix[0][0] = (0..letters).filter(|&c| rule.allowed(c)).count() as u64;
        return matrix;
    }
    for (s, row) in matrix.iter_mut().enumerate() {
        for c in 0..letters {
            let window = (s << rule.dimension) | c;
            if !rule.allowed(window) {
                continue;
            }
            row[window & (states - 1)] = 1;
        }
    }
    matrix
}

fn step(matrix: &[Vec<u64>], vector: &[u64]) -> Option<Vec<u64>> {
    let mut out = vec![0u64; vector.len()];
    for (s, row) in matrix.iter().enumerate() {
        for (t, &entry) in row.iter().enumerate() {
            if entry == 0 || vector[t] == 0 {
                continue;
            }
            out[s] = out[s].checked_add(entry.checked_mul(vector[t])?)?;
        }
    }
    Some(out)
}

// THE COUNTS

/// Returns `N_W(L)`, the count of accepted words, for `L = 1 ..= levels`, and stops early on the level whose count overruns a `u64`.
///
/// For `L < k` every word of that length is accepted, so the count is `2^(D L)`.
///
/// ```
/// let golden = mrlynum::memory::Rule::new(1, 2, 7).unwrap();
/// assert_eq!(mrlynum::memory::counts(&golden, 6), vec![2, 3, 5, 8, 13, 21]);
/// ```
pub fn counts(rule: &Rule, levels: usize) -> Vec<u64> {
    let matrix = transfer(rule);
    let mut vector = vec![1u64; rule.states()];
    let mut out = Vec::with_capacity(levels);
    for level in 1..=levels {
        if level + 1 < rule.width {
            match 1u64.checked_shl((rule.dimension * level) as u32) {
                Some(count) => out.push(count),
                None => break,
            }
            continue;
        }
        if level + 1 > rule.width {
            match step(&matrix, &vector) {
                Some(next) => vector = next,
                None => break,
            }
        }
        out.push(vector.iter().sum());
    }
    out
}

/// Returns the accepted words of the level as cell indices of the `2^L` grid, `x` from bit `0` of every digit, `y` from bit `1`, `z` from bit `2`, coarsest digit first.
///
/// The index is row major, `z side^2 + y side + x` with `side = 2^L`, so at `k = 1` the set is the level-`L` fill of the design of the same code.
///
/// ```
/// let golden = mrlynum::memory::Rule::new(1, 2, 7).unwrap();
/// assert_eq!(mrlynum::memory::cells(&golden, 2), vec![0, 1, 2]);
/// ```
pub fn cells(rule: &Rule, level: usize) -> Vec<u64> {
    let side = 1u64 << level;
    let mut out = Vec::new();
    let mut word = vec![0usize; level];
    walk(rule, level, 0, &mut word, &mut |word| {
        let mut place = vec![0u64; rule.dimension];
        for (j, &c) in word.iter().enumerate() {
            let weight = 1u64 << (level - 1 - j);
            for (axis, seat) in place.iter_mut().enumerate() {
                *seat += ((c >> axis) & 1) as u64 * weight;
            }
        }
        out.push(place.iter().rev().fold(0, |run, &seat| run * side + seat));
    });
    out
}

fn walk(
    rule: &Rule,
    level: usize,
    at: usize,
    word: &mut Vec<usize>,
    emit: &mut impl FnMut(&[usize]),
) {
    if at == level {
        emit(word);
        return;
    }
    for c in 0..rule.letters() {
        word[at] = c;
        if at + 1 >= rule.width {
            let start = at + 1 - rule.width;
            let mut w = 0;
            for &digit in &word[start..=at] {
                w = (w << rule.dimension) | digit;
            }
            if !rule.allowed(w) {
                continue;
            }
        }
        walk(rule, level, at + 1, word, emit);
    }
}

// THE GROWTH

/// The absolute `l^1` move of the normalised iterate that stops the power iteration, counted only when it holds over three consecutive sweeps.
pub const TOLERANCE: f64 = 1e-14;

/// The sweep cap of the power iteration.
pub const SWEEPS: usize = 100_000;

/// Returns the Perron root of the transfer matrix, the count's growth per level.
///
/// The digraph is split into its strongly connected components first; a component carrying no cycle contributes nothing, and a rule whose components all die past the window returns exactly `0`.
/// Each cyclic component is irreducible, so `I + B` is primitive and the power iteration on it converges geometrically; the iteration stops on a sustained Cauchy move of the normalised vector, never on a plateau of one scalar, and the Collatz-Wielandt ratios bracket the root.
/// The estimate is then made exact against the component's characteristic polynomial, taken by integer Faddeev-LeVerrier: an integer root is returned exactly, and otherwise the simple Perron root is bisected to the resolution of an `f64`.
/// The answer is the largest component root.
///
/// ```
/// let golden = mrlynum::memory::Rule::new(1, 2, 7).unwrap();
/// assert!((mrlynum::memory::perron(&golden) - 1.618_033_988_749_895).abs() < 1e-12);
/// assert_eq!(mrlynum::memory::perron(&mrlynum::memory::Rule::new(1, 2, 0).unwrap()), 0.0);
/// ```
pub fn perron(rule: &Rule) -> f64 {
    let matrix = transfer(rule);
    let reach = reachability(&matrix);
    let states = matrix.len();
    let mut seen = vec![false; states];
    let mut best = 0.0f64;
    for s in 0..states {
        if seen[s] || (reach[s] >> s) & 1 == 0 {
            continue;
        }
        let part: Vec<usize> = (0..states)
            .filter(|&t| t == s || ((reach[s] >> t) & 1 == 1 && (reach[t] >> s) & 1 == 1))
            .collect();
        for &t in &part {
            seen[t] = true;
        }
        let block: Vec<Vec<i128>> = part
            .iter()
            .map(|&i| part.iter().map(|&j| matrix[i][j] as i128).collect())
            .collect();
        let root = component_root(&block);
        if root > best {
            best = root;
        }
    }
    best
}

fn reachability(matrix: &[Vec<u64>]) -> Vec<u64> {
    let states = matrix.len();
    let mut reach: Vec<u64> = matrix
        .iter()
        .map(|row| {
            row.iter()
                .enumerate()
                .filter(|(_, &entry)| entry > 0)
                .map(|(t, _)| 1u64 << t)
                .sum()
        })
        .collect();
    for _ in 0..states {
        let old = reach.clone();
        for s in 0..states {
            for t in 0..states {
                if (old[s] >> t) & 1 == 1 {
                    reach[s] |= old[t];
                }
            }
        }
        if reach == old {
            break;
        }
    }
    reach
}

fn multiply(left: &[Vec<i128>], right: &[Vec<i128>]) -> Vec<Vec<i128>> {
    let n = left.len();
    let mut out = vec![vec![0i128; n]; n];
    for i in 0..n {
        for t in 0..n {
            if left[i][t] == 0 {
                continue;
            }
            for j in 0..n {
                out[i][j] += left[i][t] * right[t][j];
            }
        }
    }
    out
}

fn characteristic(block: &[Vec<i128>]) -> Vec<i128> {
    let n = block.len();
    let mut poly = vec![0i128; n + 1];
    poly[n] = 1;
    let mut carry = vec![vec![0i128; n]; n];
    for step in 1..=n {
        let mut product = multiply(block, &carry);
        for (i, row) in product.iter_mut().enumerate() {
            row[i] += poly[n - step + 1];
        }
        carry = product;
        let next = multiply(block, &carry);
        let trace: i128 = (0..n).map(|i| next[i][i]).sum();
        poly[n - step] = -trace / step as i128;
    }
    poly
}

fn value(poly: &[i128], x: f64) -> f64 {
    poly.iter().rev().fold(0.0f64, |run, &c| run * x + c as f64)
}

fn vanishes(poly: &[i128], x: i128) -> bool {
    let mut run = 0i128;
    for &c in poly.iter().rev() {
        run = match run.checked_mul(x).and_then(|v| v.checked_add(c)) {
            Some(v) => v,
            None => return false,
        };
    }
    run == 0
}

/// Returns the seed itself, unpolished, when no sign change of the characteristic polynomial is found within `0.25` of it, so a component whose bracket search fails reads the Collatz-Wielandt midpoint and never a wrong root.
fn component_root(block: &[Vec<i128>]) -> f64 {
    let n = block.len();
    let mut vector = vec![1.0f64 / n as f64; n];
    let mut settled = 0usize;
    for _ in 0..SWEEPS {
        let mut next = vec![0.0f64; n];
        for (s, row) in block.iter().enumerate() {
            let mut sum = vector[s];
            for (t, &entry) in row.iter().enumerate() {
                sum += entry as f64 * vector[t];
            }
            next[s] = sum;
        }
        let mass: f64 = next.iter().sum();
        for seat in next.iter_mut() {
            *seat /= mass;
        }
        let move_size: f64 = next
            .iter()
            .zip(vector.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();
        vector = next;
        if move_size <= TOLERANCE {
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
    for (s, row) in block.iter().enumerate() {
        let image: f64 = row
            .iter()
            .enumerate()
            .map(|(t, &entry)| entry as f64 * vector[t])
            .sum();
        let ratio = image / vector[s];
        low = low.min(ratio);
        high = high.max(ratio);
    }
    let seed = 0.5 * (low + high);
    let poly = characteristic(block);
    let rounded = seed.round();
    if (0.0..1e18).contains(&rounded) && vanishes(&poly, rounded as i128) {
        return rounded;
    }
    let mut under;
    let mut over;
    let mut delta = 1e-13f64;
    loop {
        under = seed - delta;
        over = seed + delta;
        if value(&poly, under) < 0.0 && value(&poly, over) > 0.0 {
            break;
        }
        delta *= 4.0;
        if delta > 0.25 {
            return seed;
        }
    }
    for _ in 0..200 {
        let middle = 0.5 * (under + over);
        if middle <= under || middle >= over {
            break;
        }
        if value(&poly, middle) < 0.0 {
            under = middle;
        } else {
            over = middle;
        }
    }
    0.5 * (under + over)
}

/// Returns the growth exponent `log_2 rho`, the growth per digit of the accepted word count.
///
/// ```
/// let full = mrlynum::memory::Rule::full(2, 1);
/// assert!((mrlynum::memory::exponent(&full) - 2.0).abs() < 1e-12);
/// ```
pub fn exponent(rule: &Rule) -> f64 {
    let rho = perron(rule);
    if rho <= 0.0 {
        return f64::NEG_INFINITY;
    }
    rho.log2()
}

/// Returns the count of allowed windows `card W`, the bits the code sets inside its window range.
///
/// ```
/// assert_eq!(mrlynum::memory::allowed_windows(&mrlynum::memory::Rule::new(1, 2, 7).unwrap()), 3);
/// ```
pub fn allowed_windows(rule: &Rule) -> usize {
    let windows = rule.windows();
    let mask = if windows >= 64 {
        u64::MAX
    } else {
        (1u64 << windows) - 1
    };
    (rule.code & mask).count_ones() as usize
}

/// Returns the memory number `kappa(W) = log_2(card W) / k - log_2 rho`, the bits a digit spends on memory.
///
/// The window budget `log_2(card W) / k` is what a word of length `mk` could carry if its `m` disjoint windows were free of each other, and `log_2 rho` is what it really carries, so `kappa >= 0` and the gap is the interference the rule pays for.
/// It is zero on every product rule `W = F^k`, where `card W = card F^k` and `rho = card F`, so every `k = 1` rule reads zero; it is `f64::INFINITY` when a window is allowed and nothing survives past the window, and zero when no window is allowed.
///
/// ```
/// let golden = mrlynum::memory::Rule::new(1, 2, 7).unwrap();
/// assert!((mrlynum::memory::kappa(&golden) - 0.098_239_336_730).abs() < 1e-9);
/// assert_eq!(mrlynum::memory::kappa(&mrlynum::memory::Rule::full(2, 2)), 0.0);
/// ```
pub fn kappa(rule: &Rule) -> f64 {
    let windows = allowed_windows(rule);
    if windows == 0 {
        return 0.0;
    }
    let rho = perron(rule);
    if rho <= 0.0 {
        return f64::INFINITY;
    }
    (windows as f64).log2() / rule.width as f64 - rho.log2()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn width_one_is_the_memoryless_design() {
        for code in [1u64, 7, 11, 13, 14] {
            let rule = Rule::new(2, 1, code).unwrap();
            let fills = code.count_ones() as u64;
            assert_eq!(
                counts(&rule, 4),
                vec![fills, fills.pow(2), fills.pow(3), fills.pow(4)]
            );
            assert_eq!(kappa(&rule), 0.0);
            let level = cells(&rule, 1);
            let want: Vec<u64> = (0..4).filter(|c| (code >> c) & 1 == 1).collect();
            assert_eq!(level, want);
        }
    }

    #[test]
    fn the_golden_rule_counts_the_fibonacci_numbers() {
        let golden = Rule::new(1, 2, 7).unwrap();
        assert_eq!(counts(&golden, 8), vec![2, 3, 5, 8, 13, 21, 34, 55]);
        assert!((perron(&golden) - 1.618_033_988_749_895).abs() < 1e-12);
        assert_eq!(format!("{:.6}", kappa(&golden)), "0.098239");
    }

    #[test]
    fn the_perron_root_is_the_spectral_radius_on_every_reducible_rule() {
        let plastic = 1.324_717_957_244_746;
        let golden = 1.618_033_988_749_895;
        let quartic = 1.380_277_569_097_614;
        for (code, want) in [
            (5u64, 1.0),
            (62, plastic),
            (91, quartic),
            (95, golden),
            (125, plastic),
            (190, plastic),
        ] {
            let rule = Rule::new(1, 3, code).unwrap();
            let root = perron(&rule);
            assert!(
                (root - want).abs() < 1e-12,
                "code {code} reads {root} against {want}"
            );
        }
    }

    #[test]
    fn the_supergolden_rule_counts_the_narayana_cows() {
        let rule = Rule::new(1, 3, 23).unwrap();
        assert_eq!(counts(&rule, 8), vec![2, 4, 4, 6, 9, 13, 19, 28]);
        let terms = counts(&rule, 12);
        for l in 5..terms.len() {
            assert_eq!(terms[l], terms[l - 1] + terms[l - 3]);
        }
    }

    #[test]
    fn the_full_rule_counts_every_word() {
        for dimension in 1..=3 {
            for width in 1..=SPAN / dimension {
                let rule = Rule::full(dimension, width);
                let want: Vec<u64> = (1..=5).map(|l| 1u64 << (dimension * l)).collect();
                assert_eq!(counts(&rule, 5), want, "d{dimension} k{width}");
                assert!((exponent(&rule) - dimension as f64).abs() < 1e-12);
            }
        }
    }

    #[test]
    fn the_empty_rule_dies_past_its_window() {
        for width in 1..=3 {
            let rule = Rule::new(1, width, 0).unwrap();
            let want: Vec<u64> = (1..=5)
                .map(|l| if l < width { 1 << l } else { 0 })
                .collect();
            assert_eq!(counts(&rule, 5), want, "k{width}");
            assert_eq!(perron(&rule), 0.0);
            assert!(cells(&rule, 5).is_empty());
        }
    }

    #[test]
    fn the_cells_are_the_accepted_words() {
        let rule = Rule::new(2, 2, 0xf1e2).unwrap();
        let level = 4;
        let side = 1u64 << level;
        let drawn = cells(&rule, level);
        assert_eq!(drawn.len() as u64, counts(&rule, level)[level - 1]);
        let mut seen = std::collections::BTreeSet::new();
        for index in &drawn {
            assert!(seen.insert(*index));
            assert!(*index < side * side);
        }
    }

    #[test]
    fn a_rule_out_of_range_is_refused() {
        assert!(Rule::new(0, 1, 0).is_err());
        assert!(Rule::new(4, 1, 0).is_err());
        assert!(Rule::new(1, 0, 0).is_err());
        assert!(Rule::new(3, 3, 0).is_err());
        assert!(Rule::new(1, 2, 16).is_err());
    }
}
