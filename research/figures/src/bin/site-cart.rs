use mrlycore::errors::Result;
use mrlyfig::{grid, ink, save, Board, Frame, Grid};

const CELL: f64 = 55.0;

const CART: [&str; 11] = [
    "11000000000000",
    "00100000000000",
    "00010000000000",
    "00001111111111",
    "00001000000001",
    "00001000000001",
    "00000100000010",
    "00000011111100",
    "00000010000100",
    "00000111001110",
    "00000010000100",
];

fn main() -> Result<()> {
    let mut board = Board::square();
    let cols = CART[0].len();
    let rows = CART.len();
    let (w, h) = (CELL * cols as f64, CELL * rows as f64);
    let x = ((board.width as f64 - w) / 2.0).round();
    let y = ((board.height as f64 - h) / 2.0).round();
    let cells = Grid::new(Frame::new(x, y, w, h), cols, rows, 0.0);
    cells.carpet(&mut board, &grid::mask(&CART, 1), ink::fg());
    for col in [5, 6, 7, 10, 11, 12] {
        cells.fill(&mut board, col, 9, ink::orange());
    }
    for row in [8, 10] {
        cells.fill(&mut board, 6, row, ink::orange());
        cells.fill(&mut board, 11, row, ink::orange());
    }
    save("site-cart", &board)?;
    Ok(())
}
