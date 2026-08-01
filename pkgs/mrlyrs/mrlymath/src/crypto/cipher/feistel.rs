use crate::crypto::hash::permute;
use crate::crypto::hash::Config as HashConfig;
use mrlycore::errors::Result;
use mrlycore::tensor::Tensor;

pub fn f(half: &[u8], round_key: &[u8], side: usize) -> Result<Vec<u8>> {
    let mut bits: Vec<u8> = half
        .iter()
        .zip(round_key.iter())
        .map(|(&a, &b)| a ^ b)
        .collect();
    bits.resize(side * side, 0);
    let cfg = HashConfig {
        side,
        rounds: 3,
        seed_tile: false,
        ..HashConfig::default()
    };
    let grid = Tensor::of(bits, vec![side, side]);
    let mixed = permute(&grid, &cfg)?;
    Ok(mixed.bytes().to_vec())
}

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(&x, &y)| x ^ y).collect()
}

fn round_forward(left: &[u8], right: &[u8], key: &[u8], side: usize) -> Result<(Vec<u8>, Vec<u8>)> {
    let fr = f(right, key, side)?;
    let new_right = xor(left, &fr);
    Ok((right.to_vec(), new_right))
}

fn round_backward(
    left: &[u8],
    right: &[u8],
    key: &[u8],
    side: usize,
) -> Result<(Vec<u8>, Vec<u8>)> {
    let fl = f(left, key, side)?;
    let new_left = xor(right, &fl);
    Ok((new_left, left.to_vec()))
}

pub fn encrypt_block(block: &[u8], keys: &[Vec<u8>], side: usize) -> Result<Vec<u8>> {
    let half = side * side;
    let (mut l, mut r) = (block[..half].to_vec(), block[half..].to_vec());
    for key in keys {
        let (nl, nr) = round_forward(&l, &r, key, side)?;
        l = nl;
        r = nr;
    }
    let mut out = l;
    out.extend_from_slice(&r);
    Ok(out)
}

pub fn decrypt_block(block: &[u8], keys: &[Vec<u8>], side: usize) -> Result<Vec<u8>> {
    let half = side * side;
    let (mut l, mut r) = (block[..half].to_vec(), block[half..].to_vec());
    for key in keys.iter().rev() {
        let (nl, nr) = round_backward(&l, &r, key, side)?;
        l = nl;
        r = nr;
    }
    let mut out = l;
    out.extend_from_slice(&r);
    Ok(out)
}

#[derive(Clone, Debug)]
pub struct RoundState {
    pub round: usize,
    pub left: Vec<u8>,
    pub right: Vec<u8>,
}

pub fn round_trace(block: &[u8], keys: &[Vec<u8>], side: usize) -> Result<Vec<RoundState>> {
    let half = side * side;
    let (mut l, mut r) = (block[..half].to_vec(), block[half..].to_vec());
    let mut trace = vec![RoundState {
        round: 0,
        left: l.clone(),
        right: r.clone(),
    }];
    for (i, key) in keys.iter().enumerate() {
        let (nl, nr) = round_forward(&l, &r, key, side)?;
        l = nl;
        r = nr;
        trace.push(RoundState {
            round: i + 1,
            left: l.clone(),
            right: r.clone(),
        });
    }
    Ok(trace)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crypto::cipher::schedule::round_keys;
    fn block(side: usize, seed: u8) -> Vec<u8> {
        (0..2 * side * side)
            .map(|i| ((i as u8).wrapping_mul(31).wrapping_add(seed)) & 1)
            .collect()
    }
    #[test]
    fn block_round_trips() {
        let side = 8;
        let keys = round_keys(b"key", 8, side * side).unwrap();
        let b = block(side, 7);
        let ct = encrypt_block(&b, &keys, side).unwrap();
        let pt = decrypt_block(&ct, &keys, side).unwrap();
        assert_eq!(pt, b);
    }
    #[test]
    fn ciphertext_differs_from_plaintext() {
        let side = 8;
        let keys = round_keys(b"key", 8, side * side).unwrap();
        let b = block(side, 1);
        let ct = encrypt_block(&b, &keys, side).unwrap();
        assert_ne!(ct, b);
    }
    #[test]
    fn trace_has_one_state_per_round_plus_initial() {
        let side = 8;
        let keys = round_keys(b"key", 6, side * side).unwrap();
        let b = block(side, 3);
        let tr = round_trace(&b, &keys, side).unwrap();
        assert_eq!(tr.len(), 7);
        let ct = encrypt_block(&b, &keys, side).unwrap();
        let half = side * side;
        assert_eq!(tr.last().unwrap().left, ct[..half]);
        assert_eq!(tr.last().unwrap().right, ct[half..]);
    }
}
