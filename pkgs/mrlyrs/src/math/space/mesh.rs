use super::vec::Vec3;

pub const SOLIDS: [&str; 4] = ["cube", "tetra", "octa", "icosa"];

const S3: f32 = 0.57735027;
const PHI: f32 = 1.618034;

#[derive(Clone, Debug)]
pub struct Mesh {
    pub verts: Vec<Vec3>,
    pub faces: Vec<[usize; 3]>,
    pub normals: Vec<Vec3>,
}

impl Mesh {
    pub fn edges(&self) -> Vec<[usize; 2]> {
        let mut seen: std::collections::HashMap<(usize, usize), Vec3> =
            std::collections::HashMap::new();
        let mut out = Vec::new();
        for (i, f) in self.faces.iter().enumerate() {
            let n = self.normals[i];
            for (a, b) in [(f[0], f[1]), (f[1], f[2]), (f[0], f[2])] {
                let key = (a.min(b), a.max(b));
                match seen.get(&key) {
                    None => {
                        seen.insert(key, n);
                    }
                    Some(prev) => {
                        if prev.dot(n) < 0.9999 {
                            out.push([key.0, key.1]);
                        }
                    }
                }
            }
        }
        out
    }
}

fn build(verts: Vec<Vec3>, faces: Vec<[usize; 3]>, k: f32, scale: f32) -> Mesh {
    let normals = faces
        .iter()
        .map(|f| {
            let (a, b, c) = (verts[f[0]], verts[f[1]], verts[f[2]]);
            let n = (b - a).cross(c - a).scale(k);
            let center = a + b + c;
            if n.dot(center) < 0.0 {
                n.scale(-1.0)
            } else {
                n
            }
        })
        .collect();
    Mesh {
        verts: verts.into_iter().map(|v| v.scale(scale)).collect(),
        faces,
        normals,
    }
}

pub fn solid(name: &str) -> Mesh {
    match name {
        "tetra" => tetra(),
        "octa" => octa(),
        "icosa" => icosa(),
        _ => cube(),
    }
}

pub fn cube() -> Mesh {
    let mut verts = Vec::new();
    for x in [-1.0f32, 1.0] {
        for y in [-1.0f32, 1.0] {
            for z in [-1.0f32, 1.0] {
                verts.push(Vec3::new(x, y, z));
            }
        }
    }
    let quads = [
        [0, 1, 3, 2],
        [4, 5, 7, 6],
        [0, 1, 5, 4],
        [2, 3, 7, 6],
        [0, 2, 6, 4],
        [1, 3, 7, 5],
    ];
    let mut faces = Vec::new();
    for q in quads {
        faces.push([q[0], q[1], q[2]]);
        faces.push([q[0], q[2], q[3]]);
    }
    build(verts, faces, 0.25, S3)
}

pub fn tetra() -> Mesh {
    let verts = vec![
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, -1.0, -1.0),
        Vec3::new(-1.0, 1.0, -1.0),
        Vec3::new(-1.0, -1.0, 1.0),
    ];
    let faces = vec![[0, 1, 2], [0, 1, 3], [0, 2, 3], [1, 2, 3]];
    build(verts, faces, 0.14433757, S3)
}

pub fn octa() -> Mesh {
    let verts = vec![
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(-1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, -1.0, 0.0),
        Vec3::new(0.0, 0.0, 1.0),
        Vec3::new(0.0, 0.0, -1.0),
    ];
    let mut faces = Vec::new();
    for x in [0, 1] {
        for y in [2, 3] {
            for z in [4, 5] {
                faces.push([x, y, z]);
            }
        }
    }
    build(verts, faces, S3, 1.0)
}

pub fn icosa() -> Mesh {
    let verts = vec![
        Vec3::new(-1.0, PHI, 0.0),
        Vec3::new(1.0, PHI, 0.0),
        Vec3::new(-1.0, -PHI, 0.0),
        Vec3::new(1.0, -PHI, 0.0),
        Vec3::new(0.0, -1.0, PHI),
        Vec3::new(0.0, 1.0, PHI),
        Vec3::new(0.0, -1.0, -PHI),
        Vec3::new(0.0, 1.0, -PHI),
        Vec3::new(PHI, 0.0, -1.0),
        Vec3::new(PHI, 0.0, 1.0),
        Vec3::new(-PHI, 0.0, -1.0),
        Vec3::new(-PHI, 0.0, 1.0),
    ];
    let faces = vec![
        [0, 11, 5],
        [0, 5, 1],
        [0, 1, 7],
        [0, 7, 10],
        [0, 10, 11],
        [1, 5, 9],
        [5, 11, 4],
        [11, 10, 2],
        [10, 7, 6],
        [7, 1, 8],
        [3, 9, 4],
        [3, 4, 2],
        [3, 2, 6],
        [3, 6, 8],
        [3, 8, 9],
        [4, 9, 5],
        [2, 4, 11],
        [6, 2, 10],
        [8, 6, 7],
        [9, 8, 1],
    ];
    build(verts, faces, 0.28867513, 0.525_731_1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn all() -> Vec<(&'static str, Mesh)> {
        SOLIDS.iter().map(|&n| (n, solid(n))).collect()
    }

    #[test]
    fn normals_are_unit_and_outward() {
        for (name, mesh) in all() {
            for (i, f) in mesh.faces.iter().enumerate() {
                let n = mesh.normals[i];
                assert!((n.dot(n) - 1.0).abs() < 1e-4, "{name} face {i} |n|");
                let center = mesh.verts[f[0]] + mesh.verts[f[1]] + mesh.verts[f[2]];
                assert!(n.dot(center) > 0.0, "{name} face {i} inward");
            }
        }
    }
    #[test]
    fn solids_close() {
        for (name, mesh) in all() {
            let mut edges = HashSet::new();
            for f in &mesh.faces {
                for (a, b) in [(f[0], f[1]), (f[1], f[2]), (f[0], f[2])] {
                    edges.insert((a.min(b), a.max(b)));
                }
            }
            let v = mesh.verts.len() as i64;
            let e = edges.len() as i64;
            let f = mesh.faces.len() as i64;
            assert_eq!(v - e + f, 2, "{name} euler");
        }
    }
    #[test]
    fn solids_fit_the_unit_ball() {
        for (name, mesh) in all() {
            for v in &mesh.verts {
                assert!(v.dot(*v) < 1.001, "{name} vert outside");
            }
        }
    }
    #[test]
    fn unknown_names_fall_back_to_cube() {
        assert_eq!(solid("soup").faces.len(), solid("cube").faces.len());
    }
    #[test]
    fn edges_skip_coplanar_diagonals() {
        for (name, count) in [("cube", 12), ("tetra", 6), ("octa", 12), ("icosa", 30)] {
            assert_eq!(solid(name).edges().len(), count, "{name}");
        }
    }
}
