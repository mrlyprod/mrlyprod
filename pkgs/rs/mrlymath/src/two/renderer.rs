use super::models::Cell2d;
use crate::dim::push_glyph;
use mrlycore::cell::mapping;
use mrlycore::colors::Color;
use mrlycore::enums::Mode;
use mrlycore::errors::{value_error, Result};
use mrlycore::tensor::Tensor;
use std::collections::HashMap;

/// The mark an SVG rendering draws at each site.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Shape {
    /// The full-cell square.
    Square,
    /// The inscribed circle.
    Circle,
    /// The inscribed diamond.
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
    mrlycore::io::png(&colors, cell.width(), cell.height(), scale)
}

/// Reads PNG bytes back into a cell, its pixels kept as colors and its dark opaque sites filled.
///
/// ```
/// let cell = mrlymath::two::carpet(3, 1).unwrap();
/// let bytes = mrlymath::two::png(&cell, 1).unwrap();
/// let back = mrlymath::two::from_png(&bytes).unwrap();
/// assert_eq!(mrlymath::two::to_strings(&back), vec!["111", "101", "111"]);
/// ```
pub fn from_png(bytes: &[u8]) -> Result<Cell2d> {
    let (width, height, pixels) = mrlycore::codec::unpng(bytes)?;
    let types: Vec<u8> = pixels
        .iter()
        .map(|&[r, g, b, a]| {
            let luminance = 299 * r as u32 + 587 * g as u32 + 114 * b as u32;
            (a != 0 && luminance < 128_000) as u8
        })
        .collect();
    let mut cell = Cell2d::new(Tensor::of(types, vec![height, width]));
    cell.cell.colors = Some(pixels);
    Ok(cell)
}

/// Renders cells into one PNG contact sheet, each centred in a slot the largest cell sizes.
///
/// The gap is drawn in the ground color and every measure is in cells, so the scale
/// multiplies the whole sheet at once.
pub fn montage(
    cells: &[Cell2d],
    columns: usize,
    scale: usize,
    gap: usize,
    ground: Color,
) -> Result<Vec<u8>> {
    if cells.is_empty() {
        return value_error("montage needs at least one cell.");
    }
    if columns == 0 {
        return value_error("montage needs at least one column.");
    }
    let slot_w = cells.iter().map(|c| c.width()).max().unwrap_or(0);
    let slot_h = cells.iter().map(|c| c.height()).max().unwrap_or(0);
    let rows = cells.len().div_ceil(columns);
    let sheet_w = columns * (slot_w + gap) + gap;
    let sheet_h = rows * (slot_h + gap) + gap;
    let mut pixels = vec![[ground.r, ground.g, ground.b, ground.a]; sheet_w * sheet_h];
    for (i, cell) in cells.iter().enumerate() {
        let colors = painted(cell);
        let (w, h) = (cell.width(), cell.height());
        let x0 = (i % columns) * (slot_w + gap) + gap + (slot_w - w) / 2;
        let y0 = (i / columns) * (slot_h + gap) + gap + (slot_h - h) / 2;
        for y in 0..h {
            for x in 0..w {
                let color = colors[y * w + x];
                if color[3] != 0 {
                    pixels[(y0 + y) * sheet_w + x0 + x] = color;
                }
            }
        }
    }
    mrlycore::io::png(&pixels, sheet_w, sheet_h, scale)
}

enum Mark {
    Outside,
    Rim,
    Inside,
}

fn mark(shape: Shape, i: usize, j: usize, scale: usize, stroke: usize) -> Mark {
    let rim = scale as f64 / 2.0;
    let dx = i as f64 + 0.5 - rim;
    let dy = j as f64 + 0.5 - rim;
    let reach = match shape {
        Shape::Square => dx.abs().max(dy.abs()),
        Shape::Circle => (dx * dx + dy * dy).sqrt(),
        Shape::Diamond => dx.abs() + dy.abs(),
    };
    if reach > rim {
        Mark::Outside
    } else if stroke > 0 && reach > rim - stroke as f64 {
        Mark::Rim
    } else {
        Mark::Inside
    }
}

