use super::Cell3d;
use crate::math::space::Vec3;

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
        grid.get(&[i, j, k]) == 0
    };
    let mut out = Vec::new();
    for i in 0..dx {
        for j in 0..dy {
            for k in 0..dz {
                if grid.get(&[i, j, k]) == 0 {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::three::ones;

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
}
