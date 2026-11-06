use super::Cell3d;
use super::faces::quads;
use mrly_math::cell::push_glyph;
use std::collections::HashMap;

pub fn text(cell: &Cell3d, glyphs: Option<&HashMap<u8, String>>) -> Vec<String> {
    let types = cell.types();
    let (depth, height, width) = (types.shape[0], types.shape[1], types.shape[2]);
    let mut rows = Vec::with_capacity(depth * height);
    for z in 0..depth {
        for y in 0..height {
            let mut row = String::with_capacity(width);
            for x in 0..width {
                let at = (z * height + y) * width + x;
                push_glyph(&mut row, types.at(at) as u8, glyphs);
            }
            rows.push(row);
        }
    }
    rows
}

pub fn to_obj(cell: &Cell3d) -> String {
    let shape = &cell.types().shape;
    let side = shape[0].max(shape[1]).max(shape[2]) as f32;
    let half = side / 2.0;
    let whole = |v: f32| (v * half + half).round() as i64;
    let mut corners: Vec<[i64; 3]> = Vec::new();
    let mut seen: HashMap<[i64; 3], usize> = HashMap::new();
    let mut faces: Vec<[usize; 4]> = Vec::new();
    for quad in quads(cell) {
        let mut face = [0usize; 4];
        for (slot, vert) in quad.verts.iter().enumerate() {
            let key = [whole(vert.x), whole(vert.y), whole(vert.z)];
            let next = seen.len() + 1;
            face[slot] = *seen.entry(key).or_insert_with(|| {
                corners.push(key);
                next
            });
        }
        faces.push(face);
    }
    let mut out = String::new();
    for corner in &corners {
        out.push_str(&format!("v {} {} {}\n", corner[0], corner[1], corner[2]));
    }
    for face in &faces {
        out.push_str(&format!(
            "f {} {} {} {}\n",
            face[0], face[1], face[2], face[3]
        ));
    }
    out
}
