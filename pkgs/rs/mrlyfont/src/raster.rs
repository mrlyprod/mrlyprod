use super::{descends, glyph, trim};

pub(crate) struct Block {
    /// The character the block draws.
    pub char: char,
    /// The glyph rows, top to bottom.
    pub rows: Vec<String>,
    /// The column the block starts at.
    pub col: usize,
    /// The rows the block sits below the baseline.
    pub offset: usize,
}

impl Block {
    /// Reports the block width in columns.
    pub fn width(&self) -> usize {
        self.rows[0].len()
    }
}

pub(crate) struct Layout {
    /// The height in rows.
    pub height: usize,
    /// The width in columns.
    pub width: usize,
    /// The placed blocks, in reading order.
    pub blocks: Vec<Block>,
}

pub(crate) fn layout(text: &str) -> Layout {
    if text.is_empty() {
        return Layout {
            height: 0,
            width: 0,
            blocks: Vec::new(),
        };
    }
    let height = if text.chars().any(descends) { 7 } else { 5 };
    let shapes: Vec<(char, Vec<String>)> = text
        .chars()
        .map(|c| match glyph(c) {
            Some(g) if c != ' ' => (c, trim(&g.rows)),
            _ => (c, vec!["000".to_string(); 5]),
        })
        .collect();
    let width = shapes.iter().map(|s| s.1[0].len()).sum::<usize>() + shapes.len() - 1;
    let mut blocks = Vec::new();
    let mut col = 0;
    for (c, rows) in shapes {
        let offset = (height - rows.len()) / 2;
        let step = rows[0].len() + 1;
        blocks.push(Block {
            char: c,
            rows,
            col,
            offset,
        });
        col += step;
    }
    Layout {
        height,
        width,
        blocks,
    }
}

/// Returns the text as a 0/1 grid, its trimmed glyphs one blank column apart.
///
/// ```
/// assert_eq!(mrlyfont::raster("42").len(), 5);
/// assert_eq!(mrlyfont::raster("(1)").len(), 7);
/// ```
pub fn raster(text: &str) -> Vec<Vec<u8>> {
    let laid = layout(text);
    if laid.blocks.is_empty() {
        return Vec::new();
    }
    let mut grid = vec![vec![0u8; laid.width]; laid.height];
    for block in &laid.blocks {
        for (r, row) in block.rows.iter().enumerate() {
            for (c, ch) in row.chars().enumerate() {
                if ch == '1' {
                    grid[block.offset + r][block.col + c] = 1;
                }
            }
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digits_string_is_five_tall_with_gapped_width() {
        let rows = raster("42");
        assert_eq!(rows.len(), 5);
        let w4 = trim(&glyph('4').unwrap().rows)[0].len();
        let w2 = trim(&glyph('2').unwrap().rows)[0].len();
        assert_eq!(rows[0].len(), w4 + w2 + 1);
    }

    #[test]
    fn descender_makes_the_grid_seven_tall() {
        assert_eq!(raster("(1)").len(), 7);
    }

    #[test]
    fn five_row_glyph_straddles_in_a_seven_row_grid() {
        let rows = raster("(1)");
        let w_paren = trim(&glyph('(').unwrap().rows)[0].len();
        let w_one = trim(&glyph('1').unwrap().rows)[0].len();
        let start = w_paren + 1;
        assert!(rows[0][start..start + w_one].iter().all(|&c| c == 0));
        assert!(rows[6][start..start + w_one].iter().all(|&c| c == 0));
    }

    #[test]
    fn empty_text_is_empty() {
        assert_eq!(raster(""), Vec::<Vec<u8>>::new());
    }

    #[test]
    fn unknown_chars_become_three_blank_columns() {
        let rows = raster("\u{00a7}");
        assert_eq!(rows.len(), 5);
        assert_eq!(rows[0].len(), 3);
        assert!(rows.iter().all(|row| row.iter().all(|&v| v == 0)));
    }
}
