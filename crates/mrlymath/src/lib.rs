#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// The universe of design codes: corners, symmetries and their counts.
pub mod bang;
/// The dimension-generic cell and the pipeline the fixed dimensions share.
pub mod dim;
/// The closed-form counts: classic sequences, fills and surfaces without rendering.
pub mod formulas;
/// The life runs: stepping, recording, rendering and their stories.
pub mod life;
/// The mrly names: one canonical string for every mathematical thing.
pub mod name;
/// The residue rules that mark a hypercube's cells.
pub mod rules;
/// The hexagon world: cubes flattened to triangle-meshed hexes.
pub mod six;
/// The 3d scene kit: vectors, solids and the packed wire format.
pub mod space;
/// The cube pipeline: designs, tiles, graphs and renderings in three dimensions.
pub mod three;
/// The flat-cell pipeline: designs, tiles, graphs and renderings in two dimensions.
pub mod two;
