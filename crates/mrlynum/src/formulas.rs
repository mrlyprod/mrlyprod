use crate::classics::primes;
use crate::factor::mobius_sieve;
use crate::series::harmonic;

pub use crate::series::li;

// SIEVES

fn primal(limit: usize) -> Vec<bool> {
    let mut flag = vec![limit >= 2; limit + 1];
    for slot in flag.iter_mut().take(2.min(limit + 1)) {
        *slot = false;
    }
    let mut p = 2;
    while p * p <= limit {
        if flag[p] {
            let mut m = p * p;
            while m <= limit {
                flag[m] = false;
                m += p;
            }
        }
        p += 1;
    }
    flag
}

fn pairs(flag: &[bool], number: usize) -> usize {
    (2..=number / 2)
        .filter(|&p| flag[p] && flag[number - p])
        .count()
}

// CONSTANTS BY SUMMATION

/// Returns the Wallis product taken to n paired factors, four k squared over four k squared less one, walking to pi over two.
///
/// ```
/// assert!((mrlynum::formulas::wallis(1) - 4.0 / 3.0).abs() < 1e-15);
/// ```
pub fn wallis(n: usize) -> f64 {
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
/// assert!((mrlynum::formulas::leibniz(2) - 2.0 / 3.0).abs() < 1e-15);
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
/// assert!((mrlynum::formulas::basel(3) - 49.0 / 36.0).abs() < 1e-15);
/// ```
pub fn basel(n: usize) -> f64 {
    (1..=n).map(|k| 1.0 / (k * k) as f64).sum()
}

/// Returns the harmonic sum of n terms less the logarithm of n, walking to the Euler-Mascheroni constant.
///
/// ```
/// assert!((mrlynum::formulas::euler_gamma_partial(1) - 1.0).abs() < 1e-15);
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
/// assert!((mrlynum::formulas::e_partial(1) - 2.0).abs() < 1e-15);
/// ```
pub fn e_partial(n: usize) -> f64 {
    if n == 0 {
        return 1.0;
    }
    (1.0 + 1.0 / n as f64).powf(n as f64)
}

// THE PRIMES COUNTED

/// Returns the count of primes at or below n.
///
/// ```
/// assert_eq!(mrlynum::formulas::prime_count(100), 25);
/// ```
pub fn prime_count(n: usize) -> usize {
    primes(n).len()
}

/// Returns the count of unordered pairs of primes summing to the number, zero below four.
///
/// ```
/// assert_eq!(mrlynum::formulas::goldbach(100), 6);
/// ```
pub fn goldbach(number: usize) -> usize {
    if number < 4 {
        return 0;
    }
    pairs(&primal(number), number)
}

/// Returns the count of prime pairs at every even number from four up to the top, one entry per even number.
///
/// ```
/// assert_eq!(mrlynum::formulas::goldbach_record(10), vec![1, 1, 1, 2]);
/// ```
pub fn goldbach_record(top: usize) -> Vec<usize> {
    if top < 4 {
        return Vec::new();
    }
    let flag = primal(top);
    (2..=top / 2).map(|k| pairs(&flag, 2 * k)).collect()
}

/// Returns the Mertens function at n, the Mobius values of one through n summed.
///
/// ```
/// assert_eq!(mrlynum::formulas::mertens(100), 1);
/// ```
pub fn mertens(n: usize) -> i64 {
    mobius_sieve(n).iter().skip(1).map(|&v| i64::from(v)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prime::splits;
    use std::f64::consts::{E, PI};

    #[test]
    fn the_small_partials_are_their_exact_fractions() {
        assert!((wallis(1) - 4.0 / 3.0).abs() < 1e-15);
        assert!((wallis(2) - 64.0 / 45.0).abs() < 1e-15);
        assert!((leibniz(1) - 1.0).abs() < 1e-15);
        assert!((leibniz(2) - 2.0 / 3.0).abs() < 1e-15);
        assert!((basel(3) - 49.0 / 36.0).abs() < 1e-15);
        assert!((euler_gamma_partial(1) - 1.0).abs() < 1e-15);
        assert!((e_partial(2) - 2.25).abs() < 1e-15);
    }

    #[test]
    fn the_five_partials_walk_to_their_constants() {
        assert!((wallis(200_000) - PI / 2.0).abs() < 1e-5);
        assert!((leibniz(200_000) - PI / 4.0).abs() < 1e-5);
        assert!((basel(200_000) - PI * PI / 6.0).abs() < 1e-4);
        assert!((euler_gamma_partial(200_000) - crate::series::EULER).abs() < 1e-5);
        assert!((e_partial(200_000) - E).abs() < 1e-4);
    }

    #[test]
    fn li_matches_its_own_series_at_two() {
        assert!((li(2.0) - 1.045_163_780_117_493).abs() < 1e-12);
    }

    #[test]
    fn the_prime_count_is_the_sieve() {
        assert_eq!(prime_count(1), 0);
        assert_eq!(prime_count(100), 25);
        assert_eq!(prime_count(1000), 168);
        assert_eq!(prime_count(10_000), 1229);
    }

    #[test]
    fn goldbach_counts_the_prime_pairs_the_long_way() {
        assert_eq!(goldbach(4), 1);
        assert_eq!(goldbach(100), 6);
        assert_eq!(goldbach(1000), 28);
        for even in (4..=600).step_by(2) {
            assert_eq!(goldbach(even), splits(even).len());
        }
    }

    #[test]
    fn the_goldbach_record_never_reaches_zero_below_ten_thousand() {
        let record = goldbach_record(10_000);
        assert_eq!(record.len(), 4999);
        assert_eq!(record[0], 1);
        assert_eq!(record[498], 28);
        assert_eq!(record.iter().copied().min(), Some(1));
    }

    #[test]
    fn mertens_sums_the_mobius_values() {
        assert_eq!(mertens(0), 0);
        assert_eq!(mertens(1), 1);
        assert_eq!(mertens(100), 1);
        assert_eq!(mertens(1000), 2);
        assert_eq!(mertens(10_000), -23);
    }
}
