use crate::core::error::{overflow_error, value_error, Result};
use crate::num::zeta::Complex;
use serde::{Deserialize, Serialize};
use std::f64::consts::{FRAC_PI_4, LN_2, PI};

/// The limit of the plane Wallis sieve's surviving area, pi over four.
pub const PLANE_LIMIT: f64 = FRAC_PI_4;

/// Returns the classical Wallis schedule, the odd sides three, five, seven and on, to the count of levels.
///
/// ```
/// assert_eq!(mrlyrs::num::sieve::odd_word(4), vec![3, 5, 7, 9]);
/// ```
pub fn odd_word(levels: usize) -> Vec<u64> {
    (1..=levels as u64).map(|k| 2 * k + 1).collect()
}

/// Returns the constant schedule, one odd side repeated to the count of levels, whose limit set is the fixed-ratio carpet.
///
/// ```
/// assert_eq!(mrlyrs::num::sieve::flat_word(3, 3), vec![3, 3, 3]);
/// ```
pub fn flat_word(side: u64, levels: usize) -> Vec<u64> {
    vec![side; levels]
}

fn letter(side: u64) -> Result<u64> {
    if side < 3 || side.is_multiple_of(2) {
        return value_error(format!("a letter is an odd side from three, not {side}."));
    }
    Ok(side)
}

/// Returns the side of the word, the product of its letters' sides.
///
/// ```
/// assert_eq!(mrlyrs::num::sieve::side(&mrlyrs::num::sieve::odd_word(4)).unwrap(), 945);
/// ```
///
/// # Errors
///
/// Errs when a letter is not an odd side from three, or when the product overruns a u128.
pub fn side(word: &[u64]) -> Result<u128> {
    let mut run = 1u128;
    for &s in word {
        run = match run.checked_mul(u128::from(letter(s)?)) {
            Some(value) => value,
            None => return overflow_error("the word's side overruns a u128."),
        };
    }
    Ok(run)
}

/// Returns the cells the word leaves, the product of its letters' fills, one punctured tile a letter.
///
/// ```
/// assert_eq!(mrlyrs::num::sieve::cells(&mrlyrs::num::sieve::odd_word(3), 2).unwrap(), 9216);
/// ```
///
/// # Errors
///
/// Errs when a letter is not an odd side from three, or when the product overruns a u128.
pub fn cells(word: &[u64], dimension: u32) -> Result<u128> {
    let mut run = 1u128;
    for &s in word {
        let fill = u128::from(letter(s)?).pow(dimension) - 1;
        run = match run.checked_mul(fill) {
            Some(value) => value,
            None => return overflow_error("the word's cells overrun a u128."),
        };
    }
    Ok(run)
}

/// Returns the punctures the word makes, one per surviving cell at every level.
///
/// ```
/// assert_eq!(mrlyrs::num::sieve::holes(&mrlyrs::num::sieve::odd_word(3), 2).unwrap(), 1 + 8 + 192);
/// ```
///
/// # Errors
///
/// Errs when a letter is not an odd side from three, or when the count overruns a u128.
pub fn holes(word: &[u64], dimension: u32) -> Result<u128> {
    let mut total = 0u128;
    for place in 0..word.len() {
        total += cells(&word[..place], dimension)?;
    }
    Ok(total)
}

/// Returns the share of the whole the word leaves, the product of one minus the inverse of each letter's site count, exact as a product of the letters' fills.
///
/// ```
/// let flat = mrlyrs::num::sieve::ratio(&mrlyrs::num::sieve::flat_word(3, 2), 2).unwrap();
/// assert!((flat - 64.0 / 81.0).abs() < 1e-15);
/// ```
///
/// # Errors
///
/// Errs when a letter is not an odd side from three.
pub fn ratio(word: &[u64], dimension: u32) -> Result<f64> {
    let mut run = 1.0f64;
    for &s in word {
        run *= 1.0 - 1.0 / (letter(s)? as f64).powi(dimension as i32);
    }
    Ok(run)
}

/// Returns the box exponent the word reads at its own scale, the logarithm of its cells over the logarithm of its side, which walks up to the dimension on a schedule of distinct growing letters and stands still on any schedule that reuses its letters.
///
/// Changing the letter is not enough: the alternating word 3, 5, 3, 5 changes at every step and freezes at log 192 / log 15, its ratio falling to nothing like a fixed-ratio schedule. The hypothesis that buys a positive area is strictly increasing odd letters, under which sum s_k^(-d) converges.
///
/// ```
/// let carpet = mrlyrs::num::sieve::exponent(&mrlyrs::num::sieve::flat_word(3, 5), 2).unwrap();
/// assert!((carpet - 8f64.ln() / 3f64.ln()).abs() < 1e-12);
/// ```
///
/// # Errors
///
/// Errs when a letter is not an odd side from three.
pub fn exponent(word: &[u64], dimension: u32) -> Result<f64> {
    let mut up = 0.0f64;
    let mut down = 0.0f64;
    for &s in word {
        let s = letter(s)? as f64;
        up += (s.powi(dimension as i32) - 1.0).ln();
        down += s.ln();
    }
    if down == 0.0 {
        return Ok(dimension as f64);
    }
    Ok(up / down)
}

/// Builds the plane sieve the word spells as a raster: its side, then one byte a site, row by row, one where the site survives and zero where a level punched it out.
///
/// ```
/// let (side, cells) = mrlyrs::num::sieve::raster(&mrlyrs::num::sieve::odd_word(2)).unwrap();
/// assert_eq!(side, 15);
/// assert_eq!(cells.iter().filter(|&&b| b == 1).count(), 192);
/// ```
///
/// # Errors
///
/// Errs when a letter is not an odd side from three.
pub fn raster(word: &[u64]) -> Result<(usize, Vec<u8>)> {
    let mut side = 1usize;
    let mut sites = vec![1u8];
    for &s in word {
        let s = letter(s)? as usize;
        let half = s / 2;
        let wide = side * s;
        let mut next = vec![0u8; wide * wide];
        for row in 0..side {
            for col in 0..side {
                if sites[row * side + col] == 0 {
                    continue;
                }
                for i in 0..s {
                    for j in 0..s {
                        if i == half && j == half {
                            continue;
                        }
                        next[(row * s + i) * wide + col * s + j] = 1;
                    }
                }
            }
        }
        side = wide;
        sites = next;
    }
    Ok((side, sites))
}

