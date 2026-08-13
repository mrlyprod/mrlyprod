#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// The gallery: seeded artworks, bare tiles and carried sheets pressed to png with a manifest.
pub mod emit;
/// The press: pouring every well to dataset folders with cards and a manifest.
pub mod press;
/// The goose trails: every playable registry app harvested as a generic well.
pub mod trails;
/// The gathering of every well the workspace declares.
pub mod wells;

pub use emit::{carry, carry_with, emit, emit_with, sheets, tiles, tiles_with};
pub use emit::{BATCH, SCALE, SIDE};
pub use press::{pour, pour_sized, ROWS, SEED};
pub use wells::{find, wells};
