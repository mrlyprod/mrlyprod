/// The seeded artwork run from tile recipe to rendered files.
pub mod artwork;
/// The payload a cell's filled sites carry, framed in a sheet.
pub mod carry;
/// The fill, void, perimeter, corner, edge and Euler counts of a flat cell.
pub mod census;
/// The builders of coded, corner, noise and carpet cells.
pub mod designs;
/// The merges, magic folds, masked mosaics and the cube lift of flat cells.
pub mod geometry;
/// The core, edge and tunnel graphs of a flat cell.
pub mod graph;
mod models;
/// The coloring of a flat cell by its types.
pub mod painter;
/// The text, PNG and SVG renderings of a flat cell.
pub mod renderer;
/// The list, string and JSON forms of a flat cell.
pub mod serializer;
/// The random tiles and the flat cells they build.
pub mod tile;

pub use carry::{capacity, embed, extract, read, sheet};
pub use census::{census, euler, fills, Census};
pub use designs::{
    carpet, create, from_corners, htree, level_set, levels_code, named, net, noise, ones, random,
    void, vtree, zeros,
};
pub use geometry::{magic, merge, mosaic, special, to_3d};
pub use models::Cell2d;
pub use painter::paint;
pub use renderer::{from_png, montage, png, raster, svg, text, Shape};
pub use serializer::{from_json, from_lists, from_strings, to_json, to_lists, to_strings};
pub use tile::{build, create as create_tile, random_tile};
