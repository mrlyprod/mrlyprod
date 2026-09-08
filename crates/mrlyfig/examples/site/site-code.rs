use mrlycore::errors::Result;
use mrlyfig::{grid, ink, save, Board, Frame, Grid};

const CELL: f64 = 51.0;

const CODE: [&str; 9] = [
    "000100000101000",
    "001100001101100",
    "011000001000110",
    "110000011000011",
    "100000010000001",
    "110000110000011",
    "011000100000110",
    "001101100001100",
    "000101000001000",
];

fn main() -> Result<()> {
    let mut board = Board::square();
    let cols = CODE[0].len();
    let rows = CODE.len();
    let (w, h) = (CELL * cols as f64, CELL * rows as f64);
    let x = ((board.width as f64 - w) / 2.0).round();
    let y = ((board.height as f64 - h) / 2.0).round();
    let cells = Grid::new(Frame::new(x, y, w, h), cols, rows, 0.0);
    cells.carpet(&mut board, &grid::mask(&CODE, 1), ink::fg());
    for row in 0..rows {
        let col = 9 - row / 2;
        cells.fill(&mut board, col, row, ink::blue());
        if row % 2 == 1 {
            cells.fill(&mut board, col - 1, row, ink::blue());
        }
    }
    save("site-code", &board)?;
    Ok(())
}
