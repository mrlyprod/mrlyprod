use crate::core::error::{value_error, Result};
use crate::num::memory::{transfer, Rule};
use crate::num::zeta::{raise, Complex};

/// The relative rounding allowance the double-precision matrix ladder charges against the scale it carries.
///
/// The truncation bound is the proved one, carried entrywise by the same recursion that carries the value vector; this constant is a measured allowance for the arithmetic itself and is not a proof.
pub const ROUNDING: f64 = 1e-13;

const PEEL_TARGET: f64 = 100.0;
const PEEL_CAP: usize = 26;
const SHIFT_START: f64 = 6.0;
const SHIFT_STEP: f64 = 6.0;
const SHIFT_CAP: f64 = 400.0;
const CUT_START: usize = 8;
const CUT_STEP: usize = 6;
const CUT_CAP: usize = 120;
const RATIO_CAP: f64 = 0.9;
const COLLATZ_WIELANDT: usize = 60;
const PIVOT_FLOOR: f64 = 1e-12;

// THE STATE SPACE

/// A memory design read as a matrix ladder: the rule, the transfer matrix on its `(k-1)`-window states, and the peel depth its Dirichlet series is continued from.
///
/// The elements are the positive integers whose minimal base-`q` string, `q = 2^D`, is accepted by the rule, and the object is `zeta_W(s) = sum n^(-s)` over them.
/// [`crate::num::memory::Rule::accepts`] holds a word shorter than the width `k` to be accepted, there being no window in it, so every integer below `q^(k-1)` with a nonzero leading digit sits in `S_W` for every rule of that width, and the head polynomial carries them.
/// With `E_j(w)` the vector whose entry `u` sums `n^(-w)` over accepted words of exactly `j` digits ending in state `u`, and `G_P = sum_(j >= P) E_j`, splitting a word on its last digit gives `(I - q^(-w) T) G_P(w) = E_P(w) + sum_(l >= 1) binom(-w, l) q^(-w-l) Gamma_l G_P(w+l)`, where `Gamma_l(u', u) = sum a^l` over the letters `a` with the window `(u, a)` allowed and `u' = shift(u, a)`, and `T = Gamma_0` is [`crate::num::memory::transfer`] transposed.
/// Then `zeta_W(w) = 1^T D_(P-1)(w) + 1^T G_P(w)` with `D_(P-1)` the Dirichlet polynomial over the accepted words of at most `P-1` digits, and the scalar ladder of [`crate::num::ladder`] is the width one case with `card F` where `T` stands.
#[derive(Clone, Debug, PartialEq)]
pub struct Automaton {
    rule: Rule,
    base: u64,
    states: usize,
    peel: usize,
    largest: u64,
    arcs: Vec<(usize, usize, u64)>,
    power: Vec<Vec<f64>>,
    low: Vec<f64>,
    mid: Vec<Vec<f64>>,
    seats: Vec<f64>,
    guide: Vec<f64>,
    perron_high: f64,
    perron_low: f64,
    charpoly: Vec<f64>,
    adjugate: Vec<Vec<Vec<f64>>>,
}

fn walk_words(
    rule: &Rule,
    base: u64,
    depth: usize,
    word: &mut Vec<usize>,
    visit: &mut impl FnMut(usize, usize, u64),
) {
    if word.len() >= depth {
        return;
    }
    let start = usize::from(word.is_empty());
    for c in start..rule.letters() {
        word.push(c);
        if rule.accepts(word) {
            let value = word.iter().fold(0u64, |acc, &d| acc * base + d as u64);
            let span = word.len() - (rule.width - 1).min(word.len());
            let state = word[span..]
                .iter()
                .fold(0usize, |acc, &d| acc * base as usize + d);
            visit(word.len(), state, value);
            walk_words(rule, base, depth, word, visit);
        }
        word.pop();
    }
}

impl Automaton {
    /// Builds the ladder of a rule, choosing the peel depth.
    ///
    /// ```
    /// let golden = mrlyrs::num::memory::Rule::new(1, 2, 7).unwrap();
    /// let ladder = mrlyrs::num::automaton::Automaton::new(&golden).unwrap();
    /// assert_eq!(ladder.states(), 2);
    /// assert!((ladder.abscissa() - 0.694_241_913_630_617_4).abs() < 1e-15);
    /// ```
    ///
    /// # Errors
    ///
    /// Errs when the rule accepts no element with a nonzero leading digit, or when its peel overruns the exact integers.
    pub fn new(rule: &Rule) -> Result<Automaton> {
        let mut peel = rule.width.max(2);
        let base = rule.letters() as u64;
        while peel < PEEL_CAP && (base as f64).powi(peel as i32 + 1) < 9.0e15 {
            let mut seen = 0f64;
            let mut word = Vec::new();
            walk_words(rule, base, peel, &mut word, &mut |len, _, _| {
                if len == peel {
                    seen += 1.0;
                }
            });
            if seen > PEEL_TARGET {
                break;
            }
            peel += 1;
        }
        Automaton::with_peel(rule, peel)
    }

