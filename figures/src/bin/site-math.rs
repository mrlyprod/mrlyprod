use mrlyfig::{ink, save, Board, Frame, Grid};
use mrlyrs::core::errors::Result;
use mrlyrs::math::two::designs;

const CODE: u128 = 7;
const BITS: usize = 8;
const SIDE: usize = 27;
const WIDE: f64 = 594.0;
const TOP: f64 = 140.0;
const THIN: f64 = 3.0;

fn main() -> Result<()> {
    let mut board = Board::square();
    let left = (board.width as f64 - WIDE) / 2.0;
    let pitch = WIDE / BITS as f64;
    let row = Grid::new(Frame::new(left, TOP, WIDE, pitch), BITS, 1, 0.14);
    let mut lit = 0u32;
    for bit in 0..BITS {
        let (x, y, w, h) = row.cell(bit, 0);
        if (CODE & (1 << bit)) != 0 {
            board.rect(x, y, w, h, ink::fg());
            lit += 1;
        } else {
            board.rect(x, y, w, h, ink::line());
            board.rect(
                x + THIN,
                y + THIN,
                w - 2.0 * THIN,
                h - 2.0 * THIN,
                ink::ground(),
            );
        }
    }
    assert_eq!(lit, CODE.count_ones());
    let carpet = designs::create(CODE, 3, 3, 0, 2)?;
    assert_eq!(carpet.types().sum(), 512);
    let under = board.height as f64 - TOP - WIDE;
    Grid::new(Frame::new(left, under, WIDE, WIDE), SIDE, SIDE, 0.0).paint(
        &mut board,
        &carpet,
        |kind| (kind != 0).then_some(ink::blue()),
    );
    save("site-math", &board)?;
    Ok(())
}