/// Renders the cell to PNG bytes, each site drawn as the shape, stroked when an outline is given.
pub fn raster(
    cell: &Cell2d,
    scale: usize,
    shape: Shape,
    outline: Option<Color>,
    width: usize,
) -> Result<Vec<u8>> {
    let stroke = if outline.is_some() { width } else { 0 };
    let colors = painted(cell);
    let (h, w) = (cell.height(), cell.width());
    let (img_w, img_h) = (w * scale + stroke * 2, h * scale + stroke * 2);
    let mut pixels = vec![[0u8; 4]; img_w * img_h];
    let edge = outline.map(|c| [c.r, c.g, c.b, c.a]);
    for y in 0..h {
        for x in 0..w {
            let fill = colors[y * w + x];
            if fill[3] == 0 {
                continue;
            }
            let (x0, y0) = (x * scale + stroke, y * scale + stroke);
            for j in 0..scale {
                for i in 0..scale {
                    let ink = match mark(shape, i, j, scale, stroke) {
                        Mark::Outside => continue,
                        Mark::Rim => edge.unwrap_or(fill),
                        Mark::Inside => fill,
                    };
                    pixels[(y0 + j) * img_w + x0 + i] = ink;
                }
            }
        }
    }
    mrlycore::io::png(&pixels, img_w, img_h, 1)
}

