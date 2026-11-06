use crate::config::Config;
use crate::permute::permute;
use mrly_error::Result;
use mrly_two::{carpet, net};

const DIGEST_BITS: usize = 256;

fn seed(config: &Config) -> Result<Vec<u8>> {
    let side = config.side;
    let (carpet, net) = (carpet(3, 3)?, net(3, 3)?);
    let (carpet, net) = (carpet.types().bytes()?, net.types().bytes()?);
    let source = carpet.len().isqrt();
    let mut bits = vec![0u8; side * side];
    for (flat, bit) in bits.iter_mut().enumerate() {
        let at = (flat / side % source) * source + flat % side % source;
        *bit = carpet[at] ^ net[at];
    }
    permute(&bits, config)
}

fn pad(bytes: &[u8], rate: usize) -> Vec<u8> {
    let mut bits: Vec<u8> = bytes
        .iter()
        .flat_map(|byte| (0..8).rev().map(move |k| (byte >> k) & 1))
        .collect();
    bits.push(1);
    while bits.len() % rate != rate - 1 {
        bits.push(0);
    }
    bits.push(1);
    bits
}

fn pack(bits: &[u8]) -> Vec<u8> {
    bits.chunks(8)
        .map(|chunk| chunk.iter().fold(0u8, |acc, &b| acc << 1 | b))
        .collect()
}

pub fn hash(bytes: &[u8], config: &Config) -> Result<Vec<u8>> {
    config.check()?;
    let rate = config.rate();
    let mut state = seed(config)?;
    for block in pad(bytes, rate).chunks(rate) {
        for (cell, bit) in state.iter_mut().zip(block) {
            *cell ^= bit;
        }
        state = permute(&state, config)?;
    }
    let mut out = Vec::with_capacity(DIGEST_BITS + rate);
    loop {
        out.extend_from_slice(&state[..rate]);
        if out.len() >= DIGEST_BITS {
            break;
        }
        state = permute(&state, config)?;
    }
    out.truncate(DIGEST_BITS);
    Ok(pack(&out))
}

pub fn digest(bytes: &[u8]) -> Vec<u8> {
    hash(bytes, &Config::default()).expect("the default config is valid")
}
