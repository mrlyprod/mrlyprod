use crate::classics::primes;
use crate::factor::factorize_wide;
use crate::series::li;

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

/// Returns the smallest prime at or above the number.
///
/// ```
/// assert_eq!(mrlynum::prime::prime_from(90), 97);
/// ```
pub fn prime_from(number: usize) -> usize {
    let mut n = number.max(2);
    while !is_prime(n) {
        n += 1;
    }
    n
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

/// The sieve of Eratosthenes taken one prime at a time, each number remembering which prime struck it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sieve {
    types: Vec<u8>,
    at: usize,
    rank: usize,
    struck: usize,
    done: bool,
}

impl Sieve {
    /// Starts a sieve over zero through the limit with every number untouched; it is done at once when no prime has its square inside.
    pub fn new(limit: usize) -> Sieve {
        let mut sieve = Sieve {
            types: vec![0; limit + 1],
            at: 2,
            rank: 0,
            struck: 0,
            done: false,
        };
        sieve.settle();
        sieve
    }
    fn settle(&mut self) {
        while self.at < self.types.len() && self.types[self.at] != 0 {
            self.at += 1;
        }
        if self.at * self.at >= self.types.len() {
            for number in 2..self.types.len() {
                if self.types[number] == 0 {
                    self.types[number] = 1;
                }
            }
            self.done = true;
        }
    }
    /// Uses the next prime: marks it prime, strikes its untouched multiples from its square with its rank plus one, and returns it; zero once done.
    ///
    /// The strike mark saturates at 255, so it is exact through the 254th prime.
    pub fn step(&mut self) -> usize {
        if self.done {
            return 0;
        }
        let prime = self.at;
        self.rank += 1;
        self.types[prime] = 1;
        self.struck = 0;
        let mark = (self.rank + 1).min(255) as u8;
        let mut multiple = prime * prime;
        while multiple < self.types.len() {
            if self.types[multiple] == 0 {
                self.types[multiple] = mark;
                self.struck += 1;
            }
            multiple += prime;
        }
        self.settle();
        prime
    }
    /// Runs the sieve to the end.
    pub fn finish(&mut self) {
        while !self.done {
            self.step();
        }
    }
    /// Returns whether every number is settled.
    pub fn done(&self) -> bool {
        self.done
    }
    /// Returns the type of every number from zero: zero untouched, one prime, and one past the rank of the prime that struck it.
    pub fn types(&self) -> &[u8] {
        &self.types
    }
    /// Returns the count of numbers marked prime so far.
    pub fn count(&self) -> usize {
        self.types.iter().filter(|&&t| t == 1).count()
    }
    /// Returns the count of numbers the last step struck.
    pub fn struck(&self) -> usize {
        self.struck
    }
    /// Returns the count of primes used so far.
    pub fn rank(&self) -> usize {
        self.rank
    }
}

/// A number as a pile of stones: its prime factors, whether it is prime, and every rectangle the stones make.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pile {
    /// The count of stones.
    pub number: u64,
    /// The prime and exponent pairs, ascending.
    pub factors: Vec<(u64, u32)>,
    /// Whether the stones make a single row and nothing else.
    pub prime: bool,
    /// Every rectangle as a pair of sides, the shorter first, ascending.
    pub rectangles: Vec<(u64, u64)>,
}

/// Reads a wide number as a pile of stones, its rectangles built from the divisors of its factorization.
///
/// ```
/// let pile = mrlynum::prime::pile(6);
/// assert_eq!(pile.rectangles, vec![(1, 6), (2, 3)]);
/// assert!(!pile.prime);
/// ```
pub fn pile(number: u64) -> Pile {
    let factors = factorize_wide(number);
    let mut sides = vec![1u64];
    for &(prime, power) in &factors {
        let mut next = Vec::with_capacity(sides.len() * (power as usize + 1));
        for &side in &sides {
            let mut value = side;
            next.push(value);
            for _ in 0..power {
                value *= prime;
                next.push(value);
            }
        }
        sides = next;
    }
    sides.sort_unstable();
    let rectangles = sides
        .iter()
        .filter(|&&a| number > 0 && a <= number / a)
        .map(|&a| (a, number / a))
        .collect();
    Pile {
        number,
        prime: factors.len() == 1 && factors[0].1 == 1,
        factors,
        rectangles,
    }
}

