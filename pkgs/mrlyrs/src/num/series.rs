use crate::core::error::{overflow_error, value_error, Result};
use crate::num::factor::{gcd, mobius_sieve};
use crate::num::prime::primes;
use std::f64::consts::PI;

/// The Basel constant, pi squared over six, the value zeta takes at two.
pub const BASEL: f64 = PI * PI / 6.0;

/// The visible density, six over pi squared, the share of lattice pairs that are coprime.
pub const VISIBLE: f64 = 6.0 / (PI * PI);

/// The Catalan constant, the value the Dirichlet beta function takes at two.
pub const CATALAN: f64 = 0.915_965_594_177_219;

/// The Apery constant, the value zeta takes at three.
pub const APERY: f64 = 1.202_056_903_159_594;

/// The Euler constant, the limit of the harmonic sum less the logarithm.
pub const EULER: f64 = 0.577_215_664_901_532_9;

// THE CLASSIC SEQUENCES

/// Returns the even numbers up to the limit.
pub fn evens(limit: usize) -> Vec<usize> {
    (0..=limit).step_by(2).collect()
}

/// Returns the odd numbers up to the limit.
pub fn odds(limit: usize) -> Vec<usize> {
    (1..=limit).step_by(2).collect()
}

/// Returns the powers of two up to the limit.
pub fn binary(limit: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let mut value = 1;
    while value <= limit {
        out.push(value);
        value *= 2;
    }
    out
}

/// Returns the distinct Fibonacci numbers up to the limit.
pub fn fibonacci(limit: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let (mut a, mut b) = (0usize, 1usize);
    while a <= limit {
        if !out.contains(&a) {
            out.push(a);
        }
        let next = a + b;
        a = b;
        b = next;
    }
    out
}

/// Returns the distinct Catalan numbers up to the limit.
///
/// ```
/// assert_eq!(mrlyrs::num::series::catalan(50), vec![1, 2, 5, 14, 42]);
/// ```
pub fn catalan(limit: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let mut value: u128 = 1;
    let mut index: u128 = 0;
    while value <= limit as u128 {
        if out.last() != Some(&(value as usize)) {
            out.push(value as usize);
        }
        value = value * 2 * (2 * index + 1) / (index + 2);
        index += 1;
    }
    out
}

// THE INFINITE SUMS

/// Returns the logarithmic integral of a positive x by the Ramanujan series, the smooth count of the primes below x.
///
/// ```
/// assert!((mrlyrs::num::series::li(1_000_000.0) - 78_627.549).abs() < 1e-3);
/// ```
pub fn li(x: f64) -> f64 {
    let log = x.ln();
    let mut sum = 0.0f64;
    let mut term = 1.0f64;
    let mut odds = 0.0f64;
    for n in 1..200u32 {
        term *= log / n as f64;
        if n > 1 {
            term /= 2.0;
        }
        if !n.is_multiple_of(2) {
            odds += 1.0 / n as f64;
        }
        let piece = term * odds;
        sum += if n.is_multiple_of(2) { -piece } else { piece };
        if piece.abs() < 1e-17 * sum.abs() {
            break;
        }
    }
    EULER + log.abs().ln() + x.sqrt() * sum
}

/// Returns the partial harmonic sum, the reciprocals of one through the term count.
pub fn harmonic(terms: usize) -> f64 {
    (1..=terms).map(|k| 1.0 / k as f64).sum()
}

/// Returns the zeta value above one, the partial sum closed by its Euler-Maclaurin tail.
///
/// ```
/// assert!((mrlyrs::num::series::zeta(2.0, 1_000)? - 1.644_934_066_8).abs() < 1e-9);
/// # Ok::<(), mrlyrs::Error>(())
/// ```
///
/// # Errors
///
/// Errs at an s of one or below, and at a NaN, where the sum does not converge.
pub fn zeta(s: f64, terms: usize) -> Result<f64> {
    if s <= 1.0 || s.is_nan() {
        return value_error(format!("zeta needs an s above one, not {s}."));
    }
    let mut sum = 0.0;
    for k in 1..=terms {
        sum += (k as f64).powf(-s);
    }
    let n = terms as f64;
    Ok(sum + n.powf(1.0 - s) / (s - 1.0) - n.powf(-s) / 2.0 + s * n.powf(-s - 1.0) / 12.0)
}

