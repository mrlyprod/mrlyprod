use super::models::Cell6d;
use super::{Orientation, Projection};
use crate::two;
use mrlycore::errors::{MrlyError, Result};
use serde_json::{json, Value};

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

pub fn to_json(cell: &Cell6d) -> String {
    let mut data: Value = serde_json::from_str(&two::to_json(&cell.cell)).unwrap();
    data["projection"] = json!(projection_name(cell.projection));
    data["orientation"] = json!(orientation_name(cell.orientation));
    data["start"] = json!(cell.start);
    data.to_string()
}

pub fn from_json(text: &str) -> Result<Cell6d> {
    let data: Value = serde_json::from_str(text).map_err(|e| MrlyError::Value(e.to_string()))?;
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
    Ok(Cell6d::new(inner, projection, orientation, start))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::six::designs::cut_design;
    #[test]
    fn json_round_trip() {
        let c = cut_design(23, 3, 1, 2).unwrap();
        let restored = from_json(&to_json(&c)).unwrap();
        assert_eq!(c, restored);
    }
}
