use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};
use mrlynum::morse::{lift, Lift};

const SIDE: usize = 32;

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let signs = lift(Lift::Parity, SIDE);
    assert_eq!(signs.len(), SIDE * SIDE);
    assert_eq!(lift(Lift::Xor, SIDE), signs);
    let cool = signs.iter().filter(|&&sign| sign == 1).count();
    assert_eq!(cool, SIDE * SIDE / 2);
    let scale = (area.w / SIDE as f64).floor().max(1.0);
    let block = SIDE as f64 * scale;
    let ox = ((board.width as f64 - block) / 2.0).round();
    let oy = ((board.height as f64 - block) / 2.0).round();
    for row in 0..SIDE {
        for col in 0..SIDE {
            let tone = if signs[row * SIDE + col] == 0 {
                ink::orange()
            } else {
                ink::blue()
            };
            let x = ox + col as f64 * scale;
            let y = oy + row as f64 * scale;
            board.rect(x, y, scale, scale, tone);
        }
    }
    save("demo-morse", &board)?;
    Ok(())
}
