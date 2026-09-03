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
/// The elementary formulas: the partial products and sums that walk to pi, e and gamma, and the prime counts beside them.
pub mod formulas;
/// The primes of the plane: the Gaussian and the Eisenstein integers, their classes, windows and ring weights.
pub mod gauss;
/// The spatial network: its nodes, branches, extraction and census.
pub mod graph;
/// The visible lattice: totients, coprime pairs, the constant a dimension recovers and the Farey nodes.
pub mod lattice;
/// The Thue-Morse world: the digit rule, the substitution, the plane lifts, the runs and the period-doubling word.
pub mod morse;
/// The prime objects: values, ranks, gaps and the shape readings of a number.
pub mod prime;
/// The infinite sums: zeta and its Dirichlet cousins, the visible count and the Bernoulli fractions.
pub mod series;
/// The symmetric eigensolver and the Laplacian spectra it reads off a network.
pub mod spectrum;
/// The turntable: the exact circle means of a raster about its centre, the profile they trace and the wheel it paints.
pub mod spin;
/// The spirals: the whole numbers wound on the square and the hexagonal lattice, marked and read along a quadratic.
pub mod spiral;
/// The critical line: zeta at one half plus i t, its zeros, and the prime staircase they rebuild.
pub mod zeta;
