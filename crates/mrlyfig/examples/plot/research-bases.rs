use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};
use mrlynum::factor::gcd;
use mrlynum::gauss::Ring;

const RADIUS: i64 = 30;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let ring = Ring::Eisenstein;
    let (cx, cy) = frame.center();
    let unit = frame.w / (2.0 * RADIUS as f64 + 0.8);
    let mut total = 0usize;
    let mut visible = 0usize;
    for a in -RADIUS..=RADIUS {
        for b in -RADIUS..=RADIUS {
            if ring.reach(a, b) > RADIUS as u64 {
                continue;
            }
            total += 1;
            let (u, v) = ring.place(a, b);
            let (x, y) = (cx + u * unit, cy - v * unit);
            if a == 0 && b == 0 {
                board.disc(x, y, unit * 0.20, ink::fade(ink::DIM, 0.9));
            } else if gcd(a.unsigned_abs() as usize, b.unsigned_abs() as usize) == 1 {
                visible += 1;
                board.disc(x, y, unit * 0.30, ink::BLUE);
            } else {
                board.disc(x, y, unit * 0.13, ink::fade(ink::DIM, 0.8));
            }
        }
    }
    assert_eq!(total, ring.count(RADIUS as u64));
    assert_eq!(visible, 1668);
    save("research-bases", &board)?;
    Ok(())
}
