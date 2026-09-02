const PRIMES: [u64; 3] = [2_147_483_647, 2_147_483_629, 2_147_483_587];

/// Adds two sequences term by term over their shared length.
pub fn add(a: &[i128], b: &[i128]) -> Vec<i128> {
    a.iter().zip(b).map(|(&x, &y)| x + y).collect()
}

/// Subtracts the second sequence from the first over their shared length.
pub fn sub(a: &[i128], b: &[i128]) -> Vec<i128> {
    a.iter().zip(b).map(|(&x, &y)| x - y).collect()
}

/// Multiplies two sequences term by term over their shared length.
pub fn hadamard(a: &[i128], b: &[i128]) -> Vec<i128> {
    a.iter().zip(b).map(|(&x, &y)| x * y).collect()
}

/// Convolves two sequences, keeping the exact prefix their shared length affords.
///
/// Panics when a convolution sum passes a signed hundred and twenty-eight bits.
pub fn cauchy(a: &[i128], b: &[i128]) -> Vec<i128> {
    let length = a.len().min(b.len());
    (0..length)
        .map(|n| {
            (0..=n).fold(0i128, |sum, i| {
                a[i].checked_mul(b[n - i])
                    .and_then(|v| sum.checked_add(v))
                    .expect("the convolution passes a hundred and twenty-eight bits")
            })
        })
        .collect()
}

/// Drops the first terms of a sequence.
pub fn shift(a: &[i128], count: usize) -> Vec<i128> {
    a.iter().skip(count).copied().collect()
}

/// Keeps every step-th term from the offset onward.
///
/// Panics at a step of zero.
pub fn decimate(a: &[i128], step: usize, offset: usize) -> Vec<i128> {
    assert!(step > 0, "decimate needs a step above zero");
    a.iter().skip(offset).step_by(step).copied().collect()
}

/// Returns the first differences of a sequence, one term shorter.
pub fn delta(a: &[i128]) -> Vec<i128> {
    a.windows(2).map(|w| w[1] - w[0]).collect()
}

/// Returns the partial sums of a sequence.
///
/// Panics when a partial sum passes a signed hundred and twenty-eight bits.
pub fn sigma(a: &[i128]) -> Vec<i128> {
    let mut total = 0i128;
    a.iter()
        .map(|&x| {
            total = total
                .checked_add(x)
                .expect("the partial sums pass a hundred and twenty-eight bits");
            total
        })
        .collect()
}

/// Multiplies every term of a sequence by the factor.
pub fn scale(a: &[i128], factor: i128) -> Vec<i128> {
    a.iter().map(|&x| x * factor).collect()
}

fn residues(terms: &[i128], prime: u64) -> Vec<u64> {
    let p = prime as i128;
    terms.iter().map(|&t| (t.rem_euclid(p)) as u64).collect()
}

fn inverse(value: u64, prime: u64) -> u64 {
    let mut power = prime - 2;
    let mut out = 1u64;
    let mut factor = value % prime;
    while power > 0 {
        if power & 1 == 1 {
            out = out * factor % prime;
        }
        factor = factor * factor % prime;
        power >>= 1;
    }
    out
}

fn massey(seq: &[u64], prime: u64) -> usize {
    let mut connection = vec![1u64];
    let mut previous = vec![1u64];
    let mut order = 0usize;
    let mut gap = 1usize;
    let mut last_delta = 1u64;
    for n in 0..seq.len() {
        let mut delta = 0u64;
        for (i, &c) in connection.iter().enumerate() {
            if i <= n {
                delta = (delta + c * seq[n - i]) % prime;
            }
        }
        if delta == 0 {
            gap += 1;
            continue;
        }
        if 2 * order > n {
            let scale = delta * inverse(last_delta, prime) % prime;
            for (i, &b) in previous.iter().enumerate() {
                let slot = i + gap;
                if slot >= connection.len() {
                    connection.resize(slot + 1, 0);
                }
                connection[slot] = (connection[slot] + prime * prime - scale * b) % prime;
            }
            gap += 1;
            continue;
        }
        let held = connection.clone();
        let scale = delta * inverse(last_delta, prime) % prime;
        for (i, &b) in previous.iter().enumerate() {
            let slot = i + gap;
            if slot >= connection.len() {
                connection.resize(slot + 1, 0);
            }
            connection[slot] = (connection[slot] + prime * prime - scale * b) % prime;
        }
        previous = held;
        last_delta = delta;
        gap = 1;
        order = n + 1 - order;
    }
    order
}

