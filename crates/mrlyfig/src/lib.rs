#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// The canvas and its anti-aliased primitives, and the frame a figure lays out in.
pub mod board;
/// The scalar fields painted through a ramp.
pub mod field;
/// The square lattice, the design painter and the Kronecker masks.
pub mod grid;
/// The triangle meshes of the hexagon world.
pub mod hex;
/// The palette of the house and the ramps built from it.
pub mod ink;
/// The isometric cube faces.
pub mod iso;
/// The one way a figure leaves the press.
pub mod out;
/// The plain marks: bars, dots, rings, staircases, curves and a bare axis.
pub mod plot;

pub use board::{Board, Frame};
pub use grid::Grid;
pub use ink::Ramp;
pub use mrlycore::Color;
pub use out::save;
