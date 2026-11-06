use crate::Mesh;
use mrly_error::{Result, value_error};
use mrly_three::Vec3;

pub const SOLIDS: [&str; 5] = ["tetra", "cube", "octa", "dodeca", "icosa"];

const PHI: f32 = 1.618_034;

pub fn solid(name: &str) -> Result<Mesh> {
    let (verts, faces) = match name {
        "tetra" => tetra(),
        "cube" => cube(),
        "octa" => octa(),
        "dodeca" => dodeca(),
        "icosa" => icosa(),
        _ => {
            return value_error(format!(
                "unknown solid {name:?}: try tetra, cube, octa, dodeca or icosa."
            ));
        }
    };
    Ok(build(verts, faces))
}

fn build(verts: Vec<Vec3>, polygons: Vec<Vec<usize>>) -> Mesh {
    let unit = |v: Vec3| v.scale(1.0 / v.dot(v).sqrt());
    let verts: Vec<Vec3> = verts.into_iter().map(unit).collect();
    let mut out = Mesh {
        verts,
        faces: Vec::new(),
        normals: Vec::new(),
        colors: None,
    };
    for mut polygon in polygons {
        let points = polygon.iter().map(|&v| out.verts[v]);
        let normal = unit(points.fold(Vec3::new(0.0, 0.0, 0.0), |a, b| a + b));
        let [a, b, c] = [0, 1, 2].map(|k| out.verts[polygon[k]]);
        if (b - a).cross(c - a).dot(normal) < 0.0 {
            polygon.reverse();
        }
        for k in 1..polygon.len() - 1 {
            out.faces.push([polygon[0], polygon[k], polygon[k + 1]]);
            out.normals.push(normal);
        }
    }
    out
}

fn tetra() -> (Vec<Vec3>, Vec<Vec<usize>>) {
    let verts = vec![
        Vec3::new(1.0, 1.0, 1.0),
        Vec3::new(1.0, -1.0, -1.0),
        Vec3::new(-1.0, 1.0, -1.0),
        Vec3::new(-1.0, -1.0, 1.0),
    ];
    (
        verts,
        vec![vec![0, 1, 2], vec![0, 1, 3], vec![0, 2, 3], vec![1, 2, 3]],
    )
}

fn cube() -> (Vec<Vec3>, Vec<Vec<usize>>) {
    let mut verts = Vec::new();
    for x in [-1.0, 1.0] {
        for y in [-1.0, 1.0] {
            for z in [-1.0, 1.0] {
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
    (verts, quads.map(|q| q.to_vec()).to_vec())
}

fn octa() -> (Vec<Vec3>, Vec<Vec<usize>>) {
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
                faces.push(vec![x, y, z]);
            }
        }
    }
    (verts, faces)
}

fn icosa() -> (Vec<Vec3>, Vec<Vec<usize>>) {
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
        vec![0, 11, 5],
        vec![0, 5, 1],
        vec![0, 1, 7],
        vec![0, 7, 10],
        vec![0, 10, 11],
        vec![1, 5, 9],
        vec![5, 11, 4],
        vec![11, 10, 2],
        vec![10, 7, 6],
        vec![7, 1, 8],
        vec![3, 9, 4],
        vec![3, 4, 2],
        vec![3, 2, 6],
        vec![3, 6, 8],
        vec![3, 8, 9],
        vec![4, 9, 5],
        vec![2, 4, 11],
        vec![6, 2, 10],
        vec![8, 6, 7],
        vec![9, 8, 1],
    ];
    (verts, faces)
}

fn dodeca() -> (Vec<Vec3>, Vec<Vec<usize>>) {
    let (ico, tris) = icosa();
    let verts: Vec<Vec3> = tris
        .iter()
        .map(|t| ico[t[0]] + ico[t[1]] + ico[t[2]])
        .collect();
    let mut faces = Vec::new();
    for corner in 0..ico.len() {
        let mut ring: Vec<usize> = (0..tris.len())
            .filter(|&t| tris[t].contains(&corner))
            .collect();
        let mut order = vec![ring.swap_remove(0)];
        while !ring.is_empty() {
            let last = &tris[order[order.len() - 1]];
            let next = ring
                .iter()
                .position(|&t| tris[t].iter().filter(|v| last.contains(v)).count() == 2)
                .unwrap_or(0);
            order.push(ring.swap_remove(next));
        }
        faces.push(order);
    }
    (verts, faces)
}
