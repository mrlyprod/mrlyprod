/// The fill, void, volume, surface and Euler counts of a cube.
pub mod census;
/// The builders of coded, corner, noise and carpet cubes.
pub mod designs;
/// The exposed quads and wire edges of a cube.
pub mod faces;
/// The orientations, merges, masked mosaics, slices and lifts of cubes.
pub mod geometry;
/// The core, edge and tunnel graphs of a cube.
pub mod graph;
mod models;
/// The coloring of a cube by its types.
pub mod painter;
/// The seeded diffusion walk over a cube's wall faces.
pub mod reach;
/// The text and OBJ renderings of a cube.
pub mod renderer;
/// The list, string and JSON forms of a cube.
pub mod serializer;
/// The rimmed hole-grid plate and its wire box.
pub mod sheets;
/// The random tiles and the cubes they build.
pub mod tile;

pub use census::{census, euler, fills, Census};
pub use designs::{
    carpet, create, from_corners, level_set, levels_code, named, net, noise, ones, random,
    random_classic, void, xtree, ytree, zeros, ztree,
};
pub use faces::{quads, wires, Quad};
pub use geometry::{extrude, magic, manhattan_layers, merge, mosaic, orientations, slice, special};
pub use graph::{core_graph, edge_graph, tunnel_graph};
pub use models::Cell3d;
pub use painter::paint;
pub use reach::{explore, sweep, walls, Reach};
pub use renderer::{obj, text};
pub use serializer::{from_json, from_lists, from_strings, to_json, to_lists, to_strings};
pub use sheets::{sheet, sheet_edges};
pub use tile::{build, create as create_tile, random_tile};
