use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;

// THE DESIGN

const CHUNK_BITS: usize = 16;
const THREADS: usize = 4;
const SIEVE_LEVEL: usize = 15;

fn read3(mut k: u64) -> u64 {
    let mut n = 0u64;
    let mut p = 1u64;
    while k > 0 {
        if k & 1 == 1 {
            n += p;
        }
        p *= 3;
        k >>= 1;
    }
    n
}

// THE FACTORING

fn small_primes(limit: u64) -> Vec<u64> {
    let mut sieve = vec![true; limit as usize + 1];
    let mut out = Vec::new();
    for p in 2..=limit as usize {
        if sieve[p] {
            out.push(p as u64);
            let mut q = p * p;
            while q <= limit as usize {
                sieve[q] = false;
                q += p;
            }
        }
    }
    out
}

fn mu_sieve(limit: usize) -> Vec<i8> {
    let mut mu = vec![1i8; limit + 1];
    let mut composite = vec![false; limit + 1];
    mu[0] = 0;
    for p in 2..=limit {
        if composite[p] {
            continue;
        }
        let mut q = p;
        while q <= limit {
            composite[q] = true;
            mu[q] = -mu[q];
            q += p;
        }
        let mut q = p * p;
        while q <= limit {
            mu[q] = 0;
            q += p * p;
        }
    }
    mu
}

fn mulmod(a: u64, b: u64, n: u64) -> u64 {
    ((a as u128 * b as u128) % n as u128) as u64
}

fn powmod(mut a: u64, mut e: u64, n: u64) -> u64 {
    let mut r = 1u64;
    a %= n;
    while e > 0 {
        if e & 1 == 1 {
            r = mulmod(r, a, n);
        }
        a = mulmod(a, a, n);
        e >>= 1;
    }
    r
}

const WITNESSES: [u64; 12] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37];

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    for &p in &WITNESSES {
        if n % p == 0 {
            return n == p;
        }
    }
    let s = (n - 1).trailing_zeros();
    let d = (n - 1) >> s;
    'outer: for &a in &WITNESSES {
        let mut x = powmod(a, d, n);
        if x == 1 || x == n - 1 {
            continue;
        }
        for _ in 1..s {
            x = mulmod(x, x, n);
            if x == n - 1 {
                continue 'outer;
            }
        }
        return false;
    }
    true
}

fn isqrt(n: u64) -> u64 {
    let mut r = (n as f64).sqrt() as u64;
    while r * r > n {
        r -= 1;
    }
    while (r + 1) * (r + 1) <= n {
        r += 1;
    }
    r
}

fn mobius(mut n: u64, primes: &[u64]) -> i8 {
    let mut mu = 1i8;
    for &p in primes {
        if p * p > n {
            break;
        }
        if n % p == 0 {
            n /= p;
            if n % p == 0 {
                return 0;
            }
            mu = -mu;
        }
    }
    if n == 1 {
        return mu;
    }
    let bound = *primes.last().unwrap();
    if n < bound * bound {
        return -mu;
    }
    let r = isqrt(n);
    if r * r == n {
        return 0;
    }
    if is_prime(n) {
        -mu
    } else {
        mu
    }
}

// THE WALK

#[derive(Clone, Copy, Debug, PartialEq)]
struct Rung {
    level: u32,
    m: i64,
    a: i64,
    max: u64,
}

#[derive(Clone, Copy, Default)]
struct Chunk {
    sum: i64,
    odd: i64,
    lo: i64,
    hi: i64,
}

fn chunk(j: u64, primes: &[u64]) -> Chunk {
    let mut c = Chunk::default();
    for k in (j << CHUNK_BITS) + 1..=(j + 1) << CHUNK_BITS {
        let v = mobius(read3(k), primes) as i64;
        c.sum += v;
        if k & 1 == 1 {
            c.odd += v;
        }
        c.lo = c.lo.min(c.sum);
        c.hi = c.hi.max(c.sum);
    }
    c
}

