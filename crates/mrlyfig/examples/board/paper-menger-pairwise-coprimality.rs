use mrlycore::errors::Result;
use mrlyfig::board::Frame;
use mrlyfig::{ink, save, Board, Grid};
use mrlymath::three;

fn outline(board: &mut Board, cell: (f64, f64, f64, f64), thick: f64) {
    let (x, y, w, h) = cell;
    board.polyline(
        &[(x, y), (x + w, y), (x + w, y + h), (x, y + h), (x, y)],
        thick,
        ink::LINE,
    );
}

fn main() -> Result<()> {
    let sponge = three::carpet(3, 1)?;
    let types = sponge.types();
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let step = frame.w / 3.0;
    let side = step - 10.0;
    let mut kept = 0usize;
    let mut near = 0usize;
    for c in 0..3 {
        let panel = Frame::new(
            frame.x + c as f64 * step,
            frame.y + c as f64 * step,
            side,
            side,
        );
        let grid = Grid::new(panel, 3, 3, 0.07);
        for a in 0..3 {
            for b in 0..3 {
                let (x, y, w, h) = grid.cell(b, a);
                if types.get(&[a, b, c]) == 0 {
                    outline(&mut board, (x, y, w, h), 4.0);
                    continue;
                }
                kept += 1;
                let zeros = [a, b, c].iter().filter(|value| **value == 0).count();
                if zeros <= 1 {
                    near += 1;
                    board.rect(x, y, w, h, ink::GREEN);
                } else {
                    board.rect(x, y, w, h, ink::DIM);
                }
            }
        }
    }
    assert_eq!((kept, near), (20, 13));
    save("paper-menger-pairwise-coprimality", &board)?;
    Ok(())
}
