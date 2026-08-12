/// The square grid of f32 samples.
pub mod field;
/// The recipe and sampling of one moire layer.
pub mod layer;
/// The quantized PNG rendering of a field.
pub mod render;
/// The lattice coordinates and code-membership tests behind the layers.
pub mod sample;
/// The stacking of layers into one combined field.
pub mod stack;

/// The sampling lattice of a moire field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lattice {
    /// The square lattice.
    Square,
    /// The hexagonal lattice.
    Hex,
}

/// The way stacked layers merge.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Combine {
    /// The layer-count sum.
    Sum,
    /// The intersection.
    And,
    /// The parity.
    Xor,
}

/// The identity of a design: its code, base and dimension.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Spec {
    /// The design code.
    pub code: u128,
    /// The residue base.
    pub base: usize,
    /// The design dimension.
    pub dimension: usize,
}

impl Spec {
    /// Builds a spec from a code, base and dimension.
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
pub use render::render;
pub use stack::{stack, stack_codes};
