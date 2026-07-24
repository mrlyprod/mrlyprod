use super::models::Cell2d;
use crate::dim::serializer::{field, parse, types_field};
use mrlycore::errors::{value_error, MrlyError, Result};
use mrlycore::tensor::Tensor;
use serde_json::json;

pub fn to_lists(cell: &Cell2d) -> Vec<Vec<u8>> {
    let (h, w) = (cell.height(), cell.width());
    (0..h)
        .map(|y| (0..w).map(|x| cell.types().get(&[y, x])).collect())
        .collect()
}

pub fn from_lists(lists: &[Vec<u8>]) -> Result<Cell2d> {
    if lists.is_empty() {
        return value_error("cannot build a cell from an empty list.");
    }
    let (h, w) = (lists.len(), lists[0].len());
    if lists.iter().any(|row| row.len() != w) {
        return value_error("all rows must have the same length.");
    }
    let data: Vec<u8> = lists.iter().flatten().copied().collect();
    Ok(Cell2d::new(Tensor::of(data, vec![h, w])))
}

pub fn to_strings(cell: &Cell2d) -> Vec<String> {
    to_lists(cell)
        .iter()
        .map(|row| row.iter().map(|v| v.to_string()).collect())
        .collect()
}

pub fn from_strings(rows: &[String]) -> Result<Cell2d> {
    let lists: Result<Vec<Vec<u8>>> = rows
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
        .collect();
    from_lists(&lists?)
}

pub fn to_json(cell: &Cell2d) -> String {
    let mut data = json!({
        "v": 1,
        "width": cell.width(),
        "height": cell.height(),
        "types": to_lists(cell),
    });
    if let Some(colors) = &cell.cell.colors {
        let w = cell.width();
        let nested: Vec<Vec<[u8; 4]>> = colors.chunks(w).map(|row| row.to_vec()).collect();
        data["colors"] = json!(nested);
    }
    if let Some(tags) = &cell.cell.tags {
        let (h, w) = (cell.height(), cell.width());
        let nested: Vec<Vec<i64>> = (0..h)
            .map(|r| (0..w).map(|c| tags.at(r * w + c)).collect())
            .collect();
        data["tags"] = json!(nested);
    }
    data.to_string()
}

pub fn from_json(text: &str) -> Result<Cell2d> {
    let data = parse(text)?;
    let lists: Vec<Vec<u8>> = field(types_field(&data)?)?;
    let mut cell = from_lists(&lists)?;
    if let Some(colors) = data.get("colors") {
        let nested: Vec<Vec<[u8; 4]>> = field(colors)?;
        cell.cell.colors = Some(nested.into_iter().flatten().collect());
    }
    if let Some(tags) = data.get("tags") {
        let nested: Vec<Vec<u8>> = field(tags)?;
        let shape = vec![cell.height(), cell.width()];
        cell.cell.tags = Some(Tensor::of(nested.into_iter().flatten().collect(), shape));
    }
    Ok(cell)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::two::designs;
    use mrlycore::cell::mapping;
    use mrlycore::enums::Mode;
    #[test]
    fn json_round_trip() {
        let c = designs::carpet(3, 2).unwrap();
        let restored = from_json(&to_json(&c)).unwrap();
        assert_eq!(c, restored);
    }
    #[test]
    fn json_round_trip_with_colors_and_tags() {
        let c = designs::carpet(3, 1)
            .unwrap()
            .layers()
            .paint(&mapping(), Mode::Type);
        let restored = from_json(&to_json(&c)).unwrap();
        assert_eq!(c, restored);
    }
    #[test]
    fn strings_round_trip() {
        let c = designs::void(4, 1).unwrap();
        assert_eq!(from_strings(&to_strings(&c)).unwrap(), c);
        assert!(from_strings(&["1x1".to_string()]).is_err());
    }
}
