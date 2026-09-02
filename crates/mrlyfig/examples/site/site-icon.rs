use mrlycore::errors::Result;
use mrlycore::Color;
use mrlyfig::{grid, ink, save, Board, Grid};

fn main() -> Result<()> {
    let mut board = Board::new(512, 512, Color::rgb(0x0b, 0x0d, 0x10));
    let frame = board.frame(0.12);
    let mark = Grid::new(frame, 5, 5, 0.0);
    mark.carpet(&mut board, &grid::mask(&grid::LOGO, 1), ink::FG);
    save("site-icon", &board)?;
    Ok(())
}
