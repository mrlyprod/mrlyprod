use super::glyph::descends;
use super::{glyph, trim};

pub(crate) struct Block {
    pub char: char,
    pub rows: Vec<String>,
    pub col: usize,
    pub offset: usize,
}

impl Block {
    pub(crate) fn width(&self) -> usize {
        self.rows.first().map_or(0, String::len)
    }
}

pub(crate) struct Layout {
    pub height: usize,
    pub width: usize,
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
    let height: usize = if text.chars().any(descends) { 7 } else { 5 };
    let shapes: Vec<(char, Vec<String>)> = text
        .chars()
        .map(|c| match glyph(c) {
            Some(g) if c != ' ' => (c, trim(&g.rows)),
            _ => (c, vec!["000".to_string(); 5]),
        })
        .collect();
    let mut blocks = Vec::new();
    let mut col = 0;
    for (c, rows) in shapes {
        let offset = height.saturating_sub(rows.len()) / 2;
        let block = Block {
            char: c,
            rows,
            col,
            offset,
        };
        col += block.width() + 1;
        blocks.push(block);
    }
    let width = col - 1;
    Layout {
        height,
        width,
        blocks,
    }
}

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
