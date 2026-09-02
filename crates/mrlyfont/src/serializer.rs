use super::models::Glyph;
use super::names::name_of;
use mrlycore::Map;
use mrlycore::{json, Json};

/// Returns the glyph's rows as strings of '0' and '1'.
pub fn to_strings(glyph: &Glyph) -> Vec<String> {
    glyph.rows.clone()
}

/// Returns the glyph's rows as lists of 0 and 1 bytes.
pub fn to_lists(glyph: &Glyph) -> Vec<Vec<u8>> {
    glyph
        .rows
        .iter()
        .map(|row| row.chars().map(|ch| (ch == '1') as u8).collect())
        .collect()
}

/// Renders the glyphs to pretty JSON keyed by character, each entry carrying name, width, height and rows.
pub fn to_json(glyphs: &[Glyph]) -> String {
    let mut map = Map::new();
    for glyph in glyphs {
        let entry = json!({
            "name": name_of(glyph.char),
            "w": glyph.width(),
            "h": glyph.height(),
            "rows": glyph.rows.clone(),
        });
        map.insert(glyph.char.to_string(), entry);
    }
    serde_json::to_string_pretty(&Json::Object(map)).unwrap_or_default()
}
