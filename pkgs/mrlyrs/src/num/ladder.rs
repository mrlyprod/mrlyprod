use crate::core::error::{value_error, Result};
use crate::num::design::elements;
use crate::num::zeta::{raise, Complex};

/// The relative rounding allowance the double-precision ladder charges against the scale it carries.
///
/// The truncation bound is the proved one, carried by the same recursion that carries the value; this constant is a measured allowance for the arithmetic itself and is not a proof.
pub const ROUNDING: f64 = 1e-13;

const PEEL_TARGET: f64 = 100.0;
const SHIFT_START: f64 = 6.0;
const SHIFT_STEP: f64 = 6.0;
const SHIFT_CAP: f64 = 400.0;
const CUT_START: usize = 8;
const CUT_STEP: usize = 6;
const CUT_CAP: usize = 120;
const RATIO_CAP: f64 = 0.9;

/// A digit design: the base `q`, the digit set `F` its elements are written with, and the peel depth `P` its ladder starts at.
///
/// The elements are the whole numbers whose base-`q` digits all lie in `F` with a nonzero leading digit, and the object is their Dirichlet series `zeta_F(s) = sum n^(-s)`, of abscissa `alpha = log_q k` for `k = card F`.
/// With `E_j` the sum over the elements of exactly `j` digits and `G_P = sum_(j >= P) E_j`, splitting an element on its last digit gives `(1 - k q^(-w)) G_P(w) = E_P(w) + sum_(l >= 1) binom(-w, l) q^(-w-l) gamma_l G_P(w+l)` with `gamma_l = sum_(a in F) a^l`, and `zeta_F = D_(P-1) + G_P` for the finite Dirichlet polynomial `D_(P-1)` over the elements below `q^(P-1)`.
/// The `l`-series has ratio `max F / q^P`, so the peel depth buys the convergence and the small quantity `G_P` is carried directly, never as a difference of two large ones.
#[derive(Clone, Debug, PartialEq)]
pub struct Design {
    base: u64,
    digits: Vec<u64>,
    peel: usize,
    lead: usize,
    largest: u64,
    low: Vec<f64>,
    mid: Vec<f64>,
    gamma: Vec<f64>,
}

impl Design {
    /// Builds a design on the base and the digit set, choosing the peel depth.
    ///
    /// ```
    /// let design = mrlyrs::num::ladder::Design::new(3, &[0, 1]).unwrap();
    /// assert_eq!(design.peel(), 7);
    /// assert!((design.abscissa() - 0.630_929_753_571_457).abs() < 1e-14);
    /// ```
    ///
    /// # Errors
    ///
    /// Errs at a base under two, at fewer than two digits, at a digit outside the base, or at an all-zero digit set.
    pub fn new(base: u64, digits: &[u64]) -> Result<Design> {
        let mut set: Vec<u64> = digits.to_vec();
        set.sort_unstable();
        set.dedup();
        if set.len() < 2 {
            return value_error("a design needs at least two digits.");
        }
        let lead = set.iter().filter(|&&d| d > 0).count();
        if lead == 0 {
            return value_error("a design needs a nonzero digit.");
        }
        let size = set.len() as f64;
        let mut peel = 2usize;
        while lead as f64 * size.powi(peel as i32) <= PEEL_TARGET {
            peel += 1;
        }
        Design::with_peel(base, &set, peel)
    }

