use super::{descends, glyph, trim};

pub fn raster(text: &str) -> Vec<Vec<u8>> {
    if text.is_empty() {
        return Vec::new();
    }
    let height = if text.chars().any(descends) { 7 } else { 5 };
    let blocks: Vec<Vec<String>> = text
        .chars()
        .map(|c| match glyph(c) {
            Some(g) if c != ' ' => trim(&g.rows),
            _ => vec!["000".to_string(); 5],
        })
        .collect();
    let width = blocks.iter().map(|b| b[0].len()).sum::<usize>() + blocks.len() - 1;
    let mut grid = vec![vec![0u8; width]; height];
    let mut col = 0;
    for block in &blocks {
        let rows = block.len();
        let cols = block[0].len();
        let offset = (height - rows) / 2;
        for (r, row) in block.iter().enumerate() {
            for (c, ch) in row.chars().enumerate() {
                if ch == '1' {
                    grid[offset + r][col + c] = 1;
                }
            }
        }
        col += cols + 1;
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