/// Lists every puncture the word makes in the given dimension: its corner along each axis and then its side, all in units of the word's finest cell, so a level-one hole is the widest block in the list.
///
/// ```
/// let holes = mrlyrs::num::sieve::punctures(&mrlyrs::num::sieve::odd_word(2), 2).unwrap();
/// assert_eq!(holes.len() / 3, 9);
/// assert_eq!(&holes[..3], &[5, 5, 5]);
/// ```
///
/// # Errors
///
/// Errs when a letter is not an odd side from three, or when the word's side overruns a u128.
pub fn punctures(word: &[u64], dimension: u32) -> Result<Vec<u64>> {
    let axes = dimension as usize;
    let mut scale = side(word)? as u64;
    let mut alive: Vec<u64> = vec![0; axes];
    let mut out: Vec<u64> = Vec::new();
    let mut digits = vec![0u64; axes];
    for (place, &s) in word.iter().enumerate() {
        let s = letter(s)?;
        let half = s / 2;
        let last = place + 1 == word.len();
        scale /= s;
        let count = s.pow(dimension);
        let mut next: Vec<u64> = Vec::new();
        for origin in alive.chunks(axes) {
            for axis in origin {
                out.push((axis * s + half) * scale);
            }
            out.push(scale);
            if last {
                continue;
            }
            for index in 0..count {
                let mut rest = index;
                let mut centre = true;
                for axis in (0..axes).rev() {
                    digits[axis] = rest % s;
                    centre &= digits[axis] == half;
                    rest /= s;
                }
                if centre {
                    continue;
                }
                for axis in 0..axes {
                    next.push(origin[axis] * s + digits[axis]);
                }
            }
        }
        if !last {
            alive = next;
        }
    }
    Ok(out)
}

// THE SOLID LIMIT

const SPLIT: u64 = 1001;

fn odd_tail(start: u64, power: i32) -> f64 {
    let n = start as f64;
    let s = f64::from(power);
    let p = n.powi(-power);
    p * n / (2.0 * (s - 1.0)) + p / 2.0 + s * p / (6.0 * n)
        - s * (s + 1.0) * (s + 2.0) * p / (90.0 * n * n * n)
}

/// Returns the limit of the solid Wallis sieve's surviving volume, the product of one minus n to the minus three over the odd n from three, in closed form.
///
/// Write m = 2k + 1 and factor m^3 - 1 = (m - 1)(m - w)(m - w^2) at w = exp(2 pi i / 3), the three cube roots of one; dividing by m^3 = 8 (k + 1/2)^3 turns the k-th factor into k (k + (1 - w)/2) (k + (1 - w^2)/2) / (k + 1/2)^3, so the product is a ratio of three rising shifts over one repeated three times.
///
/// The three shifts a1 = 0, a2 = (1 - w)/2 and a3 = (1 - w^2)/2 sum to (2 - w - w^2)/2 = 3/2, exactly three times the shift b = 1/2 below, and prod_{k=1..N} (k + a) = Gamma(N + 1 + a) / Gamma(1 + a) with Gamma(N + 1 + a) / Gamma(N + 1 + b) ~ N^(a - b), so the N-dependent factor tends to N^(a1 + a2 + a3 - 3b) = 1 and the limit is Gamma(1 + b)^3 / (Gamma(1 + a1) Gamma(1 + a2) Gamma(1 + a3)).
///
/// That is Gamma(3/2)^3 / (Gamma(1) Gamma((3 - w)/2) Gamma((3 - w^2)/2)) = pi^(3/2) / (8 |Gamma(7/4 - i sqrt(3)/4)|^2), the two gamma values being conjugates; the same Weierstrass product gives the identity prod_{k >= 1} (1 - a^3/k^3) = 1 / (Gamma(1 - a) Gamma(1 - a w) Gamma(1 - a w^2)) the ratio form of this limit leans on, at a = 1/2 against prod_{n >= 2} (1 - n^-3) = cosh(pi sqrt(3)/2) / (3 pi).
///
/// The value is read from the logarithm rather than from that gamma ratio, which cancels four digits away: log P = sum over odd n from 3 to 999 of log(1 - n^-3), taken from its smallest term, minus sum_{j = 1..3} (1/j) sum_{n odd >= 1001} n^(-3j), each inner sum by Euler-Maclaurin through the fourth Bernoulli term. Every piece carries one sign, so nothing cancels and the exponential lands within a few ulps.
///
/// ```
/// assert!((mrlyrs::num::sieve::solid_limit() - 0.948_815_485_719_679_6).abs() < 4e-16);
/// ```
pub fn solid_limit() -> f64 {
    let mut log = 0.0f64;
    let mut n = SPLIT - 2;
    while n >= 3 {
        log += (-(n as f64).powi(-3)).ln_1p();
        n -= 2;
    }
    for j in 1..=3i32 {
        log -= odd_tail(SPLIT, 3 * j) / f64::from(j);
    }
    log.exp()
}

/// Returns the limit the word's schedule walks to in the given dimension, when the word names a schedule at all.
///
/// A run of consecutive odd letters from a, of two letters or more, names the schedule a, a + 2, a + 4 and on: its letters are strictly increasing, sum s_k^(-d) converges, and the limit is positive, pi over four in the plane and the closed form above in the cube divided by the head the run skips. Two or more copies of one letter name the fixed-ratio schedule, whose limit is zero; that is what repetition costs, and an alternating word such as 3, 5, 3, 5 changes at every step and still loses the whole measure. Every other word, and every word of one letter, names no schedule and reads None.
///
/// ```
/// assert_eq!(mrlyrs::num::sieve::limit(&mrlyrs::num::sieve::odd_word(3), 2).unwrap(), Some(std::f64::consts::FRAC_PI_4));
/// assert_eq!(mrlyrs::num::sieve::limit(&mrlyrs::num::sieve::flat_word(3, 3), 2).unwrap(), Some(0.0));
/// assert!((mrlyrs::num::sieve::limit(&[5, 7, 9], 2).unwrap().unwrap() - 0.883_572_933_822_129_3).abs() < 1e-15);
/// assert_eq!(mrlyrs::num::sieve::limit(&[3, 7, 11], 2).unwrap(), None);
/// ```
///
/// # Errors
///
/// Errs when a letter is not an odd side from three.
pub fn limit(word: &[u64], dimension: u32) -> Result<Option<f64>> {
    if word.len() < 2 {
        return Ok(None);
    }
    let first = letter(word[0])?;
    let mut flat = true;
    for &s in word {
        flat &= letter(s)? == first;
    }
    if flat {
        return Ok(Some(0.0));
    }
    if !word
        .iter()
        .enumerate()
        .all(|(k, &s)| s == first + 2 * k as u64)
    {
        return Ok(None);
    }
    let whole = match dimension {
        2 => PLANE_LIMIT,
        3 => solid_limit(),
        _ => return Ok(None),
    };
    let head = odd_word(((first - 3) / 2) as usize);
    Ok(Some(whole / ratio(&head, dimension)?))
}

