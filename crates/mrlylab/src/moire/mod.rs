/// The square grid of f32 samples.
pub mod field;
/// The recipe and sampling of one moire layer.
pub mod layer;
/// The named recipes: the parity heatmap, its weave, its hive and the carpet stack.
pub mod presets;
/// The quantized PNG rendering of a field.
pub mod render;
/// The lattice coordinates and code-membership tests behind the layers.
pub mod sample;
/// The stacking of layers into one combined field.
pub mod stack;
/// The cube designs stacked into a volume, and the planes that cut it.
pub mod volume;

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
pub use presets::{all, named, Preset};
pub use render::render;
pub use stack::{merge, stack, stack_codes};
pub use volume::{frame, volume, Frame, Volume};