fn fit(seq: &[u64], order: usize, prime: u64) -> Option<Vec<u64>> {
    let rows = seq.len() - order;
    let mut matrix: Vec<Vec<u64>> = (0..rows)
        .map(|r| {
            let mut row: Vec<u64> = (0..order).map(|c| seq[r + order - 1 - c]).collect();
            row.push(seq[r + order]);
            row
        })
        .collect();
    let mut pivots = Vec::new();
    let mut lead = 0usize;
    for col in 0..order {
        let Some(pivot) = (lead..rows).find(|&r| matrix[r][col] != 0) else {
            continue;
        };
        matrix.swap(lead, pivot);
        let inv = inverse(matrix[lead][col], prime);
        for value in &mut matrix[lead][col..=order] {
            *value = *value * inv % prime;
        }
        let cleared = matrix[lead].clone();
        for (row, entries) in matrix.iter_mut().enumerate() {
            if row != lead && entries[col] != 0 {
                let factor = entries[col];
                for (value, &top) in entries[col..=order].iter_mut().zip(&cleared[col..=order]) {
                    *value = (*value + prime - factor * top % prime) % prime;
                }
            }
        }
        pivots.push((lead, col));
        lead += 1;
    }
    if matrix[lead..].iter().any(|row| row[order] != 0) {
        return None;
    }
    let mut out = vec![0u64; order];
    for &(row, col) in &pivots {
        out[col] = matrix[row][order];
    }
    Some(out)
}

fn combine(parts: [u64; 3]) -> u128 {
    let (p0, p1, p2) = (PRIMES[0] as u128, PRIMES[1] as u128, PRIMES[2] as u128);
    let m01 = p0 * p1;
    let step = (parts[1] as u128 + p1 - parts[0] as u128 % p1) % p1;
    let lift = inverse((p0 % p1) as u64, PRIMES[1]) as u128;
    let x01 = parts[0] as u128 + p0 * (step * lift % p1);
    let step2 = (parts[2] as u128 + p2 - x01 % p2) % p2;
    let lift2 = inverse((m01 % p2) as u64, PRIMES[2]) as u128;
    x01 + m01 * (step2 * lift2 % p2)
}

fn reconstruct(residue: u128) -> Option<(i128, i128)> {
    let modulus = (PRIMES[0] as u128) * (PRIMES[1] as u128) * (PRIMES[2] as u128);
    let bound: u128 = 1 << 45;
    let (mut r0, mut r1) = (modulus as i128, residue as i128);
    let (mut t0, mut t1) = (0i128, 1i128);
    while r1.unsigned_abs() > bound {
        let q = r0 / r1;
        (r0, r1) = (r1, r0 - q * r1);
        (t0, t1) = (t1, t0 - q * t1);
    }
    if t1 == 0 || t1.unsigned_abs() > bound {
        return None;
    }
    let (num, den) = if t1 < 0 { (-r1, -t1) } else { (r1, t1) };
    let (mut a, mut b) = (num.unsigned_abs(), den.unsigned_abs());
    while b != 0 {
        (a, b) = (b, a % b);
    }
    if a > 1 {
        return None;
    }
    Some((num, den))
}

fn verify(terms: &[i128], coefficients: &[(i128, i128)]) -> bool {
    let order = coefficients.len();
    let mut clear = 1i128;
    for &(_, den) in coefficients {
        clear = clear / gcd(clear, den) * den;
    }
    let weights: Vec<i128> = coefficients
        .iter()
        .map(|&(num, den)| num * (clear / den))
        .collect();
    let mut exact = true;
    for n in order..terms.len() {
        let mut sum = Some(0i128);
        for (i, &w) in weights.iter().enumerate() {
            sum = sum.and_then(|s| {
                w.checked_mul(terms[n - 1 - i])
                    .and_then(|v| s.checked_add(v))
            });
        }
        match (sum, terms[n].checked_mul(clear)) {
            (Some(left), Some(right)) => {
                if left != right {
                    return false;
                }
            }
            _ => {
                exact = false;
                break;
            }
        }
    }
    if exact {
        return true;
    }
    PRIMES.iter().all(|&prime| {
        let seq = residues(terms, prime);
        let p = prime as u128;
        let c = (clear.rem_euclid(prime as i128)) as u128;
        let ws: Vec<u128> = weights
            .iter()
            .map(|&w| (w.rem_euclid(prime as i128)) as u128)
            .collect();
        (order..terms.len()).all(|n| {
            let sum = ws
                .iter()
                .enumerate()
                .fold(0u128, |s, (i, &w)| (s + w * seq[n - 1 - i] as u128) % p);
            sum == c * seq[n] as u128 % p
        })
    })
}

