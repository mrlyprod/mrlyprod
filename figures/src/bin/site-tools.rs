use figures::{ink, save, Board, Grid};
use mrlyrs::core::error::Result;
use mrlyrs::math::two::designs;

const SIDE: usize = 81;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let sheet = designs::create(7, 3, 4, 0, 2)?;
    assert_eq!(sheet.width(), SIDE);
    assert_eq!(sheet.types().sum(), 4096);
    Grid::new(frame, SIDE, SIDE, 0.0)
        .paint(&mut board, &sheet, |kind| (kind != 0).then_some(ink::dim()));
    save("site-tools", &board)?;
    Ok(())
}
