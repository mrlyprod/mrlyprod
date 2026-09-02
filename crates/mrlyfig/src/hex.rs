use crate::board::{Board, Frame};
use mrlycore::errors::Result;
use mrlycore::Color;
use mrlymath::six::geometry::orientation;
use mrlymath::six::Cell6d;
use mrlymath::six::Orientation;

// GEOMETRY

const RATIO: f64 = 0.866_025_403_784_438_6;

fn north(x: i64, y: i64) -> [(f64, f64); 3] {
    let (x, y) = (x as f64, y as f64);
    [
        (x, 2.0 * y + 2.0),
        (x + 1.0, 2.0 * y),
        (x + 2.0, 2.0 * y + 2.0),
    ]
}

fn south(x: i64, y: i64) -> [(f64, f64); 3] {
    let (x, y) = (x as f64, y as f64);
    [(x, 2.0 * y), (x + 1.0, 2.0 * y + 2.0), (x + 2.0, 2.0 * y)]
}

fn east(x: i64, y: i64) -> [(f64, f64); 3] {
    let (x, y) = (x as f64, y as f64);
    [(2.0 * x, y), (2.0 * x, y + 2.0), (2.0 * x + 2.0, y + 1.0)]
}

fn west(x: i64, y: i64) -> [(f64, f64); 3] {
    let (x, y) = (x as f64, y as f64);
    [
        (2.0 * x + 2.0, y),
        (2.0 * x + 2.0, y + 2.0),
        (2.0 * x, y + 1.0),
    ]
}

fn shrink(pts: &[(f64, f64); 3], gap: f64) -> [(f64, f64); 3] {
    let cx = (pts[0].0 + pts[1].0 + pts[2].0) / 3.0;
    let cy = (pts[0].1 + pts[1].1 + pts[2].1) / 3.0;
    let side = |a: (f64, f64), b: (f64, f64)| ((b.0 - a.0).powi(2) + (b.1 - a.1).powi(2)).sqrt();
    let perimeter = side(pts[0], pts[1]) + side(pts[1], pts[2]) + side(pts[2], pts[0]);
    let area = ((pts[1].0 - pts[0].0) * (pts[2].1 - pts[0].1)
        - (pts[2].0 - pts[0].0) * (pts[1].1 - pts[0].1))
        .abs()
        / 2.0;
    let inradius = 2.0 * area / perimeter;
    let k = if inradius > 0.0 {
        ((inradius - gap) / inradius).max(0.0)
    } else {
        0.0
    };
    let pull = |p: (f64, f64)| (cx + (p.0 - cx) * k, cy + (p.1 - cy) * k);
    [pull(pts[0]), pull(pts[1]), pull(pts[2])]
}

fn fit(mesh: &[[(f64, f64); 3]], frame: Frame) -> impl Fn((f64, f64)) -> (f64, f64) {
    let mut lo = (f64::MAX, f64::MAX);
    let mut hi = (f64::MIN, f64::MIN);
    for tri in mesh {
        for p in tri {
            lo.0 = lo.0.min(p.0);
            lo.1 = lo.1.min(p.1);
            hi.0 = hi.0.max(p.0);
            hi.1 = hi.1.max(p.1);
        }
    }
    let (span_x, span_y) = ((hi.0 - lo.0).max(1e-9), (hi.1 - lo.1).max(1e-9));
    let scale = (frame.w / span_x).min(frame.h / span_y);
    let (ox, oy) = (
        frame.x + (frame.w - span_x * scale) / 2.0,
        frame.y + (frame.h - span_y * scale) / 2.0,
    );
    move |p: (f64, f64)| (ox + (p.0 - lo.0) * scale, oy + (p.1 - lo.1) * scale)
}

// DRAWING

