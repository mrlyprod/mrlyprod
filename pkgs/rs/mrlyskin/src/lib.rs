#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// The pixel-buffer primitives: rectangles, blits and fitted text.
pub mod draw;
/// The dress of the roles: inks, motifs, faces, tints and skins.
pub mod dress;
/// The baked tiles: atlases and the rasters laid over role grids.
pub mod paint;

pub use dress::{dressed, pixels, raster, Face, Ink, Motif, Skin, Tint, Visual, SYMBOL};