/// Returns the Wallis sieve product of a parity design, `prod_(N odd >= 3) (1 - N^-dim)` for the corners with an odd count of odd coordinates and `prod (1 + N^-dim)` for the even count, at `dim >= 2`.
///
/// At odd `N` the sum of `(-1)^(x_1 + ... + x_dim)` over the box is 1, so the two designs fill `(N^dim -+ 1)/2` and the row word's constant is this product. The logarithm is summed over odd `N` from 3 to 999, smallest term first, and the tail past 1001 is the series `sum_j (-+1)^j / j sum N^(-dim j)`, each inner sum by Euler-Maclaurin through the fourth Bernoulli term, as for the solid limit.
///
/// ```
/// let plane = mrlyrs::num::sieve::parity_product(2, true).unwrap();
/// assert!((plane - std::f64::consts::FRAC_PI_4).abs() < 1e-15);
/// ```
///
/// # Errors
///
/// Errs below two dimensions, where the product runs off to 0 or to infinity.
pub fn parity_product(dimension: u32, odd: bool) -> Result<f64> {
    if dimension < 2 {
        return value_error(format!(
            "a parity product converges from dim 2, not dim {dimension}."
        ));
    }
    let power = dimension as i32;
    let sign = if odd { -1.0 } else { 1.0 };
    let mut log = 0.0f64;
    let mut n = SPLIT - 2;
    while n >= 3 {
        log += (sign * (n as f64).powi(-power)).ln_1p();
        n -= 2;
    }
    for j in 1..=4i32 {
        let term = odd_tail(SPLIT, power * j) / f64::from(j);
        log += if odd || j % 2 == 0 { -term } else { term };
    }
    Ok(log.exp())
}

// THE ROW WORD

const ROW_DIMENSIONS: usize = 3;

const STIRLING: [f64; 8] = [
    1.0 / 12.0,
    -1.0 / 360.0,
    1.0 / 1260.0,
    -1.0 / 1680.0,
    1.0 / 1188.0,
    -691.0 / 360_360.0,
    1.0 / 156.0,
    -3617.0 / 122_400.0,
];

/// One base-2 design read along the row word of odd sides `3, 5, ..., 2L+1`, out of its fill polynomial `P_F(n) = sum_j a_j n^(dim-j) (n-1)^j`.
///
/// The fill ratio is `(w/2^dim)^L L^drift C (1 + c_1/L + O(L^-2))`, `w` the corners, `drift = dim/2 - mean` and `c_1 = dim/8 + drift - var/2` over the odd count of the corners, and `C = Gamma(3/2)^dim / prod_i Gamma(2 - r_i)` over the roots of `P_F`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Row {
    /// The odd-count profile: `a_j` corners with `j` odd coordinates, `j` from 0 to `dim`.
    pub profile: Vec<u64>,
    /// The drift, the power of `L` the fill carries past its ratio per level, as a reduced fraction.
    pub drift: (i64, i64),
    /// The first correction `c_1`, as a reduced fraction.
    pub correction: (i64, i64),
    /// The roots of the fill polynomial: the zeros, the ones, the rational roots, then the rest.
    pub roots: Vec<Complex>,
    /// The constant `C`, the Gamma form at the roots.
    pub constant: f64,
    /// The constant spelled: a rational, a power of `sqrt(pi)`, and the Gamma, `cosh`, `cos` or `sin` values the roots leave.
    pub closed: String,
    /// The constant by reflection alone, `(sqrt(pi)/2)^(m_0 + m_1) prod_pairs sin(pi r)/(4 r (1-r))`, when every root outside `0, 1/2, 1` pairs with `1 - r`.
    pub reflection: Option<f64>,
    /// The constant times the mirror design's, `prod_i sin(pi r_i)/(4 r_i (1 - r_i))` with `pi/4` at a root 0 or 1.
    pub mirror: f64,
    /// For a parity design at `dim >= 2`: whether its corners hold an odd count of odd coordinates, and its Wallis sieve product.
    pub parity: Option<(bool, f64)>,
}

struct Split {
    zero: usize,
    one: usize,
    rational: Vec<(i128, i128)>,
    rest: Vec<i128>,
}

struct Form {
    num: i128,
    den: i128,
    root: i128,
    halves: u32,
    top: Vec<String>,
    bottom: Vec<String>,
}

fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a.max(1)
}

fn binomial(n: usize, k: usize) -> i128 {
    (0..k).fold(1i128, |acc, i| acc * (n - i) as i128 / (i + 1) as i128)
}

fn reduced(p: i128, q: i128) -> (i64, i64) {
    let g = gcd(p, q) * q.signum();
    ((p / g) as i64, (q / g) as i64)
}

fn fraction(p: i128, q: i128) -> String {
    let (p, q) = reduced(p, q);
    if q == 1 {
        p.to_string()
    } else {
        format!("{p}/{q}")
    }
}

fn square_part(n: i128) -> (i128, i128) {
    let (mut s, mut t, mut f) = (1i128, n, 2i128);
    while f * f <= t {
        while t % (f * f) == 0 {
            t /= f * f;
            s *= f;
        }
        f += 1;
    }
    (s, t)
}

fn surd(s: i128, t: i128, v: i128) -> String {
    let (s, v) = reduced(s, v);
    let mut words = Vec::new();
    if s != 1 || t == 1 {
        words.push(s.to_string());
    }
    if t > 1 {
        words.push(format!("sqrt{t}"));
    }
    let text = words.join(" ");
    if v == 1 {
        text
    } else {
        format!("{text}/{v}")
    }
}

fn angle(s: i128, t: i128, v: i128) -> String {
    let (s, v) = reduced(s, v);
    let (s, v) = (i128::from(s), i128::from(v));
    let mut words = Vec::new();
    if s != 1 {
        words.push(s.to_string());
    }
    words.push("pi".to_string());
    if t > 1 && v % t == 0 {
        let k = v / t;
        let under = if k == 1 {
            format!("sqrt{t}")
        } else {
            format!("({k} sqrt{t})")
        };
        return format!("{}/{under}", words.join(" "));
    }
    if t > 1 {
        words.push(format!("sqrt{t}"));
    }
    let text = words.join(" ");
    if v == 1 {
        text
    } else {
        format!("{text}/{v}")
    }
}

