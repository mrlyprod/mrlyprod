use mrlycore::errors::Result;
use mrlyfig::board::{Board, Frame};
use mrlyfig::{hex, ink, save};
use mrlymath::six;
use mrlymath::six::Cell6d;

const CODES: [u128; 9] = [0, 1, 3, 7, 23, 31, 63, 127, 255];
const SIDE: usize = 11;
const WIDE: usize = 3;

fn at(cell: &Cell6d, row: usize, col: usize) -> u8 {
    let offset = (cell.width() - hex::row_len(SIDE, row)) / 2;
    cell.cell.types().get(&[row, col + offset])
}

fn panels(frame: Frame, gap: f64) -> Vec<Frame> {
    frame
        .rows(WIDE)
        .iter()
        .flat_map(|row| row.cols(WIDE))
        .map(|cell| cell.inset(gap))
        .collect()
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let stage = panels(frame, frame.w / WIDE as f64 * 0.05);
    let mut ink_total = 0usize;
    let mut last = 0usize;
    for (corners, code) in CODES.iter().enumerate() {
        assert_eq!(code.count_ones() as usize, corners);
        if corners > 0 {
            assert_eq!(code & CODES[corners - 1], CODES[corners - 1]);
        }
        let slice = six::cut_design(*code, SIDE, 1, 2)?;
        let fills = six::fills(&slice);
        assert!(fills >= last);
        last = fills;
        ink_total += fills;
        let panel = stage[corners];
        let edge = panel.w / (2 * SIDE) as f64;
        hex::hexagon(&mut board, panel, SIDE, 0.0, |_, _, _| Some(ink::line()));
        hex::hexagon(&mut board, panel, SIDE, edge * 0.06, |row, col, _| {
            (at(&slice, row, col) != six::FILL).then(ink::ground)
        });
        hex::hexagon(&mut board, panel, SIDE, edge * 0.06, |row, col, _| {
            (at(&slice, row, col) == six::FILL).then(ink::blue)
        });
    }
    assert_eq!(hex::count(SIDE), 726);
    assert_eq!(six::fills(&six::cut_design(CODES[8], SIDE, 1, 2)?), 726);
    assert!(ink_total > 0);
    save("demo-spectrometer", &board)?;
    Ok(())
}
