use mrlyfig::{ink, save, Board, Grid};
use mrlyrs::core::errors::Result;

const SIDE: usize = 6;

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let grid = Grid::new(area, SIDE, SIDE, 0.05);
    let inks = [ink::blue(), ink::yellow(), ink::orange(), ink::green()];
    let mut counts = [0usize; 4];
    for row in 0..SIDE {
        for col in 0..SIDE {
            let corner = 2 * (row % 2) + col % 2;
            grid.fill(&mut board, col, row, inks[corner]);
            counts[corner] += 1;
        }
    }
    assert_eq!(counts, [9, 9, 9, 9]);
    save("wiki-parity", &board)?;
    Ok(())
}
