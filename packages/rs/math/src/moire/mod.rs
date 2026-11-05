use serde::{Deserialize, Serialize};

pub mod field;
pub mod layer;
pub mod pairs;
pub mod presets;
pub mod render;
pub mod sample;
pub mod stack;
pub mod volume;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Lattice {
    Square,
    Hex,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Combine {
    Sum,
    And,
    Xor,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
pub use layer::{Layer, layer};
pub use presets::{Preset, all, named};
pub use render::render;
pub use stack::{stack, stack_codes};
pub use volume::{Frame, Volume, frame, volume};
