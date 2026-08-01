pub fn evens(limit: usize) -> Vec<usize> {
    (0..=limit).step_by(2).collect()
}

pub fn odds(limit: usize) -> Vec<usize> {
    (1..=limit).step_by(2).collect()
}

pub fn binary(limit: usize) -> Vec<usize> {
    let mut out = Vec::new();
    let mut value = 1;
    while value <= limit {
        out.push(value);
        value *= 2;
    }
    out
}

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
}
