/// The fill, void, volume and surface counts of a cube.
pub mod census;
/// The builders of coded, corner, noise and carpet cubes.
pub mod designs;
/// The exposed quads and wire edges of a cube.
pub mod faces;
/// The orientations, merges and masked mosaics of cubes.
pub mod geometry;
/// The core, edge and tunnel graphs of a cube.
pub mod graph;
mod models;
/// The coloring of a cube by its types.
pub mod painter;
/// The text and OBJ renderings of a cube.
pub mod renderer;
/// The list and JSON forms of a cube.
pub mod serializer;
/// The rimmed hole-grid plate and its wire box.
pub mod sheets;
/// The random tiles and the cubes they build.
pub mod tile;

pub use designs::{
    carpet, create, from_corners, net, noise, ones, random, void, xtree, ytree, zeros, ztree,
};
pub use faces::{quads, wires, Quad};
pub use geometry::{magic, merge, mosaic, orientations, slice, special};
pub use graph::{core_graph, edge_graph, tunnel_graph};
pub use models::Cell3d;
pub use painter::paint;
pub use renderer::{obj, text};
pub use serializer::{from_json, from_lists, to_json, to_lists};
pub use sheets::{sheet, sheet_edges};
pub use tile::{build, create as create_tile, random_tile};
