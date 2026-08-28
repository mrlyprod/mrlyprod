/// The closed-form fill and grid counts of coded fractals.
pub mod counting;
/// The lattice energies of parity designs: Catalan, zeta and beta.
pub mod energy;
/// The closed-form triangle, node and edge counts of hex slices.
pub mod six;
/// The fractal strings of coded designs: complex dimensions, poles and tube profiles.
pub mod strings;
/// The closed-form exposed-surface counts of 3d fractals.
pub mod surface;

pub use counting::{
    dimension, fill, fill_from_corners, grid, limit, positions, ratio, rational, void,
};
pub use six::{cut_fills, cut_voids, pro_fills, pro_voids};
pub use surface::{surface, surface_of_tile};
