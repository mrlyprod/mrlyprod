use crate::factor::mobius_sieve;
use crate::formulas::primes;
use std::f64::consts::PI;

/// The Basel constant, pi squared over six, the value zeta takes at two.
pub const BASEL: f64 = PI * PI / 6.0;

/// The visible density, six over pi squared, the share of lattice pairs that are coprime.
pub const VISIBLE: f64 = 6.0 / (PI * PI);

/// The Catalan constant, the value the Dirichlet beta function takes at two.
pub const CATALAN: f64 = 0.915_965_594_177_219;

/// The Apery constant, the value zeta takes at three.
pub const APERY: f64 = 1.202_056_903_159_594;

/// Returns the partial harmonic sum, the reciprocals of one through the term count.
pub fn harmonic(terms: usize) -> f64 {
    (1..=terms).map(|k| 1.0 / k as f64).sum()
}

/// Returns the zeta value above one: the partial sum closed by its Euler-Maclaurin tail.
///
/// Panics at an s of one or below, where the sum does not converge.
pub fn zeta(s: f64, terms: usize) -> f64 {
    assert!(s > 1.0, "zeta needs an s above one");
    let mut sum = 0.0;
    for k in 1..=terms {
        sum += (k as f64).powf(-s);
    }
    let n = terms as f64;
    sum + n.powf(1.0 - s) / (s - 1.0) - n.powf(-s) / 2.0 + s * n.powf(-s - 1.0) / 12.0
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
pub fn lambda(s: f64, terms: usize) -> f64 {
    (1.0 - 2f64.powf(-s)) * zeta(s, terms)
}

/// Returns the mod-eight rhythm of the number, the discriminant minus-eight character: one on one and three, minus one on five and seven, zero on the evens.
///
/// ```
/// assert_eq!(mrlymath::series::chi8(3), 1);
/// assert_eq!(mrlymath::series::chi8(5), -1);
/// ```
pub fn chi8(number: usize) -> i8 {
    [0, 1, 0, 1, 0, -1, 0, -1][number % 8]
}

/// Returns the mod-four rhythm of the number: zero, one, zero, minus one.
///
/// ```
/// assert_eq!(mrlymath::series::chi4(7), -1);
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
/// Panics at a zero dimension, and wraps once the limit to the dimension passes a signed hundred and twenty-eight bits.
pub fn visible(limit: usize, dimension: u32) -> u128 {
    assert!(dimension > 0, "visible needs a dimension above zero");
    let mu = mobius_sieve(limit);
    let mut total: i128 = 0;
    for (k, &value) in mu.iter().enumerate().skip(1) {
        if value == 0 {
            continue;
        }
        let block = (limit / k) as i128;
        total += i128::from(value) * block.pow(dimension);
    }
    total as u128
}

/// Returns the Wallis product of one minus one over the odd squares, walking to pi over four.
pub fn wallis(factors: usize) -> f64 {
    let mut out = 1.0;
    for n in 1..=factors {
        let odd = (2 * n + 1) as f64;
        out *= 1.0 - 1.0 / (odd * odd);
    }
    out
}

fn reduce(num: i128, den: i128) -> (i128, i128) {
    let (mut a, mut b) = (num.abs(), den.abs());
    while b != 0 {
        (a, b) = (b, a % b);
    }
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
/// Panics past a count of thirty-two, where the exact fractions overflow a signed hundred and twenty-eight bits.
pub fn bernoulli(count: usize) -> Vec<(i128, i128)> {
    assert!(count <= 32, "the exact fractions overflow past thirty-two");
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
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lattice::coprime_pairs;

    #[test]
    fn zeta_meets_the_basel_the_apery_and_the_quartic_sum() {
        assert!((zeta(2.0, 10_000) - BASEL).abs() < 1e-9);
        assert!((zeta(3.0, 10_000) - APERY).abs() < 1e-12);
        assert!((zeta(4.0, 10_000) - PI.powi(4) / 90.0).abs() < 1e-9);
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
        assert!((0.5 * lambda(4.0, 10_000) - PI.powi(4) / 192.0).abs() < 1e-9);
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
            assert_eq!(visible(n, 2), u128::from(coprime_pairs(n)), "window {n}");
        }
    }

    #[test]
    fn the_visible_density_is_one_over_zeta() {
        let flat = visible(10_000, 2) as f64 / 1e8;
        assert!((flat - VISIBLE).abs() < 1e-3);
        let cube = visible(1_000, 3) as f64 / 1e9;
        assert!((cube - 1.0 / zeta(3.0, 100_000)).abs() < 1e-2);
    }

    #[test]
    fn the_harmonic_walk_leaves_the_euler_mascheroni_gap() {
        let gap = harmonic(1_000_000) - 1_000_000f64.ln();
        assert!((gap - 0.577_215_664_9).abs() < 1e-5);
    }

    #[test]
    fn bernoulli_pins_the_known_fractions() {
        let list = bernoulli(32);
        assert_eq!(list[0], (1, 1));
        assert_eq!(list[1], (-1, 2));
        assert_eq!(list[2], (1, 6));
        assert_eq!(list[3], (0, 1));
        assert_eq!(list[4], (-1, 30));
        assert_eq!(list[12], (-691, 2730));
    }

    #[test]
    fn every_odd_bernoulli_past_the_first_is_zero() {
        for (index, &(num, den)) in bernoulli(32).iter().enumerate().skip(3) {
            if !index.is_multiple_of(2) {
                assert_eq!((num, den), (0, 1), "index {index}");
            }
        }
    }

    #[test]
    #[should_panic(expected = "visible needs a dimension above zero")]
    fn visible_refuses_a_zero_dimension() {
        let _ = visible(10, 0);
    }

    #[test]
    #[should_panic(expected = "the exact fractions overflow past thirty-two")]
    fn bernoulli_refuses_a_count_past_thirty_two() {
        let _ = bernoulli(33);
    }

    #[test]
    #[should_panic(expected = "zeta needs an s above one")]
    fn zeta_refuses_an_s_of_one() {
        let _ = zeta(1.0, 10);
    }

    #[test]
    fn wallis_walks_to_a_quarter_turn() {
        assert!((wallis(1_000_000) - PI / 4.0).abs() < 1e-6);
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
}