fn walk(level: usize, primes: &[u64]) -> Vec<Rung> {
    let total = 1u64 << level;
    let first = total.min(1 << CHUNK_BITS);
    let mut rungs = Vec::new();
    let (mut sum, mut odd, mut max) = (0i64, 0i64, 0u64);
    for k in 1..=first {
        let v = mobius(read3(k), primes) as i64;
        sum += v;
        if k & 1 == 1 {
            odd += v;
        }
        max = max.max(sum.unsigned_abs());
        if k.is_power_of_two() {
            rungs.push(Rung {
                level: k.trailing_zeros(),
                m: sum,
                a: odd,
                max,
            });
        }
    }
    if total > first {
        let chunks = (total >> CHUNK_BITS) as usize;
        let next = AtomicUsize::new(1);
        let mut results = vec![Chunk::default(); chunks];
        let parts: Vec<Vec<(usize, Chunk)>> = std::thread::scope(|s| {
            let workers: Vec<_> = (0..THREADS)
                .map(|_| {
                    s.spawn(|| {
                        let mut out = Vec::new();
                        loop {
                            let j = next.fetch_add(1, Ordering::Relaxed);
                            if j >= chunks {
                                return out;
                            }
                            out.push((j, chunk(j as u64, primes)));
                        }
                    })
                })
                .collect();
            workers.into_iter().map(|w| w.join().unwrap()).collect()
        });
        for (j, c) in parts.into_iter().flatten() {
            results[j] = c;
        }
        for (j, c) in results.iter().enumerate().skip(1) {
            max = max
                .max((sum + c.lo).unsigned_abs())
                .max((sum + c.hi).unsigned_abs());
            sum += c.sum;
            odd += c.odd;
            let end = ((j + 1) as u64) << CHUNK_BITS;
            if end.is_power_of_two() {
                rungs.push(Rung {
                    level: end.trailing_zeros(),
                    m: sum,
                    a: odd,
                    max,
                });
            }
        }
    }
    rungs
}

// THE READINGS

fn theta(max: u64, level: u32) -> f64 {
    (max as f64).ln() / (level as f64 * 3f64.ln())
}

fn fit(rungs: &[Rung], i: usize) -> Option<(f64, f64, f64)> {
    if i < 5 {
        return None;
    }
    let window = &rungs[i - 4..=i];
    let xs: Vec<f64> = window.iter().map(|r| r.level as f64 * 3f64.ln()).collect();
    let ys: Vec<f64> = window.iter().map(|r| (r.max as f64).ln()).collect();
    let n = xs.len() as f64;
    let (mx, my) = (xs.iter().sum::<f64>() / n, ys.iter().sum::<f64>() / n);
    let sxy: f64 = xs.iter().zip(&ys).map(|(x, y)| (x - mx) * (y - my)).sum();
    let sxx: f64 = xs.iter().map(|x| (x - mx) * (x - mx)).sum();
    let thetas: Vec<f64> = window.iter().map(|r| theta(r.max, r.level)).collect();
    let lo = thetas.iter().cloned().fold(f64::INFINITY, f64::min);
    let hi = thetas.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    Some((sxy / sxx, lo, hi))
}

fn recursion_holds(rungs: &[Rung]) -> bool {
    let mut running = 0i64;
    rungs.iter().enumerate().all(|(i, r)| {
        running += r.m;
        let prev = if i == 0 { 0 } else { rungs[i - 1].a };
        r.m == r.a - prev && r.a == running
    })
}

fn trial_bound(level: usize) -> u64 {
    let top = 3u128.pow(level as u32);
    let mut b = 3u64.pow((level / 3) as u32);
    while (b as u128).pow(3) <= top {
        b += 1;
    }
    b
}

