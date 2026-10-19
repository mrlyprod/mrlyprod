#![doc = include_str!("../README.md")]

/// The substrate: tensors, cells, colors, images, resampling, seeded chance and the canonical names.
pub use mrly_core as core;
/// The alphabet: a stroked pixel font of 108 characters, painted whole or written stroke by stroke.
pub use mrly_font as font;
/// The engine: a rule over a grid, stepped, recorded, measured and rendered.
pub use mrly_life as life;
/// The designs: codes, cells, cubes and hexagons, their counts, graphs and names, digit designs on the line, and wheels.
pub use mrly_math as math;
/// The integers: primes, divisors, series, sieves, lattices and spectra.
pub use mrly_num as num;
/// The tiles: the whole pipeline from a recipe to a file.
pub use mrly_tile as tile;

pub use mrly_core::{Error, Result};
