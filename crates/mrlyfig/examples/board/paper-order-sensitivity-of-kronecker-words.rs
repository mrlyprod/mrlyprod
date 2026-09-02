use mrlycore::errors::Result;
use mrlycore::Color;
use mrlyfig::board::Frame;
use mrlyfig::{ink, save, Board, Grid};
use mrlymath::bang::{magic, word, MagicLayer};
use mrlymath::name::Bang;

fn letter(code: u128) -> MagicLayer {
    MagicLayer::new(Bang::new(code, 2, 2), 2)
}

fn lattice(board: &mut Board, frame: Frame, side: usize, thick: f64) {
    let step = frame.w / side as f64;
    for k in 0..=side {
        let offset = k as f64 * step;
        board.segment(
            (frame.x + offset, frame.y),
            (frame.x + offset, frame.y + frame.h),
            thick,
            ink::LINE,
        );
        board.segment(
            (frame.x, frame.y + offset),
            (frame.x + frame.w, frame.y + offset),
            thick,
            ink::LINE,
        );
    }
}

fn panel(board: &mut Board, frame: Frame, order: (u128, u128), tone: Color) -> Result<usize> {
    let picture = magic(&[letter(order.0), letter(order.1)])?;
    lattice(board, frame, 4, 2.5);
    let grid = Grid::new(frame, 4, 4, 0.0);
    let mut filled = 0usize;
    for row in 0..4 {
        for col in 0..4 {
            if picture.get(&[row, col]) == 0 {
                continue;
            }
            filled += 1;
            let (x, y, w, h) = grid.cell(col, row);
            board.rect(x, y, w, h, tone);
        }
    }
    Ok(filled)
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let half = frame.w / 2.0;
    let top = Frame::new(frame.x, frame.y, half, half);
    let low = Frame::new(frame.x + half, frame.y + half, half, half);
    let first = panel(&mut board, top, (3, 6), ink::BLUE)?;
    let second = panel(&mut board, low, (6, 3), ink::ORANGE)?;
    assert_eq!((first, second), (4, 4));
    assert_eq!(
        (
            word::components(&[letter(3), letter(6)])?,
            word::components(&[letter(6), letter(3)])?
        ),
        (4, 2)
    );
    save("paper-order-sensitivity-of-kronecker-words", &board)?;
    Ok(())
}
