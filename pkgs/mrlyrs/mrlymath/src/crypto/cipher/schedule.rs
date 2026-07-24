use crate::crypto::hash::{self, Config as HashConfig};
use mrlycore::errors::Result;

pub fn round_keys(key: &[u8], rounds: usize, half_bits: usize) -> Result<Vec<Vec<u8>>> {
    let cfg = HashConfig {
        digest_bits: half_bits.max(8),
        ..HashConfig::default()
    };
    let mut keys = Vec::with_capacity(rounds);
    for r in 0..rounds {
        let mut material = key.to_vec();
        material.push(0xA5);
        material.extend_from_slice(&(r as u64).to_le_bytes());
        let source = hash::digest(&material, &cfg)?.bits;
        let mut bits = source.clone();
        bits.truncate(half_bits);
        while bits.len() < half_bits {
            let need = half_bits - bits.len();
            let take = need.min(source.len());
            bits.extend_from_slice(&source[..take]);
        }
        keys.push(bits);
    }
    Ok(keys)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn produces_one_key_per_round_of_right_length() {
        let keys = round_keys(b"secret", 8, 64).unwrap();
        assert_eq!(keys.len(), 8);
        assert!(keys.iter().all(|k| k.len() == 64));
        assert!(keys.iter().all(|k| k.iter().all(|&b| b <= 1)));
    }
    #[test]
    fn round_keys_differ_across_rounds() {
        let keys = round_keys(b"secret", 8, 64).unwrap();
        assert_ne!(keys[0], keys[1]);
    }
    #[test]
    fn one_bit_key_change_changes_schedule() {
        let a = round_keys(b"secret", 4, 64).unwrap();
        let b = round_keys(b"secres", 4, 64).unwrap();
        assert_ne!(a, b);
    }
}