fn merged(tokens: &[String]) -> Vec<String> {
    let mut seen: Vec<(String, usize)> = Vec::new();
    for token in tokens {
        match seen.iter_mut().find(|entry| entry.0 == *token) {
            Some(entry) => entry.1 += 1,
            None => seen.push((token.clone(), 1)),
        }
    }
    seen.into_iter()
        .map(|(t, k)| if k == 1 { t } else { format!("{t}^{k}") })
        .collect()
}

impl Form {
    fn scale(&mut self, p: i128, q: i128) {
        self.num *= p;
        self.den *= q;
        let g = gcd(self.num, self.den);
        self.num /= g;
        self.den /= g;
    }

    fn radical(&mut self, t: i128) {
        let (s, t) = square_part(self.root * t);
        self.root = t;
        self.scale(s, 1);
    }

    fn spell(&self) -> String {
        let mut upper = Vec::new();
        if self.num != 1 {
            upper.push(self.num.to_string());
        }
        if self.root > 1 {
            upper.push(format!("sqrt{}", self.root));
        }
        match self.halves {
            0 => {}
            1 => upper.push("sqrt(pi)".to_string()),
            2 => upper.push("pi".to_string()),
            h if h % 2 == 0 => upper.push(format!("pi^{}", h / 2)),
            h => upper.push(format!("pi^({h}/2)")),
        }
        upper.extend(merged(&self.top));
        let upper = if upper.is_empty() {
            "1".to_string()
        } else {
            upper.join(" ")
        };
        let mut lower = Vec::new();
        if self.den != 1 {
            lower.push(self.den.to_string());
        }
        lower.extend(merged(&self.bottom));
        match lower.len() {
            0 => upper,
            1 => format!("{upper}/{}", lower[0]),
            _ => format!("{upper}/({})", lower.join(" ")),
        }
    }
}

fn row_dimension(profile: &[u64]) -> Result<usize> {
    let dimension = profile.len().saturating_sub(1);
    if !(1..=ROW_DIMENSIONS).contains(&dimension) {
        return value_error(format!(
            "a row profile holds dim + 1 counts at dim 1 to {ROW_DIMENSIONS}, not {}.",
            profile.len()
        ));
    }
    for (j, &a) in profile.iter().enumerate() {
        if i128::from(a) > binomial(dimension, j) {
            return value_error(format!(
                "no design in dim {dimension} has {a} corners with {j} odd coordinates."
            ));
        }
    }
    if profile.iter().all(|&a| a == 0) {
        return value_error("the empty design fills nothing.");
    }
    Ok(dimension)
}

fn scaled(poly: &[i128], p: i128, q: i128) -> i128 {
    let degree = poly.len() - 1;
    poly.iter()
        .enumerate()
        .map(|(k, &c)| c * p.pow(k as u32) * q.pow((degree - k) as u32))
        .sum()
}

fn divide(poly: &[i128], p: i128, q: i128) -> Vec<i128> {
    let degree = poly.len() - 1;
    let mut out = vec![0i128; degree];
    out[degree - 1] = poly[degree] / q;
    for k in (1..degree).rev() {
        out[k - 1] = (poly[k] + p * out[k]) / q;
    }
    out
}

fn split(profile: &[u64], dimension: usize) -> Split {
    let low = profile.iter().position(|&a| a > 0).unwrap_or(0);
    let high = profile.iter().rposition(|&a| a > 0).unwrap_or(0);
    let mut core = vec![0i128; high - low + 1];
    for (j, &a) in profile.iter().enumerate().take(high + 1).skip(low) {
        let m = j - low;
        for k in 0..=m {
            let sign = if (m - k) % 2 == 0 { 1 } else { -1 };
            core[high - j + k] += i128::from(a) * binomial(m, k) * sign;
        }
    }
    let mut rational = Vec::new();
    'search: while core.len() > 1 {
        let lead = core[core.len() - 1].abs();
        for q in 2..=lead {
            if lead % q != 0 {
                continue;
            }
            for p in 1..q {
                if gcd(p, q) == 1 && scaled(&core, p, q) == 0 {
                    rational.push((p, q));
                    core = divide(&core, p, q);
                    continue 'search;
                }
            }
        }
        break;
    }
    Split {
        zero: dimension - high,
        one: low,
        rational,
        rest: core,
    }
}

fn quadratic(c: f64, b: f64, a: f64) -> [Complex; 2] {
    let disc = b * b - 4.0 * a * c;
    let re = -b / (2.0 * a);
    let half = disc.abs().sqrt() / (2.0 * a);
    if disc < 0.0 {
        [Complex::new(re, -half), Complex::new(re, half)]
    } else {
        [Complex::new(re - half, 0.0), Complex::new(re + half, 0.0)]
    }
}

fn rest_roots(rest: &[i128]) -> Vec<Complex> {
    let c: Vec<f64> = rest.iter().map(|&x| x as f64).collect();
    match c.len() {
        3 => quadratic(c[0], c[1], c[2]).to_vec(),
        4 => {
            let f = |x: f64| ((c[3] * x + c[2]) * x + c[1]) * x + c[0];
            let rising = f(1.0) > f(0.0);
            let (mut low, mut high) = (0.0f64, 1.0f64);
            for _ in 0..200 {
                let mid = 0.5 * (low + high);
                if (f(mid) > 0.0) == rising {
                    high = mid;
                } else {
                    low = mid;
                }
            }
            let r = 0.5 * (low + high);
            let b = c[2] + r * c[3];
            let mut out = vec![Complex::new(r, 0.0)];
            out.extend(quadratic(c[1] + r * b, b, c[3]));
            out
        }
        _ => Vec::new(),
    }
}

fn ln_one_plus(u: Complex) -> Complex {
    let size = 0.5 * (2.0 * u.re + u.re * u.re + u.im * u.im).ln_1p();
    Complex::new(size, u.im.atan2(1.0 + u.re))
}

fn stirling(z: Complex) -> Complex {
    let inverse = Complex::new(1.0, 0.0) / z;
    let square = inverse * inverse;
    let mut power = inverse;
    let mut series = Complex::new(0.0, 0.0);
    for c in STIRLING {
        series = series + power * c;
        power = power * square;
    }
    series
}

