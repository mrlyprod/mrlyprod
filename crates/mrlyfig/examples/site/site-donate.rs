use mrlycore::errors::Result;
use mrlyfig::{grid, ink, save, Board, Frame, Grid};

const CELL: f64 = 70.0;

const HEART: [&str; 10] = [
    "01100000110",
    "11110001111",
    "11111111111",
    "11111111111",
    "11111111111",
    "01111111110",
    "00111111100",
    "00011111000",
    "00001110000",
    "00000100000",
];

fn main() -> Result<()> {
    let mut board = Board::square();
    let cols = HEART[0].len();
    let rows = HEART.len();
    let w = CELL * cols as f64;
    let h = CELL * rows as f64;
    let frame = Frame::new(
        ((board.width as f64 - w) / 2.0).round(),
        ((board.height as f64 - h) / 2.0).round(),
        w,
        h,
    );
    let cells = Grid::new(frame, cols, rows, 0.0);
    cells.carpet(&mut board, &grid::mask(&HEART, 1), ink::fg());
    for (col, row) in [(2, 2), (3, 2), (2, 3)] {
        cells.fill(&mut board, col, row, ink::red());
    }
    save("site-donate", &board)?;
    Ok(())
}