    /// Builds a design at an explicit peel depth, at least two.
    ///
    /// ```
    /// let shallow = mrlyrs::num::ladder::Design::with_peel(2, &[0, 1], 4).unwrap();
    /// assert_eq!(shallow.peel(), 4);
    /// ```
    ///
    /// # Errors
    ///
    /// Errs at a bad base or digit set, and at a peel depth under two or past the exact integers.
    pub fn with_peel(base: u64, digits: &[u64], peel: usize) -> Result<Design> {
        let mut set: Vec<u64> = digits.to_vec();
        set.sort_unstable();
        set.dedup();
        if base < 2 {
            return value_error(format!("base {base} is under two."));
        }
        if set.len() < 2 {
            return value_error("a design needs at least two digits.");
        }
        if set.iter().any(|&d| d >= base) {
            return value_error(format!("a digit sits outside the base {base}."));
        }
        let lead = set.iter().filter(|&&d| d > 0).count();
        if lead == 0 {
            return value_error("a design needs a nonzero digit.");
        }
        if peel < 2 {
            return value_error(format!("peel depth {peel} is under two."));
        }
        if (base as f64).powi(peel as i32) > 9.007_199_254_740_992e15 {
            return value_error(format!("peel depth {peel} overruns the exact integers."));
        }
        let below = elements(base, &set, peel - 1);
        let through = elements(base, &set, peel);
        let low: Vec<f64> = below.iter().map(|&n| (n as f64).ln()).collect();
        let mid: Vec<f64> = through[below.len()..]
            .iter()
            .map(|&n| (n as f64).ln())
            .collect();
        let largest = *set.last().unwrap();
        let gamma: Vec<f64> = (0..=CUT_CAP)
            .map(|l| set.iter().map(|&a| (a as f64).powi(l as i32)).sum())
            .collect();
        Ok(Design {
            base,
            digits: set,
            peel,
            lead,
            largest,
            low,
            mid,
            gamma,
        })
    }

    /// Returns the base.
    pub fn base(&self) -> u64 {
        self.base
    }

    /// Returns the digit set, ascending.
    pub fn digits(&self) -> &[u64] {
        &self.digits
    }

    /// Returns the peel depth.
    pub fn peel(&self) -> usize {
        self.peel
    }

    /// Returns the abscissa `alpha = log_q k`.
    pub fn abscissa(&self) -> f64 {
        (self.digits.len() as f64).ln() / (self.base as f64).ln()
    }

    /// Returns the pole spacing `2 pi / log q`.
    pub fn period(&self) -> f64 {
        2.0 * std::f64::consts::PI / (self.base as f64).ln()
    }

    /// Returns the pole `s_(m,j) = alpha - m + 2 pi i j / log q`.
    pub fn pole(&self, m: usize, j: i64) -> Complex {
        Complex::new(self.abscissa() - m as f64, self.period() * j as f64)
    }
}

// LADDER

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

fn tail(design: &Design, sigma: f64) -> f64 {
    let ratio = design.digits.len() as f64 * (design.base as f64).powf(-sigma);
    if ratio >= 1.0 {
        return f64::INFINITY;
    }
    design.lead as f64 * ratio.powi(design.peel as i32 - 1) / (1.0 - ratio)
}

fn log_tail(design: &Design, sigma: f64) -> f64 {
    let ratio = design.digits.len() as f64 * (design.base as f64).powf(-sigma);
    if ratio >= 1.0 {
        return f64::INFINITY;
    }
    (design.lead as f64).ln() + (design.peel as f64 - 1.0) * ratio.ln() - (-ratio).ln_1p()
}

fn log_binomial(top: f64, pick: usize) -> f64 {
    (1..=pick)
        .map(|i| ((top - pick as f64 + i as f64) / i as f64).ln())
        .sum()
}

fn cut(design: &Design, w: Complex, depth: usize) -> f64 {
    let (span, sigma) = (w.abs(), w.re);
    let base = design.base as f64;
    let largest = design.largest as f64;
    let ratio = ((span + depth as f64 + 1.0) / (depth as f64 + 2.0)).max(1.0) * largest
        / base.powi(design.peel as i32);
    if ratio >= RATIO_CAP {
        return f64::INFINITY;
    }
    let rest = log_tail(design, sigma + depth as f64 + 1.0);
    if rest.is_infinite() {
        return f64::INFINITY;
    }
    let log = log_binomial(span + depth as f64, depth + 1)
        + (-sigma - depth as f64 - 1.0) * base.ln()
        + (design.digits.len() as f64).ln()
        + (depth as f64 + 1.0) * largest.ln()
        + rest;
    log.exp() / (1.0 - ratio)
}

struct Rung {
    value: Complex,
    bound: f64,
    scale: f64,
}