fn gamma_step(r: Complex) -> Complex {
    let delta = Complex::new(0.5 - r.re, -r.im);
    let mut low = 1.5f64;
    let mut sum = Complex::new(0.0, 0.0);
    while low < 16.0 {
        sum = sum - ln_one_plus(delta * (1.0 / low));
        low += 1.0;
    }
    let near = Complex::new(low, 0.0);
    let far = near + delta;
    let main = ln_one_plus(delta * (1.0 / low)) * (low - 0.5) + delta * far.ln() - delta;
    sum + main + stirling(far) - stirling(near)
}

fn sine_ratio(r: Complex) -> Complex {
    let (x, y) = (PI * r.re, PI * r.im);
    let sine = Complex::new(x.sin() * y.cosh(), x.cos() * y.sinh());
    let one = Complex::new(1.0, 0.0);
    sine / (r * (one - r) * 4.0)
}

fn poly_text(poly: &[i128]) -> String {
    let mut text = String::new();
    for (k, &c) in poly.iter().enumerate().rev() {
        if c == 0 {
            continue;
        }
        let size = c.abs();
        let body = match (k, size) {
            (0, _) => size.to_string(),
            (1, 1) => "n".to_string(),
            (1, _) => format!("{size}n"),
            (_, 1) => format!("n^{k}"),
            _ => format!("{size}n^{k}"),
        };
        if text.is_empty() {
            text = if c < 0 { format!("-{body}") } else { body };
        } else {
            text += if c < 0 { " - " } else { " + " };
            text += &body;
        }
    }
    text
}

fn spelled(split: &Split) -> String {
    let mut form = Form {
        num: 1,
        den: 1,
        root: 1,
        halves: 0,
        top: Vec::new(),
        bottom: Vec::new(),
    };
    let mut note = String::new();
    for _ in 0..split.zero + split.one {
        form.halves += 1;
        form.scale(1, 2);
    }
    let mut pool = split.rational.clone();
    while let Some((p, q)) = pool.pop() {
        if 2 * p == q {
            continue;
        }
        if let Some(at) = pool.iter().position(|&(a, b)| b == q && a == q - p) {
            pool.remove(at);
            form.scale(q * q, 4 * p * (q - p));
            let low = p.min(q - p);
            match (low, q) {
                (1, 6) => form.scale(1, 2),
                (1, 4) => {
                    form.radical(2);
                    form.scale(1, 2);
                }
                (1, 3) => {
                    form.radical(3);
                    form.scale(1, 2);
                }
                _ => form.top.push(format!("sin({})", angle(low, 1, q))),
            }
        } else {
            form.halves += 1;
            form.scale(q, 2 * (q - p));
            form.bottom.push(format!("Gamma({})", fraction(q - p, q)));
        }
    }
    let rest = &split.rest;
    if rest.len() == 3 {
        let (c, b, a) = (rest[0], rest[1], rest[2]);
        let disc = b * b - 4 * a * c;
        let (s, t) = square_part(disc.abs());
        if -b == a {
            form.scale(a, 4 * c);
            let wave = if disc < 0 { "cosh" } else { "cos" };
            form.top.push(format!("{wave}({})", angle(s, t, 2 * a)));
        } else {
            form.halves += 2;
            form.scale(1, 4);
            let x = fraction(4 * a + b, 2 * a);
            if disc < 0 {
                let y = if t > 1 {
                    format!("i {}", surd(s, t, 2 * a))
                } else {
                    let (u, v) = reduced(s, 2 * a);
                    let head = if u == 1 {
                        "i".to_string()
                    } else {
                        format!("{u}i")
                    };
                    if v == 1 {
                        head
                    } else {
                        format!("{head}/{v}")
                    }
                };
                form.bottom.push(format!("|Gamma({x} - {y})|^2"));
            } else {
                let y = surd(s, t, 2 * a);
                form.bottom.push(format!("Gamma({x} - {y})"));
                form.bottom.push(format!("Gamma({x} + {y})"));
            }
        }
    } else if rest.len() == 4 {
        form.halves += 3;
        form.scale(1, 8);
        for i in 1..=3 {
            form.bottom.push(format!("Gamma(2 - r_{i})"));
        }
        note = format!(", r_i the roots of {}", poly_text(rest));
    }
    format!("{}{note}", form.spell())
}

fn reflected(split: &Split, roots: &[Complex]) -> Option<f64> {
    let mut value = (PI.sqrt() / 2.0).powi((split.zero + split.one) as i32);
    let mut pool = split.rational.clone();
    while let Some((p, q)) = pool.pop() {
        if 2 * p == q {
            continue;
        }
        let at = pool.iter().position(|&(a, b)| b == q && a == q - p)?;
        pool.remove(at);
        value *= sine_ratio(Complex::new(p as f64 / q as f64, 0.0)).re;
    }
    match split.rest.len() {
        1 => Some(value),
        3 if -split.rest[1] == split.rest[2] => Some(value * sine_ratio(roots[roots.len() - 1]).re),
        _ => None,
    }
}

fn parity_of(profile: &[u64], dimension: usize) -> Option<bool> {
    if dimension < 2 {
        return None;
    }
    [true, false].into_iter().find(|&odd| {
        profile.iter().enumerate().all(|(j, &a)| {
            let want = if (j % 2 == 1) == odd {
                binomial(dimension, j)
            } else {
                0
            };
            i128::from(a) == want
        })
    })
}

/// Counts a base-2 design's corners by how many odd coordinates each holds, the profile the row word reads: the code is a bitmask over the corners, corner `i` is the binary digits of `i` as `math::bang::code_to_corners` reads it, so its odd coordinates are the ones of `i`.
///
/// ```
/// assert_eq!(mrlyrs::num::sieve::row_profile(7, 2).unwrap(), vec![1, 2, 0]);
/// ```
///
/// # Errors
///
/// Errs outside dim 1 to 3, or on a code out of range for the dimension.
pub fn row_profile(code: u128, dimension: usize) -> Result<Vec<u64>> {
    if !(1..=ROW_DIMENSIONS).contains(&dimension) {
        return value_error(format!(
            "the row word reads dim 1 to {ROW_DIMENSIONS}, not dim {dimension}."
        ));
    }
    let corners = 1u32 << dimension;
    if code >> corners != 0 {
        return value_error(format!(
            "code {code} out of range for dimension {dimension} base 2 (0..{}).",
            (1u128 << corners) - 1
        ));
    }
    let mut profile = vec![0u64; dimension + 1];
    for corner in (0..corners).filter(|&i| code >> i & 1 == 1) {
        profile[corner.count_ones() as usize] += 1;
    }
    Ok(profile)
}

