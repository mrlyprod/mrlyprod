use crate::num::design;
use crate::num::factor::{gcd, mobius_sieve, totient};
use crate::num::prime::flags;
use std::f64::consts::PI;

/// The bar region A sets on the certificate exponent, `alpha_1 < 1/5`: the whole `l^1` mass against the minor-arc `x^(4/5)`.
pub const BAR_A: f64 = 0.2;
/// The bar region B sets, `alpha_1 < 1/4`: the hybrid `l^1` mass against the `d^(-1/2)` decay.
pub const BAR_B: f64 = 0.25;
/// The constant `gamma'` of the digit-uniform chain, `(2/pi)(gamma + log(8/pi))` rounded up at seven decimals.
pub const GAMMA: f64 = 0.9625229;
/// The base the chain is scanned to; from here the closed-form cap `1 + sqrt(2 (2/pi) log base + 0.97)` keeps it below the bar.
pub const CAP_BASE: u64 = 1272;
/// The least base whose every one-missing-digit set carries a window certificate below `1/5`, `lab/py/prime-dissection`, verb `window`.
pub const WINDOW_WALL: u64 = 301;
/// The least base whose every one-missing-digit set carries a per-digit shifted-grid certificate below `1/5`, `lab/py/digit-transform-norms`, verbs `fifth` and `fifthbelow`; base 114 missing 56 is certified above.
pub const DIGIT_WALL: u64 = 115;
/// The least base with some missing digit certified below `1/5`, `lab/py/digit-transform-norms`, verb `fifth`.
pub const FIRST_BELOW: u64 = 65;
/// The sets below [`DIGIT_WALL`] certified below `1/5`, as `(base, missing digit)`, printed by `lab/py/digit-transform-norms`, verb `fifth`.
pub const CERTIFIED: [(u64, u64); 1] = [(65, 0)];

/// The four regions the dissection cuts the frequencies `a/y` into by their Dirichlet fraction `l/d` at `Q = y^(3/5)` and height `h = |a d - l y|`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region {
    /// The minor arcs, `d >= y^(2/5)`.
    A,
    /// The middle, `d < y^(2/5)` and `max(d, h) >= Z`.
    B,
    /// Near a fraction whose denominator has a prime outside the base, `d < Z` and `h < Z`.
    C1,
    /// Near a fraction whose denominator divides a power of the base, `d < Z` and `h < Z`.
    C2,
}

/// Returns the last continued-fraction convergent `l/d` of `a/y` whose denominator is at most the cap, the Dirichlet fraction of the dissection.
///
/// ```
/// assert_eq!(mrlyrs::num::dissection::fraction(333, 1000, 63), (1, 3));
/// ```
pub fn fraction(a: u64, y: u64, cap: u64) -> (u64, u64) {
    let (mut p0, mut q0, mut p1, mut q1) = (0u64, 1u64, 1u64, 0u64);
    let (mut n, mut d) = (a, y);
    let mut best = (0, 1);
    while d > 0 {
        let c = n / d;
        let (p2, q2) = (c * p1 + p0, c * q1 + q0);
        if q2 > cap {
            break;
        }
        (p0, q0, p1, q1) = (p1, q1, p2, q2);
        best = (p2, q2);
        (n, d) = (d, n - c * d);
    }
    best
}

/// Returns `Q = floor(y^(3/5))`, the largest denominator the dissection admits, exact in integers.
///
/// ```
/// assert_eq!(mrlyrs::num::dissection::cap(1000), 63);
/// ```
pub fn cap(y: u64) -> u64 {
    let cube = u128::from(y).pow(3);
    let mut c = (y as f64).powf(0.6) as u64;
    while u128::from(c + 1).pow(5) <= cube {
        c += 1;
    }
    while c > 0 && u128::from(c).pow(5) > cube {
        c -= 1;
    }
    c
}

