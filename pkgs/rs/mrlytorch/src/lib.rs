#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

mod math;

/// The tape autograd: values recorded forward, gradients filled backward.
pub mod graph;
/// The grid bridge: small u8 grids as one-hot planes and back.
pub mod grid;
/// The layers: linear, embed and the sequential mlp.
pub mod nn;
/// The kernel seam every heavy op routes through, held for a gpu backend.
pub mod ops;
/// The optimisers: sgd with momentum and adam.
pub mod optim;
/// The seeded splitmix stream: uniforms, normals and fills.
pub mod rng;
/// The f32 tensor: shapes, elementwise ops, matmul and conv.
pub mod tensor;

/// The crate's result: a value or a terse lowercase note.
pub type Result<T> = std::result::Result<T, &'static str>;

/// Mixes a train and a step into one reproducible seed.
///
/// ```
/// assert_eq!(mrlytorch::seed(1, 0), mrlytorch::seed(1, 0));
/// assert_ne!(mrlytorch::seed(1, 0), mrlytorch::seed(1, 1));
/// ```
pub fn seed(train: u64, step: u64) -> u64 {
    let mut x = train ^ step.rotate_left(32) ^ 0x9E3779B97F4A7C15;
    x = (x ^ (x >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94D049BB133111EB);
    x ^ (x >> 31)
}

#[cfg(test)]
mod tests {
    #[test]
    fn every_run_replays() {
        assert_eq!(super::seed(7, 125), super::seed(7, 125));
        assert_ne!(super::seed(7, 125), super::seed(8, 125));
        assert_ne!(super::seed(0, 0), 0);
    }
}