fn main() {
    let start = Instant::now();
    let level: usize = std::env::args()
        .nth(1)
        .map(|s| s.parse().expect("a level"))
        .unwrap_or(24);
    assert!((1..=30).contains(&level));
    let bound = trial_bound(level);
    let primes = small_primes(bound);
    let top = 3u64.pow(level as u32);
    let alpha = 2f64.ln() / 3f64.ln();
    println!("design base 3 digits {{0,1}}, level {level}, 3^{level} = {top}, 2^{level} = {} elements, alpha = {alpha:.6}, alpha/2 = {:.6}", 1u64 << level, alpha / 2.0);
    println!("factoring trial division by the {} primes to {bound}, {bound}^3 > 3^{level}, cofactor 1, p, p^2 or pq, Miller-Rabin on the witnesses {:?}", primes.len(), WITNESSES);
    let sieve_top = 3usize.pow(SIEVE_LEVEL as u32);
    let mu = mu_sieve(sieve_top);
    let mismatches = (1..1u64 << SIEVE_LEVEL)
        .filter(|&k| mobius(read3(k), &primes) != mu[read3(k) as usize])
        .count();
    println!("check sieve: {} design elements below 3^{SIEVE_LEVEL} factored against a linear Mobius sieve, {mismatches} mismatches", (1u64 << SIEVE_LEVEL) - 1);
    assert_eq!(mismatches, 0);
    println!("sieve done at {:.1}s", start.elapsed().as_secs_f64());
    let rungs = walk(level, &primes);
    println!(
        "walk done at {:.1}s on {THREADS} threads",
        start.elapsed().as_secs_f64()
    );
    println!("columns: n, M_F(3^n), A(3^n), Mmax = max |M_F(y)| over y <= 3^n, theta = log Mmax / log 3^n, theta/alpha = log Mmax / log 2^n, fit = least-squares slope of log Mmax on log x over n-4..n, bracket = [min, max] of theta over n-4..n, |A|/(n+1) = the floor the recursion puts under max_(j <= n) |M_F(3^j)|");
    for (i, r) in rungs.iter().enumerate() {
        let th = theta(r.max, r.level.max(1));
        let (f, lo, hi) = match fit(&rungs, i) {
            Some((f, lo, hi)) => (format!("{f:.4}"), format!("{lo:.4}"), format!("{hi:.4}")),
            None => ("-".into(), "-".into(), "-".into()),
        };
        println!(
            "row n={} M={} A={} Mmax={} theta={:.6} theta/alpha={:.4} fit={} bracket=[{}, {}] |A|/(n+1)={:.3}",
            r.level, r.m, r.a, r.max, th, th / alpha, f, lo, hi, r.a.unsigned_abs() as f64 / (r.level as f64 + 1.0)
        );
    }
    let ok = recursion_holds(&rungs);
    println!("check recursion: M_F(3^n) = A(3^n) - A(3^(n-1)) and A(3^n) = sum_(j <= n) M_F(3^j) at every n = 0..{level}: {}", if ok { "exact" } else { "BROKEN" });
    assert!(ok);
    let last = rungs.last().unwrap();
    println!("box: 3^{level}, M_F = {}, A = {}, Mmax = {}, theta = {:.6} against alpha/2 = {:.6}, {:.1}s", last.m, last.a, last.max, theta(last.max, last.level), alpha / 2.0, start.elapsed().as_secs_f64());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn primes() -> Vec<u64> {
        small_primes(6600)
    }

    #[test]
    fn trial_bound_cubes_past_the_top() {
        assert_eq!(trial_bound(24), 6562);
        assert_eq!(trial_bound(27), 19684);
        assert!((trial_bound(26) as u128).pow(3) > 3u128.pow(26));
        assert!(((trial_bound(26) - 1) as u128).pow(3) <= 3u128.pow(26));
    }

    #[test]
    fn elements_ascend_and_start_right() {
        let first: Vec<u64> = (1..8).map(read3).collect();
        assert_eq!(first, vec![1, 3, 4, 9, 10, 12, 13]);
        assert!((1..1u64 << 12).all(|k| read3(k) < read3(k + 1)));
        assert_eq!(read3(1 << 24), 3u64.pow(24));
    }

    #[test]
    fn mu_matches_sieve() {
        let mu = mu_sieve(200_000);
        let p = primes();
        assert!((1..=200_000u64).all(|n| mobius(n, &p) == mu[n as usize]));
    }

    #[test]
    fn two_large_primes_resolved() {
        let p = primes();
        assert_eq!(mobius(6607 * 6619, &p), 1);
        assert_eq!(mobius(6607 * 6607, &p), 0);
        assert_eq!(mobius(2 * 6607 * 6619, &p), -1);
        assert_eq!(mobius(281_474_976_710_597, &p), -1);
        assert_eq!(mobius(999_983 * 999_979, &p), 1);
    }

    #[test]
    fn strong_pseudoprimes_rejected() {
        for n in [
            3_215_031_751u64,
            2_152_302_898_747,
            3_474_749_660_383,
            341_550_071_728_321,
        ] {
            assert!(!is_prime(n));
        }
        for n in [2u64, 3, 5, 6607, 999_983, 281_474_976_710_597] {
            assert!(is_prime(n));
        }
    }

    #[test]
    fn isqrt_exact() {
        for r in [1u64, 2, 6600, 65_535, 524_287, 524_288] {
            assert_eq!(isqrt(r * r), r);
            assert_eq!(isqrt(r * r + 1), r);
            assert_eq!(isqrt(r * r - 1), r - 1);
        }
    }

    #[test]
    fn walk_matches_mobius_census_to_16() {
        let rungs = walk(16, &primes());
        let census = [
            (4, -2, 3),
            (6, 2, 5),
            (8, 2, 8),
            (10, 5, 13),
            (12, 56, 61),
            (14, 11, 105),
            (16, 149, 173),
        ];
        for (level, m, max) in census {
            let r = rungs.iter().find(|r| r.level == level).unwrap();
            assert_eq!((r.m, r.max), (m, max as u64), "level {level}");
        }
    }

    #[test]
    fn walk_recursion_exact_to_16() {
        assert!(recursion_holds(&walk(16, &primes())));
    }

    #[test]
    fn parallel_walk_matches_sequential() {
        let p = primes();
        let par = walk(18, &p);
        let (mut sum, mut odd, mut max) = (0i64, 0i64, 0u64);
        let mut seq = Vec::new();
        for k in 1..=1u64 << 18 {
            let v = mobius(read3(k), &p) as i64;
            sum += v;
            if k & 1 == 1 {
                odd += v;
            }
            max = max.max(sum.unsigned_abs());
            if k.is_power_of_two() {
                seq.push(Rung {
                    level: k.trailing_zeros(),
                    m: sum,
                    a: odd,
                    max,
                });
            }
        }
        assert_eq!(par, seq);
    }
}