    /// Builds the ladder at an explicit peel depth, at least the rule width and at least two.
    ///
    /// ```
    /// let full = mrlyrs::num::memory::Rule::full(1, 2).unwrap();
    /// let ladder = mrlyrs::num::automaton::Automaton::with_peel(&full, 7).unwrap();
    /// assert_eq!(ladder.peel(), 7);
    /// assert_eq!(ladder.abscissa(), 1.0);
    /// ```
    ///
    /// # Errors
    ///
    /// Errs when the rule accepts no such element, and at a peel depth under the rule width or two, or past the exact integers.
    pub fn with_peel(rule: &Rule, peel: usize) -> Result<Automaton> {
        let base = rule.letters() as u64;
        let states = rule.states();
        if peel < rule.width.max(2) {
            return value_error(format!(
                "peel depth {peel} is under the rule width {} or under two.",
                rule.width
            ));
        }
        if (base as f64).powi(peel as i32) > 9.007_199_254_740_992e15 {
            return value_error(format!("peel depth {peel} overruns the exact integers."));
        }
        let source = transfer(rule);
        let mut arcs: Vec<(usize, usize, u64)> = Vec::new();
        if rule.width == 1 {
            for c in 0..rule.letters() {
                if rule.allowed(c) {
                    arcs.push((0, 0, c as u64));
                }
            }
        } else {
            for s in 0..source.len() {
                for c in 0..rule.letters() {
                    let window = (s << rule.dimension) | c;
                    if rule.allowed(window) {
                        arcs.push((window & (states - 1), s, c as u64));
                    }
                }
            }
        }
        let largest = arcs.iter().map(|&(_, _, a)| a).max().unwrap_or(0);
        let power: Vec<Vec<f64>> = (0..base)
            .map(|a| (0..=CUT_CAP).map(|l| (a as f64).powi(l as i32)).collect())
            .collect();
        let mut low: Vec<f64> = Vec::new();
        let mut mid: Vec<Vec<f64>> = vec![Vec::new(); states];
        let mut seats = vec![0.0f64; states];
        let mut word = Vec::new();
        walk_words(rule, base, peel, &mut word, &mut |len, state, value| {
            if len < peel {
                low.push((value as f64).ln());
            } else {
                mid[state].push((value as f64).ln());
                seats[state] += 1.0;
            }
        });
        if low.is_empty() && seats.iter().all(|&c| c == 0.0) {
            return value_error("the rule accepts no element with a nonzero leading digit.");
        }
        let mut ladder = Automaton {
            rule: *rule,
            base,
            states,
            peel,
            largest,
            arcs,
            power,
            low,
            mid,
            seats,
            guide: vec![1.0; states],
            perron_high: 0.0,
            perron_low: 0.0,
            charpoly: Vec::new(),
            adjugate: Vec::new(),
        };
        ladder.tighten();
        ladder.faddeev();
        Ok(ladder)
    }

    fn tighten(&mut self) {
        let mut guide = vec![1.0f64; self.states];
        for _ in 0..COLLATZ_WIELANDT {
            let next = self.step_real(0, &guide);
            let peak = next.iter().cloned().fold(0.0f64, f64::max);
            if peak > 1e120 {
                break;
            }
            for (seat, &gain) in guide.iter_mut().zip(next.iter()) {
                *seat += gain;
            }
        }
        let image = self.step_real(0, &guide);
        let mut high = 0.0f64;
        let mut low = f64::INFINITY;
        for (&gain, &seat) in image.iter().zip(guide.iter()) {
            high = high.max(gain / seat);
            low = low.min(gain / seat);
        }
        self.guide = guide;
        self.perron_high = high * (1.0 + 1e-14) + 1e-300;
        self.perron_low = low * (1.0 - 1e-14);
    }

    fn step_real(&self, power: usize, vector: &[f64]) -> Vec<f64> {
        let mut out = vec![0.0f64; self.states];
        for &(t, s, a) in &self.arcs {
            out[t] += self.power[a as usize][power] * vector[s];
        }
        out
    }

    fn step_complex(&self, power: usize, vector: &[Complex]) -> Vec<Complex> {
        let mut out = vec![Complex::new(0.0, 0.0); self.states];
        for &(t, s, a) in &self.arcs {
            out[t] = out[t] + vector[s] * self.power[a as usize][power];
        }
        out
    }

    /// Returns the rule.
    pub fn rule(&self) -> Rule {
        self.rule
    }

    /// Returns the base `q = 2^D`.
    pub fn base(&self) -> u64 {
        self.base
    }

    /// Returns the state count `q^(k-1)`.
    pub fn states(&self) -> usize {
        self.states
    }

    /// Returns the peel depth `P`.
    pub fn peel(&self) -> usize {
        self.peel
    }

    /// Returns the transfer matrix `T = Gamma_0` the ladder runs on, the transpose of [`crate::num::memory::transfer`], entry `(u', u)` counting the letters carrying `u` to `u'`.
    ///
    /// ```
    /// let golden = mrlyrs::num::memory::Rule::new(1, 2, 7).unwrap();
    /// let ladder = mrlyrs::num::automaton::Automaton::new(&golden).unwrap();
    /// assert_eq!(ladder.matrix(), vec![vec![1.0, 1.0], vec![1.0, 0.0]]);
    /// ```
    pub fn matrix(&self) -> Vec<Vec<f64>> {
        let mut out = vec![vec![0.0f64; self.states]; self.states];
        for &(t, s, _) in &self.arcs {
            out[t][s] += 1.0;
        }
        out
    }

    /// Returns the Collatz-Wielandt bracket `(low, high)` of the Perron root of the transfer matrix, the ratios the ladder divides with.
    ///
    /// For any nonnegative `T` and any `v > 0`, `min_u (T v)_u / v_u <= rho <= max_u (T v)_u / v_u`, so the pair brackets the root with no primitivity hypothesis; the guide is `v = (I + T)^m 1` for `m` sixty, or fewer when the entries would overrun the exponent range.
    /// The ends carry a relative `1e-14` allowance of the same measured kind as [`ROUNDING`] and are not a rounding-certified interval, and the bracket is loose exactly when `T` is defective: at code `13` and `k = 2`, a Jordan block of root `1`, the upper end is `64/62`. Only the upper end is load bearing, as the `theta < 1` test that opens the Neumann branch; the root itself is [`crate::num::memory::perron`].
    pub fn perron(&self) -> (f64, f64) {
        (self.perron_low, self.perron_high)
    }