fn smooth(base: u64, mut d: u64) -> bool {
    loop {
        let g = gcd(u128::from(d), u128::from(base)) as u64;
        if g == 1 {
            return d == 1;
        }
        while d.is_multiple_of(g) {
            d /= g;
        }
    }
}

/// Returns the region of every frequency `a/y`, `a < y = base^level`, at the cut `Z`, the fraction taken by [`fraction`] at [`cap`].
pub fn regions(base: u64, level: u32, z: u64) -> Vec<Region> {
    let y = base.pow(level);
    let top = cap(y);
    let square = u128::from(y).pow(2);
    (0..y)
        .map(|a| {
            let (l, d) = fraction(a, y, top);
            let h = (i128::from(a) * i128::from(d) - i128::from(l) * i128::from(y)).unsigned_abs();
            if u128::from(d).pow(5) >= square {
                Region::A
            } else if d < z && h < u128::from(z) {
                if smooth(base, d) {
                    Region::C2
                } else {
                    Region::C1
                }
            } else {
                Region::B
            }
        })
        .collect()
}

fn modulus(base: u64, missing: &[u64], digits: &[u64], a: u64, y: u64) -> f64 {
    let turn =
        |k: u64| PI * ((u128::from(k) * u128::from(a) % (2 * u128::from(y))) as f64) / y as f64;
    let term = |k: u64| (2.0 * turn(k)).sin_cos();
    let (im, re) = if missing.len() < digits.len() {
        let kernel = if a == 0 {
            base as f64
        } else {
            turn(base).sin() / turn(1).sin()
        };
        let (sin, cos) = turn(base - 1).sin_cos();
        missing
            .iter()
            .fold((kernel * sin, kernel * cos), |(im, re), &e| {
                let (s, c) = term(e);
                (im - s, re - c)
            })
    } else {
        digits.iter().fold((0.0, 0.0), |(im, re), &d| {
            let (s, c) = term(d);
            (im + s, re + c)
        })
    };
    re.hypot(im)
}

fn rise(base: u64, digits: &[u64], row: &[f64]) -> Vec<f64> {
    let missing: Vec<u64> = (0..base).filter(|d| !digits.contains(d)).collect();
    let below = row.len() as u64;
    let y = below * base;
    (0..y)
        .map(|a| modulus(base, &missing, digits, a, y) * row[(a % below) as usize])
        .collect()
}

/// Returns `|hat F_level(a/base^level)|` at every `a < base^level`, built one digit at a time from `hat F_j(t) = hat F(t) hat F_(j-1)(base t)`.
///
/// ```
/// let row = mrlyrs::num::dissection::weights(3, &[0, 1], 1);
/// assert_eq!(row.iter().map(|w| format!("{w:.3}")).collect::<Vec<_>>(), ["2.000", "1.000", "1.000"]);
/// ```
pub fn weights(base: u64, digits: &[u64], level: u32) -> Vec<f64> {
    (0..level).fold(vec![1.0], |row, _| rise(base, digits, &row))
}

/// Returns the unshifted masses `c_j = sum_(a < base^j) |hat F_j(a/base^j)|` for `j = 0..=level`, the `l^1` mass region A pays, `c_0 = 1`.
pub fn masses(base: u64, digits: &[u64], level: u32) -> Vec<f64> {
    let mut row = vec![1.0];
    let mut out = vec![1.0];
    for _ in 0..level {
        row = rise(base, digits, &row);
        out.push(row.iter().sum());
    }
    out
}

/// Returns the `l^1` exponent the top two masses read, `log_base(c_j/(fill c_(j-1)))`: a reading of the growth region A pays, never a certificate.
pub fn reading(base: u64, fill: usize, masses: &[f64]) -> f64 {
    let n = masses.len();
    if n < 2 {
        return f64::NAN;
    }
    (masses[n - 1] / (fill as f64 * masses[n - 2])).ln() / (base as f64).ln()
}

fn cubic(base: u64, z: f64) -> f64 {
    let slope = 2.0 / PI;
    (z - 1.0).powi(3)
        - (slope * (base as f64).ln() * z
            + GAMMA * (z - 1.0)
            + slope * (z - 1.0).powi(2) / (base as f64 * z - 1.0))
}

