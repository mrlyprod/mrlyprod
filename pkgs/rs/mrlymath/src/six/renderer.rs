use super::geometry::{is_hex, orientation, tile};
use super::models::Cell6d;
use super::painter::paint;
use super::Orientation;
use mrlycore::colors::Color;
use mrlycore::enums::Mode;
use mrlycore::errors::{value_error, Result};
use mrlycore::resample::{hex_fit, Filter};

/// A screen triangle: three grid points and an RGBA color.
pub type Triangle = ([(i64, i64); 3], [u8; 4]);

/// A rectangular window onto a triangle sheet, the frame every rendering draws through.
#[derive(Clone, Debug)]
pub struct Rect {
    /// The triangles of the sheet the window looks onto.
    pub triangles: Vec<Triangle>,
    /// The window's top-left corner, in grid points.
    pub origin: (i64, i64),
    /// The window's width and height, in grid points.
    pub size: (usize, usize),
}

fn north(x: i64, y: i64) -> [(i64, i64); 3] {
    [(x, 2 * y + 2), (x + 1, 2 * y), (x + 2, 2 * y + 2)]
}

fn south(x: i64, y: i64) -> [(i64, i64); 3] {
    [(x, 2 * y), (x + 1, 2 * y + 2), (x + 2, 2 * y)]
}

fn east(x: i64, y: i64) -> [(i64, i64); 3] {
    [(2 * x, y), (2 * x, y + 2), (2 * x + 2, y + 1)]
}

fn west(x: i64, y: i64) -> [(i64, i64); 3] {
    [(2 * x + 2, y), (2 * x + 2, y + 2), (2 * x, y + 1)]
}

fn painted(cell: &Cell6d) -> Vec<[u8; 4]> {
    match &cell.cell.cell.colors {
        Some(colors) => colors.clone(),
        None => paint(cell.clone(), None, Some(Mode::Type))
            .cell
            .cell
            .colors
            .unwrap(),
    }
}

/// Folds a cell into colored screen triangles, dropping the transparent ones.
pub fn triangles(cell: &Cell6d) -> Result<Vec<Triangle>> {
    let inner = &cell.cell;
    let (height, width) = (inner.height(), inner.width());
    let colors = painted(cell);
    let orient = orientation(width, height)?;
    let start = cell.start as i64;
    let mut out = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let rgba = colors[y * width + x];
            if rgba[3] == 0 {
                continue;
            }
            let flip = (x as i64 + y as i64 + start).rem_euclid(2);
            let points = match orient {
                Orientation::Horizontal => {
                    if flip == 0 {
                        north(x as i64, y as i64)
                    } else {
                        south(x as i64, y as i64)
                    }
                }
                Orientation::Vertical => {
                    if flip == 0 {
                        east(x as i64, y as i64)
                    } else {
                        west(x as i64, y as i64)
                    }
                }
            };
            out.push((points, rgba));
        }
    }
    Ok(out)
}

fn bounds(tris: &[Triangle]) -> (i64, i64, i64, i64) {
    let xs = tris.iter().flat_map(|(p, _)| p.iter().map(|q| q.0));
    let ys = tris.iter().flat_map(|(p, _)| p.iter().map(|q| q.1));
    let min_x = xs.clone().min().unwrap();
    let max_x = xs.max().unwrap();
    let min_y = ys.clone().min().unwrap();
    let max_y = ys.max().unwrap();
    (min_x, max_x, min_y, max_y)
}

fn window(cell: &Cell6d, padding: i64) -> Result<Rect> {
    let triangles = triangles(cell)?;
    if triangles.is_empty() {
        return value_error("nothing to render.");
    }
    let (min_x, max_x, min_y, max_y) = bounds(&triangles);
    Ok(Rect {
        triangles,
        origin: (min_x - padding, min_y - padding),
        size: (
            (max_x - min_x + 2 * padding) as usize,
            (max_y - min_y + 2 * padding) as usize,
        ),
    })
}

fn stroke_of(outline: Option<Color>, width: usize) -> String {
    match outline {
        Some(c) => format!("stroke=\"{}\" stroke-width=\"{width}\"", c.to_hex()),
        None => "stroke=\"none\"".to_string(),
    }
}

