use super::geometry::{is_hex, orientation, tile};
use super::models::Cell6d;
use super::Orientation;
use crate::core::cell::{mapping, Mode};
use crate::core::colors::Color;
use crate::core::error::{value_error, Result};
use crate::math::two::Cell2d;

/// A screen triangle: three grid points and an RGBA color.
pub type Triangle = ([(i64, i64); 3], [u8; 4]);

#[derive(Clone, Debug)]
struct Rect {
    triangles: Vec<Triangle>,
    origin: (i64, i64),
    size: (usize, usize),
}

/// The three corners of the north-pointing triangle at the grid column and row.
pub fn north(x: i64, y: i64) -> [(i64, i64); 3] {
    [(x, 2 * y + 2), (x + 1, 2 * y), (x + 2, 2 * y + 2)]
}

/// The three corners of the south-pointing triangle at the grid column and row.
pub fn south(x: i64, y: i64) -> [(i64, i64); 3] {
    [(x, 2 * y), (x + 1, 2 * y + 2), (x + 2, 2 * y)]
}

/// The three corners of the east-pointing triangle at the grid column and row.
pub fn east(x: i64, y: i64) -> [(i64, i64); 3] {
    [(2 * x, y), (2 * x, y + 2), (2 * x + 2, y + 1)]
}

