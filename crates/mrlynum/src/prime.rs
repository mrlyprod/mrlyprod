use crate::classics::primes;

/// Returns whether the number is prime, by trial division on the six-step wheel.
pub fn is_prime(number: usize) -> bool {
    if number < 2 {
        return false;
    }
    if number < 4 {
        return true;
    }
    if number.is_multiple_of(2) || number.is_multiple_of(3) {
        return false;
    }
    let mut step = 5;
    while step <= number / step {
        if number.is_multiple_of(step) || number.is_multiple_of(step + 2) {
            return false;
        }
        step += 6;
    }
    true
}

/// Returns every rectangle of the number as a pair of sides, the shorter first, ascending.
///
/// ```
/// assert_eq!(mrlynum::prime::rectangles(6), vec![(1, 6), (2, 3)]);
/// ```
pub fn rectangles(number: usize) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    if number == 0 {
        return out;
    }
    let mut a = 1;
    while a <= number / a {
        if number.is_multiple_of(a) {
            out.push((a, number / a));
        }
        a += 1;
    }
    out
}

/// Returns every pair of primes summing to the number, odd numbers included, the smaller first, ascending.
pub fn splits(number: usize) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    if number < 4 {
        return out;
    }
    for p in primes(number / 2) {
        if is_prime(number - p) {
            out.push((p, number - p));
        }
    }
    out
}

/// Returns the smallest pair of positive sides whose squares sum to the number, when one exists.
pub fn squares(number: usize) -> Option<(usize, usize)> {
    let mut a = 1;
    while 2 * a * a <= number {
        let rest = number - a * a;
        let b = rest.isqrt();
        if b * b == rest {
            return Some((a, b));
        }
        a += 1;
    }
    None
}

/// The prime object: one prime with its rank, the step behind it and the shapes it makes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Prime {
    /// The prime itself.
    pub value: usize,
    /// The one-based rank in the prime sequence, so two has index one.
    pub index: usize,
    /// The distance from the previous prime, zero for two.
    pub gap: usize,
    /// The twin flag: whether a prime sits exactly two away on either side, false for two.
    pub twin: bool,
    /// The two positive sides whose squares sum to the prime, when they exist.
    pub squares: Option<(usize, usize)>,
}

/// Returns one prime object for every prime up to and including the limit.
pub fn study(limit: usize) -> Vec<Prime> {
    let list = primes(limit);
    list.iter()
        .enumerate()
        .map(|(i, &value)| Prime {
            value,
            index: i + 1,
            gap: if i == 0 { 0 } else { value - list[i - 1] },
            twin: is_prime(value - 2) || is_prime(value + 2),
            squares: squares(value),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_prime_agrees_with_the_sieve() {
        let sieved = primes(100_000);
        for number in 0..=100_000 {
            assert_eq!(
                is_prime(number),
                sieved.binary_search(&number).is_ok(),
                "{number}"
            );
        }
    }

    #[test]
    fn a_single_rectangle_means_prime_above_one() {
        assert!(rectangles(0).is_empty());
        assert_eq!(rectangles(1), vec![(1, 1)]);
        assert_eq!(
            rectangles(36),
            vec![(1, 36), (2, 18), (3, 12), (4, 9), (6, 6)]
        );
        for number in 2..=1_000 {
            assert_eq!(rectangles(number).len() == 1, is_prime(number), "{number}");
        }
    }

    #[test]
    fn every_even_number_splits_into_two_primes() {
        for number in (4..=2_000).step_by(2) {
            let pairs = splits(number);
            assert!(!pairs.is_empty(), "{number}");
            for pair in pairs.windows(2) {
                assert!(pair[0].0 < pair[1].0, "{number}");
            }
            for (p, q) in pairs {
                assert!(is_prime(p) && is_prime(q), "{number}");
                assert!(p <= q && p + q == number, "{number}");
            }
        }
    }

    #[test]
    fn an_odd_number_splits_only_through_the_two() {
        assert_eq!(splits(5), vec![(2, 3)]);
        assert_eq!(splits(9), vec![(2, 7)]);
        assert!(splits(27).is_empty());
    }

    #[test]
    fn a_prime_is_a_sum_of_two_squares_only_when_it_is_two_or_one_past_a_multiple_of_four() {
        for prime in study(10_000) {
            let expected = prime.value == 2 || prime.value % 4 == 1;
            assert_eq!(prime.squares.is_some(), expected, "{}", prime.value);
        }
    }

    #[test]
    fn study_pins_the_first_primes() {
        let found = study(13);
        let values: Vec<usize> = found.iter().map(|p| p.value).collect();
        let indices: Vec<usize> = found.iter().map(|p| p.index).collect();
        let gaps: Vec<usize> = found.iter().map(|p| p.gap).collect();
        let twins: Vec<bool> = found.iter().map(|p| p.twin).collect();
        assert_eq!(values, vec![2, 3, 5, 7, 11, 13]);
        assert_eq!(indices, vec![1, 2, 3, 4, 5, 6]);
        assert_eq!(gaps, vec![0, 1, 2, 2, 4, 2]);
        assert_eq!(twins, vec![false, true, true, true, true, true]);
        assert_eq!(found[0].squares, Some((1, 1)));
        assert_eq!(found[2].squares, Some((1, 2)));
        assert_eq!(found[3].squares, None);
        assert!(study(17).last().unwrap().twin);
        assert!(!study(23).last().unwrap().twin);
    }
}