    /// Returns the abscissa `alpha = log_q rho`, with `rho` the exact Perron root of [`crate::num::memory::perron`].
    ///
    /// The series converges absolutely on `Re s > alpha`. The bracket of [`Automaton::perron`] is a bound and not the root, so it is not read here.
    ///
    /// ```
    /// let jordan = mrlyrs::num::memory::Rule::new(1, 2, 13).unwrap();
    /// let ladder = mrlyrs::num::automaton::Automaton::new(&jordan).unwrap();
    /// assert_eq!(ladder.abscissa(), 0.0);
    /// ```
    pub fn abscissa(&self) -> f64 {
        let root = crate::num::memory::perron(&self.rule);
        if root <= 0.0 {
            return f64::NEG_INFINITY;
        }
        root.ln() / (self.base as f64).ln()
    }

    /// Returns the pole spacing `2 pi / log q`.
    pub fn period(&self) -> f64 {
        2.0 * std::f64::consts::PI / (self.base as f64).ln()
    }
}

// THE RESOLVENT

fn invert(matrix: &[Vec<Complex>]) -> Option<Vec<Vec<Complex>>> {
    let n = matrix.len();
    let mut a: Vec<Vec<Complex>> = matrix.to_vec();
    let mut b: Vec<Vec<Complex>> = (0..n)
        .map(|i| {
            (0..n)
                .map(|j| Complex::new(f64::from(u8::from(i == j)), 0.0))
                .collect()
        })
        .collect();
    let scale = matrix
        .iter()
        .flat_map(|row| row.iter())
        .map(|z| z.abs())
        .fold(0.0f64, f64::max)
        .max(1.0);
    for col in 0..n {
        let mut pick = col;
        for row in col + 1..n {
            if a[row][col].abs() > a[pick][col].abs() {
                pick = row;
            }
        }
        if a[pick][col].abs() < PIVOT_FLOOR * scale {
            return None;
        }
        a.swap(col, pick);
        b.swap(col, pick);
        let inv = Complex::new(1.0, 0.0) / a[col][col];
        for j in 0..n {
            a[col][j] = a[col][j] * inv;
            b[col][j] = b[col][j] * inv;
        }
        let prow = a[col].clone();
        let qrow = b[col].clone();
        for row in 0..n {
            if row == col {
                continue;
            }
            let factor = a[row][col];
            if factor.abs() == 0.0 {
                continue;
            }
            for j in 0..n {
                a[row][j] = a[row][j] - factor * prow[j];
                b[row][j] = b[row][j] - factor * qrow[j];
            }
        }
    }
    Some(b)
}

fn log_binomial(top: f64, pick: usize) -> f64 {
    (1..=pick)
        .map(|i| ((top - pick as f64 + i as f64) / i as f64).ln())
        .sum()
}

fn poly(logs: &[f64], w: Complex) -> Complex {
    let mut acc = Complex::new(0.0, 0.0);
    for &l in logs {
        acc = acc + ((-w) * l).exp();
    }
    acc
}

fn poly_scale(logs: &[f64], sigma: f64) -> f64 {
    logs.iter().map(|&l| (-sigma * l).exp()).sum()
}

struct Rung {
    value: Vec<Complex>,
    bound: Vec<f64>,
    scale: Vec<f64>,
}

impl Automaton {
    fn neumann(&self, sigma: f64, seed: &[f64]) -> Option<Vec<f64>> {
        let base = self.base as f64;
        let theta = base.powf(-sigma) * self.perron_high;
        if theta >= 1.0 {
            return None;
        }
        let factor = base.powf(-sigma);
        let beta = seed
            .iter()
            .zip(self.guide.iter())
            .map(|(&y, &v)| y / v)
            .fold(0.0f64, f64::max);
        let mut out = vec![0.0f64; self.states];
        let mut term = seed.to_vec();
        let mut power = 1.0f64;
        for _ in 0..4000 {
            for (seat, &gain) in out.iter_mut().zip(term.iter()) {
                *seat += gain;
            }
            term = self
                .step_real(0, &term)
                .iter()
                .map(|&v| v * factor)
                .collect();
            power *= theta;
            let peak = out.iter().cloned().fold(0.0f64, f64::max);
            if beta * power <= 1e-18 * (1.0 - theta) * peak || term.iter().all(|&v| v == 0.0) {
                break;
            }
        }
        let rest = beta * power / (1.0 - theta);
        for (seat, &v) in out.iter_mut().zip(self.guide.iter()) {
            *seat += rest * v;
        }
        Some(out)
    }

    fn tail_parts(&self, sigma: f64) -> Option<(f64, Vec<f64>)> {
        let mass = self.neumann(sigma, &self.seats)?;
        let log = -(self.peel as f64 - 1.0) * sigma * (self.base as f64).ln();
        Some((log, mass))
    }

    fn tail(&self, sigma: f64) -> Option<Vec<f64>> {
        let (log, mass) = self.tail_parts(sigma)?;
        Some(mass.iter().map(|&m| log.exp() * m).collect())
    }

