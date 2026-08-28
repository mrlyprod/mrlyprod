use super::models::Cell3d;
use crate::dim::geometry;
use crate::dim::models::{dtype_for, Cell2d};
use mrlycore::cell::remap;
use mrlycore::errors::{value_error, Result};
use mrlycore::tensor::Tensor;
use std::sync::OnceLock;

pub use crate::dim::geometry::{magic, mosaic, perforate};

/// Returns the 24 rotation triples that reach each distinct cube orientation.
pub fn orientations() -> &'static Vec<(usize, usize, usize)> {
    static TABLE: OnceLock<Vec<(usize, usize, usize)>> = OnceLock::new();
    TABLE.get_or_init(|| {
        let mut probe = Tensor::new(vec![3, 3, 3]);
        for (flat, item) in probe.bytes_mut().iter_mut().enumerate() {
            *item = flat as u8;
        }
        let mut seen: Vec<Vec<u8>> = Vec::new();
        let mut table = Vec::new();
        for a in 0..4 {
            for b in 0..4 {
                for c in 0..4 {
                    let image = probe.rot90(a, (1, 2)).rot90(b, (0, 2)).rot90(c, (0, 1));
                    if !seen.contains(&image.bytes().to_vec()) {
                        seen.push(image.bytes().to_vec());
                        table.push((a, b, c));
                    }
                }
            }
        }
        table
    })
}

/// Merges the cells into one cube arranged width by height by depth.
pub fn merge(cells: &[Cell3d], width: usize, height: usize, depth: usize) -> Result<Cell3d> {
    geometry::merge_reps(cells, &[height, width, depth])
}

/// Orients a copy of the cell by each mask value and merges them in the mask's shape.
pub fn special(mask: &Tensor, cell: &Cell3d) -> Result<Cell3d> {
    if mask.shape.len() != 3 {
        return value_error("special mask must be 3d.");
    }
    if mask.bytes().iter().any(|&v| v > 23) {
        return value_error("Invalid orientation value. Must be 0..23.");
    }
    let oriented: Result<Vec<Cell3d>> = mask
        .bytes()
        .iter()
        .map(|&k| cell.clone().orient(k as usize))
        .collect();
    let oriented = oriented?;
    merge(&oriented, mask.shape[1], mask.shape[0], mask.shape[2])
}

fn slice_map(cube: &[usize], axis: usize, index: usize) -> (Vec<usize>, Vec<usize>) {
    let strides = [cube[1] * cube[2], cube[2], 1];
    let kept: Vec<usize> = (0..3).filter(|&a| a != axis).collect();
    let shape = vec![cube[kept[0]], cube[kept[1]]];
    let mut map = Vec::with_capacity(shape[0] * shape[1]);
    for row in 0..shape[0] {
        for col in 0..shape[1] {
            let mut at = [0usize; 3];
            at[axis] = index;
            at[kept[0]] = row;
            at[kept[1]] = col;
            map.push(at[0] * strides[0] + at[1] * strides[1] + at[2]);
        }
    }
    (shape, map)
}

/// Takes the flat cell left when one axis of the cube is fixed at an index, colors and tags with it.
///
/// ```
/// let sponge = mrlymath::three::carpet(3, 2).unwrap();
/// let front = mrlymath::three::slice(&sponge, 2, 0).unwrap();
/// assert_eq!(front, mrlymath::two::carpet(3, 2).unwrap());
/// ```
pub fn slice(cell: &Cell3d, axis: usize, index: usize) -> Result<Cell2d> {
    if axis > 2 {
        return value_error("slice axis is past the cube's rank.");
    }
    if index >= cell.types().shape[axis] {
        return value_error("slice index is past the axis.");
    }
    let (shape, map) = slice_map(&cell.types().shape, axis, index);
    Ok(Cell2d {
        cell: remap(&cell.cell, &map, &shape),
    })
}

// EXTRUDE

fn lift_map(shape: &[usize], axis: usize, depth: usize) -> (Vec<usize>, Vec<usize>) {
    let mut lifted = shape.to_vec();
    lifted.insert(axis, depth);
    let strides = [lifted[1] * lifted[2], lifted[2], 1];
    let (height, width) = (shape[0], shape[1]);
    let mut map = vec![0usize; lifted.iter().product()];
    for plane in 0..depth {
        for y in 0..height {
            for x in 0..width {
                let mut at = vec![y, x];
                at.insert(axis, plane);
                map[at[0] * strides[0] + at[1] * strides[1] + at[2]] = y * width + x;
            }
        }
    }
    (lifted, map)
}

