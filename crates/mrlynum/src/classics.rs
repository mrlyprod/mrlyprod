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
/// assert_eq!(mrlynum::classics::catalan(50), vec![1, 2, 5, 14, 42]);
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

/// Returns the primes up to the limit by sieve.
///
/// ```
/// assert_eq!(mrlynum::classics::primes(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
/// ```
pub fn primes(limit: usize) -> Vec<usize> {
    if limit < 2 {
        return Vec::new();
    }
    let mut sieve = vec![true; limit + 1];
    sieve[0] = false;
    sieve[1] = false;
    let mut p = 2;
    while p * p <= limit {
        if sieve[p] {
            let mut m = p * p;
            while m <= limit {
                sieve[m] = false;
                m += p;
            }
        }
        p += 1;
    }
    (2..=limit).filter(|&n| sieve[n]).collect()
}

/// Returns the factorial of the number, the product of one through it, exact up to thirty-four.
pub fn factorial(number: usize) -> u128 {
    (1..=number as u128).product()
}

/// Returns the greatest common divisor of two numbers by the Euclidean algorithm.
///
/// ```
/// assert_eq!(mrlynum::classics::gcd(12, 18), 6);
/// assert_eq!(mrlynum::classics::gcd(7, 0), 7);
/// ```
pub fn gcd(a: u128, b: u128) -> u128 {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// Reduces a fraction to its lowest terms, a zero numerator and denominator reading as zero over one.
///
/// ```
/// assert_eq!(mrlynum::classics::reduce(64, 128), (1, 2));
/// ```
pub fn reduce(numerator: u128, denominator: u128) -> (u128, u128) {
    match gcd(numerator, denominator) {
        0 => (0, 1),
        divisor => (numerator / divisor, denominator / divisor),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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
    fn primes_to_twenty() {
        assert_eq!(primes(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
        assert_eq!(primes(1), Vec::<usize>::new());
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
}
