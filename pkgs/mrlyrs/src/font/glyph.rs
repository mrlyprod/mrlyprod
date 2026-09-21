use super::bitmaps::{DIGITS, EXTRAS, SPECIALS, UPPERS};

/// One character's pixel bitmap.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Glyph {
    /// The character the glyph draws.
    pub char: char,
    /// The bitmap rows of '0' and '1' characters.
    pub rows: Vec<String>,
}

impl Glyph {
    /// Builds a glyph from its character and rows.
    pub fn new(char: char, rows: Vec<String>) -> Glyph {
        Glyph { char, rows }
    }
    /// Returns the cell width of the first row, or 0 for an empty glyph.
    pub fn width(&self) -> usize {
        self.rows.first().map_or(0, |row| row.chars().count())
    }
    /// Returns the number of rows.
    pub fn height(&self) -> usize {
        self.rows.len()
    }
}

/// The characters that dip below the five-row baseline.
pub(crate) const DESCENDERS: &[char] = &[
    '@', '$', '\u{00a9}', '\u{00ae}', '(', ')', '[', ']', '{', '}',
];

/// Returns whether the character dips below the baseline.
pub(crate) fn descends(c: char) -> bool {
    DESCENDERS.contains(&c)
}

/// Cuts blank edge columns from a bitmap, collapsing an all-blank one to a single '0' column.
pub fn trim(rows: &[String]) -> Vec<String> {
    let grid: Vec<Vec<char>> = rows.iter().map(|row| row.chars().collect()).collect();
    if grid.is_empty() || grid[0].is_empty() {
        return rows.to_vec();
    }
    let width = grid[0].len();
    let lit = |col: usize| grid.iter().any(|row| row[col] == '1');
    let Some(start) = (0..width).find(|&col| lit(col)) else {
        return grid.iter().map(|_| "0".to_string()).collect();
    };
    let end = (0..width).rev().find(|&col| lit(col)).unwrap();
    grid.iter()
        .map(|row| row[start..=end].iter().collect())
        .collect()
}

/// Blanks the four corner cells of an uppercase bitmap into its rounded lowercase form.
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

/// Builds the twenty-six uppercase glyphs.
pub fn uppers() -> Vec<Glyph> {
    UPPERS
        .iter()
        .map(|&(c, rows)| Glyph::new(c, rows_of(rows)))
        .collect()
}

/// Builds the twenty-six lowercase glyphs by rounding the uppers' corners.
pub fn lowers() -> Vec<Glyph> {
    UPPERS
        .iter()
        .map(|&(c, rows)| {
            let lowered = c.to_ascii_lowercase();
            Glyph::new(lowered, lower(rows))
        })
        .collect()
}

/// Builds the ten digit glyphs.
pub fn digits() -> Vec<Glyph> {
    DIGITS
        .iter()
        .map(|&(c, rows)| Glyph::new(c, rows_of(rows)))
        .collect()
}

/// Builds the punctuation, symbol and arrow glyphs.
pub fn extras() -> Vec<Glyph> {
    EXTRAS
        .iter()
        .map(|&(c, rows)| Glyph::new(c, rows_of(rows)))
        .collect()
}

/// Builds the four seven-row glyphs: dollar, at, copyright and registered.
pub fn specials() -> Vec<Glyph> {
    SPECIALS
        .iter()
        .map(|&(c, rows)| Glyph::new(c, rows_of(rows)))
        .collect()
}

/// Builds every glyph in font order: uppers, lowers, digits, extras, specials.
pub fn all() -> Vec<Glyph> {
    let mut glyphs = uppers();
    glyphs.extend(lowers());
    glyphs.extend(digits());
    glyphs.extend(extras());
    glyphs.extend(specials());
    glyphs
}
