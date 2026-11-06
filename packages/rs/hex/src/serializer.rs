use super::models::CellHex;
use super::{Orientation, Projection};
use mrly_error::Result;
use mrly_json::json;
use mrly_math::cell::serializer::parse;
use mrly_two as two;

fn projection_name(p: Projection) -> &'static str {
    match p {
        Projection::Iso => "iso",
        Projection::Pro => "pro",
        Projection::Cut => "cut",
    }
}

fn orientation_name(o: Orientation) -> &'static str {
    match o {
        Orientation::Horizontal => "horizontal",
        Orientation::Vertical => "vertical",
    }
}

pub fn to_json(cell: &CellHex) -> String {
    let mut data = parse(&two::to_json(&cell.cell)).unwrap();
    data["projection"] = json!(projection_name(cell.projection));
    data["orientation"] = json!(orientation_name(cell.orientation));
    data["start"] = json!(cell.start);
    data.to_string()
}

pub fn from_json(text: &str) -> Result<CellHex> {
    let data = parse(text)?;
    let inner = two::from_json(text)?;
    let projection = match data
        .get("projection")
        .and_then(|v| v.as_str())
        .unwrap_or("iso")
    {
        "pro" => Projection::Pro,
        "cut" => Projection::Cut,
        _ => Projection::Iso,
    };
    let orientation = match data
        .get("orientation")
        .and_then(|v| v.as_str())
        .unwrap_or("vertical")
    {
        "horizontal" => Orientation::Horizontal,
        _ => Orientation::Vertical,
    };
    let start = data.get("start").and_then(|v| v.as_u64()).unwrap_or(1) as u8;
    Ok(CellHex::new(inner, projection, orientation, start))
}