/// Draws the triangle mesh of a hex slice into the frame, centred and equilateral.
///
/// The mesh is read straight from the cell's triangle grid: site (row, column) becomes one
/// unit triangle whose parity alternates from the cell's start, and the ink maps its type
/// byte to a color or to nothing. The gap is the number of pixels each triangle is pulled
/// back from its own edges.
pub fn draw(
    board: &mut Board,
    frame: Frame,
    cell: &Cell6d,
    gap: f64,
    ink: impl Fn(u8) -> Option<Color>,
) -> Result<()> {
    let (width, height) = (cell.width(), cell.height());
    let orient = orientation(width, height)?;
    let types = cell.cell.types();
    let start = cell.start as i64;
    let mut mesh = Vec::new();
    let mut paint = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let color = match ink(types.get(&[y, x])) {
                Some(color) => color,
                None => continue,
            };
            let flip = (x as i64 + y as i64 + start).rem_euclid(2);
            let points = match (orient, flip) {
                (Orientation::Horizontal, 0) => north(x as i64, y as i64),
                (Orientation::Horizontal, _) => south(x as i64, y as i64),
                (Orientation::Vertical, 0) => east(x as i64, y as i64),
                (Orientation::Vertical, _) => west(x as i64, y as i64),
            };
            let squash = |p: (f64, f64)| match orient {
                Orientation::Horizontal => (p.0, p.1 * RATIO),
                Orientation::Vertical => (p.0 * RATIO, p.1),
            };
            mesh.push([squash(points[0]), squash(points[1]), squash(points[2])]);
            paint.push(color);
        }
    }
    if mesh.is_empty() {
        return Ok(());
    }
    let place = fit(&mesh, frame);
    for (tri, color) in mesh.iter().zip(paint) {
        let screen = [place(tri[0]), place(tri[1]), place(tri[2])];
        let small = shrink(&screen, gap);
        board.triangle(small[0], small[1], small[2], color);
    }
    Ok(())
}

/// Returns the number of unit triangles in a plain hexagon of side n, which is six n squared.
pub fn count(n: usize) -> usize {
    6 * n * n
}

/// Returns the number of triangles in one row of a plain hexagon of side n, rows counted from the top.
pub fn row_len(n: usize, row: usize) -> usize {
    let reach = if row < n { row } else { 2 * n - 1 - row };
    2 * (n + reach) + 1
}

/// Draws a plain hexagon of side n cells, six n squared unit triangles, centred in the frame.
///
/// A triangle is addressed by its row from the top, its column from the left of that row, and
/// one for a triangle pointing up or zero for one pointing down. The gap is the number of
/// pixels each triangle is pulled back from its own edges.
pub fn hexagon(
    board: &mut Board,
    frame: Frame,
    n: usize,
    gap: f64,
    ink: impl Fn(usize, usize, usize) -> Option<Color>,
) {
    if n == 0 {
        return;
    }
    let side = (frame.w / (2 * n) as f64).min(frame.h / (n as f64 * 2.0 * RATIO));
    let rise = side * RATIO;
    let (cx, cy) = frame.center();
    let left = cx - side * n as f64;
    let top = cy - rise * n as f64;
    for row in 0..2 * n {
        let reach = if row < n { row } else { 2 * n - 1 - row };
        let up = row < n;
        let (long, short) = (n + reach + 1, n + reach);
        let (top_len, bot_len) = if up { (short, long) } else { (long, short) };
        let tx = left + (2 * n - top_len) as f64 * side / 2.0;
        let bx = left + (2 * n - bot_len) as f64 * side / 2.0;
        let (y0, y1) = (top + row as f64 * rise, top + (row + 1) as f64 * rise);
        for col in 0..row_len(n, row) {
            let points = if (col % 2 == 0) == up {
                let j = (col / 2) as f64;
                [
                    (bx + j * side, y1),
                    (bx + (j + 1.0) * side, y1),
                    (bx + (j + 0.5) * side, y0),
                ]
            } else {
                let i = (col / 2) as f64;
                [
                    (tx + i * side, y0),
                    (tx + (i + 1.0) * side, y0),
                    (tx + (i + 0.5) * side, y1),
                ]
            };
            let parity = usize::from((col % 2 == 0) == up);
            if let Some(color) = ink(row, col, parity) {
                let small = shrink(&points, gap);
                board.triangle(small[0], small[1], small[2], color);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ink;
    #[test]
    fn a_hexagon_of_side_s_has_six_s_squared_triangles() {
        for n in 1..8 {
            let rows: usize = (0..2 * n).map(|row| row_len(n, row)).sum();
            assert_eq!(rows, count(n));
        }
    }
    #[test]
    fn every_triangle_of_a_hexagon_is_offered_to_the_ink() {
        let mut board = Board::new(128, 128, ink::GROUND);
        let frame = board.frame(0.1);
        let seen = std::cell::Cell::new(0usize);
        hexagon(&mut board, frame, 3, 0.0, |_, _, _| {
            seen.set(seen.get() + 1);
            None
        });
        assert_eq!(seen.get(), count(3));
    }
}