    fn cut(&self, w: Complex, depth: usize) -> Vec<f64> {
        let blown = vec![f64::INFINITY; self.states];
        if self.largest == 0 {
            return vec![0.0; self.states];
        }
        let (span, sigma) = (w.abs(), w.re);
        let base = self.base as f64;
        let largest = self.largest as f64;
        let ratio = ((span + depth as f64 + 1.0) / (depth as f64 + 2.0)).max(1.0) * largest
            / base.powi(self.peel as i32);
        if ratio >= RATIO_CAP {
            return blown;
        }
        let Some((rest, mass)) = self.tail_parts(sigma + depth as f64 + 1.0) else {
            return blown;
        };
        let log = log_binomial(span + depth as f64, depth + 1)
            + (-sigma - depth as f64 - 1.0) * base.ln()
            + (depth as f64 + 1.0) * largest.ln()
            + rest;
        let front = self.step_real(0, &mass);
        front
            .iter()
            .map(|&m| log.exp() * m / (1.0 - ratio))
            .collect()
    }

    fn divide(&self, w: Complex, acc: &[Complex], err: &[f64], wide: &[f64]) -> Option<Rung> {
        let base = self.base as f64;
        let x = raise(base, -w);
        let front: Vec<Vec<Complex>> = self
            .matrix()
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, &t)| Complex::new(f64::from(u8::from(i == j)), 0.0) - x * t)
                    .collect()
            })
            .collect();
        let inverse = invert(&front)?;
        let value: Vec<Complex> = inverse
            .iter()
            .map(|row| {
                row.iter()
                    .zip(acc.iter())
                    .fold(Complex::new(0.0, 0.0), |a, (&c, &y)| a + c * y)
            })
            .collect();
        let carry = |seed: &[f64]| -> Option<Vec<f64>> {
            if let Some(out) = self.neumann(w.re, seed) {
                return Some(out);
            }
            let mut residual = 0.0f64;
            for (i, source) in front.iter().enumerate() {
                let mut row = 0.0f64;
                for j in 0..self.states {
                    let mut entry = Complex::new(f64::from(u8::from(i == j)), 0.0);
                    for (&near, image) in source.iter().zip(inverse.iter()) {
                        entry = entry - near * image[j];
                    }
                    row += entry.abs();
                }
                residual = residual.max(row);
            }
            if residual >= 1.0 {
                return None;
            }
            let peak = seed.iter().cloned().fold(0.0f64, f64::max);
            let lift = peak * residual / (1.0 - residual);
            Some(
                inverse
                    .iter()
                    .map(|row| {
                        row.iter()
                            .zip(seed.iter())
                            .map(|(&c, &y)| c.abs() * (y + lift))
                            .sum()
                    })
                    .collect(),
            )
        };
        Some(Rung {
            value,
            bound: carry(err)?,
            scale: carry(wide)?,
        })
    }

    fn rung(&self, s: Complex, shift: f64, depth: usize, whole: bool) -> Option<Rung> {
        let base = self.base as f64;
        let levels = (shift - s.re).ceil().max(0.0) as usize;
        if !whole && levels == 0 {
            let rest = self.tail(s.re)?;
            let lift = self.step_real(0, &rest);
            let factor = base.powf(-s.re);
            let out: Vec<f64> = rest
                .iter()
                .zip(lift.iter())
                .map(|(&r, &l)| r + factor * l)
                .collect();
            return Some(Rung {
                value: vec![Complex::new(0.0, 0.0); self.states],
                bound: out.clone(),
                scale: out,
            });
        }
        let rows = levels + depth + 2;
        let mut value = vec![vec![Complex::new(0.0, 0.0); self.states]; rows];
        let mut bound = vec![vec![0.0f64; self.states]; rows];
        let mut scale = vec![vec![0.0f64; self.states]; rows];
        for j in levels..rows {
            let rest = self.tail(s.re + j as f64)?;
            bound[j] = rest.clone();
            scale[j] = rest;
        }
        for j in (0..levels).rev() {
            let w = s + j as f64;
            let mut acc: Vec<Complex> = self
                .mid
                .iter()
                .map(|logs| poly(logs, w))
                .collect::<Vec<Complex>>();
            let mut err = self.cut(w, depth);
            let mut wide: Vec<f64> = self
                .mid
                .iter()
                .map(|logs| poly_scale(logs, w.re))
                .zip(err.iter())
                .map(|(p, &e)| p + e)
                .collect();
            let mut coefficient = Complex::new(1.0, 0.0);
            for l in 1..=depth {
                coefficient = coefficient * ((-w) - (l as f64 - 1.0)) * (1.0 / l as f64);
                let weight = raise(base, (-w) - l as f64) * coefficient;
                let lift = self.step_complex(l, &value[j + l]);
                let heavy = self.step_real(l, &bound[j + l]);
                let broad = self.step_real(l, &scale[j + l]);
                for u in 0..self.states {
                    acc[u] = acc[u] + weight * lift[u];
                    err[u] += weight.abs() * heavy[u];
                    wide[u] += weight.abs() * broad[u];
                }
            }
            if !whole && j == 0 {
                return Some(Rung {
                    value: acc,
                    bound: err,
                    scale: wide,
                });
            }
            let step = self.divide(w, &acc, &err, &wide)?;
            value[j] = step.value;
            bound[j] = step.bound;
            scale[j] = step.scale;
        }
        Some(Rung {
            value: value[0].clone(),
            bound: bound[0].clone(),
            scale: scale[0].clone(),
        })
    }

    fn tune(
        &self,
        s: Complex,
        tolerance: f64,
        whole: bool,
        head: f64,
    ) -> Result<(Vec<Complex>, Vec<f64>)> {
        let mut shift = SHIFT_START;
        let mut depth = CUT_START;
        let mut best = f64::INFINITY;
        let mut walked = false;
        while shift <= SHIFT_CAP && depth <= CUT_CAP {
            if let Some(step) = self.rung(s, shift, depth, whole) {
                let carried: Vec<f64> = step
                    .bound
                    .iter()
                    .zip(step.scale.iter())
                    .map(|(&b, &c)| b + ROUNDING * c)
                    .collect();
                let total: f64 = carried.iter().sum::<f64>() + ROUNDING * head;
                if total < tolerance {
                    return Ok((step.value, carried));
                }
                best = best.min(total);
            } else {
                walked = true;
            }
            shift += SHIFT_STEP;
            depth += CUT_STEP;
        }
        if walked {
            return value_error(format!(
                "the ladder cannot invert I - q^(-w) T on the walk up from s = {} + {}i: a level sits on a pole of the resolvent or left of the abscissa with no certified inverse.",
                s.re, s.im
            ));
        }
        value_error(format!(
            "tolerance {tolerance:e} is out of reach at s = {} + {}i, best bound {best:e}.",
            s.re, s.im
        ))
    }
}

