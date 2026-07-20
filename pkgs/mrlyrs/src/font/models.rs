#[derive(Clone, Debug, PartialEq, Eq)]
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
