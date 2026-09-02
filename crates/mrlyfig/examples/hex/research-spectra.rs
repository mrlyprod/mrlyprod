use mrlycore::errors::Result;
use mrlycore::Color;
use mrlyfig::board::{Board, Frame};
use mrlyfig::{hex, ink, save};
use mrlymath::six;
use mrlymath::six::Cell6d;
use mrlymath::three;

const BASE: usize = 5;
const LEVEL: usize = 2;
const SIDE: usize = 25;
const GRAIN: usize = 4;

fn at(cell: &Cell6d, row: usize, col: usize) -> u8 {
    let offset = (cell.width() - hex::row_len(SIDE, row)) / 2;
    cell.cell.types().get(&[row, col + offset])
}

fn source(row: usize, col: usize) -> usize {
    let size = GRAIN * SIDE;
    let k = (3 * (size - 1)) / 2;
    let z = 2 * row;
    let target = k - z;
    let min_x = target.saturating_sub(size - 1);
    let x = min_x + col;
    let y = target - x;
    x / GRAIN + y / GRAIN + z / GRAIN
}

fn is_hexagon(row: usize, col: usize) -> bool {
    let size = GRAIN * SIDE;
    let k = (3 * (size - 1)) / 2;
    source(row, col) + 1 == k / GRAIN
}

const RATIO: f64 = 0.866_025_403_784_438_6;

fn shrink(pts: [(f64, f64); 3], gap: f64) -> [(f64, f64); 3] {
    let cx = (pts[0].0 + pts[1].0 + pts[2].0) / 3.0;
    let cy = (pts[0].1 + pts[1].1 + pts[2].1) / 3.0;
    let edge = ((pts[1].0 - pts[0].0).powi(2) + (pts[1].1 - pts[0].1).powi(2)).sqrt();
    let inradius = edge / (2.0 * 3f64.sqrt());
    let k = ((inradius - gap) / inradius).max(0.0);
    let pull = |p: (f64, f64)| (cx + (p.0 - cx) * k, cy + (p.1 - cy) * k);
    [pull(pts[0]), pull(pts[1]), pull(pts[2])]
}

fn hexagon(
    board: &mut Board,
    frame: Frame,
    n: usize,
    gap: f64,
    ink: impl Fn(usize, usize) -> Option<Color>,
) {
    let edge = (frame.w / (2 * n) as f64).min(frame.h / (n as f64 * 2.0 * RATIO));
    let rise = edge * RATIO;
    let (cx, cy) = frame.center();
    let left = cx - edge * n as f64;
    let top = cy - rise * n as f64;
    for row in 0..2 * n {
        let reach = if row < n { row } else { 2 * n - 1 - row };
        let up = row < n;
        let (long, short) = (n + reach + 1, n + reach);
        let (top_len, bot_len) = if up { (short, long) } else { (long, short) };
        let tx = left + (2 * n - top_len) as f64 * edge / 2.0;
        let bx = left + (2 * n - bot_len) as f64 * edge / 2.0;
        let (y0, y1) = (top + row as f64 * rise, top + (row + 1) as f64 * rise);
        for col in 0..top_len + bot_len {
            let points = if (col % 2 == 0) == up {
                let j = (col / 2) as f64;
                [
                    (bx + j * edge, y1),
                    (bx + (j + 1.0) * edge, y1),
                    (bx + (j + 0.5) * edge, y0),
                ]
            } else {
                let i = (col / 2) as f64;
                [
                    (tx + i * edge, y0),
                    (tx + (i + 1.0) * edge, y0),
                    (tx + (i + 0.5) * edge, y1),
                ]
            };
            if let Some(color) = ink(row, col) {
                let small = shrink(points, gap);
                board.triangle(small[0], small[1], small[2], color);
            }
        }
    }
}

fn main() -> Result<()> {
    let slice = six::cut(&three::carpet(BASE, LEVEL)?)?;
    assert_eq!(slice.height(), 2 * SIDE);
    let mut hexagons = 0usize;
    let mut triangles = 0usize;
    for row in 0..2 * SIDE {
        for col in 0..hex::row_len(SIDE, row) {
            if at(&slice, row, col) != six::FILL {
                continue;
            }
            if is_hexagon(row, col) {
                hexagons += 1;
            } else {
                triangles += 1;
            }
        }
    }
    assert_eq!(hexagons, 139 * 6);
    assert_eq!(triangles, 330);
    assert_eq!(hexagons + triangles, six::fills(&slice));
    assert_eq!(hex::count(SIDE), 3750);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    hexagon(&mut board, frame, SIDE, 0.0, |row, col| {
        if at(&slice, row, col) != six::FILL {
            return None;
        }
        if is_hexagon(row, col) {
            Some(ink::GOLD)
        } else {
            None
        }
    });
    hexagon(&mut board, frame, SIDE, 1.2, |row, col| {
        if at(&slice, row, col) == six::FILL && !is_hexagon(row, col) {
            Some(ink::BLUE)
        } else {
            None
        }
    });
    save("research-spectra", &board)?;
    Ok(())
}