// THE DETERMINANT AND THE ADJUGATE

impl Automaton {
    fn faddeev(&mut self) {
        let n = self.states;
        let matrix = self.matrix();
        let mut blocks: Vec<Vec<Vec<f64>>> = Vec::with_capacity(n);
        let mut coefficients = vec![1.0f64];
        let mut current: Vec<Vec<f64>> = (0..n)
            .map(|i| (0..n).map(|j| f64::from(u8::from(i == j))).collect())
            .collect();
        for k in 1..=n {
            blocks.push(current.clone());
            let mut product = vec![vec![0.0f64; n]; n];
            for i in 0..n {
                for t in 0..n {
                    if matrix[i][t] == 0.0 {
                        continue;
                    }
                    for j in 0..n {
                        product[i][j] += matrix[i][t] * current[t][j];
                    }
                }
            }
            let trace: f64 = (0..n).map(|i| product[i][i]).sum();
            let coefficient = -trace / k as f64;
            coefficients.push(coefficient);
            for (i, row) in product.iter_mut().enumerate() {
                row[i] += coefficient;
            }
            current = product;
        }
        self.charpoly = coefficients;
        self.adjugate = blocks;
    }

    /// Returns the coefficients `c_0 .. c_n` of `det(I - x T) = sum c_i x^i`, the ladder denominator read as a polynomial in `x = q^(-s)`.
    ///
    /// The roots in `x` are the reciprocals of the nonzero eigenvalues of `T`, so the poles of `zeta_W` sit at `s = log_q lambda_i - m + (arg lambda_i + 2 pi j) i / log q`: one comb per distinct eigenvalue, offset by the eigenvalue's argument.
    ///
    /// ```
    /// let golden = mrlyrs::num::memory::Rule::new(1, 2, 7).unwrap();
    /// let ladder = mrlyrs::num::automaton::Automaton::new(&golden).unwrap();
    /// let coefficients = ladder.denominator();
    /// assert!((coefficients[1] + 1.0).abs() < 1e-12 && (coefficients[2] + 1.0).abs() < 1e-12);
    /// ```
    pub fn denominator(&self) -> Vec<f64> {
        self.charpoly.clone()
    }

    fn det(&self, x: Complex) -> Complex {
        let mut acc = Complex::new(0.0, 0.0);
        let mut power = Complex::new(1.0, 0.0);
        for &c in &self.charpoly {
            acc = acc + power * c;
            power = power * x;
        }
        acc
    }

    fn det_slope(&self, x: Complex) -> Complex {
        let mut acc = Complex::new(0.0, 0.0);
        let mut power = Complex::new(1.0, 0.0);
        for (k, &c) in self.charpoly.iter().enumerate().skip(1) {
            acc = acc + power * (c * k as f64);
            power = power * x;
        }
        acc
    }

    fn adjugate_apply(&self, x: Complex, vector: &[Complex]) -> Vec<Complex> {
        let mut out = vec![Complex::new(0.0, 0.0); self.states];
        let mut power = Complex::new(1.0, 0.0);
        for block in &self.adjugate {
            for (i, row) in block.iter().enumerate() {
                for (j, &entry) in row.iter().enumerate() {
                    if entry != 0.0 {
                        out[i] = out[i] + power * entry * vector[j];
                    }
                }
            }
            power = power * x;
        }
        out
    }

    fn adjugate_weights(&self, radius: f64) -> Vec<f64> {
        let mut out = vec![0.0f64; self.states];
        let mut power = 1.0f64;
        for block in &self.adjugate {
            for row in block.iter() {
                for (j, &entry) in row.iter().enumerate() {
                    out[j] += power * entry.abs();
                }
            }
            power *= radius;
        }
        out
    }
}

// READINGS

