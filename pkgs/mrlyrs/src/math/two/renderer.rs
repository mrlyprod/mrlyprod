use super::Cell2d;
use crate::core::cell::mapping;
use crate::core::cell::Mode;
use crate::core::colors::Color;
use crate::core::error::{value_error, Result};
use crate::math::cell::push_glyph;
use std::collections::HashMap;

/// The outline a flat cell's sites are drawn with.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Shape {
    /// A filled cell-sized square.
    Square,
    /// A disc inscribed in the cell-sized square.
    Circle,
    /// A rhombus through the midpoints of the cell-sized square.
    Diamond,
}

fn painted(cell: &Cell2d) -> Vec<[u8; 4]> {
    match &cell.cell.colors {
        Some(colors) => colors.clone(),
        None => cell
            .clone()
            .paint(&mapping(), Mode::Type, None)
            .ok()
            .and_then(|fresh| fresh.cell.colors)
            .unwrap_or_default(),
    }
}

/// Renders the cell as rows of glyphs, or of digits where no glyph is mapped.
///
/// ```
/// let cell = mrlyrs::math::two::carpet(3, 1).unwrap();
/// assert_eq!(mrlyrs::math::two::text(&cell, None), vec!["111", "101", "111"]);
/// ```
pub fn text(cell: &Cell2d, glyphs: Option<&HashMap<u8, String>>) -> Vec<String> {
    let (h, w) = (cell.height(), cell.width());
    let mut rows = Vec::with_capacity(h);
    for y in 0..h {
        let mut row = String::with_capacity(w);
        for x in 0..w {
            let v = cell.types().at(y * w + x) as u8;
            push_glyph(&mut row, v, glyphs);
        }
        rows.push(row);
    }
    rows
}

// SHAPES

fn diamond_points(scale: usize) -> [(f64, f64); 4] {
    let (s, h) = (scale as f64, (scale / 2) as f64);
    [(h, 0.0), (s, h), (h, s), (0.0, h)]
}

fn in_diamond(points: &[(f64, f64); 4], x: f64, y: f64) -> bool {
    let (mut low, mut high) = (false, false);
    for k in 0..4 {
        let (ax, ay) = points[k];
        let (bx, by) = points[(k + 1) % 4];
        let cross = (bx - ax) * (y - ay) - (by - ay) * (x - ax);
        low |= cross < 0.0;
        high |= cross > 0.0;
    }
    !(low && high)
}

fn stencil(shape: Shape, scale: usize) -> Vec<bool> {
    let side = scale + 1;
    let mut out = vec![true; side * side];
    match shape {
        Shape::Square => {}
        Shape::Circle => {
            let c = side as f64 / 2.0;
            for i in 0..side {
                for j in 0..side {
                    let (dy, dx) = (i as f64 + 0.5 - c, j as f64 + 0.5 - c);
                    out[i * side + j] = dx * dx + dy * dy <= c * c;
                }
            }
        }
        Shape::Diamond => {
            let points = diamond_points(scale);
            for i in 0..side {
                for j in 0..side {
                    out[i * side + j] = in_diamond(&points, j as f64, i as f64);
                }
            }
        }
    }
    out
}

fn rim(stencil: &[bool], side: usize, width: usize) -> Vec<bool> {
    let inside = |i: i64, j: i64| {
        i >= 0
            && j >= 0
            && (i as usize) < side
            && (j as usize) < side
            && stencil[i as usize * side + j as usize]
    };
    let reach = width as i64;
    let mut out = vec![false; side * side];
    for i in 0..side {
        for j in 0..side {
            if !stencil[i * side + j] {
                continue;
            }
            out[i * side + j] = (-reach..=reach)
                .any(|di| (-reach..=reach).any(|dj| !inside(i as i64 + di, j as i64 + dj)));
        }
    }
    out
}

// PNG

