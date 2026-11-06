use super::Cell3d;
use mrly_error::{Result, value_error};
use mrly_math::cell::base::remap;
use mrly_math::cell::geometry;
use mrly_math::cell::models::{Cell2d, dtype_for};
use mrly_tensor::Tensor;

pub use mrly_math::cell::geometry::{magic, mosaic, perforate};
pub use mrly_math::cell::models::orientations;

pub fn merge(cells: &[Cell3d], width: usize, height: usize, depth: usize) -> Result<Cell3d> {
    geometry::merge_reps(cells, &[depth, height, width])
}

pub fn special(mask: &Tensor, cell: &Cell3d) -> Result<Cell3d> {
    if mask.shape.len() != 3 {
        return value_error("special mask must be 3d.");
    }
    if mask.bytes()?.iter().any(|&v| v > 23) {
        return value_error("Invalid orientation value. Must be 0..23.");
    }
    let oriented: Result<Vec<Cell3d>> = mask
        .bytes()?
        .iter()
        .map(|&k| cell.clone().orient(k as usize))
        .collect();
    let oriented = oriented?;
    merge(&oriented, mask.shape[2], mask.shape[1], mask.shape[0])
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

pub fn slice(cell: &Cell3d, axis: usize, index: usize) -> Result<Cell2d> {
    if axis > 2 {
        return value_error("slice axis is past the cube's rank.");
    }
    if index >= cell.types().shape[axis] {
        return value_error("slice index is past the axis.");
    }
    let (shape, map) = slice_map(&cell.types().shape, axis, index);
    Ok(Cell2d {
        cell: remap(&cell.cell, &map, &shape)?,
    })
}

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

pub fn extrude(cell: &Cell2d, axis: usize, depth: usize) -> Result<Cell3d> {
    if axis > 2 {
        return value_error("extrude axis must be 0, 1 or 2.");
    }
    if depth == 0 {
        return value_error("extrude depth must be at least 1.");
    }
    let (shape, map) = lift_map(&cell.types().shape, axis, depth);
    Ok(Cell3d {
        cell: remap(&cell.cell, &map, &shape)?,
    })
}

pub fn extrude_cube(cell: &Cell3d, depth: usize) -> Result<Cell3d> {
    if depth == 0 {
        return value_error("extrude depth must be at least 1.");
    }
    let shape = &cell.types().shape;
    let plane = shape[1] * shape[2];
    let stacked = vec![shape[0] * depth, shape[1], shape[2]];
    let map: Vec<usize> = (0..stacked[0] * plane)
        .map(|flat| (flat / plane / depth) * plane + flat % plane)
        .collect();
    Ok(Cell3d {
        cell: remap(&cell.cell, &map, &stacked)?,
    })
}

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