/// Returns the Euler product of zeta, one over one minus p to the minus s over the primes up to the limit.
pub fn euler_product(s: f64, limit: usize) -> f64 {
    primes(limit)
        .iter()
        .map(|&p| 1.0 / (1.0 - (p as f64).powf(-s)))
        .product()
}

/// Returns the Dirichlet beta value, the alternating odd-denominator sum averaged over its last two partial sums.
pub fn beta(s: f64, terms: usize) -> f64 {
    let mut sum = 0.0;
    let mut previous = 0.0;
    for k in 0..terms {
        previous = sum;
        let term = ((2 * k + 1) as f64).powf(-s);
        sum += if k.is_multiple_of(2) { term } else { -term };
    }
    0.5 * (previous + sum)
}

/// Returns the Dirichlet lambda value, one minus two to the minus s times zeta.
///
/// # Errors
///
/// Errs at an s of one or below, and at a NaN, where zeta does not converge.
pub fn lambda(s: f64, terms: usize) -> Result<f64> {
    Ok((1.0 - 2f64.powf(-s)) * zeta(s, terms)?)
}

/// Returns the mod-eight rhythm of the number, the discriminant minus-eight character: one on one and three, minus one on five and seven, zero on the evens.
///
/// ```
/// assert_eq!(mrlyrs::num::series::chi8(3), 1);
/// assert_eq!(mrlyrs::num::series::chi8(5), -1);
/// ```
pub fn chi8(number: usize) -> i8 {
    [0, 1, 0, 1, 0, -1, 0, -1][number % 8]
}

/// Returns the mod-four rhythm of the number: zero, one, zero, minus one.
///
/// ```
/// assert_eq!(mrlyrs::num::series::chi4(7), -1);
/// ```
pub fn chi4(number: usize) -> i8 {
    [0, 1, 0, -1][number % 4]
}

/// Returns the mod-three rhythm of the number: zero, one, minus one.
pub fn chi3(number: usize) -> i8 {
    [0, 1, -1][number % 3]
}

/// Returns the L-series partial sum with a periodic rhythm painted on the terms.
pub fn dirichlet(s: f64, rhythm: &[i8], terms: usize) -> f64 {
    if rhythm.is_empty() {
        return 0.0;
    }
    let mut sum = 0.0;
    for n in 1..=terms {
        let paint = rhythm[n % rhythm.len()];
        if paint != 0 {
            sum += f64::from(paint) * (n as f64).powf(-s);
        }
    }
    sum
}

/// Counts the lattice points of the dimension-cube of the limit whose coordinates share no divisor, by Mobius inversion.
///
/// The count wraps once the limit to the dimension passes a signed hundred and twenty-eight bits.
///
/// # Errors
///
/// Errs at a zero dimension.
pub fn visible(limit: usize, dimension: u32) -> Result<u128> {
    if dimension == 0 {
        return value_error("a visible count needs a dimension above zero.");
    }
    let mu = mobius_sieve(limit);
    let mut total: i128 = 0;
    for (k, &value) in mu.iter().enumerate().skip(1) {
        if value == 0 {
            continue;
        }
        let block = (limit / k) as i128;
        total += i128::from(value) * block.pow(dimension);
    }
    Ok(total as u128)
}

// THE PARTIALS THAT WALK TO A CONSTANT

/// Returns the Wallis product of one minus one over the odd squares taken to n factors, walking to pi over four.
pub fn wallis_quarter_pi(factors: usize) -> f64 {
    let mut out = 1.0;
    for n in 1..=factors {
        let odd = (2 * n + 1) as f64;
        out *= 1.0 - 1.0 / (odd * odd);
    }
    out
}

