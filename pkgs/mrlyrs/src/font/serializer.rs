use super::models::Glyph;
use super::names::name_of;
use serde_json::{json, Map, Value};

pub fn to_strings(glyph: &Glyph) -> Vec<String> {
    glyph.rows.clone()
}

pub fn to_lists(glyph: &Glyph) -> Vec<Vec<u8>> {
    glyph
        .rows
        .iter()
        .map(|row| row.chars().map(|ch| (ch == '1') as u8).collect())
        .collect()
}

pub fn to_json(glyphs: &[Glyph]) -> String {
    let mut map = Map::new();
    for glyph in glyphs {
        let entry = json!({
            "name": name_of(glyph.char),
            "w": glyph.width(),
            "h": glyph.height(),
            "rows": glyph.rows,
        });
        map.insert(glyph.char.to_string(), entry);
    }
    serde_json::to_string_pretty(&Value::Object(map)).unwrap()
}
