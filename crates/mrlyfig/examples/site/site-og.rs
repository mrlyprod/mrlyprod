use mrlycore::errors::Result;
use mrlyfig::{grid, ink, save, Board, Frame, Grid};

const CELL: f64 = 18.0;
const ACROSS: usize = 10;
const DOWN: usize = 5;

fn main() -> Result<()> {
    let mut board = Board::og();
    let seed = grid::mask(&grid::LOGO, 1);
    let cols = 6 * ACROSS - 1;
    let rows = 6 * DOWN - 1;
    let mut mask = vec![vec![false; cols]; rows];
    for (row, line) in mask.iter_mut().enumerate() {
        for (col, on) in line.iter_mut().enumerate() {
            *on = row % 6 < 5 && col % 6 < 5 && seed[row % 6][col % 6];
        }
    }
    let w = CELL * cols as f64;
    let h = CELL * rows as f64;
    let frame = Frame::new(
        ((board.width as f64 - w) / 2.0).round(),
        ((board.height as f64 - h) / 2.0).round(),
        w,
        h,
    );
    Grid::new(frame, cols, rows, 0.0).carpet(&mut board, &mask, ink::FG);
    save("site-og", &board)?;
    Ok(())
}
