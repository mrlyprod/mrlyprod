/// The classic sequences: evens, odds, binary, Fibonacci and primes.
pub mod classics;
/// The closed-form fill and grid counts of coded fractals.
pub mod counting;
/// The closed-form triangle, node and edge counts of hex slices.
pub mod six;
/// The closed-form exposed-surface counts of 3d fractals.
pub mod surface;

pub use classics::{binary, evens, fibonacci, odds, primes};
pub use counting::{
    dimension, factorial, fill, fill_from_corners, gcd, grid, limit, positions, ratio, rational,
    reduce, void,
};
pub use six::{cut_fills, cut_voids, pro_fills, pro_voids};
pub use surface::{surface, surface_of_tile};
