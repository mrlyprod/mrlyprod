use super::models::Cell2d;
use crate::dim::push_glyph;
use mrlycore::cell::mapping;
use mrlycore::enums::Mode;
use mrlycore::errors::Result;
use std::collections::HashMap;

fn painted(cell: &Cell2d) -> Vec<[u8; 4]> {
    match &cell.cell.colors {
        Some(colors) => colors.clone(),
        None => {
            let fresh = cell.clone().paint(&mapping(), Mode::Type);
            fresh.cell.colors.unwrap()
        }
    }
}

/// Renders the cell as rows of glyphs, or of digits where no glyph is mapped.
///
/// ```
/// let cell = mrlymath::two::carpet(3, 1).unwrap();
/// assert_eq!(mrlymath::two::text(&cell, None), vec!["111", "101", "111"]);
/// ```
pub fn text(cell: &Cell2d, glyphs: Option<&HashMap<u8, char>>) -> Vec<String> {
    let (h, w) = (cell.height(), cell.width());
    let mut rows = Vec::with_capacity(h);
    for y in 0..h {
        let mut row = String::with_capacity(w);
        for x in 0..w {
            let v = cell.types().get(&[y, x]);
            push_glyph(&mut row, v, glyphs);
        }
        rows.push(row);
    }
    rows
}

/// Renders the cell to PNG bytes at the given pixel scale, painting it by type when unpainted.
pub fn png(cell: &Cell2d, scale: usize) -> Result<Vec<u8>> {
    let colors = painted(cell);
    mrlycore::png(&colors, cell.width(), cell.height(), scale)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::two::designs;
    #[test]
    fn text_digits_and_glyphs() {
        let c = designs::carpet(3, 1).unwrap();
        let t = text(&c, None);
        assert_eq!(t, vec!["111", "101", "111"]);
        let glyphs = HashMap::from([(0, ' '), (1, '#')]);
        assert_eq!(text(&c, Some(&glyphs)), vec!["###", "# #", "###"]);
    }
    #[test]
    fn png_signature_and_size() {
        let c = designs::carpet(3, 2).unwrap();
        let bytes = png(&c, 4).unwrap();
        assert_eq!(&bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
        assert!(bytes.len() > 100);
    }
}

#[cfg(test)]
mod golden {
    use super::*;
    use crate::two::designs;
    #[test]
    fn png_pixels_stay_pinned() {
        let black = [0, 0, 0, 255];
        let white = [255, 255, 255, 255];
        let cases = [
            (
                png(&designs::carpet(3, 2).unwrap(), 4).unwrap(),
                36,
                1024,
                white,
            ),
            (
                png(&designs::htree(5, 1).unwrap(), 1).unwrap(),
                5,
                15,
                black,
            ),
            (
                png(&designs::vtree(7, 1).unwrap(), 3).unwrap(),
                21,
                252,
                white,
            ),
        ];
        for (bytes, side, inked, centre) in &cases {
            let (w, h, pixels) = mrlycore::unpng(bytes).unwrap();
            assert_eq!((w, h), (*side, *side));
            assert!(pixels.iter().all(|p| *p == black || *p == white));
            assert_eq!(pixels.iter().filter(|p| **p == black).count(), *inked);
            assert_eq!(pixels[0], black);
            assert_eq!(pixels[(side / 2) * side + side / 2], *centre);
            assert_eq!(pixels[side * side - 1], black);
        }
    }
}
