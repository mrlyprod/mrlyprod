use super::Cell3d;
use super::faces::quads;
use mrly_math::cell::push_glyph;
use std::collections::HashMap;

pub fn text(cell: &Cell3d, glyphs: Option<&HashMap<u8, String>>) -> Vec<String> {
    let types = cell.types();
    let (width, height, depth) = (types.shape[0], types.shape[1], types.shape[2]);
    let mut rows = Vec::with_capacity(depth * height);
    for z in 0..depth {
        for y in (0..height).rev() {
            let mut row = String::with_capacity(width);
            for x in 0..width {
                let at = (x * height + y) * depth + z;
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
    let whole = |v: f32| (v * half + half).round() as f64;
    let corners: Vec<[f64; 3]> = quads(cell)
        .iter()
        .flat_map(|quad| quad.verts.map(|v| [whole(v.x), whole(v.y), whole(v.z)]))
        .collect();
    mrly_obj::soup(&corners, 4)
        .and_then(|(vertices, faces)| mrly_obj::encode(&vertices, &faces))
        .unwrap_or_default()
}
