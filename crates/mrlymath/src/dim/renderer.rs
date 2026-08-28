use std::collections::HashMap;

/// Appends the value's glyph to the row, or its digits when no glyph is mapped.
pub fn push_glyph(row: &mut String, value: u8, glyphs: Option<&HashMap<u8, char>>) {
    match glyphs.and_then(|g| g.get(&value)) {
        Some(&ch) => row.push(ch),
        None => row.push_str(&value.to_string()),
    }
}
