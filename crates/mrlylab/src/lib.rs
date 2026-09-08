#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// The atlas: the census of mrlylife over presets of seeds, masks and rules, every frame read as a design.
pub mod atlas;
/// The sequence registry: every measure of every design as a sequence, the curated records, and the page they render.
pub mod ledger;
/// The moire fields layered from sampled designs.
pub mod moire;
/// The sequence press: the integers a design's digit rule keeps, weighed all at once.
pub mod press;
