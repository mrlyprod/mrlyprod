use super::glyphs::{DIGITS, EXTRAS, SPECIALS, UPPERS};
use super::models::Glyph;

pub fn lower(rows: &[&str]) -> Vec<String> {
    let mut grid: Vec<Vec<char>> = rows.iter().map(|row| row.chars().collect()).collect();
    let last = grid.len() - 1;
    let right = grid[0].len() - 1;
    for &y in &[0, last] {
        grid[y][0] = '0';
        grid[y][right] = '0';
    }
    grid.into_iter()
        .map(|row| row.into_iter().collect())
        .collect()
}

fn rows_of(rows: &[&str]) -> Vec<String> {
    rows.iter().map(|row| row.to_string()).collect()
}

pub fn uppers() -> Vec<Glyph> {
    UPPERS
        .iter()
        .map(|&(c, rows)| Glyph::new(c, rows_of(rows)))
        .collect()
}

pub fn lowers() -> Vec<Glyph> {
    UPPERS
        .iter()
        .map(|&(c, rows)| {
            let lowered = c.to_ascii_lowercase();
            Glyph::new(lowered, lower(rows))
        })
        .collect()
}

pub fn digits() -> Vec<Glyph> {
    DIGITS
        .iter()
        .map(|&(c, rows)| Glyph::new(c, rows_of(rows)))
        .collect()
}

pub fn extras() -> Vec<Glyph> {
    EXTRAS
        .iter()
        .map(|&(c, rows)| Glyph::new(c, rows_of(rows)))
        .collect()
}

pub fn specials() -> Vec<Glyph> {
    SPECIALS
        .iter()
        .map(|&(c, rows)| Glyph::new(c, rows_of(rows)))
        .collect()
}

pub fn all() -> Vec<Glyph> {
    let mut glyphs = uppers();
    glyphs.extend(lowers());
    glyphs.extend(digits());
    glyphs.extend(extras());
    glyphs.extend(specials());
    glyphs
}
