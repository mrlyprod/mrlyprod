#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// The press: pouring every well to dataset folders with cards and a manifest.
pub mod press;
/// The goose trails: every playable registry app harvested as a generic well.
pub mod trails;
/// The gathering of every well the workspace declares.
pub mod wells;

pub use press::{pour, pour_sized, ROWS, SEED};
pub use wells::{find, wells};
