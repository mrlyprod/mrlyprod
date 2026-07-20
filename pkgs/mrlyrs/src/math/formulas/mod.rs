pub mod classics;
pub mod counting;
pub mod six;
pub mod surface;

pub use classics::{binary, evens, fibonacci, odds, primes};
pub use counting::{dimension, fill, fill_from_corners, grid, positions, ratio, void};
pub use six::{cut_fills, cut_voids, pro_fills, pro_voids};
pub use surface::{surface, surface_of_tile};
