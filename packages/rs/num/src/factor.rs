use mrly_core::error::{Result, overflow_error};

pub fn gcd(a: u128, b: u128) -> u128 {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

pub fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        return 0;
    }
    a / gcd(a as u128, b as u128) as usize * b
}

pub fn coprime(a: usize, b: usize) -> bool {
    gcd(a as u128, b as u128) == 1
}

pub fn reduce(numerator: u128, denominator: u128) -> (u128, u128) {
    match gcd(numerator, denominator) {
        0 => (0, 1),
        divisor => (numerator / divisor, denominator / divisor),
    }
}

pub fn factorial(number: usize) -> Result<u128> {
    let mut out: u128 = 1;
    for step in 1..=number as u128 {
        match out.checked_mul(step) {
            Some(next) => out = next,
            None => return overflow_error(format!("a factorial of {number} passes u128")),
        }
    }
    Ok(out)
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

pub fn factorize(number: usize) -> Vec<(usize, u32)> {
    factorize_wide(number as u64)
        .into_iter()
        .map(|(prime, power)| (prime as usize, power))
        .collect()
}

pub fn divisors(number: u64) -> Vec<u64> {
    if number == 0 {
        return Vec::new();
    }
    let mut out = vec![1];
    for (prime, power) in factorize_wide(number) {
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

pub fn sigma(number: usize, power: u32) -> u128 {
    divisors(number as u64)
        .iter()
        .map(|&divisor| u128::from(divisor).pow(power))
        .sum()
}

pub fn aliquot(number: usize) -> usize {
    (sigma(number, 1) - number as u128) as usize
}

pub fn twisted(number: usize, rhythm: &[i8]) -> i64 {
    if rhythm.is_empty() {
        return 0;
    }
    divisors(number as u64)
        .iter()
        .map(|&divisor| i64::from(rhythm[divisor as usize % rhythm.len()]))
        .sum()
}

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

pub fn totients(n: usize) -> Vec<u64> {
    let mut phi: Vec<u64> = (0..=n as u64).collect();
    for p in 2..=n {
        if phi[p] == p as u64 {
            for m in (p..=n).step_by(p) {
                phi[m] -= phi[m] / p as u64;
            }
        }
    }
    phi
}

pub fn radical(number: usize) -> usize {
    if number == 0 {
        return 0;
    }
    factorize(number).iter().map(|&(prime, _)| prime).product()
}

pub fn squarefree(number: usize) -> bool {
    number != 0 && factorize(number).iter().all(|&(_, power)| power < 2)
}
