use std::collections::HashSet;

fn bits(bytes: &[u8]) -> impl Iterator<Item = u8> + '_ {
    bytes
        .iter()
        .flat_map(|byte| (0..8).rev().map(move |k| (byte >> k) & 1))
}

pub fn avalanche(hasher: &dyn Fn(&[u8]) -> Vec<u8>, trials: usize) -> f64 {
    let mut total = 0.0;
    for t in 0..trials {
        let base = format!("mrly avatar probe {t}").into_bytes();
        let reference = hasher(&base);
        let mut message = base.clone();
        let flip = t * 7 % (message.len() * 8);
        message[flip / 8] ^= 1 << (flip % 8);
        let flipped = hasher(&message);
        let changed = bits(&reference)
            .zip(bits(&flipped))
            .filter(|(a, b)| a != b)
            .count();
        total += changed as f64 / (reference.len() * 8) as f64;
    }
    total / trials as f64
}

pub fn balance(hasher: &dyn Fn(&[u8]) -> Vec<u8>, trials: usize) -> f64 {
    let mut ones: Vec<f64> = Vec::new();
    for t in 0..trials {
        let hashed = hasher(format!("balance-{t}").as_bytes());
        ones.resize(hashed.len() * 8, 0.0);
        for (sum, bit) in ones.iter_mut().zip(bits(&hashed)) {
            *sum += f64::from(bit);
        }
    }
    let n = trials as f64;
    ones.iter()
        .map(|&sum| (sum / n - 0.5).abs() / (0.5 / n.sqrt()))
        .fold(0.0, f64::max)
}

pub fn entropy(hasher: &dyn Fn(&[u8]) -> Vec<u8>, trials: usize) -> f64 {
    let mut counts = [0usize; 256];
    for t in 0..trials {
        for byte in hasher(format!("entropy-{t}").as_bytes()) {
            counts[usize::from(byte)] += 1;
        }
    }
    let total: usize = counts.iter().sum();
    counts
        .iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f64 / total as f64;
            -p * p.log2()
        })
        .sum()
}

pub fn collisions(hasher: &dyn Fn(&[u8]) -> Vec<u8>, trials: usize, width: usize) -> usize {
    let width = width.clamp(1, 64);
    let mut seen = HashSet::new();
    for t in 0..trials {
        let hashed = hasher(format!("key-{t}").as_bytes());
        let mut word = [0u8; 8];
        for (slot, byte) in word.iter_mut().zip(&hashed) {
            *slot = *byte;
        }
        seen.insert(u64::from_be_bytes(word) >> (64 - width));
    }
    trials - seen.len()
}

pub fn expected(trials: usize, width: usize) -> f64 {
    let slots = 2f64.powi(width.clamp(1, 64) as i32);
    let n = trials as f64;
    n - slots * (1.0 - (-n / slots).exp())
}
