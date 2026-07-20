use super::config::Config;
use super::sbox;
use super::Boundary;
use crate::core::atoms::carpet_2d;
use crate::core::errors::Result;
use crate::core::tensor::Tensor;
use crate::math::life::{self, Boundary as LifeBoundary};
use crate::math::two::Cell2d;

fn moore() -> Tensor {
    carpet_2d(3)
}

fn life_boundary(b: Boundary) -> LifeBoundary {
    match b {
        Boundary::Wrap => LifeBoundary::Wrap,
        Boundary::Constant => LifeBoundary::Constant,
    }
}

fn ca_pass(grid: &Tensor, config: &Config) -> Result<Tensor> {
    let (birth, survive) = config.rule.counts();
    let cell = Cell2d::new(grid.clone());
    let next = life::next_grid(
        &cell,
        &birth,
        &survive,
        &moore(),
        life_boundary(config.boundary),
    )?;
    Ok(next.types().clone())
}

fn sbox_pass(bits: &[u8]) -> Vec<u8> {
    let n = bits.len();
    let mut out = vec![0u8; n];
    let mut i = 0;
    while i < n {
        let mut nib = 0u8;
        let take = (n - i).min(4);
        for k in 0..take {
            nib |= bits[i + k] << (3 - k);
        }
        let mapped = sbox::apply(nib);
        for k in 0..take {
            out[i + k] = (mapped >> (3 - k)) & 1;
        }
        i += 4;
    }
    out
}

fn rotate_pass(bits: &[u8], round: usize) -> Vec<u8> {
    let n = bits.len();
    if n == 0 {
        return Vec::new();
    }
    let shift = (7 + 13 * round) % n;
    let mut out = vec![0u8; n];
    for (i, &b) in bits.iter().enumerate() {
        out[(i + shift) % n] = b;
    }
    out
}

fn round_constant(bits: &mut [u8], side: usize, round: usize) {
    for (flat, b) in bits.iter_mut().enumerate() {
        let x = flat / side;
        let y = flat % side;
        let v = ((x * 131 + y * 67 + round * 977 + ((round * round) << 1)) >> 1) & 1;
        *b ^= v as u8;
    }
}

pub fn permute(grid: &Tensor, config: &Config) -> Result<Tensor> {
    let side = config.side;
    let mut g = grid.clone();
    for r in 0..config.rounds {
        g = ca_pass(&g, config)?;
        let mut bits = sbox_pass(g.bytes());
        bits = rotate_pass(&bits, r);
        round_constant(&mut bits, side, r);
        g = Tensor::of(bits, vec![side, side]);
    }
    Ok(g)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn grid(side: usize, fill: impl Fn(usize, usize) -> u8) -> Tensor {
        let mut t = Tensor::new(vec![side, side]);
        for x in 0..side {
            for y in 0..side {
                t.set(&[x, y], fill(x, y));
            }
        }
        t
    }
    #[test]
    fn permute_is_deterministic() {
        let cfg = Config::default();
        let g = grid(cfg.side, |x, y| ((x + y) & 1) as u8);
        let a = permute(&g, &cfg).unwrap();
        let b = permute(&g, &cfg).unwrap();
        assert_eq!(a.bytes(), b.bytes());
    }
    #[test]
    fn permute_changes_the_state() {
        let cfg = Config::default();
        let g = grid(cfg.side, |_, _| 0);
        let out = permute(&g, &cfg).unwrap();
        assert_ne!(out.bytes(), g.bytes());
    }
    #[test]
    fn sbox_pass_roundtrips_through_inverse() {
        let bits = vec![1u8, 0, 1, 1, 0, 0, 1, 0];
        let subbed = sbox_pass(&bits);
        let mut restored = vec![0u8; bits.len()];
        let mut i = 0;
        while i < bits.len() {
            let mut nib = 0u8;
            for k in 0..4 {
                nib |= subbed[i + k] << (3 - k);
            }
            let inv = sbox::INV_SBOX[nib as usize];
            for k in 0..4 {
                restored[i + k] = (inv >> (3 - k)) & 1;
            }
            i += 4;
        }
        assert_eq!(restored, bits);
    }
}
