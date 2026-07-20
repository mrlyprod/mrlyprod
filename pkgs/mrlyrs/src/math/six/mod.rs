pub mod census;
pub mod designs;
pub mod geometry;
pub mod graph;
mod models;
pub mod painter;
pub mod renderer;
pub mod serializer;
pub mod tile;

pub const VOID: u8 = 0;
pub const FILL: u8 = 1;
pub const GRID: u8 = 2;
pub const UP: u8 = 3;
pub const LEFT: u8 = 4;
pub const RIGHT: u8 = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Projection {
    Iso,
    Pro,
    Cut,
}

pub use designs::{cut_design, iso_design, pro_design};
pub use geometry::{
    blank, cut, is_cube, is_hex, iso, orientation, pad, pro, radial, radial_mask, tessellate, tile,
    tile_crop,
};
pub use models::Cell6d;
pub use painter::paint;
pub use renderer::{png, svg, triangles};
pub use serializer::{from_json, to_json};
pub use tile::{build as build_tile, random_tile, HexTile};
