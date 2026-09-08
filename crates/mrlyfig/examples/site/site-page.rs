use mrlycore::errors::Result;
use mrlyfig::{grid, ink, save, Board, Frame, Grid};

const CELL: f64 = 70.0;

const SHEET: [&str; 13] = [
    "11111111000",
    "10000000100",
    "10000000010",
    "10000000001",
    "10000000001",
    "10000000001",
    "10000000001",
    "10000000001",
    "10000000001",
    "10000000001",
    "10000000001",
    "10000000001",
    "11111111111",
];

fn main() -> Result<()> {
    let mut board = Board::square();
    let cols = SHEET[0].len();
    let rows = SHEET.len();
    let (w, h) = (CELL * cols as f64, CELL * rows as f64);
    let x = ((board.width as f64 - w) / 2.0).round();
    let y = ((board.height as f64 - h) / 2.0).round();
    let cells = Grid::new(Frame::new(x, y, w, h), cols, rows, 0.0);
    cells.carpet(&mut board, &grid::mask(&SHEET, 1), ink::fg());
    for (row, end) in [(4, 9), (6, 9), (8, 7)] {
        for col in 2..end {
            cells.fill(&mut board, col, row, ink::blue());
        }
    }
    save("site-page", &board)?;
    Ok(())
}
