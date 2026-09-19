use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};
use mrlynum::gauss::{Ring, Window};

const REACH: u64 = 8;
const POINTS: usize = 217;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.07);
    let ring = Ring::Eisenstein;
    let window = Window::new(ring, REACH);
    let points = window.points();
    assert_eq!(points.len(), POINTS);
    let units = ring.associates(1, 0);
    assert_eq!(units.len(), 6);
    let scale = frame.w / (2 * REACH + 1) as f64;
    let (cx, cy) = frame.center();
    let place = |a: i64, b: i64| {
        let (x, y) = ring.place(a, b);
        (cx + x * scale, cy - y * scale)
    };
    for &(a, b) in &points {
        for &(da, db) in &units[..3] {
            if window.holds(a + da, b + db) {
                board.segment(place(a, b), place(a + da, b + db), 1.2, ink::line());
            }
        }
    }
    for &(a, b) in &points {
        let (x, y) = place(a, b);
        board.disc(x, y, scale * 0.16, ink::blue());
    }
    for &(a, b) in &units {
        let (x, y) = place(a, b);
        board.disc(x, y, scale * 0.3, ink::yellow());
    }
    let (x, y) = place(0, 0);
    board.disc(x, y, scale * 0.34, ink::orange());
    save("wiki-eisenstein-integers", &board)?;
    Ok(())
}
