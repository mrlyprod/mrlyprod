/// Returns the greatest common divisor of two numbers by the Euclidean algorithm, zero for two zeroes.
pub fn gcd(a: usize, b: usize) -> usize {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// Returns the least common multiple of two numbers, zero when either side is zero.
///
/// Wraps silently once the multiple passes the pointer width.
pub fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        return 0;
    }
    a / gcd(a, b) * b
}

/// Returns whether two numbers share no divisor above one.
pub fn coprime(a: usize, b: usize) -> bool {
    gcd(a, b) == 1
}

fn peel(prime: u64, rest: &mut u64, out: &mut Vec<(u64, u32)>) {
    let mut power = 0;
    while rest.is_multiple_of(prime) {
        *rest /= prime;
        power += 1;
    }
    if power > 0 {
        out.push((prime, power));
    }
}

/// Returns the prime and exponent pairs of a wide number in ascending primes, by trial division on the six-step wheel.
///
/// ```
/// assert_eq!(mrlynum::factor::factorize_wide(999_999_999_989), vec![(999_999_999_989, 1)]);
/// ```
pub fn factorize_wide(number: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    let mut rest = number;
    if rest < 2 {
        return out;
    }
    peel(2, &mut rest, &mut out);
    peel(3, &mut rest, &mut out);
    let mut step = 5;
    while step <= rest / step {
        peel(step, &mut rest, &mut out);
        peel(step + 2, &mut rest, &mut out);
        step += 6;
    }
    if rest > 1 {
        out.push((rest, 1));
    }
    out
}

/// Returns the prime and exponent pairs of the number in ascending primes, by trial division on the six-step wheel.
///
/// ```
/// assert_eq!(mrlynum::factor::factorize(240), vec![(2, 4), (3, 1), (5, 1)]);
/// ```
pub fn factorize(number: usize) -> Vec<(usize, u32)> {
    factorize_wide(number as u64)
        .into_iter()
        .map(|(prime, power)| (prime as usize, power))
        .collect()
}

/// Builds every divisor of the number from its factorization, ascending, empty for zero.
pub fn divisors(number: usize) -> Vec<usize> {
    if number == 0 {
        return Vec::new();
    }
    let mut out = vec![1];
    for (prime, power) in factorize(number) {
        let mut next = Vec::with_capacity(out.len() * (power as usize + 1));
        for &divisor in &out {
            let mut value = divisor;
            next.push(value);
            for _ in 0..power {
                value *= prime;
                next.push(value);
            }
        }
        out = next;
    }
    out.sort_unstable();
    out
}

/// Returns the sum of every divisor of the number raised to the power, so power zero counts them.
///
/// Stays exact for a power of two or below, above which the sum can wrap past a hundred and twenty-eight bits.
pub fn sigma(number: usize, power: u32) -> u128 {
    divisors(number)
        .iter()
        .map(|&divisor| (divisor as u128).pow(power))
        .sum()
}

/// Returns the sum of the proper divisors of the number, its divisor sum less itself, zero for zero and for one.
pub fn aliquot(number: usize) -> usize {
    (sigma(number, 1) - number as u128) as usize
}

/// Returns the divisor sum with a periodic rhythm painted on each divisor, zero for zero and for an empty rhythm.
///
/// ```
/// assert_eq!(mrlynum::factor::twisted(5, &[0, 1, 0, -1]), 2);
/// ```
pub fn twisted(number: usize, rhythm: &[i8]) -> i64 {
    if rhythm.is_empty() {
        return 0;
    }
    divisors(number)
        .iter()
        .map(|&divisor| i64::from(rhythm[divisor % rhythm.len()]))
        .sum()
}

/// Returns the Mobius value of the number: zero for zero or a squared factor, else minus one to the count of primes.
pub fn mobius(number: usize) -> i8 {
    if number == 0 {
        return 0;
    }
    let mut out = 1;
    for (_, power) in factorize(number) {
        if power > 1 {
            return 0;
        }
        out = -out;
    }
    out
}

/// Sieves the Mobius values of zero through the limit in one pass.
pub fn mobius_sieve(limit: usize) -> Vec<i8> {
    let mut out = vec![1i8; limit + 1];
    let mut composite = vec![false; limit + 1];
    out[0] = 0;
    for p in 2..=limit {
        if composite[p] {
            continue;
        }
        for m in (p..=limit).step_by(p) {
            composite[m] = m != p;
            out[m] = -out[m];
        }
        if p <= limit / p {
            for m in (p * p..=limit).step_by(p * p) {
                out[m] = 0;
            }
        }
    }
    out
}

/// Returns the Euler totient of the number from its factorization, zero for zero and one for one.
pub fn totient(number: usize) -> usize {
    if number == 0 {
        return 0;
    }
    let mut out = number;
    for (prime, _) in factorize(number) {
        out -= out / prime;
    }
    out
}

/// Returns the radical of the number, the product of its distinct primes, zero for zero and one for one.
pub fn radical(number: usize) -> usize {
    if number == 0 {
        return 0;
    }
    factorize(number).iter().map(|&(prime, _)| prime).product()
}

