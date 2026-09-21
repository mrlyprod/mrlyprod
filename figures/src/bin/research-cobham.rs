use figures::board::Frame;
use figures::{ink, save, Board, Grid};
use mrlyrs::core::error::Result;

const M: u32 = 5;
const K: u32 = 7;
const PX: f64 = 6.0;

fn in_base2_gasket(x: u32, y: u32) -> bool {
    x & y == 0
}

fn in_base3_gasket(mut x: u32, mut y: u32) -> bool {
    while x > 0 || y > 0 {
        if x % 3 > 1 || y % 3 > 1 || (x % 3 == 1 && y % 3 == 1) {
            return false;
        }
        x /= 3;
        y /= 3;
    }
    true
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let side = 2usize.pow(K);
    assert!(side > (3usize.pow(M) - 1) / 2);
    let edge = side as f64 * PX;
    let area = Frame::new(
        ((board.width as f64 - edge) / 2.0).round(),
        ((board.height as f64 - edge) / 2.0).round(),
        edge,
        edge,
    );
    let grid = Grid::new(area, side, side, 0.0);
    let (mut b, mut both, mut axes) = (0usize, 0usize, 0usize);
    for row in 0..side {
        for col in 0..side {
            let (x, y) = (col as u32, (side - 1 - row) as u32);
            let a = in_base2_gasket(x, y);
            let in_b = in_base3_gasket(x, y);
            b += in_b as usize;
            both += (a && in_b) as usize;
            axes += (a && in_b && (x == 0 || y == 0)) as usize;
            let tone = match (a, in_b) {
                (true, true) => ink::yellow(),
                (false, true) => ink::blue(),
                (true, false) => ink::fade(ink::dim(), 0.45),
                (false, false) => continue,
            };
            grid.fill(&mut board, col, row, tone);
        }
    }
    assert_eq!(b, 3usize.pow(M));
    assert_eq!(both, 111);
    assert_eq!(axes, 2usize.pow(M + 1) - 1);
    save("research-cobham", &board)?;
    Ok(())
}
