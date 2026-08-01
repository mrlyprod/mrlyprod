use super::config::Config;
use super::hasher::digest;
use mrlycore::errors::Result;

pub fn avalanche(trials: usize, config: &Config) -> Result<f64> {
    let base = b"mrlyhash avalanche probe message....";
    let mut total = 0.0;
    for t in 0..trials {
        let mut msg = base.to_vec();
        let idx = t % msg.len();
        msg[idx] ^= 1 << (t % 8);
        let a = digest(base, config)?.bits;
        let b = digest(&msg, config)?.bits;
        let diff = a.iter().zip(b.iter()).filter(|(x, y)| x != y).count();
        total += diff as f64 / a.len() as f64;
    }
    Ok(total / trials as f64)
}

pub fn bit_balance_sigma(trials: usize, config: &Config) -> Result<f64> {
    let mut acc = vec![0.0f64; config.digest_bits];
    for t in 0..trials {
        let bits = digest(format!("balance-{t}").as_bytes(), config)?.bits;
        for (a, &b) in acc.iter_mut().zip(bits.iter()) {
            *a += b as f64;
        }
    }
    let sigma = 0.5 / (trials as f64).sqrt();
    let worst = acc
        .iter()
        .map(|&s| ((s / trials as f64) - 0.5).abs() / sigma)
        .fold(0.0f64, f64::max);
    Ok(worst)
}

pub fn entropy(trials: usize, config: &Config) -> Result<f64> {
    let mut counts = [0u64; 256];
    let mut total = 0u64;
    for t in 0..trials {
        let bytes = digest(format!("entropy-{t}").as_bytes(), config)?.to_bytes();
        for b in bytes {
            counts[b as usize] += 1;
            total += 1;
        }
    }
    if total == 0 {
        return Ok(0.0);
    }
    let mut h = 0.0;
    for &c in &counts {
        if c > 0 {
            let p = c as f64 / total as f64;
            h -= p * p.log2();
        }
    }
    Ok(h)
}

pub fn collisions(trials: usize, config: &Config) -> Result<usize> {
    use std::collections::HashSet;
    let mut seen = HashSet::new();
    let mut hits = 0;
    for t in 0..trials {
        let h = super::hasher::hexdigest(format!("k{t}").as_bytes(), config)?;
        if !seen.insert(h) {
            hits += 1;
        }
    }
    Ok(hits)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "slow: hundreds of hashes; run with --ignored"]
    fn avalanche_is_near_half() {
        let cfg = Config::default();
        let av = avalanche(40, &cfg).unwrap();
        assert!(av > 0.40 && av < 0.60, "avalanche {av}");
    }
    #[test]
    #[ignore = "slow: hundreds of hashes; run with --ignored"]
    fn no_collisions_on_small_set() {
        let cfg = Config::default();
        assert_eq!(collisions(500, &cfg).unwrap(), 0);
    }
    #[test]
    #[ignore = "slow: hundreds of hashes; run with --ignored"]
    fn entropy_is_high() {
        let cfg = Config::default();
        let e = entropy(200, &cfg).unwrap();
        assert!(e > 7.0, "entropy {e} bits/byte (ideal ~8)");
    }
}