/// Returns whether no prime squares into the number, true for one and false for zero.
pub fn squarefree(number: usize) -> bool {
    number != 0 && factorize(number).iter().all(|&(_, power)| power < 2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lattice::totients;
    use crate::prime::rectangles;
    use crate::series::{chi4, chi8};

    fn represented(m: usize, weight: usize) -> usize {
        let mut count = 0;
        for a in -20i64..=20 {
            for b in -20i64..=20 {
                if a * a + weight as i64 * b * b == m as i64 {
                    count += 1;
                }
            }
        }
        count
    }

    #[test]
    fn factorize_pins_the_hand_checked_shapes() {
        assert_eq!(factorize(240), vec![(2, 4), (3, 1), (5, 1)]);
        assert!(factorize(1).is_empty());
        assert!(factorize(0).is_empty());
        assert_eq!(factorize(97), vec![(97, 1)]);
        assert_eq!(factorize(243), vec![(3, 5)]);
        assert_eq!(factorize(1_000_003), vec![(1_000_003, 1)]);
    }

    #[test]
    fn a_factorization_multiplies_back_to_its_number() {
        for number in 2..=5_000usize {
            let parts = factorize(number);
            let rebuilt: usize = parts.iter().map(|&(p, e)| p.pow(e)).product();
            assert_eq!(rebuilt, number, "{number}");
            for pair in parts.windows(2) {
                assert!(pair[0].0 < pair[1].0, "{number}");
            }
        }
    }

    #[test]
    fn the_divisor_count_is_sigma_zero_and_twice_the_rectangles_less_a_square() {
        assert!(divisors(0).is_empty());
        assert_eq!(divisors(1), vec![1]);
        assert_eq!(divisors(28), vec![1, 2, 4, 7, 14, 28]);
        for number in 1..=2_000usize {
            let count = divisors(number).len() as u128;
            assert_eq!(count, sigma(number, 0), "{number}");
            let root = number.isqrt();
            let square = u128::from(root * root == number);
            assert_eq!(
                count,
                2 * rectangles(number).len() as u128 - square,
                "{number}"
            );
        }
    }

    #[test]
    fn sigma_pins_the_hand_checked_sums() {
        assert_eq!(sigma(6, 1), 12);
        assert_eq!(sigma(240, 0), 20);
        assert_eq!(sigma(1, 3), 1);
        assert_eq!(sigma(0, 1), 0);
    }

    #[test]
    fn the_aliquot_sum_is_sigma_one_less_the_number() {
        assert_eq!(aliquot(0), 0);
        assert_eq!(aliquot(1), 0);
        assert_eq!(aliquot(6), 6);
        assert_eq!(aliquot(28), 28);
        assert_eq!(aliquot(12), 16);
        for number in 1..=2_000usize {
            assert_eq!(
                sigma(number, 1),
                aliquot(number) as u128 + number as u128,
                "{number}"
            );
        }
    }

    #[test]
    fn the_mod_four_twist_counts_the_sums_of_two_squares() {
        let rhythm: Vec<i8> = (0..4).map(chi4).collect();
        for m in 1..=300usize {
            assert_eq!(represented(m, 1) as i64, 4 * twisted(m, &rhythm), "{m}");
        }
    }

    #[test]
    fn the_mod_eight_twist_counts_a_square_plus_two_squares() {
        let rhythm: Vec<i8> = (0..8).map(chi8).collect();
        for m in 1..=300usize {
            assert_eq!(represented(m, 2) as i64, 2 * twisted(m, &rhythm), "{m}");
        }
    }

    #[test]
    fn the_flat_twist_is_the_divisor_count() {
        assert_eq!(twisted(0, &[1]), 0);
        assert_eq!(twisted(12, &[]), 0);
        for number in 1..=1_000usize {
            assert_eq!(twisted(number, &[1]) as u128, sigma(number, 0), "{number}");
        }
    }

    #[test]
    fn mobius_starts_the_known_sequence() {
        let start: Vec<i8> = (1..=10).map(mobius).collect();
        assert_eq!(start, vec![1, -1, -1, 0, -1, 1, -1, 0, 0, 1]);
        assert_eq!(mobius(0), 0);
        assert_eq!(mobius(240), 0);
        assert_eq!(mobius(30), -1);
    }

    #[test]
    fn the_mobius_sieve_agrees_with_the_single_value() {
        let sieved = mobius_sieve(10_000);
        for (number, &value) in sieved.iter().enumerate() {
            assert_eq!(value, mobius(number), "{number}");
        }
    }

    #[test]
    fn totient_agrees_with_the_lattice_sieve() {
        let sieved = totients(5_000);
        for (number, &value) in sieved.iter().enumerate() {
            assert_eq!(totient(number) as u64, value, "{number}");
        }
    }

    #[test]
    fn the_radical_keeps_one_of_each_prime_and_equals_a_squarefree_number() {
        assert_eq!(radical(240), 30);
        assert_eq!(radical(0), 0);
        assert_eq!(radical(1), 1);
        assert!(squarefree(30));
        assert!(!squarefree(240));
        assert!(squarefree(1));
        assert!(!squarefree(0));
        for number in 1..=2_000usize {
            assert_eq!(squarefree(number), radical(number) == number, "{number}");
        }
    }

    #[test]
    fn gcd_times_lcm_is_the_product_and_a_gcd_of_one_means_coprime() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(lcm(0, 7), 0);
        for a in 1..=60usize {
            for b in 1..=60usize {
                assert_eq!(gcd(a, b) * lcm(a, b), a * b, "{a} {b}");
                assert_eq!(coprime(a, b), gcd(a, b) == 1, "{a} {b}");
            }
        }
    }
}