/// Lifts a flat cell into a cube by repeating it depth times along a new axis, colors and tags with it.
///
/// This is the inverse of slice: every slice of the lift on that axis is the flat cell again.
///
/// ```
/// let flat = mrlymath::two::carpet(3, 2).unwrap();
/// let cube = mrlymath::three::extrude(&flat, 2, 4).unwrap();
/// assert_eq!(cube.depth(), 4);
/// assert_eq!(mrlymath::three::slice(&cube, 2, 3).unwrap(), flat);
/// ```
pub fn extrude(cell: &Cell2d, axis: usize, depth: usize) -> Result<Cell3d> {
    if axis > 2 {
        return value_error("extrude axis must be 0, 1 or 2.");
    }
    if depth == 0 {
        return value_error("extrude depth must be at least 1.");
    }
    let (shape, map) = lift_map(&cell.types().shape, axis, depth);
    Ok(Cell3d {
        cell: remap(&cell.cell, &map, &shape),
    })
}

// LAYERS

/// Tags every site with its Manhattan distance from the cube's center, the diamond shells.
///
/// The cell's own layers count Chebyshev shells, which are boxes; these are octahedra.
pub fn manhattan_layers(mut cell: Cell3d) -> Cell3d {
    let shape = cell.types().shape.clone();
    let strides = [shape[1] * shape[2], shape[2], 1];
    let mut rings = vec![0i64; shape.iter().product()];
    for i in 0..shape[0] {
        for j in 0..shape[1] {
            for k in 0..shape[2] {
                let at = [i, j, k];
                let reach: f64 = (0..3)
                    .map(|axis| (at[axis] as f64 - (shape[axis] as f64 - 1.0) / 2.0).abs())
                    .sum();
                rings[i * strides[0] + j * strides[1] + k] = reach.floor() as i64;
            }
        }
    }
    let peak = rings.iter().copied().max().unwrap_or(0);
    let mut tags = Tensor::typed(shape, dtype_for(peak));
    for (flat, &ring) in rings.iter().enumerate() {
        tags.put(flat, ring);
    }
    cell.cell.tags = Some(tags);
    cell
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::three::designs;
    use crate::two;
    #[test]
    fn exactly_24_orientations() {
        let table = orientations();
        assert_eq!(table.len(), 24);
        assert_eq!(table[0], (0, 0, 0));
    }
    #[test]
    fn orientations_preserve_sum_and_shape() {
        let c = designs::carpet(3, 1).unwrap();
        for i in 0..24 {
            let o = c.clone().orient(i).unwrap();
            assert_eq!(o.types().sum(), c.types().sum());
            assert_eq!(o.types().shape, c.types().shape);
        }
        assert!(c.clone().orient(24).is_err());
    }
    #[test]
    fn orientations_are_distinct_on_chiral_design() {
        let tree = designs::xtree(3, 1).unwrap();
        let images: Vec<Vec<u8>> = (0..24)
            .map(|i| tree.clone().orient(i).unwrap().types().bytes().to_vec())
            .collect();
        let mut unique = images.clone();
        unique.sort();
        unique.dedup();
        assert!(unique.len() >= 3);
    }
    #[test]
    fn special_identity_is_tile() {
        let c = designs::carpet(3, 1).unwrap();
        let mask = Tensor::new(vec![2, 2, 2]);
        let s = special(&mask, &c).unwrap();
        assert_eq!(s, c.clone().tile(2, 2, 2));
    }
    #[test]
    fn perforate_zero_mask_is_identity() {
        let c = designs::carpet(3, 1).unwrap();
        let mask = Tensor::new(c.types().shape.clone());
        let p = perforate(&mask, &c, 9).unwrap();
        assert_eq!(p, c);
    }
    #[test]
    fn only_the_carpet_and_two_trees_face_a_flat_name() {
        for n in [3, 5] {
            for level in [1, 2] {
                let carpet = slice(&designs::carpet(n, level).unwrap(), 2, 0).unwrap();
                let net = slice(&designs::net(n, level).unwrap(), 2, 0).unwrap();
                let xtree = slice(&designs::xtree(n, level).unwrap(), 2, 0).unwrap();
                let ytree = slice(&designs::ytree(n, level).unwrap(), 2, 0).unwrap();
                let ztree = slice(&designs::ztree(n, level).unwrap(), 2, 0).unwrap();
                let void = slice(&designs::void(n, level).unwrap(), 2, 0).unwrap();
                assert_eq!(carpet, two::carpet(n, level).unwrap());
                assert_eq!(xtree, two::vtree(n, level).unwrap());
                assert_eq!(ytree, two::htree(n, level).unwrap());
                assert_eq!(void, ztree);
                assert_ne!(net, two::net(n, level).unwrap());
                assert_ne!(void, two::void(n, level).unwrap());
                assert_ne!(ztree, two::htree(n, level).unwrap());
                assert_ne!(ztree, two::vtree(n, level).unwrap());
            }
        }
    }
    #[test]
    fn extrude_undoes_slice_on_every_axis() {
        use mrlycore::cell::mapping;
        use mrlycore::enums::Mode;
        let flat = two::carpet(3, 2)
            .unwrap()
            .layers()
            .paint(&mapping(), Mode::Index);
        for axis in 0..3 {
            let cube = extrude(&flat, axis, 4).unwrap();
            assert_eq!(cube.types().shape[axis], 4);
            for index in 0..4 {
                assert_eq!(slice(&cube, axis, index).unwrap(), flat);
            }
            assert_eq!(cube.types().sum(), 4 * flat.types().sum());
        }
        assert!(extrude(&flat, 3, 2).is_err());
        assert!(extrude(&flat, 0, 0).is_err());
        assert!(slice(&extrude(&flat, 2, 1).unwrap(), 3, 0).is_err());
        assert!(slice(&extrude(&flat, 2, 1).unwrap(), 2, 1).is_err());
    }
    #[test]
    fn extrude_lifts_a_flat_face_of_a_cube_back() {
        let sponge = designs::carpet(3, 2).unwrap();
        let face = slice(&sponge, 2, 0).unwrap();
        let column = extrude(&face, 2, sponge.depth()).unwrap();
        assert_eq!(column.types().shape, sponge.types().shape);
        assert_eq!(slice(&column, 2, 5).unwrap(), face);
    }
    #[test]
    fn extrude_carries_colors_and_tags() {
        use mrlycore::cell::mapping;
        use mrlycore::enums::Mode;
        let flat = two::carpet(3, 1)
            .unwrap()
            .layers()
            .paint(&mapping(), Mode::Type);
        let cube = extrude(&flat, 0, 3).unwrap();
        let colors = cube.cell.colors.as_ref().unwrap();
        let tags = cube.cell.tags.as_ref().unwrap();
        let flat_colors = flat.cell.colors.as_ref().unwrap();
        let flat_tags = flat.cell.tags.as_ref().unwrap();
        assert_eq!(colors.len(), 27);
        assert_eq!(tags.shape, vec![3, 3, 3]);
        for plane in 0..3 {
            for site in 0..9 {
                assert_eq!(colors[plane * 9 + site], flat_colors[site]);
                assert_eq!(tags.at(plane * 9 + site), flat_tags.at(site));
            }
        }
    }
    #[test]
    fn manhattan_layers_are_diamond_shells() {
        let cube = manhattan_layers(designs::ones(3, 1).unwrap());
        let tags = cube.cell.tags.as_ref().unwrap();
        assert_eq!(tags.get(&[1, 1, 1]), 0);
        assert_eq!(tags.get(&[0, 1, 1]), 1);
        assert_eq!(tags.get(&[0, 0, 1]), 2);
        assert_eq!(tags.get(&[0, 0, 0]), 3);
        let boxes = designs::ones(3, 1).unwrap().layers();
        assert_eq!(boxes.cell.tags.as_ref().unwrap().get(&[0, 0, 0]), 1);
    }
    #[test]
    fn manhattan_layers_widen_past_a_byte() {
        use mrlycore::tensor::Dtype;
        let small = manhattan_layers(designs::ones(3, 1).unwrap());
        assert_eq!(small.cell.tags.as_ref().unwrap().dtype(), Dtype::U8);
        let long = manhattan_layers(Cell3d::new(Tensor::full(vec![1, 1, 600], 1)));
        let tags = long.cell.tags.as_ref().unwrap();
        assert_eq!(tags.dtype(), Dtype::U16);
        assert_eq!(tags.at(0), 299);
    }
}
