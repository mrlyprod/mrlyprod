use super::config::Config;
use super::permute::permute;
use crate::two;
use mrlycore::errors::Result;
use mrlycore::tensor::Tensor;

pub fn initial_state(config: &Config) -> Result<Tensor> {
    let side = config.side;
    if !config.seed_tile {
        return Ok(Tensor::new(vec![side, side]));
    }
    let carpet = two::carpet(3, 3)?;
    let net = two::net(3, 3)?;
    let cs = &carpet.types().bytes();
    let ns = &net.types().bytes();
    let src_side = carpet.width();
    let mut grid = Tensor::new(vec![side, side]);
    for x in 0..side {
        for y in 0..side {
            let sx = x % src_side;
            let sy = y % src_side;
            let a = cs[sx * src_side + sy];
            let b = ns[sx * src_side + sy];
            grid.set(&[x, y], a ^ b);
        }
    }
    permute(&grid, config)
}

fn pad_message(data: &[u8], rate_bits: usize) -> Vec<u8> {
    let mut bits: Vec<u8> = Vec::with_capacity(data.len() * 8 + rate_bits);
    for &byte in data {
        for k in (0..8).rev() {
            bits.push((byte >> k) & 1);
        }
    }
    bits.push(1);
    while bits.len() % rate_bits != rate_bits - 1 {
        bits.push(0);
    }
    bits.push(1);
    bits
}

fn absorb(mut state: Tensor, message: &[u8], config: &Config) -> Result<Tensor> {
    let rate = config.rate_bits();
    let bits = pad_message(message, rate);
    for block in bits.chunks(rate) {
        for (i, &b) in block.iter().enumerate() {
            state.bytes_mut()[i] ^= b;
        }
        state = permute(&state, config)?;
    }
    Ok(state)
}

fn squeeze(mut state: Tensor, config: &Config) -> Result<Vec<u8>> {
    let rate = config.rate_bits();
    let mut out: Vec<u8> = Vec::with_capacity(config.digest_bits);
    while out.len() < config.digest_bits {
        out.extend_from_slice(&state.bytes()[..rate.min(state.bytes().len())]);
        if out.len() < config.digest_bits {
            state = permute(&state, config)?;
        }
    }
    out.truncate(config.digest_bits);
    Ok(out)
}

pub fn sponge_hash(message: &[u8], config: &Config) -> Result<Vec<u8>> {
    let state = initial_state(config)?;
    let state = absorb(state, message, config)?;
    squeeze(state, config)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn padding_lands_on_rate_boundary() {
        let bits = pad_message(b"hi", 16);
        assert_eq!(bits.len() % 16, 0);
        assert_eq!(*bits.last().unwrap(), 1);
    }
    #[test]
    fn hash_has_requested_length() {
        let cfg = Config::default();
        let d = sponge_hash(b"hello", &cfg).unwrap();
        assert_eq!(d.len(), cfg.digest_bits);
        assert!(d.iter().all(|&b| b <= 1));
    }
    #[test]
    fn hash_is_deterministic() {
        let cfg = Config::default();
        assert_eq!(
            sponge_hash(b"determinism", &cfg).unwrap(),
            sponge_hash(b"determinism", &cfg).unwrap()
        );
    }
}
