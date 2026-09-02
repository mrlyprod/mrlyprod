use mrlycore::errors::Result;
use mrlycore::Color;
use mrlyfig::board::Frame;
use mrlyfig::{ink, save, Board, Grid};
use mrlymath::bang::{magic, word, MagicLayer};
use mrlymath::name::Bang;
use mrlymath::two::Cell2d;

const SIDE: usize = 15;

fn letters(first: (u128, usize), second: (u128, usize)) -> [MagicLayer; 2] {
    [
        MagicLayer::new(Bang::new(first.0, 2, 2), first.1),
        MagicLayer::new(Bang::new(second.0, 2, 2), second.1),
    ]
}

fn panel(board: &mut Board, frame: Frame, cells: &Cell2d, color: Color) {
    Grid::new(frame, SIDE, SIDE, 0.09).paint(board, cells, |kind| (kind != 0).then_some(color));
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let gutter = area.w * 0.055;
    let side = (area.w - gutter) / 2.0;
    let top = area.y + (area.h - side) / 2.0;

    let forward = letters((7, 3), (14, 5));
    let reverse = letters((14, 5), (7, 3));
    assert_eq!(word::side(&forward)?, SIDE as u128);
    assert_eq!(word::side(&reverse)?, SIDE as u128);
    assert_eq!(word::fill(&forward)?, word::fill(&reverse)?);

    let left = Cell2d::new(magic(&forward)?);
    let right = Cell2d::new(magic(&reverse)?);
    assert_eq!((left.width(), left.height()), (SIDE, SIDE));
    assert_eq!((right.width(), right.height()), (SIDE, SIDE));
    assert_eq!(left.types().sum(), right.types().sum());
    assert_ne!(left.types(), right.types());

    panel(
        &mut board,
        Frame::new(area.x, top, side, side),
        &left,
        ink::BLUE,
    );
    panel(
        &mut board,
        Frame::new(area.x + side + gutter, top, side, side),
        &right,
        ink::ORANGE,
    );
    save("research-magic", &board)?;
    Ok(())
}
