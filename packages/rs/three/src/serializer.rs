use super::Cell3d;
use mrly_error::{Result, value_error};
use mrly_json::{Json, json};
use mrly_math::cell::serializer::{byte_cube, count_cube, parse, tag_layer, types_field};
use mrly_tensor::Tensor;

fn to_lists(cell: &Cell3d) -> Vec<Vec<Vec<u8>>> {
    let types = cell.types();
    let shape = &types.shape;
    (0..shape[0])
        .map(|i| {
            (0..shape[1])
                .map(|j| {
                    (0..shape[2])
                        .map(|k| types.at(types.index(&[i, j, k])) as u8)
                        .collect()
                })
                .collect()
        })
        .collect()
}

fn from_lists(lists: &[Vec<Vec<u8>>]) -> Result<Cell3d> {
    if lists.is_empty() || lists[0].is_empty() || lists[0][0].is_empty() {
        return value_error("cannot build a cell from an empty list.");
    }
    let (a, b, c) = (lists.len(), lists[0].len(), lists[0][0].len());
    for plane in lists {
        if plane.len() != b || plane.iter().any(|row| row.len() != c) {
            return value_error("all planes and rows must have the same lengths.");
        }
    }
    let data: Vec<u8> = lists.iter().flatten().flatten().copied().collect();
    Cell3d::new(Tensor::of(data, vec![a, b, c])?)
}

fn color_cube(value: &Json) -> Result<Vec<Vec<Vec<[u8; 4]>>>> {
    mrly_json::take(value)
}

fn planes<T: Clone>(flat: &[T], shape: &[usize]) -> Vec<Vec<Vec<T>>> {
    flat.chunks(shape[1] * shape[2])
        .map(|plane| plane.chunks(shape[2]).map(<[T]>::to_vec).collect())
        .collect()
}

pub fn to_json(cell: &Cell3d) -> String {
    let shape = &cell.types().shape;
    let mut data = json!({
        "v": 1,
        "width": shape[0],
        "height": shape[1],
        "depth": shape[2],
        "types": to_lists(cell),
    });
    if let Some(colors) = &cell.cell.colors {
        data["colors"] = json!(planes(colors, shape));
    }
    if let Some(tags) = &cell.cell.tags {
        let flat: Vec<i64> = (0..tags.size()).map(|at| tags.at(at)).collect();
        data["tags"] = json!(planes(&flat, shape));
    }
    data.to_string()
}

pub fn from_json(text: &str) -> Result<Cell3d> {
    let data = parse(text)?;
    let lists = byte_cube(types_field(&data)?)?;
    let mut cell = from_lists(&lists)?;
    if let Some(colors) = data.get("colors") {
        let nested = color_cube(colors)?;
        cell.cell.colors = Some(nested.into_iter().flatten().flatten().collect());
    }
    if let Some(tags) = data.get("tags") {
        let shape = cell.types().shape.clone();
        cell.cell.tags = Some(tag_layer(&count_cube(tags)?, shape)?);
    }
    Ok(cell)
}

pub fn to_strings(cell: &Cell3d) -> Vec<Vec<String>> {
    let types = cell.types();
    let (width, height, depth) = (types.shape[0], types.shape[1], types.shape[2]);
    (0..depth)
        .map(|z| {
            (0..height)
                .rev()
                .map(|y| {
                    (0..width)
                        .map(|x| (types.at(types.index(&[x, y, z])) as u8).to_string())
                        .collect()
                })
                .collect()
        })
        .collect()
}

pub fn from_strings(data: &[Vec<String>]) -> Result<Cell3d> {
    let (depth, height) = (data.len(), data.first().map_or(0, Vec::len));
    let width = data
        .first()
        .and_then(|plane| plane.first())
        .map_or(0, |row| row.chars().count());
    if depth == 0 || height == 0 || width == 0 {
        return value_error("cannot build a cell from an empty list.");
    }
    let mut types = vec![0u8; width * height * depth];
    for (z, plane) in data.iter().enumerate() {
        if plane.len() != height || plane.iter().any(|row| row.chars().count() != width) {
            return value_error("all planes and rows must have the same lengths.");
        }
        for (row, line) in plane.iter().enumerate() {
            let y = height - 1 - row;
            for (x, site) in line.chars().enumerate() {
                match site.to_digit(10) {
                    Some(value) => types[(x * height + y) * depth + z] = value as u8,
                    None => return value_error(format!("'{site}' is not a digit.")),
                }
            }
        }
    }
    Cell3d::new(Tensor::of(types, vec![width, height, depth])?)
}
