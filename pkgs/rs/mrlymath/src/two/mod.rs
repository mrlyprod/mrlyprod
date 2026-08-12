/// The seeded artwork run from tile recipe to rendered files.
pub mod artwork;
/// The fill, void and perimeter counts of a flat cell.
pub mod census;
/// The builders of coded, corner, noise and carpet cells.
pub mod designs;
/// The merges, magic folds and masked mosaics of flat cells.
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

pub use census::fills;
pub use designs::{
    carpet, create, from_corners, htree, net, noise, ones, random, void, vtree, zeros,
};
pub use geometry::{magic, merge, mosaic, special};
pub use models::Cell2d;
pub use painter::paint;
pub use renderer::{png, svg, text};
pub use serializer::{from_json, from_lists, from_strings, to_json, to_lists, to_strings};
pub use tile::{build, create as create_tile, random_tile};