fn polygon(
    points: &[(i64, i64); 3],
    rgba: [u8; 4],
    origin: (i64, i64),
    scale: usize,
    stroke: &str,
) -> String {
    let [r, g, b, a] = rgba;
    let fill = Color::rgba(r, g, b, a).to_hex();
    let pts: Vec<String> = points
        .iter()
        .map(|(x, y)| {
            format!(
                "{},{}",
                (x - origin.0) * scale as i64,
                (y - origin.1) * scale as i64
            )
        })
        .collect();
    format!(
        "<polygon points=\"{}\" fill=\"{fill}\" {stroke}/>",
        pts.join(" ")
    )
}

fn sheet(view: &Rect, scale: usize, stroke: &str) -> String {
    let (img_w, img_h) = (view.size.0 * scale, view.size.1 * scale);
    let mut out = vec![format!(
        "<svg width=\"{img_w}\" height=\"{img_h}\" viewBox=\"0 0 {img_w} {img_h}\" xmlns=\"http://www.w3.org/2000/svg\">"
    )];
    for (points, rgba) in &view.triangles {
        out.push(polygon(points, *rgba, view.origin, scale, stroke));
    }
    out.push("</svg>".to_string());
    out.join("\n")
}

/// Renders a cell's triangles to an SVG string at the given scale, stroked and padded when an outline is given, or an error when nothing renders.
pub fn svg(cell: &Cell6d, scale: usize, outline: Option<Color>, width: usize) -> Result<String> {
    let padding = if outline.is_some() { width as i64 } else { 0 };
    Ok(sheet(
        &window(cell, padding)?,
        scale,
        &stroke_of(outline, width),
    ))
}

fn fill_triangles(
    tris: &[Triangle],
    origin: (i64, i64),
    size: (usize, usize),
    scale: usize,
) -> Vec<[u8; 4]> {
    let (img_w, img_h) = size;
    let mut pixels = vec![[0u8; 4]; img_w * img_h];
    for (points, rgba) in tris {
        let scaled: Vec<(f64, f64)> = points
            .iter()
            .map(|(x, y)| {
                (
                    ((x - origin.0) * scale as i64) as f64,
                    ((y - origin.1) * scale as i64) as f64,
                )
            })
            .collect();
        let span = |pick: fn(&(f64, f64)) -> f64, limit: usize| -> (usize, usize) {
            let lo = scaled.iter().map(pick).fold(f64::MAX, f64::min);
            let hi = scaled.iter().map(pick).fold(f64::MIN, f64::max);
            (
                lo.floor().clamp(0.0, limit as f64) as usize,
                hi.ceil().clamp(0.0, limit as f64) as usize,
            )
        };
        let (x0, x1) = span(|p| p.0, img_w);
        let (y0, y1) = span(|p| p.1, img_h);
        let edge = |a: (f64, f64), b: (f64, f64), p: (f64, f64)| -> f64 {
            (b.0 - a.0) * (p.1 - a.1) - (b.1 - a.1) * (p.0 - a.0)
        };
        for py in y0..y1 {
            for px in x0..x1 {
                let p = (px as f64 + 0.5, py as f64 + 0.5);
                let e0 = edge(scaled[0], scaled[1], p);
                let e1 = edge(scaled[1], scaled[2], p);
                let e2 = edge(scaled[2], scaled[0], p);
                let inside =
                    (e0 >= 0.0 && e1 >= 0.0 && e2 >= 0.0) || (e0 <= 0.0 && e1 <= 0.0 && e2 <= 0.0);
                if inside {
                    pixels[py * img_w + px] = *rgba;
                }
            }
        }
    }
    pixels
}

fn canvas(view: &Rect, scale: usize) -> ((usize, usize), Vec<[u8; 4]>) {
    let size = (view.size.0 * scale, view.size.1 * scale);
    (
        size,
        fill_triangles(&view.triangles, view.origin, size, scale),
    )
}

fn frame(view: &Rect, scale: usize) -> Result<Vec<u8>> {
    let ((img_w, img_h), pixels) = canvas(view, scale);
    mrlycore::io::png(&pixels, img_w, img_h, 1)
}

/// Rasterizes a cell's triangles to PNG bytes at the given scale, or an error when nothing renders.
pub fn png(cell: &Cell6d, scale: usize) -> Result<Vec<u8>> {
    frame(&window(cell, 0)?, scale)
}

