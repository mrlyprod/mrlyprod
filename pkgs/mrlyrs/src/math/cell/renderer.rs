use std::collections::HashMap;

/// Appends the value's glyph to the row, or its digits when no glyph is mapped.
pub fn push_glyph(row: &mut String, value: u8, glyphs: Option<&HashMap<u8, String>>) {
    match glyphs.and_then(|g| g.get(&value)) {
        Some(glyph) => row.push_str(glyph),
        None => row.push_str(&value.to_string()),
    }
}
