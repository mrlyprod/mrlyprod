#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// The painted share card and its PNG.
pub mod card;
/// The native eye: a route's canvas drawn to pixels without a browser.
pub mod eye;
/// The seeded random player that exercises apps through their verbs.
pub mod goose;
/// The rosters: apps, loadouts, gpu programs and skin wardrobes.
pub mod registry;
/// The WGSL programs and their assembly.
pub mod shaders;

pub use goose::Goose;