fn gcd(a: i128, b: i128) -> i128 {
    let (mut a, mut b) = (a.abs(), b.abs());
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a.max(1)
}

/// Finds the smallest linear constant-coefficient recurrence that fits every supplied term.
///
/// The coefficients come back as reduced fractions with the newest term first, so a
/// result of one and one twice is the Fibonacci rule. The hunt runs modulo three
/// primes, rebuilds the rationals by remainder reconstruction, and verifies the rule
/// on every term before answering; too few terms to trust an order returns nothing,
/// and the zero sequence returns the empty rule.
///
/// ```
/// let fib = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
/// assert_eq!(mrlynum::blend::recurrence(&fib), Some(vec![(1, 1), (1, 1)]));
/// ```
pub fn recurrence(terms: &[i128]) -> Option<Vec<(i128, i128)>> {
    if terms.len() < 4 {
        return None;
    }
    if terms.iter().all(|&t| t == 0) {
        return Some(Vec::new());
    }
    let order = PRIMES
        .iter()
        .map(|&p| massey(&residues(terms, p), p))
        .max()?;
    if order == 0 || 2 * order + 2 > terms.len() {
        return None;
    }
    let mut parts = Vec::new();
    for &prime in &PRIMES {
        parts.push(fit(&residues(terms, prime), order, prime)?);
    }
    let coefficients: Option<Vec<(i128, i128)>> = (0..order)
        .map(|i| reconstruct(combine([parts[0][i], parts[1][i], parts[2][i]])))
        .collect();
    let coefficients = coefficients?;
    verify(terms, &coefficients).then_some(coefficients)
}

/// Returns the monic characteristic polynomial of a recurrence, highest power first.
pub fn characteristic(coefficients: &[(i128, i128)]) -> Vec<(i128, i128)> {
    let mut out = vec![(1, 1)];
    out.extend(coefficients.iter().map(|&(num, den)| (-num, den)));
    out
}

