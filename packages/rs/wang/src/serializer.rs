use crate::designs::from_tags;
use crate::models::{CellWang, Result};
use std::str::Chars;

pub fn from_strings(rows: &[&str]) -> Result<CellWang> {
    let width = rows.first().map_or(0, |row| row.len());
    if rows.iter().any(|row| row.len() != width) {
        return Err("all rows must have the same length.".to_string());
    }
    let mut tags = Vec::with_capacity(width * rows.len());
    for glyph in rows.iter().flat_map(|row| row.chars()) {
        match glyph.to_digit(16) {
            Some(value) => tags.push(value as u8),
            None => return Err(format!("'{glyph}' is not a hex digit.")),
        }
    }
    from_tags(width, rows.len(), &tags)
}

fn label(glyph: Option<char>) -> Result<u8> {
    match glyph.and_then(|g| g.to_digit(36)) {
        Some(value) => Ok(value as u8),
        None => Err(format!("{glyph:?} is not a tag.")),
    }
}

fn next(chars: &mut Chars<'_>) -> Result<u8> {
    label(chars.next())
}

pub fn from_tokens(width: usize, height: usize, data: &str) -> Result<CellWang> {
    let mut chars = data.chars();
    let mut cells = Vec::with_capacity(width * height);
    while let Some(glyph) = chars.next() {
        cells.push(match glyph {
            '/' => {
                let (a, b) = (next(&mut chars)?, next(&mut chars)?);
                [a, b, b, a]
            }
            '\\' => {
                let (a, b) = (next(&mut chars)?, next(&mut chars)?);
                [a, a, b, b]
            }
            '(' => {
                let sides = [
                    next(&mut chars)?,
                    next(&mut chars)?,
                    next(&mut chars)?,
                    next(&mut chars)?,
                ];
                if chars.next() != Some(')') {
                    return Err("a quartered cell must end with ).".to_string());
                }
                sides
            }
            whole => [label(Some(whole))?; 4],
        });
    }
    if cells.len() != width * height {
        return Err(format!(
            "{width}x{height} wants {} cells, got {}.",
            width * height,
            cells.len()
        ));
    }
    let mut out = CellWang::new(width, height);
    for (flat, &sides) in cells.iter().enumerate() {
        out.put(flat % width, flat / width, sides);
    }
    Ok(out)
}
