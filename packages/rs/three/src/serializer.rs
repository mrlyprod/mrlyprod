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
        "depth": shape[0],
        "height": shape[1],
        "width": shape[2],
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
    to_lists(cell)
        .iter()
        .map(|plane| {
            plane
                .iter()
                .map(|row| row.iter().map(|site| site.to_string()).collect())
                .collect()
        })
        .collect()
}

pub fn from_strings(data: &[Vec<String>]) -> Result<Cell3d> {
    let mut lists = Vec::with_capacity(data.len());
    for plane in data {
        let mut rows = Vec::with_capacity(plane.len());
        for row in plane {
            let mut sites = Vec::with_capacity(row.len());
            for site in row.chars() {
                match site.to_digit(10) {
                    Some(value) => sites.push(value as u8),
                    None => return value_error(format!("'{site}' is not a digit.")),
                }
            }
            rows.push(sites);
        }
        lists.push(rows);
    }
    from_lists(&lists)
}
