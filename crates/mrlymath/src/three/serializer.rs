use super::models::Cell3d;
use crate::dim::serializer::{byte_cube, count_cube, parse, tag_layer, types_field};
use mrlycore::errors::{value_error, MrlyError, Result};
use mrlycore::tensor::Tensor;
use mrlycore::{json, Json};
use serde::Deserialize;

/// Unrolls the cell into nested lists, plane by row by site.
pub fn to_lists(cell: &Cell3d) -> Vec<Vec<Vec<u8>>> {
    let shape = &cell.types().shape;
    (0..shape[0])
        .map(|i| {
            (0..shape[1])
                .map(|j| {
                    (0..shape[2])
                        .map(|k| cell.types().get(&[i, j, k]))
                        .collect()
                })
                .collect()
        })
        .collect()
}

/// Builds a cell from nested lists, or an error when they are empty or ragged.
pub fn from_lists(lists: &[Vec<Vec<u8>>]) -> Result<Cell3d> {
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
    Ok(Cell3d::new(Tensor::of(data, vec![a, b, c])))
}

/// Returns the cell's types as planes of digit-string rows.
pub fn to_strings(cell: &Cell3d) -> Vec<Vec<String>> {
    to_lists(cell)
        .iter()
        .map(|plane| {
            plane
                .iter()
                .map(|row| row.iter().map(|v| v.to_string()).collect())
                .collect()
        })
        .collect()
}

/// Builds a cell from planes of digit-string rows, or an error at any non-digit.
///
/// ```
/// let planes = vec![vec!["11".to_string(), "10".to_string()]];
/// let cell = mrlymath::three::from_strings(&planes).unwrap();
/// assert_eq!(cell.types().sum(), 3);
/// ```
pub fn from_strings(planes: &[Vec<String>]) -> Result<Cell3d> {
    let lists: Result<Vec<Vec<Vec<u8>>>> = planes
        .iter()
        .map(|plane| {
            plane
                .iter()
                .map(|row| {
                    row.chars()
                        .map(|ch| {
                            ch.to_digit(10)
                                .map(|d| d as u8)
                                .ok_or_else(|| MrlyError::Value(format!("invalid digit {ch:?}.")))
                        })
                        .collect()
                })
                .collect()
        })
        .collect();
    from_lists(&lists?)
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
        "height": shape[0],
        "width": shape[1],
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

/// Parses a cell from its JSON, colors and tags included, or a parse error.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::three::designs;
    use mrlycore::cell::mapping;
    use mrlycore::enums::Mode;
    #[test]
    fn json_round_trip_with_colors_and_tags() {
        let c = designs::carpet(3, 1)
            .unwrap()
            .layers()
            .paint(&mapping(), Mode::Type);
        let restored = from_json(&to_json(&c)).unwrap();
        assert_eq!(c, restored);
        assert!(restored.cell.colors.is_some());
        assert!(restored.cell.tags.is_some());
    }
    #[test]
    fn json_round_trip_with_tags_past_a_byte() {
        use crate::three::manhattan_layers;
        use mrlycore::tensor::Dtype;
        let long = manhattan_layers(Cell3d::new(Tensor::full(vec![1, 1, 600], 1)));
        let tags = long.cell.tags.as_ref().unwrap();
        assert_eq!(tags.dtype(), Dtype::U16);
        assert_eq!(tags.at(0), 299);
        let restored = from_json(&to_json(&long)).unwrap();
        assert_eq!(restored, long);
        assert_eq!(restored.cell.tags.as_ref().unwrap().at(0), 299);
    }
    #[test]
    fn lists_round_trip() {
        let c = designs::void(4, 1).unwrap();
        assert_eq!(from_lists(&to_lists(&c)).unwrap(), c);
    }
    #[test]
    fn strings_round_trip() {
        let c = designs::net(3, 2).unwrap();
        assert_eq!(from_strings(&to_strings(&c)).unwrap(), c);
        assert_eq!(to_strings(&designs::ones(2, 1).unwrap())[0], ["11", "11"]);
        assert!(from_strings(&[vec!["1x1".to_string()]]).is_err());
    }
}
