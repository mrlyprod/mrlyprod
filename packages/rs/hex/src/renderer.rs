use super::Orientation;
use super::geometry::{is_hex, orientation, tile};
use super::models::CellHex;
use mrly_color::Color;
use mrly_error::{Result, value_error};
use mrly_image::Image;
use mrly_math::cell::base::{Mode, mapping};
use mrly_two::Cell2d;

pub type Triangle = ([(i64, i64); 3], [u8; 4]);

#[derive(Clone, Debug)]
struct Rect {
    triangles: Vec<Triangle>,
    origin: (i64, i64),
    size: (usize, usize),
}

pub fn north(x: i64, y: i64) -> [(i64, i64); 3] {
    [(x, 2 * y + 2), (x + 1, 2 * y), (x + 2, 2 * y + 2)]
}

pub fn south(x: i64, y: i64) -> [(i64, i64); 3] {
    [(x, 2 * y), (x + 1, 2 * y + 2), (x + 2, 2 * y)]
}

pub fn east(x: i64, y: i64) -> [(i64, i64); 3] {
    [(2 * x, y), (2 * x, y + 2), (2 * x + 2, y + 1)]
}

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

fn parity(cell: &CellHex, start: Option<usize>) -> i64 {
    match start {
        Some(s) => s as i64,
        None => i64::from(cell.start),
    }
}

pub fn triangles(cell: &CellHex, start: Option<usize>) -> Result<Vec<Triangle>> {
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

fn window(cell: &CellHex, padding: i64, start: Option<usize>) -> Result<Rect> {
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

fn rectangle(cell: &CellHex, start: Option<usize>) -> Result<Rect> {
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

pub fn svg(
    cell: &CellHex,
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

pub fn rect_svg(cell: &CellHex, scale: usize, start: Option<usize>) -> Result<String> {
    Ok(sheet(&rectangle(cell, start)?, scale, "stroke=\"none\""))
}

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
        for x in x0.max(0)..=x1.min(self.width as i64 - 1) {
            self.pixels[y as usize * self.width + x as usize] = rgba;
        }
    }
}

struct Edge {
    x0: f64,
    y0: i64,
    top: i64,
    bottom: i64,
    slope: f64,
}

impl Edge {
    fn at(&self, y: i64) -> f64 {
        (y - self.y0) as f64 * self.slope + self.x0
    }
    fn holds(&self, y: i64) -> bool {
        y >= self.top && y <= self.bottom
    }
    fn ends(&self, y: i64) -> bool {
        y == self.top || y == self.bottom
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

fn corner(earlier: &[Edge], edge: &Edge, y: i64, x: f64) -> f64 {
    let step = if y == edge.bottom { y - 1 } else { y + 1 };
    for other in earlier {
        if !other.ends(y) || other.slope == 0.0 || x.round() != other.at(y).round() {
            continue;
        }
        if !other.holds(step) {
            continue;
        }
        let (mine, theirs) = (edge.at(step), other.at(step));
        if x > mine + 1.0 && x > theirs + 1.0 {
            return mine.max(theirs).round() + 1.0;
        }
        if x < mine - 1.0 && x < theirs - 1.0 {
            return mine.min(theirs).round() - 1.0;
        }
        return x;
    }
    x
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
        lo = lo.min(top);
        hi = hi.max(bottom);
        if top == bottom {
            canvas.span(x0.min(x1), top, x0.max(x1), rgba);
            continue;
        }
        edges.push(Edge {
            x0: x0 as f64,
            y0,
            top,
            bottom,
            slope: (x1 - x0) as f64 / (y1 - y0) as f64,
        });
    }
    let hi = hi.min(canvas.height as i64);
    let mut crossings: Vec<f64> = Vec::with_capacity(edges.len() * 2);
    for y in lo.max(0)..=hi {
        crossings.clear();
        for (i, edge) in edges.iter().enumerate() {
            if !edge.holds(y) {
                continue;
            }
            let x = edge.at(y);
            if y == edge.bottom && y < hi {
                crossings.extend([x, x]);
            } else if edge.ends(y) && edge.slope != 0.0 {
                crossings.push(corner(&edges[..i], edge, y, x));
            } else {
                crossings.push(x);
            }
        }
        crossings.sort_by(f64::total_cmp);
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
        if let Some(ink) = ink
            && ink != *rgba
            && width != 0
        {
            edge(&mut canvas, &points, width, ink);
        }
    }
    canvas
}

pub fn image(cell: &CellHex, scale: usize, outline: Option<Color>, width: usize) -> Result<Image> {
    let padding = if outline.is_some() { width as i64 } else { 0 };
    let view = window(cell, padding, None)?;
    plate(&view, scale, outline, width)
}

pub fn rect_image(cell: &CellHex, scale: usize, start: Option<usize>) -> Result<Image> {
    plate(&rectangle(cell, start)?, scale, None, 0)
}

fn plate(view: &Rect, scale: usize, outline: Option<Color>, width: usize) -> Result<Image> {
    if scale < 1 {
        return value_error("scale must be at least 1.");
    }
    let raster = canvas(view, scale, outline, width);
    Ok(Image {
        width: raster.width,
        height: raster.height,
        colors: raster.pixels,
    })
}
