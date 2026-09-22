//! The designs in space.
//!
//! A small integer code picks the filled corners of a hypercube, that seed grows level by level
//! into a fractal design, and the same code always unfolds into the same shape, so a design can be
//! named, counted and drawn again from its number alone. Half the module generates and half
//! measures; everything rests on the tensors and cells of [`crate::core`] and the sequences of
//! [`crate::num`].
//!
//! - `atoms` fills a tensor with a carpet, a net, beams or noise.
//! - `bang` enumerates the design codes, their symmetries and their counts.
//! - `cell` holds the N-dimensional cell and the pipeline the fixed dimensions share.
//! - `two`, `three` and `six` run that pipeline for flat cells, cubes and hexagons.
//! - `counts` gives the same fills, surfaces, hex slices and carry ladder in closed form.
//! - `graph` lifts a grid into nodes and branches; `spectrum` reads the Laplacian spectra off it.
//! - `shape` crops a cell against a rational shape, cell by cell, with no floats.
//! - `rules` marks the cells of a hypercube whose coordinate residues satisfy a rule.
//! - `moire` layers one design at many scales into an interference field.
//! - `press` weighs the integers a design's digit rule keeps.
//! - `spin` spins a raster about its centre; `tourbillon` stacks the turned parity carpets.
//! - `spirograph` rolls a byte grid as a wheel; `roulette` counts where its curves cross.
//! - `name` prints and parses the one canonical JSON object of every design, rule, tile and word.
//!
//! The doors are [`crate::math::atoms::carpet_2d`], [`crate::math::bang::bang`],
//! [`crate::math::two::carpet`], [`crate::math::two::census()`], [`crate::math::two::to_json`],
//! [`crate::math::counts::fill`], [`crate::math::three::census::surface`] and
//! [`crate::math::spectrum::laplacian_spectrum`].

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
