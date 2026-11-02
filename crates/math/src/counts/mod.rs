//! The closed-form counts.
//!
//! Every number the censuses reach by walking a built cell, this folder reaches by formula:
//! fills, grids, exposed surfaces, hex slices and the base-q carry ladder.

/// The closed forms a design sequence carries: powers, differences, odd-side polynomials and recurrences.
pub mod closed;
/// The closed-form fill and grid counts of coded fractals.
pub mod counting;
/// The diagonal profile of any tile's power, as a digit polynomial.
pub mod diagonal;
/// The closed-form vertex, edge, face and Euler counts of any design's level power.
pub mod faces;
/// The base-q slice carry automaton: its digit polynomial, its matrix, its ladder and its sign law.
pub mod ladder;
/// The closed-form triangle, node and edge counts of hex slices.
pub mod six;
/// The closed-form exposed-surface counts of 3d fractals.
pub mod surface;

pub use closed::{Closed, fill_polynomial, odd_power};
pub use counting::{
    dimension, fill, fill_from_corners, grid, limit, positions, ratio, rational, void,
};
pub use diagonal::profile_of_tile;
pub use faces::{Placements, Skeleton, skeleton, skeleton_of_tile, skeleton_recurrence};
pub use six::{centered_hexagonal, cut_fills, cut_voids, pro_fills, pro_voids};
pub use surface::{
    Exposure, edges_of_tile, exposure, exposure_of_tile, exposure_recurrence, pairs, surface,
};

#[cfg(test)]
mod tests {
    use super::Exposure;
    use crate::round_trip;

    #[test]
    fn serde_round_trips() {
        round_trip(Exposure {
            occupancy: 8,
            exposed: 20,
            axes: vec![(2, 3), (1, 3)],
        });
    }
}
