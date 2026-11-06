use serde::{Deserialize, Serialize};

pub mod census;
pub mod designs;
pub mod geometry;
pub mod graph;
mod models;
pub mod painter;
pub mod raster;
pub mod renderer;
pub mod serializer;
pub mod topology;

pub const VOID: u8 = 0;
pub const FILL: u8 = 1;
pub const GRID: u8 = 2;
pub const UP: u8 = 3;
pub const LEFT: u8 = 4;
pub const RIGHT: u8 = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Projection {
    Iso,
    Pro,
    Cut,
}

pub use census::{Census, census, euler, fills, fills_only};
pub use designs::{cut_design, iso_design, pro_design};
pub use geometry::{
    blank, copies, cut, disc, framed, is_cube, is_hex, iso, orientation, pad, pro, radial,
    radial_crop, radial_mask, skin, tessellate, tile, tile_cell, tile_crop, tile_step,
};
pub use graph::{slice_core_graph, slice_dual_graph, slice_edge_graph, slice_tunnel_graph};
pub use models::Cell6d;
pub use painter::paint;
pub use raster::raster;
pub use renderer::{east, image, north, rect_image, rect_svg, south, svg, triangles, west};
pub use serializer::{from_json, to_json};
pub use topology::{components, giant, giant_network, holes, rim_holes, spectral_exponent};
