#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// The universe of design codes: corners, symmetries and their counts.
pub mod bang;
/// The boolean-function measures: Walsh spectra, nonlinearity, balance and avalanche.
pub mod boolean;
/// The grid tallies: value counts and exposed faces.
pub mod census;
/// The automaton cryptography: the sponge hash and the Feistel cipher.
pub mod crypto;
/// The wells this crate pours: tiles, designs, hashes and formulas as rows.
pub mod data;
/// The dimension-generic cell and the pipeline the fixed dimensions share.
pub mod dim;
/// The fast Fourier transform in one and two dimensions.
pub mod fft;
/// The closed-form counts: classic sequences, fills and surfaces without rendering.
pub mod formulas;
/// The escape-time fractals: Mandelbrot and Julia, their viewports and their scout.
pub mod fractal;
/// The spatial network: its nodes, branches, extraction and census.
pub mod graph;
/// The visible lattice: totients, coprime pairs, the pi estimate and the Farey nodes.
pub mod lattice;
/// The life runs: stepping, recording, rendering and their stories.
pub mod life;
/// The moire fields layered from sampled designs.
pub mod moire;
/// The mrly names: one canonical string for every mathematical thing.
pub mod name;
/// The sixteen named designs and their rendered tiles.
pub mod pick;
/// The residue rules that mark a hypercube's cells.
pub mod rules;
/// The saga: seeded op sequences that carry a grid forward.
pub mod saga;
/// The hexagon world: cubes flattened to triangle-meshed hexes.
pub mod six;
/// The 3d scene kit: vectors, solids and the packed wire format.
pub mod space;
/// The cube pipeline: designs, tiles, graphs and renderings in three dimensions.
pub mod three;
/// The flat-cell pipeline: designs, tiles, graphs and renderings in two dimensions.
pub mod two;
/// The acoustic wave runs: two-speed corridors, transmission spectra and their band gaps.
pub mod wave;