fn rung(design: &Design, s: Complex, shift: f64, depth: usize, whole: bool) -> Rung {
    let base = design.base as f64;
    let size = design.digits.len() as f64;
    let levels = (shift - s.re).ceil().max(0.0) as usize;
    if !whole && levels == 0 {
        let front = Complex::new(1.0, 0.0) - raise(base, -s) * size;
        let rest = front.abs() * tail(design, s.re);
        return Rung {
            value: Complex::new(0.0, 0.0),
            bound: rest,
            scale: rest,
        };
    }
    let mut value = vec![Complex::new(0.0, 0.0); levels + depth + 2];
    let mut bound = vec![0.0f64; levels + depth + 2];
    let mut scale = vec![0.0f64; levels + depth + 2];
    for j in levels..levels + depth + 2 {
        bound[j] = tail(design, s.re + j as f64);
        scale[j] = bound[j];
    }
    for j in (0..levels).rev() {
        let w = s + j as f64;
        let mut acc = poly(&design.mid, w);
        let mut err = cut(design, w, depth);
        let mut wide = poly_scale(&design.mid, w.re) + err;
        let mut coefficient = Complex::new(1.0, 0.0);
        for l in 1..=depth {
            coefficient = coefficient * ((-w) - (l as f64 - 1.0)) * (1.0 / l as f64);
            let weight = raise(base, (-w) - l as f64) * coefficient * design.gamma[l];
            acc = acc + weight * value[j + l];
            err += weight.abs() * bound[j + l];
            wide += weight.abs() * scale[j + l];
        }
        if !whole && j == 0 {
            return Rung {
                value: acc,
                bound: err,
                scale: wide,
            };
        }
        let den = Complex::new(1.0, 0.0) - raise(base, -w) * size;
        value[j] = acc / den;
        bound[j] = err / den.abs();
        scale[j] = wide / den.abs();
    }
    Rung {
        value: value[0],
        bound: bound[0],
        scale: scale[0],
    }
}

fn crossing(design: &Design, s: Complex, whole: bool) -> Option<(i64, i64)> {
    let turn = s.im / design.period();
    let j = turn.round();
    if (turn - j).abs() > 1e-9 {
        return None;
    }
    let drop = design.abscissa() - s.re;
    let m = drop.round();
    let first = if whole { 0.0 } else { 1.0 };
    if (drop - m).abs() > 1e-9 || m < first {
        return None;
    }
    Some((m as i64, j as i64))
}

fn tune(
    design: &Design,
    s: Complex,
    tolerance: f64,
    whole: bool,
    head: f64,
) -> Result<(Complex, f64)> {
    if let Some((m, j)) = crossing(design, s, whole) {
        return value_error(format!(
            "the ladder walks through the pole s_({m},{j}) = alpha - {m} + 2 pi i {j} / log q and cannot read that point."
        ));
    }
    let mut shift = SHIFT_START;
    let mut depth = CUT_START;
    let mut best = f64::INFINITY;
    while shift <= SHIFT_CAP && depth <= CUT_CAP {
        let step = rung(design, s, shift, depth, whole);
        let carried = step.bound + ROUNDING * (step.scale + head);
        if carried < tolerance {
            return Ok((step.value, carried));
        }
        best = best.min(carried);
        shift += SHIFT_STEP;
        depth += CUT_STEP;
    }
    value_error(format!(
        "tolerance {tolerance:e} is out of reach at s = {} + {}i, best bound {best:e}.",
        s.re, s.im
    ))
}

// READINGS

/// Returns `zeta_F(s)` and the bound it is known to.
///
/// The bound is the propagated truncation bound, which is proved, plus [`ROUNDING`] times the scale the ladder carries, which is a measured allowance; the acceptance test charges every term the returned bound carries, the Dirichlet polynomial's own scale included, so the returned bound is never above the tolerance asked.
/// The recursion walks `w = s + j` upward and divides by `1 - k q^(-w)`, so it cannot read `s = alpha - m + 2 pi i j / log q` for a whole `m >= 0`: at those points it raises and names the pole. On a full digit set that lattice is `s = 1, 0, -1, -2, ...`, where `zeta_F` is `zeta` and only `s = 1` is singular.
///
/// ```
/// let design = mrlyrs::num::ladder::Design::new(2, &[0, 1]).unwrap();
/// let s = mrlyrs::num::zeta::Complex::new(2.0, 0.0);
/// let (value, bound) = mrlyrs::num::ladder::zeta(&design, s, 1e-10).unwrap();
/// assert!((value.re - std::f64::consts::PI * std::f64::consts::PI / 6.0).abs() < bound);
/// ```
///
/// # Errors
///
/// Errs when the tolerance is out of reach, or when the point sits on the blind lattice.
pub fn zeta(design: &Design, s: Complex, tolerance: f64) -> Result<(Complex, f64)> {
    let (value, bound) = tune(design, s, tolerance, true, poly_scale(&design.low, s.re))?;
    Ok((poly(&design.low, s) + value, bound))
}

