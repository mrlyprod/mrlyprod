#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// The moire fields layered from sampled designs.
pub mod moire;
/// The sequence press: the integers a design's digit rule keeps, weighed all at once.
pub mod press;
/// The matched-noise race: a design against random controls of the same mass.
pub mod race;
/// The level spacing statistics of spectra.
pub mod spacings;
/// The spectral dimension and band-edge readings of spectra.
pub mod spectral;
/// The walk and spectral dimensions of designs.
pub mod walk;