/// Returns the cells a design of this profile fills at one side, `sum_j a_j E^(dim-j) O^j` with `E` and `O` the even and the odd positions an axis holds: `P_F(n)` at side `2n - 1` and `w n^dim` at side `2n`.
///
/// # Errors
///
/// Errs on a profile no design in dim 1 to 3 holds, or when the fill overruns a u128.
pub fn row_fill(profile: &[u64], side: u64) -> Result<u128> {
    fill_at(profile, row_dimension(profile)?, side)
}

fn fill_at(profile: &[u64], dimension: usize, side: u64) -> Result<u128> {
    let even = u128::from(side.div_ceil(2));
    let odd = u128::from(side / 2);
    let mut fill = 0u128;
    for (j, &a) in profile.iter().enumerate() {
        let term = even
            .checked_pow((dimension - j) as u32)
            .and_then(|e| odd.checked_pow(j as u32).and_then(|o| e.checked_mul(o)))
            .and_then(|t| t.checked_mul(u128::from(a)))
            .and_then(|t| fill.checked_add(t));
        fill = match term {
            Some(total) => total,
            None => return overflow_error(format!("the fill at side {side} overruns a u128.")),
        };
    }
    Ok(fill)
}

/// Reads the design of this profile along the row word: its drift, its first correction, the roots of its fill polynomial, its constant from the Gamma form, the constant spelled, and the constant again by reflection, against the mirror and as a Wallis sieve product wherever those apply.
///
/// The fill polynomial factors as `w n^(m_0) (n-1)^(m_1)` times a core with integer coefficients; its rational roots are found exactly and divided out, and what is left is a constant, an irreducible quadratic or, at dim 3, an irreducible cubic, whose roots are taken in floating point. Each `Gamma(2 - r)` is read against one `Gamma(3/2)` as a ratio, by the recurrence up past 16 and Stirling's series there, so the large terms cancel before they round.
///
/// ```
/// let row = mrlyrs::num::sieve::row_law(&[1, 0, 1]).unwrap();
/// assert_eq!(row.closed, "cosh(pi/2)/2");
/// assert!((row.constant - (std::f64::consts::PI / 2.0).cosh() / 2.0).abs() < 1e-14);
/// ```
///
/// # Errors
///
/// Errs on a profile no design in dim 1 to 3 holds.
pub fn row_law(profile: &[u64]) -> Result<Row> {
    let dimension = row_dimension(profile)?;
    let d = dimension as i128;
    let w: i128 = profile.iter().map(|&a| i128::from(a)).sum();
    let first: i128 = profile
        .iter()
        .enumerate()
        .map(|(j, &a)| j as i128 * i128::from(a))
        .sum();
    let second: i128 = profile
        .iter()
        .enumerate()
        .map(|(j, &a)| (j * j) as i128 * i128::from(a))
        .sum();
    let drift = d * w - 2 * first;
    let correction = d * w * w + 4 * w * drift - 4 * (w * second - first * first);
    let split = split(profile, dimension);
    let mut roots = vec![Complex::new(0.0, 0.0); split.zero];
    roots.extend(vec![Complex::new(1.0, 0.0); split.one]);
    for &(p, q) in &split.rational {
        roots.push(Complex::new(p as f64 / q as f64, 0.0));
    }
    roots.extend(rest_roots(&split.rest));
    let mut log = (split.zero + split.one) as f64 * (0.5 * PI.ln() - LN_2);
    let mut mirror = Complex::new(FRAC_PI_4.powi((split.zero + split.one) as i32), 0.0);
    for r in &roots[split.zero + split.one..] {
        log -= gamma_step(*r).re;
        mirror = mirror * sine_ratio(*r);
    }
    let parity = match parity_of(profile, dimension) {
        Some(odd) => Some((odd, parity_product(dimension as u32, odd)?)),
        None => None,
    };
    Ok(Row {
        profile: profile.to_vec(),
        drift: reduced(drift, 2 * w),
        correction: reduced(correction, 8 * w * w),
        constant: log.exp(),
        closed: spelled(&split),
        reflection: reflected(&split, &roots),
        mirror: mirror.re,
        parity,
        roots,
    })
}