impl Automaton {
    /// Returns `zeta_W(s)` and the bound it is known to.
    ///
    /// The bound is the propagated truncation bound, carried entrywise as a nonnegative vector through `(I - q^(-w) T)^(-1)`, plus [`ROUNDING`] times the scale the ladder carries, which is a measured allowance and not a proof.
    /// Right of the abscissa the inverse is majorised by its Neumann series, `sum_i (q^(-Re w) T)^i`, which is entrywise nonnegative and needs no norm and no primitivity; left of it the bound runs through the computed inverse certified by its own residual, `abs(C) (y + norm(y) r/(1-r) 1)` with `r = norm(I - (I - q^(-w) T) C)`, and the module raises rather than return when `r >= 1`.
    ///
    /// ```
    /// let full = mrlyrs::num::memory::Rule::full(1, 2).unwrap();
    /// let ladder = mrlyrs::num::automaton::Automaton::new(&full).unwrap();
    /// let s = mrlyrs::num::zeta::Complex::new(2.0, 0.0);
    /// let (value, bound) = ladder.zeta(s, 1e-10).unwrap();
    /// assert!((value.re - std::f64::consts::PI * std::f64::consts::PI / 6.0).abs() < bound);
    /// ```
    ///
    /// # Errors
    ///
    /// Errs when the tolerance is out of reach, or when a level of the walk sits on a pole of the resolvent.
    pub fn zeta(&self, s: Complex, tolerance: f64) -> Result<(Complex, f64)> {
        let head = poly_scale(&self.low, s.re);
        let (value, carried) = self.tune(s, tolerance, true, head)?;
        let sum = value.iter().fold(Complex::new(0.0, 0.0), |acc, &z| acc + z);
        let bound = carried.iter().sum::<f64>() + ROUNDING * head;
        Ok((poly(&self.low, s) + sum, bound))
    }

    /// Returns the matrix Lyndon cofactor `Z_W(s) = det(I - q^(-s) T) zeta_W(s)` and the bound it is known to.
    ///
    /// The scalar `1 - k q^(-s)` becomes the determinant, and the cofactor is carried as `det(I - q^(-s) T) D_(P-1)(s) + 1^T adj(I - q^(-s) T) N(s)` with `N` the ladder numerator, so it is read on the whole `m = 0` pole comb where `zeta_W` itself is singular.
    ///
    /// # Errors
    ///
    /// Errs when the tolerance is out of reach.
    pub fn cofactor(&self, s: Complex, tolerance: f64) -> Result<(Complex, f64)> {
        let x = raise(self.base as f64, -s);
        let det = self.det(x);
        let head = det.abs() * poly_scale(&self.low, s.re);
        let (value, carried) = self.tune(s, tolerance, false, head)?;
        let lifted = self.adjugate_apply(x, &value);
        let sum = lifted
            .iter()
            .fold(Complex::new(0.0, 0.0), |acc, &z| acc + z);
        let weights = self.adjugate_weights(x.abs());
        let bound = weights
            .iter()
            .zip(carried.iter())
            .map(|(&m, &b)| m * b)
            .sum::<f64>()
            + ROUNDING * head;
        Ok((det * poly(&self.low, s) + sum, bound))
    }

