use std::collections::HashMap;

pub fn push_glyph(row: &mut String, value: u8, glyphs: Option<&HashMap<u8, char>>) {
    match glyphs.and_then(|g| g.get(&value)) {
        Some(&ch) => row.push(ch),
        None => row.push_str(&value.to_string()),
    }
}
