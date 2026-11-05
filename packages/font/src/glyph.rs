use super::bitmaps::{DIGITS, EXTRAS, SPECIALS, UPPERS};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Glyph {
    pub char: char,
    pub rows: Vec<String>,
}

impl Glyph {
    pub fn new(char: char, rows: Vec<String>) -> Glyph {
        Glyph { char, rows }
    }
    pub fn width(&self) -> usize {
        self.rows.first().map_or(0, |row| row.chars().count())
    }
    pub fn height(&self) -> usize {
        self.rows.len()
    }
}

pub(crate) const DESCENDERS: &[char] = &[
    '@', '$', '\u{00a9}', '\u{00ae}', '(', ')', '[', ']', '{', '}',
];

pub(crate) fn descends(c: char) -> bool {
    DESCENDERS.contains(&c)
}

pub fn trim(rows: &[String]) -> Vec<String> {
    let grid: Vec<Vec<char>> = rows.iter().map(|row| row.chars().collect()).collect();
    let width = grid.iter().map(Vec::len).max().unwrap_or(0);
    if width == 0 {
        return rows.to_vec();
    }
    let lit = |col: usize| grid.iter().any(|row| row.get(col) == Some(&'1'));
    let (Some(start), Some(end)) = (
        (0..width).find(|&col| lit(col)),
        (0..width).rev().find(|&col| lit(col)),
    ) else {
        return grid.iter().map(|_| "0".to_string()).collect();
    };
    grid.iter()
        .map(|row| row.iter().skip(start).take(end + 1 - start).collect())
        .collect()
}

pub fn lower(rows: &[&str]) -> Vec<String> {
    let mut grid: Vec<Vec<char>> = rows.iter().map(|row| row.chars().collect()).collect();
    let last = grid.len().saturating_sub(1);
    for y in [0, last] {
        if let Some(row) = grid.get_mut(y) {
            if let Some(first) = row.first_mut() {
                *first = '0';
            }
            if let Some(end) = row.last_mut() {
                *end = '0';
            }
        }
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