    /// Returns the residue of `zeta_W` at a simple pole `w0` of the resolvent and the bound it is known to.
    ///
    /// At such a point the resolvent is `adj(I - x T) / det(I - x T)` with `x = q^(-w)`, so the residue is `1^T adj(I - x0 T) N(w0)` over `-x0 log q det'(x0)`: the adjugate is the spectral projector in polynomial form, and no eigenvector is solved for.
    /// The head term `det(I - q^(-s) T) D_(P-1)(s)` is dropped, since the determinant vanishes at the pole.
    ///
    /// # Errors
    ///
    /// Errs when the tolerance is out of reach, or when `w0` is not a simple root of `det(I - q^(-w) T)`.
    pub fn residue(&self, w0: Complex, tolerance: f64) -> Result<(Complex, f64)> {
        let base = self.base as f64;
        let x = raise(base, -w0);
        let slope = self.det_slope(x);
        let den = -x * base.ln() * slope;
        if self.det(x).abs() > 1e-8 * self.charpoly.iter().map(|c| c.abs()).sum::<f64>().max(1.0) {
            return value_error(format!(
                "w0 = {} + {}i is not a root of det(I - q^(-w) T).",
                w0.re, w0.im
            ));
        }
        if den.abs() < 1e-12 {
            return value_error(format!(
                "the root at w0 = {} + {}i is not simple, so the pole order exceeds one.",
                w0.re, w0.im
            ));
        }
        let (value, carried) = self.tune(w0, tolerance, false, 0.0)?;
        let lifted = self.adjugate_apply(x, &value);
        let sum = lifted
            .iter()
            .fold(Complex::new(0.0, 0.0), |acc, &z| acc + z);
        let weights = self.adjugate_weights(x.abs());
        let bound = weights
            .iter()
            .zip(carried.iter())
            .map(|(&m, &b)| m * b)
            .sum::<f64>()
            / den.abs();
        Ok((sum / den, bound))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::num::ladder::Design;

    const PINNED: f64 = 1e-9;

    fn near(value: Complex, re: f64, im: f64, bound: f64) {
        assert!(
            (value.re - re).abs() < bound && (value.im - im).abs() < bound,
            "{value:?} is not {re} + {im}i inside {bound:e}"
        );
    }

    fn golden() -> Automaton {
        Automaton::new(&Rule::new(1, 2, 7).unwrap()).unwrap()
    }

    #[test]
    fn the_full_rule_is_the_base_two_design() {
        let design = Design::new(2, &[0, 1]).unwrap();
        for width in [2usize, 3] {
            let ladder = Automaton::new(&Rule::full(1, width).unwrap()).unwrap();
            assert_eq!(ladder.states(), 1 << (width - 1));
            assert_eq!(ladder.abscissa(), 1.0);
            let mut wanted = vec![0.0; 1 << (width - 1)];
            wanted[0] = 1.0;
            if width > 1 {
                wanted[1] = -2.0;
            }
            wanted.push(0.0);
            assert_eq!(ladder.denominator(), wanted);
            for s in [Complex::new(2.0, 0.0), Complex::new(0.3, 40.0)] {
                let (mine, one) = ladder.zeta(s, 1e-8).unwrap();
                let (theirs, two) = crate::num::ladder::zeta(&design, s, 1e-8).unwrap();
                assert!(
                    (mine - theirs).abs() < one + two,
                    "width {width} at {s:?}: {mine:?} {theirs:?} {one:e} {two:e}"
                );
            }
        }
    }

    #[test]
    fn the_product_rule_is_the_mersenne_series() {
        let ladder = Automaton::new(&Rule::new(1, 2, 8).unwrap()).unwrap();
        assert!(ladder.abscissa().abs() < 1e-12);
        for s in [Complex::new(2.0, 0.0), Complex::new(1.5, 3.0)] {
            let (value, bound) = ladder.zeta(s, 1e-9).unwrap();
            let mut want = Complex::new(0.0, 0.0);
            for m in 1..=60u32 {
                want = want + ((-s) * ((2f64.powi(m as i32) - 1.0).ln())).exp();
            }
            assert!(
                (value - want).abs() < bound + 1e-15,
                "{s:?} {value:?} {want:?}"
            );
        }
    }

    #[test]
    fn the_golden_rule_is_the_fibbinary_series() {
        let ladder = golden();
        assert_eq!(ladder.peel(), 12);
        let (value, bound) = ladder.zeta(Complex::new(3.0, 0.0), PINNED).unwrap();
        let (mut want, mut drift) = (0.0f64, 0.0f64);
        for n in 1u64..1 << 22 {
            if n & (n << 1) == 0 {
                let term = (n as f64).powf(-3.0) - drift;
                let total = want + term;
                drift = (total - want) - term;
                want = total;
            }
        }
        let (mut a, mut b) = (1.0f64, 1.0f64);
        let mut rest = 0.0f64;
        for _ in 1..=22u32 {
            let next = a + b;
            a = b;
            b = next;
        }
        for step in 0..400u32 {
            rest += a * 2f64.powf(-3.0 * (22 + step) as f64);
            let next = a + b;
            a = b;
            b = next;
        }
        assert!(
            value.re > want - bound && value.re < want + rest + bound,
            "{value:?} {want} {rest:e} {bound:e}"
        );
    }

    #[test]
    fn the_golden_transfer_carries_the_two_combs() {
        let ladder = golden();
        let phi = (1.0 + 5f64.sqrt()) / 2.0;
        let (low, high) = ladder.perron();
        assert!(low <= phi && phi <= high, "{low} {phi} {high}");
        assert!(high - low < 1e-9, "{low} {high}");
        let coefficients = ladder.denominator();
        assert_eq!(coefficients.len(), 3);
        assert!((coefficients[1] + 1.0).abs() < 1e-12);
        assert!((coefficients[2] + 1.0).abs() < 1e-12);
        assert_eq!(ladder.abscissa(), phi.log2());
    }

    #[test]
    fn the_residues_on_the_first_comb_are_the_contour_averages() {
        let ladder = golden();
        let phi = (1.0 + 5f64.sqrt()) / 2.0;
        for j in 0..3i64 {
            let w0 = Complex::new(phi.log2(), ladder.period() * j as f64);
            let (want, bound) = ladder.residue(w0, PINNED).unwrap();
            let (radius, nodes) = (0.05, 8);
            let mut got = Complex::new(0.0, 0.0);
            let mut carried = 0.0;
            for k in 0..nodes {
                let turn = Complex::turn(std::f64::consts::TAU * k as f64 / nodes as f64);
                let (value, other) = ladder.zeta(w0 + turn * radius, 1e-9).unwrap();
                got = got + turn * radius * value * (1.0 / nodes as f64);
                carried += radius * other / nodes as f64;
            }
            assert!(
                (got - want).abs() < bound + carried + 1e-6,
                "j {j} {got:?} {want:?}"
            );
        }
    }

    #[test]
    fn the_cofactor_is_the_series_times_the_determinant() {
        let ladder = golden();
        for s in [Complex::new(2.0, 0.0), Complex::new(1.2, 9.0)] {
            let x = raise(2.0, -s);
            let mut det = Complex::new(1.0, 0.0);
            let mut power = Complex::new(1.0, 0.0);
            for &c in ladder.denominator().iter().skip(1) {
                power = power * x;
                det = det + power * c;
            }
            let (value, bound) = ladder.cofactor(s, 1e-8).unwrap();
            let (series, other) = ladder.zeta(s, 1e-8).unwrap();
            assert!(
                (value - det * series).abs() < bound + det.abs() * other,
                "{s:?} {value:?} {:?}",
                det * series
            );
        }
    }

    #[test]
    fn the_golden_residues_on_the_first_comb_are_pinned() {
        let ladder = golden();
        let phi = (1.0 + 5f64.sqrt()) / 2.0;
        let wanted = [
            (0.946_743_395_641_970_1, 0.0),
            (0.210_170_579_042_707_6, -0.581_938_842_807_736_5),
            (0.062_192_494_764_691_71, -0.051_732_573_832_334_72),
        ];
        for (j, (re, im)) in wanted.iter().enumerate() {
            let w0 = Complex::new(phi.log2(), ladder.period() * j as f64);
            let (value, bound) = ladder.residue(w0, PINNED).unwrap();
            assert!(bound < 1.3e-13, "j {j} bound {bound:e}");
            near(value, *re, *im, bound);
        }
    }

    #[test]
    fn the_second_comb_is_genuine() {
        let ladder = golden();
        let phi = (1.0 + 5f64.sqrt()) / 2.0;
        let half = std::f64::consts::PI / 2f64.ln();
        let wanted = [
            (-0.259_501_222_742_937_13, -0.592_535_006_433_179_4),
            (0.896_350_590_641_920_6, 1.403_072_744_223_695_2),
            (0.491_379_388_883_392_94, -3.790_264_223_035_055_4),
        ];
        for (j, (re, im)) in wanted.iter().enumerate() {
            let w0 = Complex::new(-phi.log2(), half * (2 * j + 1) as f64);
            let (value, bound) = ladder.residue(w0, 1e-7).unwrap();
            assert!(value.abs() > 0.6, "j {j} residue {value:?}");
            near(value, *re, *im, bound);
            let (radius, nodes) = (0.05, 12);
            let mut got = Complex::new(0.0, 0.0);
            let mut carried = 0.0;
            for k in 0..nodes {
                let turn = Complex::turn(std::f64::consts::TAU * k as f64 / nodes as f64);
                let (point, other) = ladder.zeta(w0 + turn * radius, 1e-5).unwrap();
                got = got + turn * radius * point * (1.0 / nodes as f64);
                carried += radius * other / nodes as f64;
            }
            assert!(
                (got - value).abs() < bound + carried + 1e-6,
                "j {j} {got:?} {value:?}"
            );
        }
    }

    #[test]
    fn the_residue_at_the_abscissa_is_the_limit_of_the_digit_sums() {
        let ladder = golden();
        let alpha = ladder.abscissa();
        let (value, bound) = ladder.residue(Complex::new(alpha, 0.0), PINNED).unwrap();
        let wanted = [
            (16usize, 0.946_747_043_404_283_4, 4e-6),
            (20, 0.946_743_630_023_051_8, 3e-7),
            (24, 0.946_743_410_426_742, 2e-8),
        ];
        for (length, want, reach) in wanted {
            let (mut sum, mut drift) = (0.0f64, 0.0f64);
            for n in (1u64 << (length - 1))..(1u64 << length) {
                if n & (n << 1) == 0 {
                    let term = (n as f64).powf(-alpha) - drift;
                    let total = sum + term;
                    drift = (total - sum) - term;
                    sum = total;
                }
            }
            let got = sum / 2f64.ln();
            assert!((got - want).abs() < 1e-12, "L {length} {got}");
            assert!(
                (got - value.re).abs() < reach + bound,
                "L {length} {got} {value:?}"
            );
        }
    }

    #[test]
    fn the_golden_readings_are_pinned() {
        let ladder = golden();
        let pinned = [
            (Complex::new(3.0, 0.0), 1.154_012_963_277_640_1, 0.0),
            (Complex::new(2.0, 0.0), 1.415_825_532_884_778_4, 0.0),
            (
                Complex::new(1.2, 9.0),
                1.906_409_024_243_908_5,
                -0.453_243_424_778_265_46,
            ),
            (Complex::new(0.8, 0.0), 9.536_379_694_275_015, 0.0),
        ];
        for (s, re, im) in pinned {
            let (value, bound) = ladder.zeta(s, PINNED).unwrap();
            near(value, re, im, bound);
        }
        let cofactors = [
            (Complex::new(3.0, 0.0), 0.991_729_890_316_722),
            (Complex::new(2.0, 0.0), 0.973_380_053_858_285_1),
            (Complex::new(0.8, 0.0), 0.913_335_748_872_126_1),
        ];
        for (s, re) in cofactors {
            let (value, bound) = ladder.cofactor(s, PINNED).unwrap();
            near(value, re, 0.0, bound);
        }
    }

    #[test]
    fn the_fibbinary_set_carries_no_euler_product() {
        let fib = |n: u64| n & (n << 1) == 0;
        let mut found = None;
        'outer: for product in 2u64..1 << 16 {
            for a in 2u64..=(product as f64).sqrt() as u64 + 1 {
                if product % a != 0 {
                    continue;
                }
                let b = product / a;
                if a >= b || !fib(a) || !fib(b) {
                    continue;
                }
                let (mut x, mut y) = (a, b);
                while y != 0 {
                    let t = x % y;
                    x = y;
                    y = t;
                }
                if x != 1 || fib(product) {
                    continue;
                }
                found = Some((a, b, product));
                break 'outer;
            }
        }
        assert_eq!(found, Some((5, 9, 45)));
    }

    #[test]
    fn the_abscissa_is_the_exact_perron_root_and_not_the_bracket() {
        let jordan = Automaton::new(&Rule::new(1, 2, 13).unwrap()).unwrap();
        assert_eq!(jordan.matrix(), vec![vec![1.0, 1.0], vec![0.0, 1.0]]);
        assert_eq!(jordan.abscissa(), 0.0);
        let (_, high) = jordan.perron();
        assert!(high > 1.03 && high < 1.04, "{high}");
    }

    #[test]
    fn refuses_a_level_it_cannot_invert() {
        let ladder = golden();
        let phi = (1.0 + 5f64.sqrt()) / 2.0;
        let w0 = Complex::new(phi.log2(), 0.0);
        assert!(ladder.zeta(w0, 1e-6).is_err());
        assert!(ladder.residue(Complex::new(2.0, 0.0), 1e-6).is_err());
    }
}
