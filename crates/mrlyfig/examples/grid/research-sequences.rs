use mrlycore::errors::Result;
use mrlyfig::board::Frame;
use mrlyfig::{ink, save, Board, Grid};
use mrlymath::formulas::counting;
use mrlymath::two::designs;

const CODES: [u128; 6] = [1, 3, 7, 9, 11, 15];
const SIDES: [usize; 4] = [3, 5, 7, 9];

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let pitch = area.h / CODES.len() as f64;
    let block = Frame::new(
        area.x + (area.w - pitch * SIDES.len() as f64) / 2.0,
        area.y,
        pitch * SIDES.len() as f64,
        area.h,
    );
    let rule = 1.5;
    for col in 0..=SIDES.len() {
        let x = block.x + col as f64 * pitch;
        board.rect(x - rule / 2.0, block.y, rule, block.h, ink::LINE);
    }
    for row in 0..=CODES.len() {
        let y = block.y + row as f64 * pitch;
        board.rect(block.x, y - rule / 2.0, block.w, rule, ink::LINE);
    }
    for (row, code) in CODES.iter().enumerate() {
        for (col, number) in SIDES.iter().enumerate() {
            let cells = designs::create(*code, *number, 1, 0, 2)?;
            assert_eq!(cells.width(), *number);
            assert_eq!(
                cells.types().sum() as u128,
                counting::fill(*code, *number, 2, 1, 2)?
            );
            let tile = Frame::new(
                block.x + col as f64 * pitch,
                block.y + row as f64 * pitch,
                pitch,
                pitch,
            )
            .inset(pitch * 0.10);
            Grid::new(tile, *number, *number, 0.10)
                .paint(&mut board, &cells, |kind| (kind != 0).then_some(ink::GOLD));
        }
    }
    save("research-sequences", &board)?;
    Ok(())
}