/// Rasterizes a cell's triangles to PNG bytes squashed to the true hex aspect, the stretched axis resampled by the filter.
pub fn hex_png(cell: &Cell6d, scale: usize, filter: Filter) -> Result<Vec<u8>> {
    let vertical = orientation(cell.width(), cell.height())? == Orientation::Vertical;
    let ((width, height), pixels) = canvas(&window(cell, 0)?, scale);
    let (img_w, img_h, fitted) = hex_fit(&pixels, width, height, vertical, filter)?;
    mrlycore::io::png(&fitted, img_w, img_h, 1)
}

/// Tessellates a hexagon and frames the rectangular fundamental domain of that tiling.
pub fn rect(cell: &Cell6d) -> Result<Rect> {
    let inner = &cell.cell;
    if !is_hex(inner) {
        return value_error("Cell must be a hexagon.");
    }
    let (tile_h, tile_w) = (inner.height(), inner.width());
    let (step_x, step_y, origin) = match orientation(tile_w, tile_h)? {
        Orientation::Horizontal => (
            (3 * (tile_w + 1)) / 4,
            tile_h,
            (tile_w.div_ceil(2) as i64, tile_h as i64),
        ),
        Orientation::Vertical => (
            tile_w,
            (3 * (tile_h + 1)) / 4,
            (tile_w as i64, tile_h.div_ceil(2) as i64),
        ),
    };
    let sheet = tile(cell, 3, 3)?;
    let orient = orientation(sheet.width(), sheet.height())?;
    let tiled = Cell6d::new(sheet, cell.projection, orient, cell.start);
    Ok(Rect {
        triangles: triangles(&tiled)?,
        origin,
        size: (2 * step_x, 2 * step_y),
    })
}

/// Renders the hexagon's rectangular fundamental domain to an SVG string at the given scale, stroked when an outline is given.
pub fn rect_svg(
    cell: &Cell6d,
    scale: usize,
    outline: Option<Color>,
    width: usize,
) -> Result<String> {
    Ok(sheet(&rect(cell)?, scale, &stroke_of(outline, width)))
}