/// Returns the root `z > 1` of the digit-uniform chain at one missing digit, `(z-1)^3 = (2/pi)(log base) z + gamma'(z-1) + (2/pi)(z-1)^2/(base z - 1)`, by bisection.
pub fn chain_root(base: u64) -> f64 {
    let (mut low, mut high) = (1.0, 11.0);
    while cubic(base, high) < 0.0 {
        high *= 2.0;
    }
    for _ in 0..200 {
        let mid = 0.5 * (low + high);
        if cubic(base, mid) < 0.0 {
            low = mid;
        } else {
            high = mid;
        }
    }
    high
}

/// Returns the chain's certificate exponent at one missing digit, `alpha_1 = log_base(z base/(base - 1))`, the same at every missing digit.
pub fn chain_exponent(base: u64) -> f64 {
    let q = base as f64;
    (chain_root(base) * q / (q - 1.0)).ln() / q.ln()
}

/// Returns the chain's margin at the bar, the cubic cleared of denominators at `w = base^(1/5)(1 - 1/base)`, positive exactly when the root sits below `w` and `alpha_1 < 1/5`.
pub fn chain_margin(base: u64) -> f64 {
    let q = base as f64;
    cubic(base, q.powf(BAR_A) * (1.0 - 1.0 / q))
}

/// Returns the chain's wall, one past the last base up to [`CAP_BASE`] whose margin is not positive.
pub fn chain_wall() -> u64 {
    (3..=CAP_BASE)
        .rev()
        .find(|&q| chain_margin(q) <= 0.0)
        .map_or(3, |q| q + 1)
}

/// Returns how the theorem reaches the set missing one digit: `proof` from the chain's wall, `certificate` at every base from [`DIGIT_WALL`] below it and at the [`CERTIFIED`] sets, `none` elsewhere.
///
/// ```
/// use mrlyrs::num::dissection::reach;
/// assert_eq!([reach(584, 3), reach(115, 57), reach(65, 0), reach(65, 32)], ["proof", "certificate", "certificate", "none"]);
/// ```
pub fn reach(base: u64, missing: u64) -> &'static str {
    if base >= chain_wall() {
        "proof"
    } else if base >= DIGIT_WALL || CERTIFIED.contains(&(base, missing)) {
        "certificate"
    } else {
        "none"
    }
}

/// Returns `kappa_F = (base/phi(base)) #{f in F : gcd(f, base) = 1}/fill`, the main-term constant of the prime count, as a reduced fraction.
///
/// ```
/// assert_eq!(mrlyrs::num::dissection::kappa(10, &[1, 2, 3, 4, 6, 7, 8, 9]), (5, 4));
/// ```
pub fn kappa(base: u64, digits: &[u64]) -> (u64, u64) {
    let units = digits
        .iter()
        .filter(|&&f| gcd(u128::from(f), u128::from(base)) == 1)
        .count() as u64;
    let top = base * units;
    let bottom = totient(base as usize) as u64 * digits.len() as u64;
    let g = gcd(u128::from(top), u128::from(bottom)).max(1) as u64;
    (top / g, bottom / g)
}

/// Returns whether the digit set keeps two consecutive digits, the hypothesis region C1 reads.
pub fn consecutive(digits: &[u64]) -> bool {
    digits.iter().any(|d| digits.contains(&(d + 1)))
}

/// The set's own sums read on a log grid of `x`: the mass `A_F(x)`, the meter `M_F(x)` and the prime count `psi_F(x) = sum Lambda(n)` over the elements up to `x`.
pub struct Tally {
    /// The log of `x` at every sample, uniform from the first element to the last.
    pub log_x: Vec<f64>,
    /// The mass `A_F(x)`, the count of elements up to `x`.
    pub count: Vec<u64>,
    /// The meter `M_F(x)`, the sum of `mu(n)` over the elements up to `x`.
    pub meter: Vec<i64>,
    /// The prime count `psi_F(x)`, the sum of `Lambda(n)` over the elements up to `x`.
    pub primes: Vec<f64>,
}

