#![doc = include_str!("README.md")]

/// Ready-made tensors: zeros, ones, noise and carpets in two or three dimensions.
pub mod atoms;
/// The universe of design codes: corners, symmetries and their counts.
pub mod bang;
/// The dimension-generic cell and the pipeline the fixed dimensions share.
pub mod cell;
/// The closed-form counts: fills, surfaces, hex slices and the carry ladder, without rendering.
pub mod counts;
/// The spatial network: its nodes, branches, extraction and census.
pub mod graph;
/// The moire fields layered from sampled designs.
pub mod moire;
/// The mrly names: one canonical JSON object for every mathematical thing.
pub mod name;
/// The sequence press: the integers a design's digit rule keeps, weighed all at once.
pub mod press;
/// The nodes of a roulette: where the curves a wheel's pencils draw cross themselves and one another.
pub mod roulette;
/// The residue rules that mark a hypercube's cells.
pub mod rules;
/// The exact crop machinery: rational shapes classified cell by cell, no floats.
pub mod shape;
/// The hexagon world: cubes flattened to triangle-meshed hexes.
pub mod six;
/// The symmetric eigensolver and the Laplacian spectra it reads off a network.
pub mod spectrum;
/// The turntable: the exact circle means of a raster about its centre, the profile they trace and the wheel it paints.
pub mod spin;
/// The spirograph: a byte grid as a wheel with a pencil in every cell, rolled on a line, a circle or a polygon, and the curves it draws.
pub mod spirograph;
/// The cube pipeline: designs, tiles, graphs and renderings in three dimensions.
pub mod three;
/// The tourbillon: the odd parity carpets turned one angle a layer and stacked inside the inscribed disc.
pub mod tourbillon;
/// The flat-cell pipeline: designs, tiles, graphs and renderings in two dimensions.
pub mod two;
