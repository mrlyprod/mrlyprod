use super::Cell3d;
use super::edge_graph;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
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
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn scale(self, s: f32) -> Vec3 {
        Vec3::new(self.x * s, self.y * s, self.z * s)
    }
    pub fn dot(self, o: Vec3) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
    pub fn cross(self, o: Vec3) -> Vec3 {
        Vec3::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }
}

#[derive(Serialize, Deserialize)]
pub struct Quad {
    pub normal: Vec3,
    pub verts: [Vec3; 4],
}

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
