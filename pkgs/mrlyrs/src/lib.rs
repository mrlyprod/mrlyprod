#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// The substrate: tensors, cells, colors, images, codecs, resampling and seeded chance.
pub mod core;
/// The alphabet: the stroked pixel glyphs, their rasters and their writing animations.
pub mod font;
/// The engine: a rule over a grid, stepped, recorded, measured and rendered.
pub mod life;
/// The designs in space: codes, cells, cubes, hexagons, their counts, graphs and names.
pub mod math;
/// The integers: primes, divisors, series, lattices, spectra and networks.
pub mod num;
