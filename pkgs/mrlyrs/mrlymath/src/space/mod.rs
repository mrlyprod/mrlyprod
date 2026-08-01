pub mod camera;
pub mod mesh;
pub mod pack;
pub mod vec;

pub use camera::{beam, project, view, Camera, Rig, DIST_MAX, DIST_MIN, PAN_MAX, PITCH_MAX, TURN};
pub use mesh::{cube, icosa, octa, solid, tetra, Mesh, SOLIDS};
pub use pack::{axis_edges, Edge, Pack, MESH_WGSL};
pub use vec::{Mat3, Vec3};
