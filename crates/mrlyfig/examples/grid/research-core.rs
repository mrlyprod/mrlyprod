use mrlycore::errors::Result;
use mrlyfig::board::{Board, Frame};
use mrlyfig::{ink, save, Grid};
use mrlymath::bang;
use mrlymath::two::designs;

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let universe = bang::bang(2);
    assert_eq!(universe.total, 16);
    assert_eq!(universe.distinct(), 6);
    let canonical: Vec<u128> = universe.canonical().iter().map(|d| d.i).collect();
    assert_eq!(canonical, vec![0, 1, 3, 6, 7, 15]);
    let gutter = area.w * 0.030;
    let side = (area.w - 3.0 * gutter) / 4.0;
    for code in 0..16u128 {
        let design = universe.design(code);
        let color = if design.canonical {
            ink::GOLD
        } else {
            ink::BLUE
        };
        let x = area.x + (code % 4) as f64 * (side + gutter);
        let y = area.y + (code / 4) as f64 * (side + gutter);
        let frame = Frame::new(x, y, side, side);
        board.rect(frame.x, frame.y, frame.w, frame.h, ink::PANEL);
        let cells = designs::create(code, 2, 4, 0, 2)?;
        assert_eq!(cells.width(), 16);
        Grid::new(frame, 16, 16, 0.0)
            .paint(&mut board, &cells, |kind| (kind != 0).then_some(color));
    }
    save("research-core", &board)?;
    Ok(())
}
