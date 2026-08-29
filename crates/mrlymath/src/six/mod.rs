/// The triangle, corner and edge tallies of a hex cell, whose census and euler take the extra include_grid flag.
pub mod census;
/// The coded cubes projected to iso, pro and cut hexagons.
pub mod designs;
/// The hexagon shapes, projections and tessellations.
pub mod geometry;
/// The core, dual and edge networks of a hex slice.
pub mod graph;
mod models;
/// The coloring of a hex cell's triangles by type.
pub mod painter;
/// The triangle, SVG and PNG renderings of a hex cell.
pub mod renderer;
/// The JSON form of a hex cell and its projection.
pub mod serializer;
/// The random 3d tiles flattened through their projections.
pub mod tile;
/// The pieces, holes and enclosed voids of a hex slice's fill.
pub mod topology;

/// The triangle code for an empty site.
pub const VOID: u8 = 0;
/// The triangle code for a filled site.
pub const FILL: u8 = 1;
/// The triangle code for the backdrop outside the figure.
pub const GRID: u8 = 2;
/// The triangle code for a cube's top face in the iso view.
pub const UP: u8 = 3;
/// The triangle code for a cube's left face in the iso view.
pub const LEFT: u8 = 4;
/// The triangle code for a cube's right face in the iso view.
pub const RIGHT: u8 = 5;

/// The two ways a hexagon can point.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    /// The hexagon wider than it is tall.
    Horizontal,
    /// The hexagon taller than it is wide.
    Vertical,
}

/// The three ways a cube flattens to a hexagon.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Projection {
    /// The isometric view of top, left and right faces.
    Iso,
    /// The three-face view keeping each face's fills and voids.
    Pro,
    /// The hexagonal slice through the cube's center.
    Cut,
}

pub use census::{census, euler, fills, fills_only, Census};
pub use designs::{cut_design, iso_design, pro_design};
pub use geometry::{
    blank, cut, is_cube, is_hex, iso, orientation, pad, pro, radial, radial_crop, radial_mask,
    tessellate, tile, tile_crop,
};
pub use models::Cell6d;
pub use painter::paint;
pub use renderer::{hex_png, png, rect, rect_png, rect_svg, svg, triangles, Rect};
pub use serializer::{from_json, to_json};
pub use tile::{build as build_tile, random_tile, HexTile};
pub use topology::{components, giant, giant_network, holes, rim_holes, spectral_exponent};
