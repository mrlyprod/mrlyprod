use mrlycore::errors::Result;
use mrlycore::Color;
use mrlyfig::board::Frame;
use mrlyfig::{ink, save, Board, Grid};
use mrlymath::two::designs;

const SIDE: usize = 5;
const GAP: f64 = 0.14;

fn outline(board: &mut Board, frame: Frame, thick: f64) {
    board.polyline(
        &[
            (frame.x, frame.y),
            (frame.x + frame.w, frame.y),
            (frame.x + frame.w, frame.y + frame.h),
            (frame.x, frame.y + frame.h),
            (frame.x, frame.y),
        ],
        thick,
        ink::LINE,
    );
}

fn panel(board: &mut Board, frame: Frame, code: u128, tone: Color) -> Result<usize> {
    let design = designs::create(code, SIDE, 1, 0, 2)?;
    let types = design.types();
    let grid = Grid::new(frame, SIDE, SIDE, GAP);
    let mut filled = 0usize;
    for row in 0..SIDE {
        for col in 0..SIDE {
            if types.get(&[row, col]) == 0 {
                continue;
            }
            filled += 1;
            let (x, y, w, h) = grid.cell(col, row);
            board.rect(x, y, w, h, tone);
        }
    }
    let pitch = frame.w / SIDE as f64;
    outline(
        board,
        Frame::new(frame.x + pitch, frame.y + pitch, 3.0 * pitch, 3.0 * pitch),
        2.0,
    );
    Ok(filled)
}

fn main() -> Result<()> {
    let codes = [1u128, 3, 7, 9, 11, 15];
    let tones = [
        ink::BLUE,
        ink::BLUE,
        ink::BLUE,
        ink::VIOLET,
        ink::VIOLET,
        ink::VIOLET,
    ];
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let height = frame.h * 0.80;
    let block = Frame::new(frame.x, frame.y + (frame.h - height) / 2.0, frame.w, height);
    let mut counts = Vec::new();
    for (band, row) in block.rows(2).iter().enumerate() {
        for (index, slot) in row.cols(3).iter().enumerate() {
            let place = band * 3 + index;
            counts.push(panel(
                &mut board,
                slot.inset(16.0),
                codes[place],
                tones[place],
            )?);
        }
    }
    assert_eq!(counts, vec![9, 15, 21, 13, 19, 25]);
    save("paper-sequence-census", &board)?;
    Ok(())
}
