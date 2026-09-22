//! The cube pipeline.
//!
//! The shared cell pipeline pinned to three dimensions: coded and carpet cubes, their censuses,
//! their diagonal slices, their exposed quads and their JSON.

/// The fill, void, volume, surface and Euler counts of a cube.
pub mod census;
/// The builders of coded, corner, noise and carpet cubes.
pub mod designs;
/// The diagonal slices of a cube: their profile, their cells, their projection and their drawing.
pub mod diagonal;
/// The exposed quads and wire edges of a cube.
pub mod faces;
/// The orientations, merges, masked mosaics, slices and lifts of cubes.
pub mod geometry;
/// The JSON form of a cube.
pub mod serializer;

pub use crate::math::cell::graph::{core_graph, edge_graph, tunnel_graph};
pub use crate::math::cell::models::Cell3d;
pub use crate::math::cell::paint;
pub use census::{census, euler, fills, hidden, Census};
pub use designs::{
    carpet, create, dust, from_corners, level_set, levels_code, named, net, ones, point, star,
    void, xline, xtree, yline, ytree, zeros, zline, ztree,
};
pub use diagonal::{
    profile, project, shadow, slice as diagonal_slice, support, svg as diagonal_svg,
};
pub use faces::{quads, wires, Quad, Vec3};
pub use geometry::{extrude, magic, manhattan_layers, merge, mosaic, orientations, slice, special};
pub use serializer::{from_json, to_json};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn menger_graphs() {
        let cell = designs::carpet(3, 1).unwrap();
        let core = core_graph(&cell).unwrap();
        assert_eq!(core.nodes.len(), 20);
        let tunnels = tunnel_graph(&cell).unwrap();
        assert_eq!(tunnels.nodes.len(), 7);
    }
}
