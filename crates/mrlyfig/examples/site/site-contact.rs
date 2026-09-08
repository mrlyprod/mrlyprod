use mrlycore::errors::Result;
use mrlyfig::{grid, ink, save, Board, Frame, Grid};

const CELL: f64 = 55.0;

const LETTER: [&str; 10] = [
    "11111111111111",
    "10000000000001",
    "10000000000001",
    "10000000000001",
    "10000000000001",
    "10000000000001",
    "10000000000001",
    "10000000000001",
    "10000000000001",
    "11111111111111",
];

fn main() -> Result<()> {
    let mut board = Board::square();
    let cols = LETTER[0].len();
    let rows = LETTER.len();
    let w = CELL * cols as f64;
    let h = CELL * rows as f64;
    let frame = Frame::new(
        ((board.width as f64 - w) / 2.0).round(),
        ((board.height as f64 - h) / 2.0).round(),
        w,
        h,
    );
    let cells = Grid::new(frame, cols, rows, 0.0);
    cells.carpet(&mut board, &grid::mask(&LETTER, 1), ink::fg());
    for step in 0..6 {
        cells.fill(&mut board, 1 + step, 1 + step, ink::blue());
        cells.fill(&mut board, 12 - step, 1 + step, ink::blue());
    }
    save("site-contact", &board)?;
    Ok(())
}
