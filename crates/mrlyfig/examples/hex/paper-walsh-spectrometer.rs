use mrlycore::errors::Result;
use mrlycore::Color;
use mrlyfig::board::{Board, Frame};
use mrlyfig::{hex, ink, save};
use mrlymath::six;
use mrlymath::six::Cell6d;

const SIDE: usize = 7;

fn at(cell: &Cell6d, row: usize, col: usize) -> u8 {
    let offset = (cell.width() - hex::row_len(SIDE, row)) / 2;
    cell.cell.types().get(&[row, col + offset])
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
    let slice = six::cut_design(105, SIDE, 1, 2)?;
    assert_eq!(hex::count(SIDE), 294);
    assert_eq!(six::fills(&slice), 72);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    hexagon(&mut board, frame, SIDE, 0.0, |_, _| Some(ink::LINE));
    hexagon(&mut board, frame, SIDE, 2.6, |row, col| {
        if at(&slice, row, col) == six::FILL {
            None
        } else {
            Some(ink::GROUND)
        }
    });
    hexagon(&mut board, frame, SIDE, 2.6, |row, col| {
        if at(&slice, row, col) == six::FILL {
            Some(ink::BLUE)
        } else {
            None
        }
    });
    save("paper-walsh-spectrometer", &board)?;
    Ok(())
}
