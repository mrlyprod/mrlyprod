use super::Cell3d;
use crate::core::error::{value_error, Result};
use crate::core::tensor::Tensor;
use crate::core::{json, Json};
use crate::math::cell::serializer::{byte_cube, count_cube, parse, tag_layer, types_field};
use serde::Deserialize;

/// Unrolls the cell into nested lists, plane by row by site.
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

/// Builds a cell from nested lists, or an error when they are empty or ragged.
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
    Ok(Vec::deserialize(value)?)
}

fn planes<T: Clone>(flat: &[T], shape: &[usize]) -> Vec<Vec<Vec<T>>> {
    flat.chunks(shape[1] * shape[2])
        .map(|plane| plane.chunks(shape[2]).map(<[T]>::to_vec).collect())
        .collect()
}

/// Serializes the cell's shape and types to JSON, with colors and tags when present.
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

/// Parses a cell from its JSON, colors and tags included.
///
/// # Errors
///
/// Errors when the text is not a cube.
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

/// Unrolls the cube into one string of digits per row, grouped plane by plane.
///
/// ```
/// let rows = mrlyrs::math::three::to_strings(&mrlyrs::math::three::carpet(3, 1).unwrap());
/// assert_eq!(rows[0], ["111", "101", "111"]);
/// ```
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

/// Builds a cube from one string of digits per row, grouped plane by plane.
///
/// # Errors
///
/// Errors when the planes are empty, ragged, or hold anything but digits.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::cell::mapping;
    use crate::core::cell::Mode;
    use crate::math::three::designs;
    #[test]
    fn json_round_trip_with_colors_and_tags() {
        let c = designs::carpet(3, 1)
            .unwrap()
            .layers()
            .paint(&mapping(), Mode::Type, None)
            .unwrap();
        let restored = from_json(&to_json(&c)).unwrap();
        assert_eq!(c, restored);
        assert!(restored.cell.colors.is_some());
        assert!(restored.cell.tags.is_some());
    }
    #[test]
    fn json_round_trip_with_tags_past_a_byte() {
        use crate::core::tensor::Dtype;
        use crate::math::three::manhattan_layers;
        let long = manhattan_layers(Cell3d::new(Tensor::full(vec![1, 1, 600], 1)).unwrap());
        let tags = long.cell.tags.as_ref().unwrap();
        assert_eq!(tags.dtype(), Dtype::U16);
        assert_eq!(tags.at(0), 299);
        let restored = from_json(&to_json(&long)).unwrap();
        assert_eq!(restored, long);
        assert_eq!(restored.cell.tags.as_ref().unwrap().at(0), 299);
    }
    #[test]
    fn strings_round_trip_the_sponge() {
        let c = designs::carpet(3, 1).unwrap();
        let rows = to_strings(&c);
        assert_eq!(rows[0], ["111", "101", "111"]);
        assert_eq!(from_strings(&rows).unwrap(), c);
    }

    #[test]
    fn lists_round_trip() {
        let c = designs::void(4, 1).unwrap();
        assert_eq!(from_lists(&to_lists(&c)).unwrap(), c);
    }
}
