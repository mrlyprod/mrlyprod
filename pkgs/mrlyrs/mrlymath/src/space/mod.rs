pub mod mesh;
pub mod pack;
pub mod vec;

pub const TURN: i64 = mrlycore::trig::N as i64;
pub const PITCH_MAX: i64 = 56;

pub use mesh::{cube, icosa, octa, solid, tetra, Mesh, SOLIDS};
pub use pack::{axis_edges, Edge, Pack, MESH_WGSL};
pub use vec::{Mat3, Vec3};
