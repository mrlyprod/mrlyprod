use crate::config::{Config, Rule};
use crate::sbox;
use mrly_error::{Result, shape_error};
use mrly_life::{Boundary, next_grid};
use mrly_math::cell::base::moore;
use mrly_tensor::Tensor;
use mrly_two::Cell2d;

fn automaton(bits: &[u8], side: usize, rule: Rule, mask: &Tensor) -> Result<Vec<u8>> {
    let cell = Cell2d::new(Tensor::of(bits.to_vec(), vec![side, side])?)?;
    let (birth, survive) = rule.counts();
    let next = next_grid(&cell, &birth, &survive, mask, Boundary::Wrap)?;
    Ok(next.types().bytes()?.to_vec())
}

fn rotate(bits: &[u8], index: usize) -> Vec<u8> {
    let n = bits.len();
    let shift = (7 + 13 * index) % n;
    let mut out = vec![0u8; n];
    for (i, &b) in bits.iter().enumerate() {
        out[(i + shift) % n] = b;
    }
    out
}

fn constant(bits: &mut [u8], side: usize, index: usize) {
    for (flat, bit) in bits.iter_mut().enumerate() {
        let (x, y) = (flat / side, flat % side);
        *bit ^= (((x * 131 + y * 67 + index * 977 + ((index * index) << 1)) >> 1) & 1) as u8;
    }
}

fn stir(bits: &[u8], config: &Config, index: usize, mask: &Tensor) -> Result<Vec<u8>> {
    let next = sbox::apply(&automaton(bits, config.side, config.rule, mask)?);
    let mut next = rotate(&next, index);
    constant(&mut next, config.side, index);
    Ok(next)
}

pub fn round(state: &[u8], config: &Config, index: usize) -> Result<Vec<u8>> {
    config.check()?;
    if state.len() != config.bits() {
        return shape_error("state length must equal side * side.");
    }
    stir(state, config, index, &moore(2))
}

pub(crate) fn permute(state: &[u8], config: &Config) -> Result<Vec<u8>> {
    config.check()?;
    if state.len() != config.bits() {
        return shape_error("state length must equal side * side.");
    }
    let mask = moore(2);
    let mut bits = state.to_vec();
    for index in 0..config.rounds {
        bits = stir(&bits, config, index, &mask)?;
    }
    Ok(bits)
}
