use super::edge_graph;
use super::Cell3d;
use serde::{Deserialize, Serialize};

/// A three-component vector of f32.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vec3 {
    /// The x component.
    pub x: f32,
    /// The y component.
    pub y: f32,
    /// The z component.
    pub z: f32,
}

impl std::ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, o: Vec3) -> Vec3 {
        Vec3::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, o: Vec3) -> Vec3 {
        Vec3::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}

impl Vec3 {
    /// Builds a vector from its components.
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
    /// Multiplies every component by the scalar.
    pub fn scale(self, s: f32) -> Vec3 {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }
    /// Returns the dot product of the two vectors.
    pub fn dot(self, o: Vec3) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
    /// Returns the cross product, perpendicular to both vectors.
    ///
    /// ```
    /// use mrlyrs::math::three::Vec3;
    /// let n = Vec3::new(1.0, 0.0, 0.0).cross(Vec3::new(0.0, 1.0, 0.0));
    /// assert_eq!(n, Vec3::new(0.0, 0.0, 1.0));
    /// ```
    pub fn cross(self, o: Vec3) -> Vec3 {
        Vec3::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }
}

/// An outward face of a filled site: its normal and four corners.
#[derive(Serialize, Deserialize)]
pub struct Quad {
    /// The outward unit normal.
    pub normal: Vec3,
    /// The four corners in winding order.
    pub verts: [Vec3; 4],
}

/// Returns one outward quad per exposed face, scaled into the unit box.
pub fn quads(cell: &Cell3d) -> Vec<Quad> {
    let grid = cell.types();
    let (dx, dy, dz) = (grid.shape[0], grid.shape[1], grid.shape[2]);
    let side = dx.max(dy).max(dz) as f32;
    let half = side / 2.0;
    let point = |i: usize, j: usize, k: usize| -> Vec3 {
        Vec3::new(
            (i as f32 - half) / half,
            (j as f32 - half) / half,
            (k as f32 - half) / half,
        )
    };
    let empty = |i: isize, j: isize, k: isize| -> bool {
        if i < 0 || j < 0 || k < 0 {
            return true;
        }
        let (i, j, k) = (i as usize, j as usize, k as usize);
        if i >= dx || j >= dy || k >= dz {
            return true;
        }
        grid.at(grid.index(&[i, j, k])) == 0
    };
    let mut out = Vec::new();
    for i in 0..dx {
        for j in 0..dy {
            for k in 0..dz {
                if grid.at(grid.index(&[i, j, k])) == 0 {
                    continue;
                }
                let (ii, jj, kk) = (i as isize, j as isize, k as isize);
                if empty(ii - 1, jj, kk) {
                    out.push(Quad {
                        normal: Vec3::new(-1.0, 0.0, 0.0),
                        verts: [
                            point(i, j, k),
                            point(i, j, k + 1),
                            point(i, j + 1, k + 1),
                            point(i, j + 1, k),
                        ],
                    });
                }
                if empty(ii + 1, jj, kk) {
                    out.push(Quad {
                        normal: Vec3::new(1.0, 0.0, 0.0),
                        verts: [
                            point(i + 1, j, k),
                            point(i + 1, j + 1, k),
                            point(i + 1, j + 1, k + 1),
                            point(i + 1, j, k + 1),
                        ],
                    });
                }
                if empty(ii, jj - 1, kk) {
                    out.push(Quad {
                        normal: Vec3::new(0.0, -1.0, 0.0),
                        verts: [
                            point(i, j, k),
                            point(i + 1, j, k),
                            point(i + 1, j, k + 1),
                            point(i, j, k + 1),
                        ],
                    });
                }
                if empty(ii, jj + 1, kk) {
                    out.push(Quad {
                        normal: Vec3::new(0.0, 1.0, 0.0),
                        verts: [
                            point(i, j + 1, k),
                            point(i, j + 1, k + 1),
                            point(i + 1, j + 1, k + 1),
                            point(i + 1, j + 1, k),
                        ],
                    });
                }
                if empty(ii, jj, kk - 1) {
                    out.push(Quad {
                        normal: Vec3::new(0.0, 0.0, -1.0),
                        verts: [
                            point(i, j, k),
                            point(i, j + 1, k),
                            point(i + 1, j + 1, k),
                            point(i + 1, j, k),
                        ],
                    });
                }
                if empty(ii, jj, kk + 1) {
                    out.push(Quad {
                        normal: Vec3::new(0.0, 0.0, 1.0),
                        verts: [
                            point(i, j, k + 1),
                            point(i + 1, j, k + 1),
                            point(i + 1, j + 1, k + 1),
                            point(i, j + 1, k + 1),
                        ],
                    });
                }
            }
        }
    }
    out
}

/// Returns the cell's edge-graph segments, scaled into the unit box.
pub fn wires(cell: &Cell3d) -> Vec<[Vec3; 2]> {
    let grid = cell.types();
    let side = grid.shape[0].max(grid.shape[1]).max(grid.shape[2]) as f32;
    let half = side / 2.0;
    let point = |p: &[f64]| {
        Vec3::new(
            (p[2] as f32 - half) / half,
            (p[1] as f32 - half) / half,
            (p[0] as f32 - half) / half,
        )
    };
    let Ok(net) = edge_graph(cell) else {
        return Vec::new();
    };
    net.branches
        .iter()
        .map(|branch| {
            [
                point(&net.nodes[branch.parent].position),
                point(&net.nodes[branch.child].position),
            ]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::three::{ones, xtree};
    #[test]
    fn cross_is_perpendicular() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(-2.0, 0.5, 1.0);
        let c = a.cross(b);
        assert!(c.dot(a).abs() < 1e-6);
        assert!(c.dot(b).abs() < 1e-6);
    }
    #[test]
    fn a_solid_cell_has_six_outward_faces() {
        let cell = ones(1, 1).unwrap();
        let faces = quads(&cell);
        assert_eq!(faces.len(), 6);
        let normals: Vec<Vec3> = faces.iter().map(|q| q.normal).collect();
        for axis in [
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(-1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, -1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(0.0, 0.0, -1.0),
        ] {
            assert!(normals.contains(&axis));
        }
    }

    #[test]
    fn adjacent_solid_cells_hide_their_shared_face() {
        let cell = ones(2, 1).unwrap();
        let faces = quads(&cell);
        assert_eq!(faces.len(), 6 * 8 - 12 * 2);
    }

    #[test]
    fn a_solid_cell_wires_its_twelve_edges() {
        let lines = wires(&ones(1, 1).unwrap());
        assert_eq!(lines.len(), 12);
        for [a, b] in lines {
            assert!(a.x.abs() <= 1.0 && a.y.abs() <= 1.0 && a.z.abs() <= 1.0);
            assert!(a != b);
        }
    }

    #[test]
    fn the_wires_of_an_asymmetric_cell_frame_its_quads() {
        let cell = xtree(3, 2).unwrap();
        let mut ends: Vec<Vec3> = Vec::new();
        for [a, b] in wires(&cell) {
            ends.push(a);
            ends.push(b);
        }
        for quad in quads(&cell) {
            for vert in quad.verts {
                assert!(ends.contains(&vert));
            }
        }
    }
}
