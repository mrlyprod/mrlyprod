pub mod field;
pub mod layer;
pub mod metrics;
pub mod render;
pub mod sample;
pub mod stack;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lattice {
    Square,
    Hex,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Combine {
    Sum,
    And,
    Xor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Spec {
    pub code: u128,
    pub base: usize,
    pub dimension: usize,
}

impl Spec {
    pub fn new(code: u128, base: usize, dimension: usize) -> Spec {
        Spec {
            code,
            base,
            dimension,
        }
    }
}

pub use field::Field;
pub use layer::{layer, Layer};
pub use metrics::{corr_by_gcd, metrics, Metrics};
pub use render::render;
pub use stack::{stack, stack_codes};