/// Tallies the set below `base^level` on a log grid of the given size, with the Mobius values and the primes sieved to the span.
pub fn tally(base: u64, digits: &[u64], level: usize, samples: usize) -> Tally {
    let values = design::elements(base, digits, level);
    let top = base.pow(level as u32) as usize;
    let mu = mobius_sieve(top);
    let prime = flags(top);
    let mut powers: Vec<(u64, f64)> = Vec::new();
    for p in (2..=top).take_while(|p| p * p <= top).filter(|&p| prime[p]) {
        let mut power = p * p;
        while power <= top {
            powers.push((power as u64, (p as f64).ln()));
            power = match power.checked_mul(p) {
                Some(next) => next,
                None => break,
            };
        }
    }
    powers.sort_by_key(|row| row.0);
    let mut meter = Vec::with_capacity(values.len());
    let mut psi = Vec::with_capacity(values.len());
    let (mut m, mut s) = (0i64, 0.0f64);
    for &n in &values {
        m += i64::from(mu[n as usize]);
        if prime[n as usize] {
            s += (n as f64).ln();
        } else if let Ok(slot) = powers.binary_search_by_key(&n, |row| row.0) {
            s += powers[slot].1;
        }
        meter.push(m);
        psi.push(s);
    }
    let log_x = design::log_grid(&values, samples);
    let slots: Vec<usize> = log_x
        .iter()
        .map(|&t| values.partition_point(|&v| (v as f64).ln() <= t))
        .collect();
    Tally {
        count: slots.iter().map(|&k| k as u64).collect(),
        meter: slots
            .iter()
            .map(|&k| if k == 0 { 0 } else { meter[k - 1] })
            .collect(),
        primes: slots
            .iter()
            .map(|&k| if k == 0 { 0.0 } else { psi[k - 1] })
            .collect(),
        log_x,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn missing(base: u64, digit: u64) -> Vec<u64> {
        (0..base).filter(|&d| d != digit).collect()
    }

    #[test]
    fn the_paper_figure_cut_holds_732_202_26_40() {
        let cut = regions(10, 3, 8);
        let count = |r: Region| cut.iter().filter(|&&c| c == r).count();
        assert_eq!(
            [
                count(Region::A),
                count(Region::B),
                count(Region::C1),
                count(Region::C2)
            ],
            [732, 202, 26, 40]
        );
    }

    #[test]
    fn the_mass_ratios_match_the_wall_readings() {
        let ratio = |base: u64, digit: u64, level: u32| {
            let c = masses(base, &missing(base, digit), level);
            format!("{:.4}", c[level as usize] / c[level as usize - 1])
        };
        assert_eq!(ratio(33, 16, 4), "74.1654");
        assert_eq!(ratio(33, 0, 4), "70.5661");
        assert_eq!(ratio(17, 8, 5), "35.5234");
    }

    #[test]
    fn the_two_digit_column_reads_log_two_over_log_three() {
        let c = masses(3, &[0, 1], 1);
        assert_eq!((reading(3, 2, &c) * 1e6).floor(), 630929.0);
    }

    #[test]
    fn the_chain_clears_the_bar_from_584() {
        assert_eq!(chain_wall(), 584);
        assert_eq!(format!("{:.4e}", chain_margin(584)), "6.0170e-3");
        assert_eq!(format!("{:.4e}", chain_margin(583)), "-8.3138e-3");
        assert_eq!((chain_exponent(584) * 1e6).ceil(), 199983.0);
    }

    #[test]
    fn the_tally_meets_the_count_by_hand() {
        let t = tally(10, &missing(10, 7), 6, 64);
        assert_eq!(
            (t.count[63], t.meter[63], format!("{:.4}", t.primes[63])),
            (531440, -9, "441976.4885".to_string())
        );
    }
}