/// Renders the cell to PNG bytes at the given pixel scale, stroked and padded when an outline is given.
///
/// # Errors
///
/// Errors when the encoder refuses the cell's size or scale.
pub fn png(
    cell: &Cell2d,
    scale: usize,
    outline: Option<Color>,
    width: usize,
    shape: Shape,
) -> Result<Vec<u8>> {
    if scale < 1 {
        return value_error("scale must be at least 1.");
    }
    let colors = painted(cell);
    let padding = if outline.is_some() { width } else { 0 };
    let (h, w) = (cell.height(), cell.width());
    let (img_w, img_h) = (w * scale + padding * 2, h * scale + padding * 2);
    let side = scale + 1;
    let stencil = stencil(shape, scale);
    let edge = outline.map(|c| (rim(&stencil, side, width), [c.r, c.g, c.b, c.a]));
    let mut pixels = vec![[0u8; 4]; img_w * img_h];
    for y in 0..h {
        for x in 0..w {
            let rgba = colors[y * w + x];
            if rgba[3] == 0 {
                continue;
            }
            let (x0, y0) = (x * scale + padding, y * scale + padding);
            for i in 0..side {
                for j in 0..side {
                    if !stencil[i * side + j] || y0 + i >= img_h || x0 + j >= img_w {
                        continue;
                    }
                    pixels[(y0 + i) * img_w + x0 + j] = match &edge {
                        Some((rim, ink)) if rim[i * side + j] => *ink,
                        _ => rgba,
                    };
                }
            }
        }
    }
    crate::core::png(&pixels, img_w, img_h, 1)
}

// SVG

fn stroke_of(outline: Option<Color>, width: usize) -> String {
    match outline {
        Some(c) => format!("stroke=\"{}\" stroke-width=\"{width}\"", c.to_hex()),
        None => "stroke=\"none\"".to_string(),
    }
}

fn element(shape: Shape, x0: usize, y0: usize, scale: usize, fill: &str, stroke: &str) -> String {
    match shape {
        Shape::Square => format!(
            "<rect x=\"{x0}\" y=\"{y0}\" width=\"{scale}\" height=\"{scale}\" fill=\"{fill}\" {stroke}/>"
        ),
        Shape::Circle => {
            let r = scale / 2;
            let (cx, cy) = (x0 + r, y0 + r);
            format!("<circle cx=\"{cx}\" cy=\"{cy}\" r=\"{r}\" fill=\"{fill}\" {stroke}/>")
        }
        Shape::Diamond => {
            let h = scale / 2;
            let points = [
                (x0 + h, y0),
                (x0 + scale, y0 + h),
                (x0 + h, y0 + scale),
                (x0, y0 + h),
            ];
            let pts: Vec<String> = points.iter().map(|(x, y)| format!("{x},{y}")).collect();
            format!(
                "<polygon points=\"{}\" fill=\"{fill}\" {stroke}/>",
                pts.join(" ")
            )
        }
    }
}

