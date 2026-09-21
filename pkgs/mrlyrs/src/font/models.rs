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