/// Walks the renormalised fill `R_L (2^dim/w)^L / L^drift` of the design of this profile to every stop, `R_L` the product of the letters' fill ratios, on the odd sides `3, 5, ..., 2L+1` or on the even sides `2, 4, ..., 2L`.
///
/// Each letter adds `log(1 + (2^dim fill - w side^dim) / (w side^dim))`, the excess taken in exact integers; at an even side the excess is zero and the even word carries no drift, so its walk stands at 1.
///
/// # Errors
///
/// Errs on a profile no design in dim 1 to 3 holds, or on stops that are not rising whole numbers from 1.
pub fn row_settle(profile: &[u64], stops: &[usize], even: bool) -> Result<Vec<f64>> {
    let dimension = row_dimension(profile)?;
    if stops.first().is_some_and(|&s| s == 0) || stops.windows(2).any(|pair| pair[0] >= pair[1]) {
        return value_error("the stops must rise from level 1.");
    }
    let w: u128 = profile.iter().map(|&a| u128::from(a)).sum();
    let drift = if even {
        0.0
    } else {
        let row = row_law(profile)?;
        row.drift.0 as f64 / row.drift.1 as f64
    };
    let mut out = Vec::with_capacity(stops.len());
    let (mut sum, mut carry) = (0.0f64, 0.0f64);
    let mut next = 0;
    for level in 1..=stops.last().copied().unwrap_or(0) {
        let side = (if even { 2 * level } else { 2 * level + 1 }) as u64;
        let fill = fill_at(profile, dimension, side)?;
        let whole = w * u128::from(side).pow(dimension as u32);
        let excess = (fill << dimension) as i128 - whole as i128;
        let term = (excess as f64 / whole as f64).ln_1p() - carry;
        let total = sum + term;
        carry = (total - sum) - term;
        sum = total;
        if stops[next] == level {
            out.push((sum - drift * (level as f64).ln()).exp());
            next += 1;
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::num::series::wallis_quarter_pi;
    use std::f64::consts::PI;

    fn truncated(start: u64, step: u64, stop: u64, scale: f64) -> f64 {
        let count = (stop - start) / step + 1;
        let mut sum = 0.0f64;
        for place in (0..count).rev() {
            let n = (start + place * step) as f64;
            sum += (-scale / n.powi(3)).ln_1p();
        }
        sum.exp()
    }

    fn brute(word: &[u64]) -> usize {
        raster(word).unwrap().1.iter().filter(|&&b| b == 1).count()
    }

    #[test]
    fn the_odd_word_walks_the_sides_of_the_classical_sieve() {
        let word = odd_word(4);
        let sides: Vec<u128> = (1..=4).map(|n| side(&word[..n]).unwrap()).collect();
        assert_eq!(sides, vec![3, 15, 105, 945]);
        let counts: Vec<u128> = (1..=4).map(|n| cells(&word[..n], 2).unwrap()).collect();
        assert_eq!(counts, vec![8, 192, 9216, 737_280]);
    }

    #[test]
    fn the_raster_counts_what_the_product_promises() {
        let word = odd_word(3);
        for n in 1..=3 {
            assert_eq!(
                brute(&word[..n]) as u128,
                cells(&word[..n], 2).unwrap(),
                "level {n}"
            );
        }
        for n in 1..=4 {
            let flat = flat_word(3, n);
            assert_eq!(
                brute(&flat) as u128,
                cells(&flat, 2).unwrap(),
                "carpet level {n}"
            );
        }
        let five = flat_word(5, 2);
        assert_eq!(brute(&five) as u128, cells(&five, 2).unwrap());
    }

    #[test]
    fn the_raster_is_the_ratio_times_the_area() {
        for n in 1..=3 {
            let word = odd_word(n);
            let (wide, sites) = raster(&word).unwrap();
            let lit = sites.iter().filter(|&&b| b == 1).count() as f64;
            assert!(
                (lit / (wide * wide) as f64 - ratio(&word, 2).unwrap()).abs() < 1e-12,
                "level {n}"
            );
        }
    }

    #[test]
    fn the_punctures_fill_the_gap_the_survivors_leave() {
        for dimension in 2..=3u32 {
            for n in 1..=3 {
                let word = odd_word(n);
                let axes = dimension as usize;
                let list = punctures(&word, dimension).unwrap();
                assert_eq!(
                    list.len() / (axes + 1),
                    holes(&word, dimension).unwrap() as usize
                );
                let volume: u128 = list
                    .chunks(axes + 1)
                    .map(|hole| u128::from(hole[axes]).pow(dimension))
                    .sum();
                let whole = side(&word).unwrap().pow(dimension);
                assert_eq!(
                    volume + cells(&word, dimension).unwrap(),
                    whole,
                    "{dimension}d level {n}"
                );
            }
        }
    }

    #[test]
    fn the_plane_ratio_is_the_wallis_product_the_series_walks() {
        for n in 1..=40 {
            assert!(
                (ratio(&odd_word(n), 2).unwrap() - wallis_quarter_pi(n)).abs() < 1e-15,
                "level {n}"
            );
        }
        assert!((ratio(&odd_word(200_000), 2).unwrap() - PLANE_LIMIT).abs() < 1e-5);
    }

    #[test]
    fn the_carpet_schedule_freezes_the_ratio_and_the_exponent() {
        for n in 1..=8 {
            let word = flat_word(3, n);
            assert!((ratio(&word, 2).unwrap() - (8.0f64 / 9.0).powi(n as i32)).abs() < 1e-15);
            assert!((exponent(&word, 2).unwrap() - 8f64.ln() / 3f64.ln()).abs() < 1e-12);
        }
        let long = odd_word(4_000);
        let read = exponent(&long, 2).unwrap();
        assert!(read > 1.999_5 && read < 2.0);
    }

    #[test]
    fn the_solid_limit_holds_the_true_constant_to_four_ulps() {
        assert!((solid_limit() - 0.948_815_485_719_679_6).abs() < 4e-16);
    }

    #[test]
    fn the_solid_limit_matches_its_truncated_product_to_one_part_in_1e14() {
        let product = truncated(3, 2, 8_000_001, 1.0);
        assert!((solid_limit() - product).abs() < 1e-14, "{product}");
    }

    #[test]
    fn the_solid_limit_splits_into_the_cosh_and_the_even_product() {
        let all = truncated(2, 1, 2_000_000, 1.0);
        let cosh = (PI * 3f64.sqrt() / 2.0).cosh() / (3.0 * PI);
        assert!((all - cosh).abs() < 1e-12, "{all} {cosh}");
        let even = truncated(1, 1, 1_000_000, 0.125);
        assert!((solid_limit() - cosh / even).abs() < 1e-12);
    }

    #[test]
    fn the_alternating_word_changes_at_every_step_and_freezes() {
        let pinned = 192f64.ln() / 15f64.ln();
        for pairs in 1..=4 {
            let word: Vec<u64> = [3u64, 5].iter().cycle().take(2 * pairs).copied().collect();
            assert!(
                (exponent(&word, 2).unwrap() - pinned).abs() < 1e-12,
                "{pairs}"
            );
        }
        assert_eq!(format!("{pinned:.6}"), "1.941432");
        assert!(ratio(&[3, 5, 3, 5], 2).unwrap() < ratio(&odd_word(4), 2).unwrap());
    }

    #[test]
    fn the_limit_names_a_schedule_or_says_it_cannot() {
        let plane = limit(&[5, 7, 9], 2).unwrap().unwrap();
        assert!((plane - FRAC_PI_4 / (8.0 / 9.0)).abs() < 1e-15, "{plane}");
        assert_eq!(format!("{plane:.6}"), "0.883573");
        let solid = limit(&[5, 7, 9], 3).unwrap().unwrap();
        assert!(
            (solid - solid_limit() / (26.0 / 27.0)).abs() < 1e-15,
            "{solid}"
        );
        assert_eq!(limit(&odd_word(4), 2).unwrap(), Some(PLANE_LIMIT));
        assert_eq!(limit(&flat_word(7, 3), 2).unwrap(), Some(0.0));
        assert_eq!(limit(&[3, 7, 11], 2).unwrap(), None);
        assert_eq!(limit(&[5], 2).unwrap(), None);
        assert_eq!(limit(&[3, 5, 3, 5], 2).unwrap(), None);
        assert_eq!(limit(&odd_word(4), 4).unwrap(), None);
    }

    #[test]
    fn the_plane_ratio_at_two_million_factors_rounds_to_nine_digits() {
        let read = ratio(&odd_word(2_000_000), 2).unwrap();
        assert_eq!(format!("{read:.9}"), "0.785398262");
        assert!(read > PLANE_LIMIT);
    }

    #[test]
    fn the_row_fill_counts_every_rendered_letter() {
        for dimension in 1..=3usize {
            for code in 1..(1u128 << (1 << dimension)) {
                let profile = row_profile(code, dimension).unwrap();
                for side in 2..=7u64 {
                    let tile = crate::math::bang::factory::create(
                        crate::math::bang::Code::from(code),
                        side as usize,
                        dimension,
                        2,
                        1,
                    )
                    .unwrap();
                    assert_eq!(
                        row_fill(&profile, side).unwrap(),
                        u128::from(tile.sum()),
                        "dim {dimension} code {code} side {side}"
                    );
                }
            }
        }
    }

    #[test]
    fn the_row_constants_meet_their_closed_forms() {
        let named: [(usize, u128, f64, &str, bool); 11] = [
            (1, 1, 0.886_226_925_452_758, "sqrt(pi)/2", true),
            (2, 7, 0.879_525_401_447_625_3, "3 pi/(4 Gamma(1/3))", false),
            (2, 14, 0.870_010_809_837_738_6, "3 pi/(8 Gamma(2/3))", false),
            (2, 6, FRAC_PI_4, "pi/4", true),
            (2, 9, 1.254_589_239_329_028, "cosh(pi/2)/2", true),
            (2, 11, 1.080_152_394_264_927, "3 cosh(pi/(2 sqrt3))/4", true),
            (
                3,
                23,
                0.767_916_038_651_074_6,
                "pi^(3/2)/(2 Gamma(1/4))",
                false,
            ),
            (
                3,
                232,
                0.757_338_025_727_611_8,
                "pi^(3/2)/(6 Gamma(3/4))",
                false,
            ),
            (
                3,
                150,
                0.948_815_485_719_679_6,
                "pi^(3/2)/(8 |Gamma(7/4 - i sqrt3/4)|^2)",
                false,
            ),
            (
                3,
                105,
                1.052_420_668_167_879,
                "pi^(3/2)/(8 |Gamma(5/4 - i sqrt3/4)|^2)",
                false,
            ),
            (3, 129, 1.907_095_803_094_885, "cosh(pi sqrt3/2)/4", true),
        ];
        for (dimension, code, want, closed, reflects) in named {
            let row = row_law(&row_profile(code, dimension).unwrap()).unwrap();
            assert!(
                (row.constant - want).abs() < 1e-14,
                "code {code} {}",
                row.constant
            );
            assert_eq!(row.closed, closed, "code {code}");
            assert_eq!(row.reflection.is_some(), reflects, "code {code}");
            if let Some(value) = row.reflection {
                assert!((value - want).abs() < 1e-14, "code {code} {value}");
            }
            if let Some((_, product)) = row.parity {
                assert!((product - want).abs() < 1e-14, "code {code} {product}");
            }
        }
        assert!((solid_limit() - parity_product(3, true).unwrap()).abs() < 1e-15);
    }

    #[test]
    fn a_design_times_its_mirror_reduces_by_reflection() {
        for dimension in 1..=3usize {
            let flip = (1usize << dimension) - 1;
            for code in 1..(1u128 << (1 << dimension)) {
                let mirror = (0..1usize << dimension)
                    .filter(|&i| code >> i & 1 == 1)
                    .fold(0u128, |acc, i| acc | 1 << (i ^ flip));
                let own = row_law(&row_profile(code, dimension).unwrap()).unwrap();
                let other = row_law(&row_profile(mirror, dimension).unwrap()).unwrap();
                let product = own.constant * other.constant;
                assert!(
                    (own.mirror - product).abs() < 1e-13,
                    "dim {dimension} code {code}"
                );
            }
        }
        let seven = row_law(&[1, 2, 0]).unwrap();
        assert!((seven.mirror - 9.0 * 3f64.sqrt() * PI / 64.0).abs() < 1e-15);
    }

    #[test]
    fn the_row_settles_on_its_constant_past_the_first_correction() {
        let row = row_law(&[1, 2, 0]).unwrap();
        let c1 = row.correction.0 as f64 / row.correction.1 as f64;
        let stops = [1_000usize, 10_000, 100_000, 1_000_000];
        let walk = row_settle(&row.profile, &stops, false).unwrap();
        for (&level, &value) in stops.iter().zip(&walk) {
            let law = row.constant * (1.0 + c1 / level as f64);
            let gap = (value - law).abs() * (level * level) as f64;
            assert!(gap < 2.0, "level {level} {value}");
        }
        assert!((walk[3] - row.constant).abs() < 5e-7);
    }

    #[test]
    fn even_sides_carry_no_drift_and_no_constant() {
        for profile in [vec![1, 2, 0], vec![1, 0, 1], vec![1, 3, 0, 0], vec![1, 0]] {
            let walk = row_settle(&profile, &[1, 2, 10, 1000], true).unwrap();
            assert!(walk.iter().all(|&value| value == 1.0), "{profile:?}");
        }
    }

    #[test]
    fn the_row_refuses_what_no_design_holds() {
        assert!(row_law(&[2, 0, 0]).is_err());
        assert!(row_law(&[0, 0, 0]).is_err());
        assert!(row_law(&[1, 0, 0, 0, 0]).is_err());
        assert!(row_profile(7, 4).is_err());
        assert!(row_settle(&[1, 2, 0], &[3, 2], false).is_err());
        assert!(parity_product(1, true).is_err());
    }

    #[test]
    fn refuses_a_letter_that_is_not_an_odd_side_from_three() {
        for bad in [0u64, 1, 2, 4, 6] {
            assert!(side(&[bad]).is_err(), "{bad}");
            assert!(cells(&[bad], 2).is_err(), "{bad}");
            assert!(holes(&[bad, 3], 2).is_err(), "{bad}");
            assert!(ratio(&[bad], 2).is_err(), "{bad}");
            assert!(exponent(&[bad], 2).is_err(), "{bad}");
            assert!(raster(&[bad]).is_err(), "{bad}");
            assert!(punctures(&[bad], 2).is_err(), "{bad}");
            assert!(limit(&[bad, bad + 2], 2).is_err(), "{bad}");
        }
        assert!(side(&[3]).is_ok());
    }
}
