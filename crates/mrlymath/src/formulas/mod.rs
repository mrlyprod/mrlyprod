/// The closed-form fill and grid counts of coded fractals.
pub mod counting;
/// The diagonal profile of any tile's power, as a digit polynomial.
pub mod diagonal;
/// The closed-form triangle, node and edge counts of hex slices.
pub mod six;
/// The closed-form exposed-surface counts of 3d fractals.
pub mod surface;

pub use counting::{
    dimension, fill, fill_from_corners, grid, limit, positions, ratio, rational, void,
};
pub use diagonal::profile_of_tile;
pub use six::{centered_hexagonal, cut_fills, cut_voids, pro_fills, pro_voids};
pub use surface::{exposure, exposure_of_tile, exposure_recurrence, surface, Exposure};