/// Renders the cell to an SVG string at the given scale, stroked and padded when an outline is given.
///
/// ```
/// let cell = mrlyrs::math::two::carpet(3, 1).unwrap();
/// let sheet = mrlyrs::math::two::svg(&cell, 1, None, 1, mrlyrs::math::two::Shape::Square);
/// assert_eq!(sheet.len(), 698);
/// ```
pub fn svg(
    cell: &Cell2d,
    scale: usize,
    outline: Option<Color>,
    width: usize,
    shape: Shape,
) -> String {
    let colors = painted(cell);
    let padding = if outline.is_some() { width } else { 0 };
    let (h, w) = (cell.height(), cell.width());
    let (img_w, img_h) = (w * scale + padding * 2, h * scale + padding * 2);
    let stroke = stroke_of(outline, width);
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
            out.push(element(shape, x0, y0, scale, &fill, &stroke));
        }
    }
    out.push("</svg>".to_string());
    out.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::colors::RED;
    use crate::core::tensor::Tensor;
    use crate::core::PNG_MAGIC;
    use crate::math::two::designs;
    #[test]
    fn text_digits_and_glyphs() {
        let c = designs::carpet(3, 1).unwrap();
        let t = text(&c, None);
        assert_eq!(t, vec!["111", "101", "111"]);
        let glyphs = HashMap::from([(0, " ".to_string()), (1, "#".to_string())]);
        assert_eq!(text(&c, Some(&glyphs)), vec!["###", "# #", "###"]);
    }
    #[test]
    fn text_glyphs_may_be_wider_than_one_char() {
        let c = Cell2d::new(Tensor::of(vec![0, 1], vec![1, 2]).unwrap()).unwrap();
        let glyphs = HashMap::from([(0, "..".to_string()), (1, "##".to_string())]);
        assert_eq!(text(&c, Some(&glyphs)), vec!["..##"]);
    }
    #[test]
    fn png_signature_and_size() {
        let c = designs::carpet(3, 2).unwrap();
        let bytes = png(&c, 4, None, 1, Shape::Square).unwrap();
        assert_eq!(&bytes[0..8], &PNG_MAGIC);
        assert!(bytes.len() > 100);
    }
    #[test]
    fn outlined_and_round_pngs_differ_from_the_bare_square() {
        let c = designs::carpet(3, 1).unwrap();
        let bare = png(&c, 3, None, 1, Shape::Square).unwrap();
        let lined = png(&c, 3, Some(RED), 1, Shape::Square).unwrap();
        let round = png(&c, 3, None, 1, Shape::Circle).unwrap();
        assert!(lined.len() > bare.len());
        assert_ne!(round, bare);
    }
    #[test]
    fn svg_square_holds_a_rect_a_site() {
        let c = designs::carpet(3, 1).unwrap();
        let sheet = svg(&c, 1, None, 1, Shape::Square);
        assert_eq!(sheet.len(), 698);
        assert!(sheet.starts_with("<svg width=\"3\" height=\"3\""));
        assert_eq!(sheet.matches("<rect").count(), 9);
    }
    #[test]
    fn svg_diamond_holds_a_polygon_a_site() {
        let c = designs::carpet(3, 1).unwrap();
        let sheet = svg(&c, 1, None, 1, Shape::Diamond);
        assert_eq!(sheet.matches("<polygon").count(), 9);
        assert!(sheet.contains("points=\"0,0 1,0 0,1 0,0\""));
    }
    #[test]
    fn svg_outline_strokes_and_pads() {
        let c = designs::carpet(3, 1).unwrap();
        let sheet = svg(&c, 4, Some(RED), 2, Shape::Circle);
        assert!(sheet.starts_with("<svg width=\"16\" height=\"16\""));
        assert!(sheet.contains("stroke-width=\"2\""));
        assert_eq!(sheet.matches("<circle").count(), 9);
    }
}

#[cfg(test)]
mod golden {
    use super::*;
    use crate::math::two::designs;
    #[test]
    fn png_pixels_stay_pinned() {
        let black = [0, 0, 0, 255];
        let white = [255, 255, 255, 255];
        let cases = [
            (
                png(&designs::carpet(3, 2).unwrap(), 4, None, 1, Shape::Square).unwrap(),
                36,
                1024,
                white,
            ),
            (
                png(&designs::htree(5, 1).unwrap(), 1, None, 1, Shape::Square).unwrap(),
                5,
                15,
                black,
            ),
            (
                png(&designs::vtree(7, 1).unwrap(), 3, None, 1, Shape::Square).unwrap(),
                21,
                252,
                white,
            ),
        ];
        for (bytes, side, inked, centre) in &cases {
            let (w, h, pixels) = crate::core::unpng(bytes).unwrap();
            assert_eq!((w, h), (*side, *side));
            assert!(pixels.iter().all(|p| *p == black || *p == white));
            assert_eq!(pixels.iter().filter(|p| **p == black).count(), *inked);
            assert_eq!(pixels[0], black);
            assert_eq!(pixels[(side / 2) * side + side / 2], *centre);
            assert_eq!(pixels[side * side - 1], black);
        }
    }
}
