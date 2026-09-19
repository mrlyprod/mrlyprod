use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board, Grid};
use mrlymath::two::designs;

const CODE: u128 = 495;
const LEVEL: usize = 4;

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let design = designs::create(CODE, 3, LEVEL, 0, 3)?;
    let side = design.width();
    assert_eq!(side, 81);
    assert_eq!(design.types().sum(), 4096);
    Grid::new(area, side, side, 0.0)
        .paint(&mut board, &design, |kind| (kind != 0).then_some(ink::blue()));
    save("wiki-sierpinski-carpet", &board)?;
    Ok(())
}