/// Returns the largest positive real root of a recurrence's characteristic polynomial, the growth rate, or a not-a-number where no real root lands.
///
/// A simple root lands at machine precision; a repeated root lands to a few decimals only.
///
/// ```
/// let rate = mrlynum::blend::growth(&[(1, 1), (1, 1)]);
/// assert!((rate - 1.618033988749895).abs() < 1e-12);
/// ```
pub fn growth(coefficients: &[(i128, i128)]) -> f64 {
    let poly: Vec<f64> = characteristic(coefficients)
        .iter()
        .map(|&(num, den)| num as f64 / den as f64)
        .collect();
    let value = |x: f64| poly.iter().fold(0.0, |acc, &c| acc * x + c);
    let bound = 1.0 + poly.iter().skip(1).map(|c| c.abs()).fold(0.0f64, f64::max);
    let steps = 8192;
    let mut best = f64::NAN;
    let mut gap = f64::INFINITY;
    for i in (0..steps).rev() {
        let (lo, hi) = (
            bound * i as f64 / steps as f64,
            bound * (i + 1) as f64 / steps as f64,
        );
        if value(lo) <= 0.0 && value(hi) >= 0.0 {
            let (mut lo, mut hi) = (lo, hi);
            for _ in 0..200 {
                let mid = (lo + hi) / 2.0;
                if value(mid) <= 0.0 {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            return (lo + hi) / 2.0;
        }
        let mid = (lo + hi) / 2.0;
        if value(mid).abs() < gap {
            gap = value(mid).abs();
            best = mid;
        }
    }
    let mut x = best;
    for _ in 0..100 {
        let h = 1e-7 * x.abs().max(1.0);
        let slope = (value(x + h) - value(x - h)) / (2.0 * h);
        if slope == 0.0 {
            break;
        }
        x -= value(x) / slope;
    }
    if value(x).abs() < 1e-6 * (1.0 + x.abs().powi(poly.len() as i32 - 1)) {
        x
    } else {
        f64::NAN
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::classics::{catalan, primes};

    fn fibonacci(count: usize) -> Vec<i128> {
        let mut out = vec![0i128, 1];
        while out.len() < count {
            out.push(out[out.len() - 1] + out[out.len() - 2]);
        }
        out.truncate(count);
        out
    }

    #[test]
    fn the_fibonacci_rule_and_its_golden_growth() {
        let fib = fibonacci(30);
        let rule = recurrence(&fib).unwrap();
        assert_eq!(rule, vec![(1, 1), (1, 1)]);
        assert!((growth(&rule) - 1.618_033_988_749_895).abs() < 1e-12);
    }

    #[test]
    fn a_cubic_polynomial_sequence_needs_order_four() {
        let terms: Vec<i128> = (0..20).map(|n| n * n * (4 * n + 3)).collect();
        let rule = recurrence(&terms).unwrap();
        assert_eq!(rule, vec![(4, 1), (-6, 1), (4, 1), (-1, 1)]);
        assert!((growth(&rule) - 1.0).abs() < 1e-3);
    }

    #[test]
    fn the_hexagram_rule_grows_by_its_perron_root() {
        let mut terms = vec![1i128, 6];
        while terms.len() < 14 {
            let n = terms.len();
            terms.push(9 * terms[n - 1] - 12 * terms[n - 2]);
        }
        let rule = recurrence(&terms).unwrap();
        assert_eq!(rule, vec![(9, 1), (-12, 1)]);
        assert!((growth(&rule) - (9.0 + 33f64.sqrt()) / 2.0).abs() < 1e-12);
    }

    #[test]
    fn a_halving_sequence_carries_a_rational_rule() {
        let terms = vec![1024i128, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1];
        assert_eq!(recurrence(&terms), Some(vec![(1, 2)]));
    }

    #[test]
    fn the_primes_and_the_catalan_numbers_refuse_every_rule() {
        let ps: Vec<i128> = primes(200).iter().map(|&p| p as i128).collect();
        assert_eq!(recurrence(&ps), None);
        let cs: Vec<i128> = catalan(40_000_000).iter().map(|&c| c as i128).collect();
        assert_eq!(recurrence(&cs), None);
    }

    #[test]
    fn blends_of_c_finite_sequences_stay_c_finite() {
        let fib = fibonacci(40);
        let squared = hadamard(&fib, &fib);
        assert_eq!(recurrence(&squared).unwrap().len(), 3);
        let summed = sigma(&fib);
        assert_eq!(recurrence(&summed).unwrap().len(), 3);
        let paired = decimate(&fib, 2, 0);
        let rule = recurrence(&paired).unwrap();
        assert_eq!(rule, vec![(3, 1), (-1, 1)]);
        let convolved = cauchy(&fib, &fib);
        assert_eq!(recurrence(&convolved).unwrap().len(), 4);
    }

    #[test]
    fn the_term_ops_keep_their_hand_checked_values() {
        let a = vec![1i128, 2, 3, 4, 5];
        let b = vec![10i128, 20, 30];
        assert_eq!(add(&a, &b), vec![11, 22, 33]);
        assert_eq!(sub(&b, &a), vec![9, 18, 27]);
        assert_eq!(hadamard(&a, &b), vec![10, 40, 90]);
        assert_eq!(cauchy(&a, &b), vec![10, 40, 100]);
        assert_eq!(shift(&a, 2), vec![3, 4, 5]);
        assert_eq!(decimate(&a, 2, 1), vec![2, 4]);
        assert_eq!(delta(&a), vec![1, 1, 1, 1]);
        assert_eq!(sigma(&a), vec![1, 3, 6, 10, 15]);
        assert_eq!(scale(&a, -2), vec![-2, -4, -6, -8, -10]);
    }

    #[test]
    fn the_zero_sequence_returns_the_empty_rule() {
        assert_eq!(recurrence(&[0, 0, 0, 0, 0, 0]), Some(vec![]));
        assert_eq!(recurrence(&[1, 2]), None);
    }

    #[test]
    fn a_short_prefix_is_not_enough_evidence() {
        assert_eq!(recurrence(&[1, 2, 4]), None);
        assert_eq!(recurrence(&[1, 2, 4, 8]), Some(vec![(2, 1)]));
    }

    #[test]
    #[should_panic(expected = "decimate needs a step above zero")]
    fn decimate_refuses_a_zero_step() {
        let _ = decimate(&[1, 2, 3], 0, 0);
    }
}
