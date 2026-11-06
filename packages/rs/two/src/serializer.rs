use super::Cell2d;
use mrly_error::{Result, value_error};
use mrly_json::json;
use mrly_math::cell::serializer::{
    byte_grid, color_grid, count_grid, parse, tag_layer, types_field,
};
use mrly_tensor::Tensor;

fn to_lists(cell: &Cell2d) -> Vec<Vec<u8>> {
    let (h, w) = (cell.height(), cell.width());
    (0..h)
        .map(|y| (0..w).map(|x| cell.types().at(y * w + x) as u8).collect())
        .collect()
}

fn from_lists(lists: &[Vec<u8>]) -> Result<Cell2d> {
    if lists.is_empty() {
        return value_error("cannot build a cell from an empty list.");
    }
    let (h, w) = (lists.len(), lists[0].len());
    if lists.iter().any(|row| row.len() != w) {
        return value_error("all rows must have the same length.");
    }
    let data: Vec<u8> = lists.iter().flatten().copied().collect();
    Cell2d::new(Tensor::of(data, vec![h, w])?)
}

pub fn to_json(cell: &Cell2d) -> String {
    let mut data = json!({
        "v": 1,
        "width": cell.width(),
        "height": cell.height(),
        "types": to_lists(cell),
    });
    if let Some(colors) = &cell.cell.colors {
        data["colors"] = json!(colors.chunks(cell.width()).collect::<Vec<_>>());
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
    let lists = byte_grid(types_field(&data)?)?;
    let mut cell = from_lists(&lists)?;
    if let Some(colors) = data.get("colors") {
        let nested = color_grid(colors)?;
        cell.cell.colors = Some(nested.into_iter().flatten().collect());
    }
    if let Some(tags) = data.get("tags") {
        let shape = vec![cell.height(), cell.width()];
        cell.cell.tags = Some(tag_layer(&count_grid(tags)?, shape)?);
    }
    Ok(cell)
}

pub fn from_strings(rows: &[String]) -> Result<Cell2d> {
    let mut lists = Vec::with_capacity(rows.len());
    for row in rows {
        let mut digits = Vec::with_capacity(row.len());
        for glyph in row.chars() {
            match glyph.to_digit(10) {
                Some(d) => digits.push(d as u8),
                None => return value_error(format!("'{glyph}' is not a digit.")),
            }
        }
        lists.push(digits);
    }
    from_lists(&lists)
}
