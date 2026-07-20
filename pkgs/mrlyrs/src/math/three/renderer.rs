use super::models::Cell3d;
use crate::math::dim::push_glyph;
use std::collections::HashMap;

pub fn text(cell: &Cell3d, glyphs: Option<&HashMap<u8, char>>) -> Vec<Vec<String>> {
    let shape = &cell.types().shape;
    let mut slices = Vec::with_capacity(shape[2]);
    for z in 0..shape[2] {
        let mut rows = Vec::with_capacity(shape[0]);
        for y in 0..shape[0] {
            let mut row = String::with_capacity(shape[1]);
            for x in 0..shape[1] {
                let v = cell.types().get(&[y, x, z]);
                push_glyph(&mut row, v, glyphs);
            }
            rows.push(row);
        }
        slices.push(rows);
    }
    slices
}

pub fn obj(cell: &Cell3d) -> String {
    let grid = cell.types();
    let (dx, dy, dz) = (grid.shape[0], grid.shape[1], grid.shape[2]);
    let mut vertices: Vec<(usize, usize, usize)> = Vec::new();
    let mut vertex_map: HashMap<(usize, usize, usize), usize> = HashMap::new();
    let mut faces: Vec<(usize, usize, usize, usize)> = Vec::new();
    let add = |map: &mut HashMap<(usize, usize, usize), usize>,
               verts: &mut Vec<(usize, usize, usize)>,
               v: (usize, usize, usize)|
     -> usize {
        *map.entry(v).or_insert_with(|| {
            verts.push(v);
            verts.len()
        })
    };
    for i in 0..dx {
        for j in 0..dy {
            for k in 0..dz {
                if grid.get(&[i, j, k]) == 0 {
                    continue;
                }
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
                let (ii, jj, kk) = (i as isize, j as isize, k as isize);
                if empty(ii - 1, jj, kk) {
                    let v1 = add(&mut vertex_map, &mut vertices, (i, j, k));
                    let v2 = add(&mut vertex_map, &mut vertices, (i, j, k + 1));
                    let v3 = add(&mut vertex_map, &mut vertices, (i, j + 1, k + 1));
                    let v4 = add(&mut vertex_map, &mut vertices, (i, j + 1, k));
                    faces.push((v1, v2, v3, v4));
                }
                if empty(ii + 1, jj, kk) {
                    let v1 = add(&mut vertex_map, &mut vertices, (i + 1, j, k));
                    let v2 = add(&mut vertex_map, &mut vertices, (i + 1, j + 1, k));
                    let v3 = add(&mut vertex_map, &mut vertices, (i + 1, j + 1, k + 1));
                    let v4 = add(&mut vertex_map, &mut vertices, (i + 1, j, k + 1));
                    faces.push((v1, v2, v3, v4));
                }
                if empty(ii, jj - 1, kk) {
                    let v1 = add(&mut vertex_map, &mut vertices, (i, j, k));
                    let v2 = add(&mut vertex_map, &mut vertices, (i + 1, j, k));
                    let v3 = add(&mut vertex_map, &mut vertices, (i + 1, j, k + 1));
                    let v4 = add(&mut vertex_map, &mut vertices, (i, j, k + 1));
                    faces.push((v1, v2, v3, v4));
                }
                if empty(ii, jj + 1, kk) {
                    let v1 = add(&mut vertex_map, &mut vertices, (i, j + 1, k));
                    let v2 = add(&mut vertex_map, &mut vertices, (i, j + 1, k + 1));
                    let v3 = add(&mut vertex_map, &mut vertices, (i + 1, j + 1, k + 1));
                    let v4 = add(&mut vertex_map, &mut vertices, (i + 1, j + 1, k));
                    faces.push((v1, v2, v3, v4));
                }
                if empty(ii, jj, kk - 1) {
                    let v1 = add(&mut vertex_map, &mut vertices, (i, j, k));
                    let v2 = add(&mut vertex_map, &mut vertices, (i, j + 1, k));
                    let v3 = add(&mut vertex_map, &mut vertices, (i + 1, j + 1, k));
                    let v4 = add(&mut vertex_map, &mut vertices, (i + 1, j, k));
                    faces.push((v1, v2, v3, v4));
                }
                if empty(ii, jj, kk + 1) {
                    let v1 = add(&mut vertex_map, &mut vertices, (i, j, k + 1));
                    let v2 = add(&mut vertex_map, &mut vertices, (i + 1, j, k + 1));
                    let v3 = add(&mut vertex_map, &mut vertices, (i + 1, j + 1, k + 1));
                    let v4 = add(&mut vertex_map, &mut vertices, (i, j + 1, k + 1));
                    faces.push((v1, v2, v3, v4));
                }
            }
        }
    }
    let mut out = String::new();
    for (x, y, z) in &vertices {
        out.push_str(&format!("v {x} {z} {}\n", dy - y));
    }
    for (a, b, c, d) in &faces {
        out.push_str(&format!("f {a} {b} {c} {d}\n"));
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::three::designs;
    #[test]
    fn single_cube_obj() {
        let c = designs::ones(1, 1).unwrap();
        let o = obj(&c);
        assert_eq!(o.matches("v ").count(), 8);
        assert_eq!(o.matches("f ").count(), 6);
    }
    #[test]
    fn menger_obj_has_interior_faces_culled() {
        let c = designs::carpet(3, 1).unwrap();
        let o = obj(&c);
        let faces = o.matches("f ").count();
        assert!(faces < 20 * 6);
        assert!(faces > 6);
    }
    #[test]
    fn text_slices() {
        let c = designs::ztree(3, 1).unwrap();
        let slices = text(&c, None);
        assert_eq!(slices.len(), 3);
        assert_eq!(slices[0].len(), 3);
        assert_eq!(slices[0][0].len(), 3);
    }
}
