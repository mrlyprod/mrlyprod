use mrlycore::errors::Result;
use mrlyfig::board::{Board, Frame};
use mrlyfig::{ink, save, Grid};
use mrlymath::two::designs;

fn panel(board: &mut Board, frame: Frame, level: usize, gap: f64) -> Result<()> {
    let cells = designs::create(495, 3, level, 0, 3)?;
    let side = cells.width();
    assert_eq!(side, 3usize.pow(level as u32));
    Grid::new(frame, side, side, gap).paint(board, &cells, |kind| (kind != 0).then_some(ink::GOLD));
    Ok(())
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let gutter = area.w * 0.036;
    let side = (area.w - gutter) / 2.0;
    let gaps = [0.16, 0.10, 0.055, 0.0];
    for (slot, gap) in gaps.iter().enumerate() {
        let x = area.x + (slot % 2) as f64 * (side + gutter);
        let y = area.y + (slot / 2) as f64 * (side + gutter);
        panel(&mut board, Frame::new(x, y, side, side), slot + 1, *gap)?;
    }
    save("research-index", &board)?;
    Ok(())
}