/// Renders the cell as SVG marks at the given scale, stroked and padded when an outline is given.
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
    #[test]
    fn svg_counts_filled() {
        let c = designs::carpet(3, 1).unwrap();
        let s = svg(&c, 10, Shape::Square, None, 0);
        assert_eq!(s.matches("<rect").count(), 9);
        assert!(s.starts_with("<svg width=\"30\" height=\"30\""));
        let d = svg(&c, 10, Shape::Diamond, None, 0);
        assert_eq!(d.matches("<polygon").count(), 9);
    }
    #[test]
    fn from_png_round_trips_a_rendering() {
        let c = designs::carpet(3, 2).unwrap();
        let bytes = png(&c, 1).unwrap();
        let back = from_png(&bytes).unwrap();
        assert_eq!(back.width(), 9);
        assert_eq!(back.types(), c.types());
        assert_eq!(png(&back, 1).unwrap(), bytes);
        assert!(from_png(b"not a png").is_err());
    }
    #[test]
    fn from_png_reads_a_scaled_rendering() {
        let c = designs::htree(5, 1).unwrap();
        let back = from_png(&png(&c, 3).unwrap()).unwrap();
        assert_eq!((back.width(), back.height()), (15, 15));
        assert_eq!(back.types().sum(), 9 * c.types().sum());
    }
    #[test]
    fn raster_sizes_match_the_outline_padding() {
        let c = designs::carpet(3, 1).unwrap();
        let bare = raster(&c, 8, Shape::Square, None, 0).unwrap();
        assert_eq!(&bare[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
        assert_eq!(from_png(&bare).unwrap().width(), 24);
        let edged = raster(&c, 8, Shape::Square, Some(mrlycore::colors::RED), 2).unwrap();
        assert_eq!(from_png(&edged).unwrap().width(), 28);
    }
    #[test]
    fn raster_shapes_carve_the_corners() {
        let c = designs::ones(1, 1).unwrap();
        let counts: Vec<usize> = [Shape::Square, Shape::Circle, Shape::Diamond]
            .into_iter()
            .map(|shape| {
                let bytes = raster(&c, 16, shape, None, 0).unwrap();
                let cell = from_png(&bytes).unwrap();
                cell.cell
                    .colors
                    .as_ref()
                    .unwrap()
                    .iter()
                    .filter(|p| p[3] != 0)
                    .count()
            })
            .collect();
        assert_eq!(counts, vec![256, 208, 144]);
    }
    #[test]
    fn montage_lays_cells_out_in_a_gapped_grid() {
        let ground = mrlycore::colors::RED;
        let cells = vec![designs::carpet(3, 1).unwrap(); 4];
        let bytes = montage(&cells, 2, 1, 1, ground).unwrap();
        let sheet = from_png(&bytes).unwrap();
        assert_eq!((sheet.width(), sheet.height()), (9, 9));
        let colors = sheet.cell.colors.as_ref().unwrap();
        let ink = [ground.r, ground.g, ground.b, 255];
        for x in 0..9 {
            assert_eq!(colors[x], ink, "top gap at {x}");
            assert_eq!(colors[4 * 9 + x], ink, "middle gap at {x}");
        }
        assert_eq!(colors[9 + 1], [0, 0, 0, 255]);
        assert_eq!(colors[2 * 9 + 2], [255, 255, 255, 255]);
    }

    #[test]
    fn montage_centres_smaller_cells_in_the_slot() {
        let cells = vec![designs::ones(4, 1).unwrap(), designs::ones(2, 1).unwrap()];
        let bytes = montage(&cells, 2, 1, 0, mrlycore::colors::WHITE).unwrap();
        let sheet = from_png(&bytes).unwrap();
        assert_eq!((sheet.width(), sheet.height()), (8, 4));
        let colors = sheet.cell.colors.as_ref().unwrap();
        assert_eq!(colors[0], [0, 0, 0, 255]);
        assert_eq!(colors[5], [255, 255, 255, 255]);
        assert_eq!(colors[8 + 5], [0, 0, 0, 255]);
        assert_eq!(colors[3 * 8 + 5], [255, 255, 255, 255]);
    }

    #[test]
    fn montage_rejects_an_empty_sheet() {
        let cells = vec![designs::carpet(3, 1).unwrap()];
        assert!(montage(&[], 2, 1, 1, mrlycore::colors::WHITE).is_err());
        assert!(montage(&cells, 0, 1, 1, mrlycore::colors::WHITE).is_err());
        assert!(montage(&cells, 1, 0, 1, mrlycore::colors::WHITE).is_err());
    }

    #[test]
    fn raster_outline_paints_the_rim() {
        let c = designs::ones(1, 1).unwrap();
        let bytes = raster(&c, 9, Shape::Square, Some(mrlycore::colors::RED), 1).unwrap();
        let back = from_png(&bytes).unwrap();
        let colors = back.cell.colors.as_ref().unwrap();
        let red = [
            mrlycore::colors::RED.r,
            mrlycore::colors::RED.g,
            mrlycore::colors::RED.b,
            255,
        ];
        assert_eq!(back.width(), 11);
        assert_eq!(colors[0], [0, 0, 0, 0]);
        assert_eq!(colors[back.width() + 1], red);
        assert_eq!(colors[5 * back.width() + 5], [0, 0, 0, 255]);
    }
}

#[cfg(test)]
mod golden {
    use super::*;
    use crate::two::designs;
    #[test]
    fn png_bytes_stay_pinned() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let cases: Vec<Vec<u8>> = vec![
            png(&designs::carpet(3, 2).unwrap(), 4).unwrap(),
            png(&designs::htree(5, 1).unwrap(), 1).unwrap(),
            png(&designs::vtree(7, 1).unwrap(), 3).unwrap(),
        ];
        let pins: [(usize, u64); 3] = [
            (149, 944213515687961147),
            (75, 5606712636878441020),
            (90, 17826857258100605843),
        ];
        for (bytes, (len, hash)) in cases.iter().zip(pins) {
            let mut h = DefaultHasher::new();
            bytes.hash(&mut h);
            assert_eq!(
                bytes.len(),
                len,
                "byte length drifted from the pinned png bytes"
            );
            assert_eq!(
                h.finish(),
                hash,
                "bytes drifted from the pinned png rendering"
            );
        }
    }
}
