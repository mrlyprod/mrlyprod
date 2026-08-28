/// The triangulated solids and their outward normals.
pub mod mesh;
/// The flat float buffer that carries triangles and lines to the shader.
pub mod pack;
/// The three-component vector and its rotation matrix.
pub mod vec;

/// The number of angle steps in one full turn.
pub const TURN: i64 = mrlycore::trig::N as i64;
/// The pitch limit in angle steps.
pub const PITCH_MAX: i64 = 56;

pub use mesh::{cube, icosa, octa, solid, tetra, Mesh, SOLIDS};
pub use pack::{axis_edges, Edge, Pack, MESH_WGSL};
pub use vec::{Mat3, Vec3};