/// One reading of the prime count against its two smooth guesses.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Reading {
    /// The point on the number line.
    pub x: usize,
    /// The count of primes up to it.
    pub pi: usize,
    /// The guess x over ln x.
    pub ratio: f64,
    /// The logarithmic integral.
    pub li: f64,
}

/// Reads the prime count against x over ln x and li at evenly spaced points from two up to the top, at most the given count of them, the top always last.
///
/// ```
/// let readings = mrlynum::prime::chart(100, 10);
/// assert_eq!((readings.len(), readings[9].pi), (10, 25));
/// ```
pub fn chart(top: usize, bins: usize) -> Vec<Reading> {
    let list = primes(top);
    let step = (top / bins.max(1)).max(1);
    let mut out = Vec::new();
    let mut x = step;
    while x <= top {
        if x >= 2 {
            out.push(Reading {
                x,
                pi: list.partition_point(|&p| p <= x),
                ratio: x as f64 / (x as f64).ln(),
                li: li(x as f64),
            });
        }
        if x == top {
            break;
        }
        x = (x + step).min(top);
    }
    out
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
    fn the_prime_from_a_number_is_the_first_at_or_above_it() {
        assert_eq!(prime_from(90), 97);
        assert_eq!(prime_from(0), 2);
        assert_eq!(prime_from(41), 41);
        for number in 0..=1_000 {
            let next = prime_from(number);
            assert!(is_prime(next) && next >= number, "{number}");
            assert!((number..next).all(|n| !is_prime(n)), "{number}");
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

    #[test]
    fn the_sieve_strikes_one_prime_at_a_time() {
        let mut sieve = Sieve::new(30);
        assert!(!sieve.done());
        assert_eq!((sieve.step(), sieve.struck(), sieve.count()), (2, 14, 1));
        assert_eq!((sieve.step(), sieve.struck(), sieve.count()), (3, 4, 2));
        assert_eq!((sieve.step(), sieve.struck()), (5, 1));
        assert!(sieve.done());
        assert_eq!((sieve.rank(), sieve.count(), sieve.step()), (3, 10, 0));
        assert_eq!(
            &sieve.types()[..13],
            &[0, 0, 1, 1, 2, 1, 2, 1, 2, 3, 2, 1, 2]
        );
        assert_eq!(sieve.types()[25], 4);
        let mut hundred = Sieve::new(100);
        hundred.finish();
        assert_eq!((hundred.rank(), hundred.count()), (4, 25));
        let listed: Vec<usize> = (0..=100).filter(|&n| hundred.types()[n] == 1).collect();
        assert_eq!(listed, primes(100));
        assert_eq!(Sieve::new(3).count(), 2);
        assert_eq!(Sieve::new(0).count(), 0);
    }

    #[test]
    fn the_pile_agrees_with_the_rectangles_and_the_wheel() {
        let stones = pile(360);
        assert_eq!(stones.factors, vec![(2, 3), (3, 2), (5, 1)]);
        assert_eq!(stones.rectangles.len(), 12);
        assert_eq!(stones.rectangles[11], (18, 20));
        assert!(pile(13).prime && !pile(1).prime && !pile(0).prime);
        assert!(pile(0).rectangles.is_empty());
        assert_eq!(pile(1).rectangles, vec![(1, 1)]);
        for number in 1..=2_000u64 {
            let want: Vec<(u64, u64)> = rectangles(number as usize)
                .iter()
                .map(|&(a, b)| (a as u64, b as u64))
                .collect();
            assert_eq!(pile(number).rectangles, want, "{number}");
            assert_eq!(pile(number).prime, is_prime(number as usize), "{number}");
        }
        assert_eq!(pile(1_000_000_000_000).rectangles.len(), 85);
        assert!(pile(999_999_999_989).prime);
    }

    #[test]
    fn the_chart_reads_the_prime_count_at_the_top() {
        let readings = chart(10_000, 400);
        assert_eq!(readings.len(), 400);
        let last = readings[399];
        assert_eq!((last.x, last.pi), (10_000, 1229));
        assert!((last.ratio - 1085.7360).abs() < 1e-3);
        assert!((last.li - 1246.1372).abs() < 1e-3);
        assert_eq!(chart(100_000, 100)[99].pi, 9592);
        assert_eq!(chart(100, 400).len(), 99);
        assert_eq!(chart(7, 3)[0].x, 2);
        assert!(chart(1, 5).is_empty());
    }
}