/// Returns the Wallis product taken to n paired factors, four k squared over four k squared less one, walking to pi over two.
///
/// ```
/// assert!((mrlyrs::num::series::wallis_half_pi(1) - 4.0 / 3.0).abs() < 1e-15);
/// ```
pub fn wallis_half_pi(n: usize) -> f64 {
    let mut out = 1.0;
    for k in 1..=n {
        let square = ((2 * k) * (2 * k)) as f64;
        out *= square / (square - 1.0);
    }
    out
}

/// Returns the Leibniz alternating sum of the odd reciprocals over n terms, walking to pi over four.
///
/// ```
/// assert!((mrlyrs::num::series::leibniz(2) - 2.0 / 3.0).abs() < 1e-15);
/// ```
pub fn leibniz(n: usize) -> f64 {
    let mut out = 0.0;
    for k in 0..n {
        let term = 1.0 / (2 * k + 1) as f64;
        out += if k.is_multiple_of(2) { term } else { -term };
    }
    out
}

/// Returns the Basel sum of the reciprocal squares over n terms, walking to pi squared over six.
///
/// ```
/// assert!((mrlyrs::num::series::basel(3) - 49.0 / 36.0).abs() < 1e-15);
/// ```
pub fn basel(n: usize) -> f64 {
    (1..=n).map(|k| 1.0 / (k * k) as f64).sum()
}

/// Returns the harmonic sum of n terms less the logarithm of n, walking to the Euler-Mascheroni constant.
///
/// ```
/// assert!((mrlyrs::num::series::euler_gamma_partial(1) - 1.0).abs() < 1e-15);
/// ```
pub fn euler_gamma_partial(n: usize) -> f64 {
    if n == 0 {
        return 0.0;
    }
    harmonic(n) - (n as f64).ln()
}

/// Returns one plus one over n raised to the n, walking to the natural base.
///
/// ```
/// assert!((mrlyrs::num::series::e_partial(1) - 2.0).abs() < 1e-15);
/// ```
pub fn e_partial(n: usize) -> f64 {
    if n == 0 {
        return 1.0;
    }
    (1.0 + 1.0 / n as f64).powf(n as f64)
}

/// Returns the Mertens function at n, the Mobius values of one through n summed.
///
/// ```
/// assert_eq!(mrlyrs::num::series::mertens(100), 1);
/// ```
pub fn mertens(n: usize) -> i64 {
    mobius_sieve(n).iter().skip(1).map(|&v| i64::from(v)).sum()
}

// THE BERNOULLI FRACTIONS

fn reduce(num: i128, den: i128) -> (i128, i128) {
    let a = gcd(num.unsigned_abs(), den.unsigned_abs()) as i128;
    let sign = if den < 0 { -1 } else { 1 };
    if a == 0 {
        return (0, 1);
    }
    (sign * num / a, sign * den / a)
}

fn binomial(n: usize, k: usize) -> i128 {
    let mut out: i128 = 1;
    for i in 0..k {
        out = out * (n - i) as i128 / (i + 1) as i128;
    }
    out
}

