use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};
use mrlymath::two::designs;

const TILE: usize = 27;
const CELL: f64 = 4.0;
const GUTTER: f64 = 17.0;
const COLS: usize = 7;
const ROWS: usize = 4;

const DESIGNS: [u128; 28] = [
    15, 30, 57, 85, 102, 108, 325, 31, 79, 103, 115, 122, 173, 341, 119, 111, 125, 187, 231, 245,
    189, 127, 191, 239, 254, 351, 367, 381,
];

fn main() -> Result<()> {
    let mut board = Board::square();
    let side = TILE as f64 * CELL;
    let width = COLS as f64 * side + (COLS - 1) as f64 * GUTTER;
    let height = ROWS as f64 * side + (ROWS - 1) as f64 * GUTTER;
    let left = ((board.width as f64 - width) / 2.0).round();
    let top = ((board.height as f64 - height) / 2.0).round();
    let mut painted = 0usize;
    for (place, code) in DESIGNS.iter().enumerate() {
        let design = designs::create(*code, 3, 3, 0, 3)?;
        let types = design.types();
        let tone = ink::INKS[place % 3];
        let x = left + (place % COLS) as f64 * (side + GUTTER);
        let y = top + (place / COLS) as f64 * (side + GUTTER);
        let mut filled = 0usize;
        for row in 0..TILE {
            for col in 0..TILE {
                if types.get(&[row, col]) == 0 {
                    continue;
                }
                filled += 1;
                board.rect(
                    x + col as f64 * CELL,
                    y + row as f64 * CELL,
                    CELL,
                    CELL,
                    tone,
                );
            }
        }
        assert_eq!(filled, (code.count_ones() as usize).pow(3));
        painted += filled;
    }
    assert_eq!(painted, 5236);
    save("site-demos", &board)?;
    Ok(())
}
