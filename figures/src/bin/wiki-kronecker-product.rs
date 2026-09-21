use mrlyfig::{ink, save, Board, Frame, Grid};
use mrlyrs::core::errors::Result;
use mrlyrs::math::two::designs;

const CODE: u128 = 7;
const LEVELS: [usize; 3] = [1, 2, 3];
const GUTTER: f64 = 60.0;

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let tile = (area.w - 2.0 * GUTTER) / 3.0;
    let top = area.y + (area.h - tile) / 2.0;
    let mut fills = Vec::new();
    for (index, level) in LEVELS.iter().enumerate() {
        let design = designs::create(CODE, 3, *level, 0, 2)?;
        let side = design.width();
        assert_eq!(side, 3usize.pow(*level as u32));
        let x = area.x + index as f64 * (tile + GUTTER);
        let tone = if index == 0 {
            ink::yellow()
        } else {
            ink::blue()
        };
        Grid::new(Frame::new(x, top, tile, tile), side, side, 0.0).paint(
            &mut board,
            &design,
            |kind| (kind != 0).then_some(tone),
        );
        fills.push(design.types().sum());
    }
    assert_eq!(fills, [8u64, 64, 512]);
    save("wiki-kronecker-product", &board)?;
    Ok(())
}