/// Builds the first Bernoulli numbers as exact reduced fractions on the minus one half convention.
///
/// # Errors
///
/// Errs past a count of thirty-two, where the fractions overrun a signed hundred and twenty-eight bits.
pub fn bernoulli(count: usize) -> Result<Vec<(i128, i128)>> {
    if count > 32 {
        return overflow_error(format!(
            "the exact fractions overflow past thirty-two, not {count}."
        ));
    }
    let mut out: Vec<(i128, i128)> = Vec::with_capacity(count);
    for m in 0..count {
        if m == 0 {
            out.push((1, 1));
            continue;
        }
        let mut sum = (0i128, 1i128);
        for (j, &(num, den)) in out.iter().enumerate() {
            let weight = binomial(m + 1, j) * num;
            sum = reduce(sum.0 * den + weight * sum.1, sum.1 * den);
        }
        out.push(reduce(-sum.0, sum.1 * (m + 1) as i128));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::num::lattice::coprime_pairs;
    use std::f64::consts::E;

    #[test]
    fn evens_and_odds() {
        assert_eq!(evens(8), vec![0, 2, 4, 6, 8]);
        assert_eq!(odds(8), vec![1, 3, 5, 7]);
    }

    #[test]
    fn binary_powers() {
        assert_eq!(binary(20), vec![1, 2, 4, 8, 16]);
        assert_eq!(binary(0), Vec::<usize>::new());
    }

    #[test]
    fn fibonacci_dedups_zero_one() {
        assert_eq!(fibonacci(13), vec![0, 1, 2, 3, 5, 8, 13]);
    }

    #[test]
    fn catalan_dedups_the_double_one() {
        assert_eq!(catalan(1500), vec![1, 2, 5, 14, 42, 132, 429, 1430]);
        assert_eq!(catalan(0), Vec::<usize>::new());
    }

    #[test]
    fn catalan_matches_the_binomial_form() {
        let list = catalan(40_000_000);
        for (n, &value) in list.iter().enumerate().skip(1) {
            let m = n + 1;
            let mut binom: u128 = 1;
            for i in 0..m {
                binom = binom * (2 * m - i) as u128 / (i + 1) as u128;
            }
            assert_eq!(value as u128, binom / (m as u128 + 1), "{m}");
        }
    }

    #[test]
    fn zeta_meets_the_basel_the_apery_and_the_quartic_sum() {
        assert!((zeta(2.0, 10_000).unwrap() - BASEL).abs() < 1e-9);
        assert!((zeta(3.0, 10_000).unwrap() - APERY).abs() < 1e-12);
        assert!((zeta(4.0, 10_000).unwrap() - PI.powi(4) / 90.0).abs() < 1e-9);
    }

    #[test]
    fn the_euler_product_meets_the_basel_and_the_apery_sum() {
        assert!((euler_product(2.0, 100_000) - BASEL).abs() < 1e-5);
        assert!((euler_product(3.0, 100_000) - APERY).abs() < 1e-6);
    }

    #[test]
    fn beta_walks_to_catalan_and_to_a_quarter_turn() {
        assert!((beta(2.0, 1_000_000) - CATALAN).abs() < 1e-9);
        assert!((beta(1.0, 1_000_000) - PI / 4.0).abs() < 1e-6);
    }

    #[test]
    fn half_lambda_is_the_grid_fluctuation_constant() {
        assert!((0.5 * lambda(4.0, 10_000).unwrap() - PI.powi(4) / 192.0).abs() < 1e-9);
    }

    #[test]
    fn the_mod_four_rhythm_paints_the_beta_series() {
        let painted = dirichlet(2.0, &[0, 1, 0, -1], 1_000_000);
        assert!((painted - beta(2.0, 1_000_000)).abs() < 1e-9);
    }

    #[test]
    fn the_mod_three_rhythm_paints_the_l_series() {
        let painted = dirichlet(1.0, &[0, 1, -1], 1_000_000);
        assert!((painted - PI / (3.0 * 3f64.sqrt())).abs() < 1e-5);
    }

    #[test]
    fn visible_counts_the_coprime_pairs_of_a_window() {
        for n in 1..=2_000 {
            assert_eq!(
                visible(n, 2).unwrap(),
                u128::from(coprime_pairs(n)),
                "window {n}"
            );
        }
    }

    #[test]
    fn the_visible_density_is_one_over_zeta() {
        let flat = visible(10_000, 2).unwrap() as f64 / 1e8;
        assert!((flat - VISIBLE).abs() < 1e-3);
        let cube = visible(1_000, 3).unwrap() as f64 / 1e9;
        assert!((cube - 1.0 / zeta(3.0, 100_000).unwrap()).abs() < 1e-2);
    }

    #[test]
    fn the_harmonic_walk_leaves_the_euler_mascheroni_gap() {
        let gap = harmonic(1_000_000) - 1_000_000f64.ln();
        assert!((gap - 0.577_215_664_9).abs() < 1e-5);
    }

    #[test]
    fn bernoulli_pins_the_known_fractions() {
        let list = bernoulli(32).unwrap();
        assert_eq!(list[0], (1, 1));
        assert_eq!(list[1], (-1, 2));
        assert_eq!(list[2], (1, 6));
        assert_eq!(list[3], (0, 1));
        assert_eq!(list[4], (-1, 30));
        assert_eq!(list[12], (-691, 2730));
    }

    #[test]
    fn every_odd_bernoulli_past_the_first_is_zero() {
        for (index, &(num, den)) in bernoulli(32).unwrap().iter().enumerate().skip(3) {
            if !index.is_multiple_of(2) {
                assert_eq!((num, den), (0, 1), "index {index}");
            }
        }
    }

    #[test]
    fn refuses_a_zero_dimension() {
        assert!(visible(10, 0).is_err());
        assert!(visible(10, 1).is_ok());
    }

    #[test]
    fn refuses_a_count_past_thirty_two() {
        assert!(bernoulli(33).is_err());
        assert!(bernoulli(32).is_ok());
    }

    #[test]
    fn refuses_an_s_at_one_or_below() {
        assert!(zeta(1.0, 10).is_err());
        assert!(zeta(0.5, 10).is_err());
        assert!(zeta(f64::NAN, 10).is_err());
        assert!(zeta(1.5, 10).is_ok());
    }

    #[test]
    fn the_small_partials_are_their_exact_fractions() {
        assert!((wallis_half_pi(1) - 4.0 / 3.0).abs() < 1e-15);
        assert!((wallis_half_pi(2) - 64.0 / 45.0).abs() < 1e-15);
        assert!((wallis_quarter_pi(1) - 8.0 / 9.0).abs() < 1e-15);
        assert!((leibniz(1) - 1.0).abs() < 1e-15);
        assert!((leibniz(2) - 2.0 / 3.0).abs() < 1e-15);
        assert!((basel(3) - 49.0 / 36.0).abs() < 1e-15);
        assert!((euler_gamma_partial(1) - 1.0).abs() < 1e-15);
        assert!((e_partial(2) - 2.25).abs() < 1e-15);
    }

    #[test]
    fn the_six_partials_walk_to_their_constants() {
        assert!((wallis_quarter_pi(1_000_000) - PI / 4.0).abs() < 1e-6);
        assert!((wallis_half_pi(200_000) - PI / 2.0).abs() < 1e-5);
        assert!((leibniz(200_000) - PI / 4.0).abs() < 1e-5);
        assert!((basel(200_000) - PI * PI / 6.0).abs() < 1e-4);
        assert!((euler_gamma_partial(200_000) - EULER).abs() < 1e-5);
        assert!((e_partial(200_000) - E).abs() < 1e-4);
    }

    #[test]
    fn mertens_sums_the_mobius_values() {
        assert_eq!(mertens(0), 0);
        assert_eq!(mertens(1), 1);
        assert_eq!(mertens(100), 1);
        assert_eq!(mertens(1000), 2);
        assert_eq!(mertens(10_000), -23);
    }

    #[test]
    fn the_rhythms_repeat_over_their_full_period() {
        let four: Vec<i8> = (0..8).map(chi4).collect();
        assert_eq!(four, vec![0, 1, 0, -1, 0, 1, 0, -1]);
        let three: Vec<i8> = (0..6).map(chi3).collect();
        assert_eq!(three, vec![0, 1, -1, 0, 1, -1]);
        let eight: Vec<i8> = (0..16).map(chi8).collect();
        assert_eq!(
            eight,
            vec![0, 1, 0, 1, 0, -1, 0, -1, 0, 1, 0, 1, 0, -1, 0, -1]
        );
    }

    #[test]
    fn the_mod_eight_rhythm_multiplies_across_the_odd_numbers() {
        for a in (1..=99usize).step_by(2) {
            for b in (1..=99usize).step_by(2) {
                assert_eq!(chi8(a) * chi8(b), chi8(a * b), "{a} {b}");
            }
        }
    }

    #[test]
    fn li_pins_the_smooth_prime_counts() {
        assert!((li(2.0) - 1.045_163_780_1).abs() < 1e-9);
        assert!((li(1_000.0) - 177.609_657_990_2).abs() < 1e-8);
        assert!((li(10_000.0) - 1_246.137_215_9).abs() < 1e-6);
        assert!((li(100_000.0) - 9_629.809_001_1).abs() < 1e-6);
        assert!((li(1_000_000.0) - 78_627.549_159_5).abs() < 1e-6);
    }
}