/// Rasterizes the hexagon's rectangular fundamental domain to PNG bytes at the given scale.
pub fn rect_png(cell: &Cell6d, scale: usize) -> Result<Vec<u8>> {
    frame(&rect(cell)?, scale)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::six::designs::iso_design;
    use crate::six::geometry::blank;
    use crate::six::models::Cell6d;
    use crate::six::{Orientation, Projection};
    #[test]
    fn triangle_geometry() {
        assert_eq!(north(0, 0), [(0, 2), (1, 0), (2, 2)]);
        assert_eq!(south(0, 0), [(0, 0), (1, 2), (2, 0)]);
        assert_eq!(east(1, 1), [(2, 1), (2, 3), (4, 2)]);
    }
    #[test]
    fn iso_renders_triangles() {
        let i = iso_design(23, 3, 1, 2).unwrap();
        let tris = triangles(&i).unwrap();
        assert!(!tris.is_empty());
        let s = svg(&i, 10, None, 1).unwrap();
        assert!(s.contains("<polygon"));
        assert!(s.contains("stroke=\"none\""));
        let bytes = png(&i, 10).unwrap();
        assert_eq!(&bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
    }
    #[test]
    fn outline_strokes_and_pads_the_svg() {
        let i = iso_design(23, 3, 1, 2).unwrap();
        let plain = svg(&i, 4, None, 1).unwrap();
        let lined = svg(&i, 4, Some(Color::rgba(255, 0, 0, 255)), 2).unwrap();
        assert!(lined.contains("stroke-width=\"2\""));
        assert!(lined.contains(&Color::rgba(255, 0, 0, 255).to_hex()));
        let size = |svg: &str| -> usize {
            svg.split("width=\"")
                .nth(1)
                .unwrap()
                .split('"')
                .next()
                .unwrap()
                .parse()
                .unwrap()
        };
        assert_eq!(size(&lined), size(&plain) + 4 * 4);
    }
    #[test]
    fn hexagon_renders() {
        let hex = Cell6d::new(
            blank(3, Orientation::Horizontal, 1, 0),
            Projection::Cut,
            Orientation::Horizontal,
            0,
        );
        let tris = triangles(&hex).unwrap();
        assert!(!tris.is_empty());
    }
    fn tiling_hex(radius: usize, orient: Orientation) -> Cell6d {
        Cell6d::new(
            blank(radius, orient, crate::six::FILL, crate::six::GRID),
            Projection::Cut,
            orient,
            0,
        )
    }
    #[test]
    fn rect_frames_the_fundamental_domain() {
        let hex = tiling_hex(2, Orientation::Horizontal);
        let domain = rect(&hex).unwrap();
        assert_eq!(domain.size, (12, 8));
        assert_eq!(domain.origin, (4, 4));
        let s = rect_svg(&hex, 3, None, 1).unwrap();
        assert!(s.contains("viewBox=\"0 0 36 24\""));
        assert!(s.contains("stroke=\"none\""));
        let lined = rect_svg(&hex, 3, Some(Color::rgba(0, 0, 255, 255)), 2).unwrap();
        assert!(lined.contains("viewBox=\"0 0 36 24\""));
        assert!(lined.contains("stroke-width=\"2\""));
        assert!(lined.contains(&Color::rgba(0, 0, 255, 255).to_hex()));
        let bytes = rect_png(&hex, 3).unwrap();
        assert_eq!(&bytes[0..8], &[137, 80, 78, 71, 13, 10, 26, 10]);
        let vertical = tiling_hex(2, Orientation::Vertical);
        let tall = rect(&vertical).unwrap();
        assert!(tall.size.1 > tall.size.0);
        assert!(rect(&Cell6d::new(
            crate::two::Cell2d::new(mrlycore::Tensor::full(vec![4, 4], 1)),
            Projection::Cut,
            Orientation::Horizontal,
            0,
        ))
        .is_err());
    }
    #[test]
    fn rect_window_has_no_gaps() {
        for orient in [Orientation::Horizontal, Orientation::Vertical] {
            let hex = tiling_hex(3, orient);
            let domain = rect(&hex).unwrap();
            let pixels = fill_triangles(&domain.triangles, domain.origin, domain.size, 1);
            let clear = pixels.iter().filter(|p| p[3] == 0).count();
            assert_eq!(clear, 0, "{orient:?} of {} pixels", pixels.len());
        }
    }
    #[test]
    fn nothing_to_render_errors_in_both_doors() {
        let bare = Cell6d::new(
            crate::two::Cell2d::new(mrlycore::Tensor::full(vec![2, 3], crate::six::GRID)),
            Projection::Cut,
            Orientation::Horizontal,
            0,
        );
        assert!(triangles(&bare).unwrap().is_empty());
        assert!(svg(&bare, 4, None, 1).is_err());
        assert!(png(&bare, 4).is_err());
        assert!(hex_png(&bare, 4, Filter::Box).is_err());
    }
    #[test]
    fn hex_png_squashes_the_stretched_axis() {
        let sides = |bytes: &[u8]| -> (usize, usize) {
            let (w, h, _) = mrlycore::unpng(bytes).unwrap();
            (w, h)
        };
        let tall = iso_design(23, 3, 1, 2).unwrap();
        let (w, h) = sides(&png(&tall, 4).unwrap());
        assert_eq!(
            sides(&hex_png(&tall, 4, Filter::Box).unwrap()),
            mrlycore::resample::hex_size(w, h, true)
        );
        let wide = tiling_hex(3, Orientation::Horizontal);
        let (w, h) = sides(&png(&wide, 4).unwrap());
        assert_eq!(
            sides(&hex_png(&wide, 4, Filter::Nearest).unwrap()),
            mrlycore::resample::hex_size(w, h, false)
        );
    }
}

#[cfg(test)]
mod golden {
    use super::*;
    use crate::six::designs::iso_design;
    #[test]
    fn png_bytes_stay_pinned() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let cases: Vec<Vec<u8>> = vec![
            png(&iso_design(23, 3, 1, 2).unwrap(), 10).unwrap(),
            png(&iso_design(5, 4, 1, 2).unwrap(), 3).unwrap(),
        ];
        let pins: [(usize, u64); 2] = [(1113, 10745774475925788832), (411, 1938154634769888273)];
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
