#![doc = include_str!("../README.md")]
#![deny(missing_docs)]
#![forbid(unsafe_code)]

/// The substrate: tensors, cells, colors, images, resampling and seeded chance.
pub use mrly_core as core;
/// The alphabet: the stroked pixel glyphs, their rasters and their writing animations.
pub use mrly_font as font;
/// The engine: a rule over a grid, stepped, recorded, measured and rendered.
pub use mrly_life as life;
/// The designs in space: codes, cells, cubes, hexagons, their counts, graphs and names.
pub use mrly_math as math;
/// The integers: primes, divisors, series, lattices, spectra and networks.
pub use mrly_num as num;
/// The tiles: the whole pipeline from a recipe to a file.
pub use mrly_tile as tile;

pub use mrly_core::{Error, Result};
