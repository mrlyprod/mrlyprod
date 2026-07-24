use super::models::Cell3d;
use crate::dim::serializer::{field, parse, types_field};
use mrlycore::errors::{value_error, Result};
use mrlycore::tensor::Tensor;
use serde_json::json;

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

pub fn to_json(cell: &Cell3d) -> String {
    let shape = &cell.types().shape;
    json!({
        "v": 1,
        "height": shape[0],
        "width": shape[1],
        "depth": shape[2],
        "types": to_lists(cell),
    })
    .to_string()
}

pub fn from_json(text: &str) -> Result<Cell3d> {
    let data = parse(text)?;
    let lists: Vec<Vec<Vec<u8>>> = field(types_field(&data)?)?;
    from_lists(&lists)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::three::designs;
    #[test]
    fn json_round_trip() {
        let c = designs::carpet(3, 2).unwrap();
        assert_eq!(from_json(&to_json(&c)).unwrap(), c);
    }
    #[test]
    fn lists_round_trip() {
        let c = designs::void(4, 1).unwrap();
        assert_eq!(from_lists(&to_lists(&c)).unwrap(), c);
    }
}
