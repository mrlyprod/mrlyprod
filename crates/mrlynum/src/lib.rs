#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// The sequence blender: term ops, exact recurrences and growth rates.
pub mod blend;
/// The boolean-function measures: Walsh spectra, nonlinearity, balance and avalanche.
pub mod boolean;
/// The grid tallies: value counts and exposed faces.
pub mod census;
/// The classic sequences and the exact arithmetic under them.
pub mod classics;
/// The divisor arithmetic: factorizations, divisors, radicals and the Mobius values.
pub mod factor;
/// The fast Fourier transform in one and two dimensions.
pub mod fft;
/// The spatial network: its nodes, branches, extraction and census.
pub mod graph;
/// The visible lattice: totients, coprime pairs, the pi estimate and the Farey nodes.
pub mod lattice;
/// The prime objects: values, ranks, gaps and the shape readings of a number.
pub mod prime;
/// The infinite sums: zeta and its Dirichlet cousins, the visible count and the Bernoulli fractions.
pub mod series;
