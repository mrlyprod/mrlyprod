pub mod camera;
pub mod mesh;
pub mod pack;
pub mod vec;

pub use camera::{beam, project, view, PITCH_MAX, TURN};
pub use mesh::{cube, icosa, octa, solid, tetra, Mesh, SOLIDS};
pub use pack::{axis_edges, Edge, Pack, MESH_WGSL};
pub use vec::{Mat3, Vec3};
