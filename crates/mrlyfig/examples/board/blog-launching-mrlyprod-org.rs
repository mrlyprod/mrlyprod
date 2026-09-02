use mrlycore::errors::Result;
use mrlycore::Color;
use mrlyfig::{ink, save, Board, Grid};

const SIDE: usize = 8;
const GUTTER: f64 = 3.0;
const EDGES: [usize; 4] = [28, 40, 60, 64];
const TONES: [Color; 4] = [ink::BLUE, ink::GOLD, ink::GREEN, ink::GROUND];

fn band(index: usize) -> usize {
    EDGES.iter().position(|edge| index < *edge).unwrap_or(3)
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    board.rect(frame.x, frame.y, frame.w, frame.h, ink::LINE);
    let grid = Grid::new(frame, SIDE, SIDE, 0.0);
    let mut counts = [0usize; 4];
    for row in 0..SIDE {
        for col in 0..SIDE {
            let index = row * SIDE + col;
            let slot = band(index);
            let (x, y, w, h) = grid.cell(col, row);
            board.rect(
                x + GUTTER / 2.0,
                y + GUTTER / 2.0,
                w - GUTTER,
                h - GUTTER,
                TONES[slot],
            );
            counts[slot] += 1;
        }
    }
    assert_eq!(counts, [28, 12, 20, 4]);
    save("blog-launching-mrlyprod-org", &board)?;
    Ok(())
}