/// The three corners of the west-pointing triangle at the grid column and row.
pub fn west(x: i64, y: i64) -> [(i64, i64); 3] {
    [(2 * x + 2, y), (2 * x + 2, y + 2), (2 * x, y + 1)]
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

fn mesh(cell: &Cell2d, start: i64) -> Result<Vec<Triangle>> {
    let (height, width) = (cell.height(), cell.width());
    let colors = painted(cell);
    let orient = orientation(width, height)?;
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

fn parity(cell: &Cell6d, start: Option<usize>) -> i64 {
    match start {
        Some(s) => s as i64,
        None => i64::from(cell.start),
    }
}

/// Folds a cell into colored screen triangles, dropping the transparent ones, at the cell's start parity or the given override.
///
/// # Errors
///
/// Errors for a cell that is not a hexagon.
pub fn triangles(cell: &Cell6d, start: Option<usize>) -> Result<Vec<Triangle>> {
    mesh(&cell.cell, parity(cell, start))
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

fn window(cell: &Cell6d, padding: i64, start: Option<usize>) -> Result<Rect> {
    let triangles = triangles(cell, start)?;
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

fn rectangle(cell: &Cell6d, start: Option<usize>) -> Result<Rect> {
    if !is_hex(&cell.cell) {
        return value_error("Cell must be a hexagon.");
    }
    let (tile_h, tile_w) = (cell.height(), cell.width());
    let (dx, dy, origin) = match orientation(tile_w, tile_h)? {
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
    Ok(Rect {
        triangles: mesh(&sheet, parity(cell, start))?,
        origin,
        size: (2 * dx, 2 * dy),
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

/// Renders a cell's triangles to an SVG string at the given scale, stroked and padded when an outline is given.
///
/// # Errors
///
/// Errors when nothing renders.
pub fn svg(
    cell: &Cell6d,
    scale: usize,
    outline: Option<Color>,
    width: usize,
    start: Option<usize>,
) -> Result<String> {
    let padding = if outline.is_some() { width as i64 } else { 0 };
    Ok(sheet(
        &window(cell, padding, start)?,
        scale,
        &stroke_of(outline, width),
    ))
}

/// Renders the hexagon tiled three by three and cropped to one interlocking rectangle as an SVG string.
///
/// # Errors
///
/// Errors for a cell that is not a hexagon, or when nothing renders.
pub fn rect_svg(cell: &Cell6d, scale: usize, start: Option<usize>) -> Result<String> {
    Ok(sheet(&rectangle(cell, start)?, scale, "stroke=\"none\""))
}

// RASTER

struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<[u8; 4]>,
}

impl Canvas {
    fn new(width: usize, height: usize) -> Canvas {
        Canvas {
            width,
            height,
            pixels: vec![[0, 0, 0, 0]; width * height],
        }
    }
    fn dot(&mut self, x: i64, y: i64, rgba: [u8; 4]) {
        if x < 0 || y < 0 || x >= self.width as i64 || y >= self.height as i64 {
            return;
        }
        self.pixels[y as usize * self.width + x as usize] = rgba;
    }
    fn span(&mut self, x0: i64, y: i64, x1: i64, rgba: [u8; 4]) {
        if y < 0 || y >= self.height as i64 {
            return;
        }
        let lo = x0.min(x1).max(0);
        let hi = x0.max(x1).min(self.width as i64 - 1);
        for x in lo..=hi {
            self.pixels[y as usize * self.width + x as usize] = rgba;
        }
    }
}

fn round_up(v: f64) -> i64 {
    if v >= 0.0 {
        (v + 0.5).floor() as i64
    } else {
        -((v.abs() + 0.5).floor() as i64)
    }
}

fn round_down(v: f64) -> i64 {
    if v >= 0.0 {
        (v - 0.5).ceil() as i64
    } else {
        -((v.abs() - 0.5).ceil() as i64)
    }
}

fn flood(canvas: &mut Canvas, points: &[(i64, i64)], rgba: [u8; 4]) {
    let n = points.len();
    let mut edges = Vec::with_capacity(n);
    let mut lo = canvas.height as i64 - 1;
    let mut hi = 0;
    for i in 0..n {
        let (x0, y0) = points[i];
        let (x1, y1) = points[(i + 1) % n];
        let (top, bottom) = (y0.min(y1), y0.max(y1));
        if top == bottom {
            canvas.span(x0, top, x1, rgba);
            continue;
        }
        lo = lo.min(top);
        hi = hi.max(bottom);
        edges.push((
            x0 as f64,
            y0,
            top,
            bottom,
            (x1 - x0) as f64 / (y1 - y0) as f64,
        ));
    }
    let mut crossings: Vec<f64> = Vec::with_capacity(edges.len());
    for y in lo.max(0)..=hi.min(canvas.height as i64) {
        crossings.clear();
        for &(x0, y0, top, bottom, slope) in &edges {
            if y >= top && y <= bottom {
                crossings.push((y - y0) as f64 * slope + x0);
            }
        }
        crossings.sort_by(|a, b| a.partial_cmp(b).unwrap());
        for pair in crossings.chunks_exact(2) {
            canvas.span(round_up(pair[0]), y, round_down(pair[1]), rgba);
        }
    }
}

fn trace(canvas: &mut Canvas, a: (i64, i64), b: (i64, i64), rgba: [u8; 4]) {
    let (mut x, mut y) = a;
    let (mut dx, mut dy) = (b.0 - a.0, b.1 - a.1);
    let xs = if dx < 0 { -1 } else { 1 };
    let ys = if dy < 0 { -1 } else { 1 };
    dx = dx.abs();
    dy = dy.abs();
    if dx == 0 {
        for _ in 0..dy {
            canvas.dot(x, y, rgba);
            y += ys;
        }
    } else if dy == 0 {
        for _ in 0..dx {
            canvas.dot(x, y, rgba);
            x += xs;
        }
    } else if dx > dy {
        let mut e = dy + dy - dx;
        for _ in 0..dx {
            canvas.dot(x, y, rgba);
            if e >= 0 {
                y += ys;
                e -= dx + dx;
            }
            e += dy + dy;
            x += xs;
        }
    } else {
        let mut e = dx + dx - dy;
        for _ in 0..dy {
            canvas.dot(x, y, rgba);
            if e >= 0 {
                x += xs;
                e -= dy + dy;
            }
            e += dx + dx;
            y += ys;
        }
    }
}

fn thick(canvas: &mut Canvas, a: (i64, i64), b: (i64, i64), width: usize, rgba: [u8; 4]) {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    if dx == 0 && dy == 0 {
        canvas.dot(a.0, a.1, rgba);
        return;
    }
    let long = ((dx * dx + dy * dy) as f64).sqrt();
    let short = (width as f64 - 1.0) / 2.0;
    let far = round_up(short) as f64 / long;
    let near = round_down(short) as f64 / long;
    let dxmin = round_down(near * dy as f64);
    let dxmax = round_down(far * dy as f64);
    let dymin = round_down(near * dx as f64);
    let dymax = round_down(far * dx as f64);
    flood(
        canvas,
        &[
            (a.0 - dxmin, a.1 + dymax),
            (b.0 - dxmin, b.1 + dymax),
            (b.0 + dxmax, b.1 - dymin),
            (a.0 + dxmax, a.1 - dymin),
        ],
        rgba,
    );
}

fn edge(canvas: &mut Canvas, points: &[(i64, i64)], width: usize, rgba: [u8; 4]) {
    let n = points.len();
    if width == 1 {
        for i in 0..n {
            trace(canvas, points[i], points[(i + 1) % n], rgba);
        }
        return;
    }
    let mark = [255, 255, 255, 255];
    let mut body = Canvas::new(canvas.width, canvas.height);
    flood(&mut body, points, mark);
    let mut band = Canvas::new(canvas.width, canvas.height);
    for i in 0..n {
        thick(
            &mut band,
            points[i],
            points[(i + 1) % n],
            width * 2 - 1,
            mark,
        );
    }
    for i in 0..canvas.pixels.len() {
        if body.pixels[i][3] != 0 && band.pixels[i][3] != 0 {
            canvas.pixels[i] = rgba;
        }
    }
}

fn canvas(view: &Rect, scale: usize, outline: Option<Color>, width: usize) -> Canvas {
    let mut canvas = Canvas::new(view.size.0 * scale, view.size.1 * scale);
    let ink = outline.map(|c| [c.r, c.g, c.b, c.a]);
    let mut points = Vec::with_capacity(3);
    for (corners, rgba) in &view.triangles {
        points.clear();
        points.extend(corners.iter().map(|(x, y)| {
            (
                (x - view.origin.0) * scale as i64,
                (y - view.origin.1) * scale as i64,
            )
        }));
        flood(&mut canvas, &points, *rgba);
        if let Some(ink) = ink {
            if ink != *rgba && width != 0 {
                edge(&mut canvas, &points, width, ink);
            }
        }
    }
    canvas
}

/// Rasters a cell's triangles to PNG bytes at the given scale, stroked and padded when an outline is given.
///
/// # Errors
///
/// Errors when nothing renders, when the scale is zero or when the encoder refuses the size.
pub fn png(cell: &Cell6d, scale: usize, outline: Option<Color>, width: usize) -> Result<Vec<u8>> {
    let padding = if outline.is_some() { width as i64 } else { 0 };
    let view = window(cell, padding, None)?;
    plate(&view, scale, outline, width)
}

/// Rasters the hexagon tiled three by three and cropped to one interlocking rectangle to PNG bytes.
///
/// # Errors
///
/// Errors for a cell that is not a hexagon, when the scale is zero or when the encoder refuses the size.
pub fn rect_png(cell: &Cell6d, scale: usize, start: Option<usize>) -> Result<Vec<u8>> {
    plate(&rectangle(cell, start)?, scale, None, 0)
}

fn plate(view: &Rect, scale: usize, outline: Option<Color>, width: usize) -> Result<Vec<u8>> {
    if scale < 1 {
        return value_error("scale must be at least 1.");
    }
    let raster = canvas(view, scale, outline, width);
    crate::core::png(&raster.pixels, raster.width, raster.height, 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::unpng;
    use crate::math::bang::Code;
    use crate::math::six::designs::iso_design;
    use crate::math::six::geometry::{blank, cut};
    use crate::math::six::models::Cell6d;
    use crate::math::six::{Orientation, Projection};
    use crate::math::three;
    fn carpet_cut() -> Cell6d {
        cut(&three::carpet(3, 1).unwrap()).unwrap()
    }
    #[test]
    fn triangle_geometry() {
        assert_eq!(north(0, 0), [(0, 2), (1, 0), (2, 2)]);
        assert_eq!(south(0, 0), [(0, 0), (1, 2), (2, 0)]);
        assert_eq!(east(1, 1), [(2, 1), (2, 3), (4, 2)]);
    }
    #[test]
    fn iso_renders_triangles() {
        let i = iso_design(Code(23), 3, 1, 2).unwrap();
        let tris = triangles(&i, None).unwrap();
        assert!(!tris.is_empty());
        let s = svg(&i, 10, None, 1, None).unwrap();
        assert!(s.contains("<polygon"));
        assert!(s.contains("stroke=\"none\""));
    }
    #[test]
    fn outline_strokes_and_pads_the_svg() {
        let i = iso_design(Code(23), 3, 1, 2).unwrap();
        let plain = svg(&i, 4, None, 1, None).unwrap();
        let lined = svg(&i, 4, Some(Color::rgba(255, 0, 0, 255)), 2, None).unwrap();
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
            blank(3, Orientation::Horizontal, 1, 0).unwrap(),
            Projection::Cut,
            Orientation::Horizontal,
            0,
        );
        let tris = triangles(&hex, None).unwrap();
        assert!(!tris.is_empty());
    }
    #[test]
    fn refuses_a_sheet_with_nothing_to_draw() {
        let bare = Cell6d::new(
            crate::math::two::Cell2d::new(crate::core::Tensor::full(
                vec![2, 3],
                crate::math::six::GRID,
            ))
            .unwrap(),
            Projection::Cut,
            Orientation::Horizontal,
            0,
        );
        assert!(triangles(&bare, None).unwrap().is_empty());
        assert!(svg(&bare, 4, None, 1, None).is_err());
    }
    #[test]
    fn a_start_override_flips_the_first_triangle() {
        let hex = carpet_cut();
        let plain = triangles(&hex, None).unwrap();
        let flipped = triangles(&hex, Some(1)).unwrap();
        assert_eq!(plain.len(), flipped.len());
        assert_ne!(plain[0].0, flipped[0].0);
        assert_eq!(plain[0].0, north(2, 0));
        assert_eq!(flipped[0].0, south(2, 0));
    }
    #[test]
    fn png_matches_the_drawn_size_and_fill() {
        let bytes = png(&carpet_cut(), 10, None, 0).unwrap();
        let (width, height, pixels) = unpng(&bytes).unwrap();
        assert_eq!((width, height), (120, 120));
        assert_eq!(pixels.iter().filter(|p| p[3] != 0).count(), 10859);
    }
    #[test]
    fn rect_svg_carries_the_cropped_window() {
        let s = rect_svg(&carpet_cut(), 10, None).unwrap();
        assert!(s.starts_with("<svg width=\"180\" height=\"120\" viewBox=\"0 0 180 120\""));
        let (width, height, pixels) = unpng(&rect_png(&carpet_cut(), 10, None).unwrap()).unwrap();
        assert_eq!((width, height), (180, 120));
        assert_eq!(pixels.iter().filter(|p| p[3] != 0).count(), 21600);
    }
    #[test]
    fn an_outlined_png_pads_and_strokes() {
        let (width, height, pixels) =
            unpng(&png(&carpet_cut(), 10, Some(Color::rgba(255, 0, 0, 255)), 2).unwrap()).unwrap();
        assert_eq!((width, height), (160, 160));
        assert!(pixels.contains(&[255, 0, 0, 255]));
    }
}
