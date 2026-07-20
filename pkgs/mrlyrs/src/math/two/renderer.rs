use super::models::Cell2d;
use crate::core::cell::mapping;
use crate::core::colors::Color;
use crate::core::enums::Mode;
use crate::core::errors::Result;
use crate::math::dim::push_glyph;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    Square,
    Circle,
    Diamond,
}

fn painted(cell: &Cell2d) -> Vec<[u8; 4]> {
    match &cell.cell.colors {
        Some(colors) => colors.clone(),
        None => {
            let fresh = cell.clone().paint(&mapping(), Mode::Type);
            fresh.cell.colors.unwrap()
        }
    }
}

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

pub fn png(cell: &Cell2d, scale: usize) -> Result<Vec<u8>> {
    let colors = painted(cell);
    crate::io::png(&colors, cell.width(), cell.height(), scale)
}

pub fn svg(
    cell: &Cell2d,
    scale: usize,
    shape: Shape,
    outline: Option<Color>,
    width: usize,
) -> String {
    let padding = if outline.is_some() { width } else { 0 };
    let colors = painted(cell);
    let (h, w) = (cell.height(), cell.width());
    let (img_w, img_h) = (w * scale + padding * 2, h * scale + padding * 2);
    let stroke = match outline {
        Some(c) => format!("stroke=\"{}\" stroke-width=\"{width}\"", c.to_hex()),
        None => "stroke=\"none\"".to_string(),
    };
    let mut out = vec![format!(
        "<svg width=\"{img_w}\" height=\"{img_h}\" xmlns=\"http://www.w3.org/2000/svg\">"
    )];
    for y in 0..h {
        for x in 0..w {
            let [r, g, b, a] = colors[y * w + x];
            if a == 0 {
                continue;
            }
            let fill = Color::rgba(r, g, b, a).to_hex();
            let (x0, y0) = (x * scale + padding, y * scale + padding);
            let element = match shape {
                Shape::Square => format!(
                    "<rect x=\"{x0}\" y=\"{y0}\" width=\"{scale}\" height=\"{scale}\" fill=\"{fill}\" {stroke}/>"
                ),
                Shape::Circle => {
                    let radius = scale as f64 / 2.0;
                    let (cx, cy) = (x0 as f64 + radius, y0 as f64 + radius);
                    format!("<circle cx=\"{cx}\" cy=\"{cy}\" r=\"{radius}\" fill=\"{fill}\" {stroke}/>")
                }
                Shape::Diamond => {
                    let half = scale as f64 / 2.0;
                    let (mx, my) = (x0 as f64 + half, y0 as f64 + half);
                    let (x1, y1) = (x0 + scale, y0 + scale);
                    format!(
                        "<polygon points=\"{mx},{y0} {x1},{my} {mx},{y1} {x0},{my}\" fill=\"{fill}\" {stroke}/>"
                    )
                }
            };
            out.push(element);
        }
    }
    out.push("</svg>".to_string());
    out.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::two::designs;
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
    #[test]
    fn svg_counts_filled() {
        let c = designs::carpet(3, 1).unwrap();
        let s = svg(&c, 10, Shape::Square, None, 0);
        assert_eq!(s.matches("<rect").count(), 9);
        assert!(s.starts_with("<svg width=\"30\" height=\"30\""));
        let d = svg(&c, 10, Shape::Diamond, None, 0);
        assert_eq!(d.matches("<polygon").count(), 9);
    }
}

#[cfg(test)]
mod golden {
    use super::*;
    use crate::math::two::designs;
    #[test]
    fn golden_parity_with_pre_io_bytes() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let cases: Vec<Vec<u8>> = vec![
            png(&designs::carpet(3, 2).unwrap(), 4).unwrap(),
            png(&designs::htree(5, 1).unwrap(), 1).unwrap(),
            png(&designs::vtree(7, 1).unwrap(), 3).unwrap(),
        ];
        let pins: [(usize, u64); 3] = [
            (526, 16276027416531335811),
            (140, 470415841427865153),
            (544, 13897638462203747366),
        ];
        for (bytes, (len, hash)) in cases.iter().zip(pins) {
            let mut h = DefaultHasher::new();
            bytes.hash(&mut h);
            assert_eq!(
                bytes.len(),
                len,
                "byte length drifted from pre-io::png bytes"
            );
            assert_eq!(h.finish(), hash, "bytes drifted from pre-io::png rendering");
        }
    }
}