/// Returns the Lyndon cofactor `Z(s) = zeta_F(s) (1 - k q^(-s))` and the bound it is known to.
///
/// The cofactor is the ladder numerator over the finite polynomial, so it is analytic on `Re s > alpha - 1` and a zero census on it needs no pole-free strip.
/// The blind lattice here is `s = alpha - m + 2 pi i j / log q` for a whole `m >= 1`: the numerator itself is read at `m = 0`, which is what makes the residue available.
///
/// # Errors
///
/// Errs when the tolerance is out of reach, or when the point sits on the blind lattice.
pub fn cofactor(design: &Design, s: Complex, tolerance: f64) -> Result<(Complex, f64)> {
    let base = design.base as f64;
    let size = design.digits.len() as f64;
    let front = Complex::new(1.0, 0.0) - raise(base, -s) * size;
    let head = front.abs() * poly_scale(&design.low, s.re);
    let (value, bound) = tune(design, s, tolerance, false, head)?;
    Ok((front * poly(&design.low, s) + value, bound))
}

/// Returns the residue of `zeta_F` at `s_(m,j) = alpha - m + 2 pi i j / log q` and the bound it is known to.
///
/// At `m = 0` the factor `1 - k q^(-s)` has derivative `log q`, so the residue is the ladder numerator over `log q`; the column below comes off the same recursion, `(1 - q^m) R_m = sum_(l = 1)^m binom(-s_(m,j), l) q^(-s_(m,j)-l) gamma_l R_(m-l)`.
/// The head term `(1 - k q^(-s)) D_(P-1)(s)` is dropped, since the factor vanishes at the pole; in double precision the factor is not exactly zero, and the residue of the dropped term, about `1e-15` at base 10, is not charged separately and sits inside the carried scale.
///
/// # Errors
///
/// Errs when the tolerance is out of reach.
pub fn residue(design: &Design, m: usize, j: i64, tolerance: f64) -> Result<(Complex, f64)> {
    let base = design.base as f64;
    let (head, err) = tune(design, design.pole(0, j), tolerance, false, 0.0)?;
    let mut values = vec![head * (1.0 / base.ln())];
    let mut bounds = vec![err / base.ln()];
    for level in 1..=m {
        let w = design.pole(level, j);
        let mut acc = Complex::new(0.0, 0.0);
        let mut err = 0.0;
        let mut coefficient = Complex::new(1.0, 0.0);
        for l in 1..=level {
            coefficient = coefficient * ((-w) - (l as f64 - 1.0)) * (1.0 / l as f64);
            let weight = raise(base, (-w) - l as f64) * coefficient * design.gamma[l];
            acc = acc + weight * values[level - l];
            err += weight.abs() * (bounds[level - l] + ROUNDING * values[level - l].abs());
        }
        let den = 1.0 - base.powi(level as i32);
        values.push(acc * (1.0 / den));
        bounds.push(err / den.abs());
    }
    Ok((values[m], bounds[m]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::num::zeta::Line;

    const PINNED: f64 = 1e-10;

    fn near(value: Complex, re: f64, im: f64, bound: f64) {
        assert!(
            (value.re - re).abs() < bound && (value.im - im).abs() < bound,
            "{value:?} is not {re} + {im}i inside {bound:e}"
        );
    }

    #[test]
    fn the_full_base_two_design_is_the_zeta_function() {
        let design = Design::new(2, &[0, 1]).unwrap();
        assert_eq!(design.peel(), 7);
        assert_eq!(design.abscissa(), 1.0);
        let pinned = [
            (Complex::new(2.0, 0.0), 1.644_934_066_848_226_4, 0.0),
            (
                Complex::new(0.3, 40.0),
                0.748_775_209_504_225_8,
                -1.440_885_440_634_440_5,
            ),
            (
                Complex::new(-1.0, 2.0),
                0.168_915_669_770_834_4,
                -0.070_515_988_908_254_42,
            ),
        ];
        for (s, re, im) in pinned {
            let (value, bound) = zeta(&design, s, 1e-8).unwrap();
            near(value, re, im, bound);
        }
    }

    #[test]
    fn the_ladder_meets_the_critical_line_engine() {
        let design = Design::new(2, &[0, 1]).unwrap();
        let line = Line::new();
        for t in [12.0, 18.5] {
            let (value, bound) = zeta(&design, Complex::new(0.5, t), PINNED).unwrap();
            let want = line.maclaurin(t);
            assert!(
                (value - want).abs() < bound + 1e-9,
                "t {t} {value:?} {want:?}"
            );
        }
    }

    #[test]
    fn the_residue_is_one_at_the_abscissa_and_zero_above_it() {
        let design = Design::new(2, &[0, 1]).unwrap();
        let (one, bound) = residue(&design, 0, 0, PINNED).unwrap();
        near(one, 1.0, 0.0, bound);
        let (off, bound) = residue(&design, 0, 1, PINNED).unwrap();
        near(off, 0.0, 0.0, bound);
    }

    #[test]
    fn the_base_three_residues_meet_the_harvest_midpoints() {
        let pinned = [
            (
                vec![0u64, 1],
                [
                    (0.799_023_642_655_835, 0.0),
                    (0.231_891_517_689_918_2, -0.501_067_414_481_068_3),
                    (0.003_963_750_587_374_229, -0.033_400_689_771_018_48),
                ],
            ),
            (
                vec![0, 2],
                [
                    (0.515_977_601_099_115_1, 0.0),
                    (0.135_292_875_345_483_3, 0.329_874_091_244_466_17),
                    (-0.021_699_535_708_644_655, -0.000_946_804_031_498_436_2),
                ],
            ),
        ];
        for (digits, wanted) in pinned {
            let design = Design::new(3, &digits).unwrap();
            for (j, (re, im)) in wanted.iter().enumerate() {
                let (value, bound) = residue(&design, 0, j as i64, PINNED).unwrap();
                near(value, *re, *im, bound);
            }
        }
    }

    #[test]
    fn the_second_digit_set_scales_the_first() {
        let one = Design::new(3, &[0, 1]).unwrap();
        let two = Design::new(3, &[0, 2]).unwrap();
        for s in [Complex::new(2.0, 3.0), Complex::new(0.8, 23.0)] {
            let (a, ba) = zeta(&two, s, PINNED).unwrap();
            let (b, bb) = zeta(&one, s, PINNED).unwrap();
            assert!((a - raise(2.0, -s) * b).abs() < ba + bb);
        }
    }

    #[test]
    fn the_base_ten_design_misses_its_last_digit() {
        let design = Design::new(10, &[0, 1, 2, 3, 4, 5, 6, 7, 8]).unwrap();
        assert_eq!(design.peel(), 2);
        let (value, bound) = zeta(&design, Complex::new(2.0, 0.0), PINNED).unwrap();
        near(value, 1.623_924_927_084_640_5, 0.0, bound);
        let (value, bound) = zeta(&design, Complex::new(1.1, 3.0), PINNED).unwrap();
        near(
            value,
            0.618_774_761_630_085,
            -0.025_931_036_103_864_726,
            bound,
        );
        let (value, bound) = residue(&design, 0, 0, PINNED).unwrap();
        near(value, 1.022_344_896_589_484_1, 0.0, bound);
    }

    #[test]
    fn the_cofactor_is_the_series_times_the_lyndon_factor() {
        let design = Design::new(2, &[0, 1]).unwrap();
        for s in [Complex::new(2.0, 0.0), Complex::new(0.3, 40.0)] {
            let (value, bound) = cofactor(&design, s, 1e-8).unwrap();
            let (series, other) = zeta(&design, s, 1e-8).unwrap();
            let front = Complex::new(1.0, 0.0) - raise(2.0, -s) * 2.0;
            assert!((value - front * series).abs() < bound + front.abs() * other);
        }
        let (value, bound) = cofactor(&design, Complex::new(2.0, 0.0), PINNED).unwrap();
        near(value, 0.822_467_033_424_113_2, 0.0, bound);
    }

    #[test]
    fn the_bound_survives_a_change_of_peel_depth() {
        for s in [Complex::new(2.0, 0.0), Complex::new(0.3, 40.0)] {
            let shallow = Design::with_peel(2, &[0, 1], 4).unwrap();
            let deep = Design::with_peel(2, &[0, 1], 8).unwrap();
            let (a, ba) = zeta(&shallow, s, 1e-8).unwrap();
            let (b, bb) = zeta(&deep, s, 1e-8).unwrap();
            assert!((a - b).abs() < ba + bb, "{s:?} {a:?} {b:?} {ba:e} {bb:e}");
        }
    }

    #[test]
    fn the_column_below_a_pole_is_the_contour_average_around_it() {
        let design = Design::new(3, &[0, 1]).unwrap();
        let (want, bound) = residue(&design, 1, 1, PINNED).unwrap();
        let centre = design.pole(1, 1);
        let (radius, nodes) = (0.05, 8);
        let mut got = Complex::new(0.0, 0.0);
        let mut carried = 0.0;
        for k in 0..nodes {
            let turn = Complex::turn(std::f64::consts::TAU * k as f64 / nodes as f64);
            let (value, other) = zeta(&design, centre + turn * radius, 1e-9).unwrap();
            got = got + turn * radius * value * (1.0 / nodes as f64);
            carried += radius * other / nodes as f64;
        }
        assert!(
            (got - want).abs() < bound + carried + 1e-9,
            "{got:?} {want:?}"
        );
    }

    #[test]
    fn the_ladder_raises_when_the_tolerance_is_out_of_reach() {
        let design = Design::new(2, &[0, 1]).unwrap();
        assert!(zeta(&design, Complex::new(-1.0, 2.0), 1e-11).is_err());
        assert!(zeta(&design, Complex::new(-1.0, 2.0), 1e-9).is_ok());
    }

    #[test]
    fn refuses_a_design_it_cannot_peel() {
        assert!(Design::new(5, &[3]).is_err());
        assert!(Design::new(5, &[0]).is_err());
        assert!(Design::new(1, &[0, 1]).is_err());
        assert!(Design::with_peel(5, &[0, 7], 3).is_err());
        assert!(Design::with_peel(2, &[0, 1], 1).is_err());
    }

    #[test]
    fn the_returned_bound_never_exceeds_the_tolerance_asked() {
        let design = Design::new(2, &[0, 1]).unwrap();
        for s in [
            Complex::new(-2.0, 2.0),
            Complex::new(-6.0, 2.0),
            Complex::new(0.3, 40.0),
            Complex::new(2.0, 0.0),
        ] {
            for tolerance in [1e-1, 1e-5, 1e-8] {
                if let Ok((_, bound)) = zeta(&design, s, tolerance) {
                    assert!(bound < tolerance, "zeta {s:?} {tolerance:e} {bound:e}");
                }
                if let Ok((_, bound)) = cofactor(&design, s, tolerance) {
                    assert!(bound < tolerance, "cofactor {s:?} {tolerance:e} {bound:e}");
                }
            }
        }
    }

    #[test]
    fn the_cofactor_bound_carries_the_lyndon_factor_on_an_empty_rung() {
        let design = Design::new(2, &[0, 1]).unwrap();
        let s = Complex::new(6.0, std::f64::consts::PI / 2f64.ln());
        let front = Complex::new(1.0, 0.0) - raise(2.0, -s) * 2.0;
        assert!((front.abs() - 1.031_25).abs() < 1e-12);
        let (_, bound) = cofactor(&design, s, 1e-7).unwrap();
        let (_, other) = zeta(&design, s, 1e-7).unwrap();
        assert!(bound > front.abs() * 9.6e-10, "{bound:e}");
        assert!(bound > other * 1.03, "{bound:e} {other:e}");
    }

    #[test]
    fn the_ladder_names_the_pole_it_walks_through() {
        let design = Design::new(2, &[0, 1]).unwrap();
        for m in 0..3 {
            let s = design.pole(m, 0);
            let message = zeta(&design, s, 1e-7).unwrap_err().to_string();
            assert!(message.contains("walks through the pole"), "{message}");
        }
        assert!(zeta(&design, Complex::new(-1.0, 1e-6), 1e-7).is_ok());
    }
}
