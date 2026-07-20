use super::feistel::{decrypt_block, encrypt_block};
use super::schedule::round_keys;
use crate::core::errors::{value_error, Result};

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub side: usize,
    pub rounds: usize,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            side: 8,
            rounds: 12,
        }
    }
}

impl Config {
    pub fn half_bits(&self) -> usize {
        self.side * self.side
    }
    pub fn block_bits(&self) -> usize {
        2 * self.half_bits()
    }
    pub fn block_bytes(&self) -> usize {
        self.block_bits() / 8
    }
}

pub struct Cipher {
    config: Config,
    keys: Vec<Vec<u8>>,
}

impl Cipher {
    pub fn new(key: &[u8], config: Config) -> Result<Cipher> {
        if !config.block_bits().is_multiple_of(8) {
            return value_error("block size must be a whole number of bytes.");
        }
        let keys = round_keys(key, config.rounds, config.half_bits())?;
        Ok(Cipher { config, keys })
    }
}

fn bytes_to_bits(bytes: &[u8]) -> Vec<u8> {
    let mut bits = Vec::with_capacity(bytes.len() * 8);
    for &byte in bytes {
        for k in (0..8).rev() {
            bits.push((byte >> k) & 1);
        }
    }
    bits
}

fn bits_to_bytes(bits: &[u8]) -> Vec<u8> {
    bits.chunks(8)
        .map(|c| {
            let mut byte = 0u8;
            for (k, &b) in c.iter().enumerate() {
                byte |= (b & 1) << (7 - k);
            }
            byte
        })
        .collect()
}

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(&x, &y)| x ^ y).collect()
}

fn pad(data: &[u8], block_bytes: usize) -> Vec<u8> {
    let pad_len = block_bytes - (data.len() % block_bytes);
    let mut out = data.to_vec();
    out.extend(std::iter::repeat_n(pad_len as u8, pad_len));
    out
}

fn unpad(data: &[u8]) -> Result<Vec<u8>> {
    let pad_len = *data
        .last()
        .ok_or_else(|| crate::core::MrlyError::Value("cannot unpad empty data.".into()))?
        as usize;
    if pad_len == 0 || pad_len > data.len() {
        return value_error("invalid padding (wrong key or corrupt ciphertext?).");
    }
    Ok(data[..data.len() - pad_len].to_vec())
}

pub fn encrypt(cipher: &Cipher, message: &[u8]) -> Result<Vec<u8>> {
    let bb = cipher.config.block_bytes();
    let padded = pad(message, bb);
    let mut prev = iv(cipher);
    let mut out = Vec::with_capacity(padded.len());
    for chunk in padded.chunks(bb) {
        let mixed = xor(chunk, &prev);
        let bits = bytes_to_bits(&mixed);
        let ct_bits = encrypt_block(&bits, &cipher.keys, cipher.config.side)?;
        let ct = bits_to_bytes(&ct_bits);
        out.extend_from_slice(&ct);
        prev = ct;
    }
    Ok(out)
}

pub fn decrypt(cipher: &Cipher, ciphertext: &[u8]) -> Result<Vec<u8>> {
    let bb = cipher.config.block_bytes();
    if ciphertext.is_empty() || !ciphertext.len().is_multiple_of(bb) {
        return value_error("ciphertext length must be a non-zero multiple of the block size.");
    }
    let mut prev = iv(cipher);
    let mut out = Vec::with_capacity(ciphertext.len());
    for chunk in ciphertext.chunks(bb) {
        let bits = bytes_to_bits(chunk);
        let pt_bits = decrypt_block(&bits, &cipher.keys, cipher.config.side)?;
        let mixed = bits_to_bytes(&pt_bits);
        let pt = xor(&mixed, &prev);
        out.extend_from_slice(&pt);
        prev = chunk.to_vec();
    }
    unpad(&out)
}

fn iv(cipher: &Cipher) -> Vec<u8> {
    let bb = cipher.config.block_bytes();
    let first = bits_to_bytes(&cipher.keys[0]);
    let mut iv = vec![0u8; bb];
    for (i, b) in iv.iter_mut().enumerate() {
        *b = first[i % first.len()];
    }
    iv
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn round_trips_arbitrary_lengths() {
        let cfg = Config::default();
        let cipher = Cipher::new(b"my secret key", cfg).unwrap();
        for msg in [
            &b""[..],
            &b"a"[..],
            &b"hello"[..],
            &b"exactly sixteen!"[..],
            &b"a longer message spanning several cipher blocks for good measure"[..],
        ] {
            let ct = encrypt(&cipher, msg).unwrap();
            let pt = decrypt(&cipher, &ct).unwrap();
            assert_eq!(pt, msg, "round-trip failed for {msg:?}");
        }
    }
    #[test]
    fn wrong_key_does_not_recover_message() {
        let cfg = Config::default();
        let a = Cipher::new(b"correct key", cfg).unwrap();
        let b = Cipher::new(b"wrong key!!", cfg).unwrap();
        let ct = encrypt(&a, b"attack at dawn..").unwrap();
        if let Ok(pt) = decrypt(&b, &ct) {
            assert_ne!(pt, b"attack at dawn..");
        }
    }
    #[test]
    fn identical_blocks_encrypt_differently() {
        let cfg = Config::default();
        let cipher = Cipher::new(b"key", cfg).unwrap();
        let msg = b"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";
        let ct = encrypt(&cipher, msg).unwrap();
        assert_ne!(ct[..16], ct[16..32]);
    }
    #[test]
    fn ciphertext_avalanches_on_message() {
        let cfg = Config::default();
        let cipher = Cipher::new(b"key", cfg).unwrap();
        let m1 = b"avalanche test..";
        let mut m2 = *m1;
        m2[0] ^= 1;
        let c1 = encrypt(&cipher, m1).unwrap();
        let c2 = encrypt(&cipher, &m2).unwrap();
        let diff: usize = c1
            .iter()
            .zip(c2.iter())
            .map(|(&a, &b)| (a ^ b).count_ones() as usize)
            .sum();
        let frac = diff as f64 / (c1.len() * 8) as f64;
        assert!(frac > 0.30, "avalanche too weak: {frac}");
    }
}
