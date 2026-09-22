use super::faces::quads;
use super::Cell3d;
use crate::math::cell::push_glyph;
use std::collections::HashMap;

/// Renders the cube as rows of glyphs, plane after plane, or of digits where no glyph is mapped.
///
/// ```
/// let cell = mrlyrs::math::three::carpet(3, 1).unwrap();
/// let rows = mrlyrs::math::three::text(&cell, None);
/// assert_eq!(rows.len(), 9);
/// assert_eq!(&rows[..3], ["111", "101", "111"]);
/// ```
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

/// Writes the cube's exposed quads as a Wavefront OBJ, one shared vertex per corner.
///
/// ```
/// let obj = mrlyrs::math::three::to_obj(&mrlyrs::math::three::carpet(3, 1).unwrap());
/// assert_eq!(obj.lines().filter(|line| line.starts_with("v ")).count(), 64);
/// assert_eq!(obj.lines().filter(|line| line.starts_with("f ")).count(), 72);
/// ```
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::three::designs;
    #[test]
    fn text_walks_every_plane_of_the_sponge() {
        let rows = text(&designs::carpet(3, 1).unwrap(), None);
        assert_eq!(rows.len(), 9);
        assert_eq!(&rows[..3], ["111", "101", "111"]);
    }

    #[test]
    fn the_sponge_writes_its_obj() {
        let obj = to_obj(&designs::carpet(3, 1).unwrap());
        assert_eq!(obj.lines().filter(|l| l.starts_with("v ")).count(), 64);
        assert_eq!(obj.lines().filter(|l| l.starts_with("f ")).count(), 72);
        assert_eq!(obj.len(), 1481);
    }
}
