use mrlyfig::{ink, save, Board};
use mrlyrs::core::errors::Result;
use mrlyrs::num::lattice::zeta_whole;

const TERMS: usize = 40;
const POWERS: [u32; 3] = [2, 3, 4];
const FLOOR: f64 = 0.94;
const ROOF: f64 = 1.72;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.09);
    let place = |n: usize, value: f64| {
        (
            frame.x + frame.w * (n - 1) as f64 / (TERMS - 1) as f64,
            frame.y + frame.h * (1.0 - (value - FLOOR) / (ROOF - FLOOR)),
        )
    };
    let limits: Vec<f64> = POWERS.iter().map(|&s| zeta_whole(s)).collect();
    for limit in &limits {
        let (_, y) = place(1, *limit);
        board.segment((frame.x, y), (frame.x + frame.w, y), 1.5, ink::line());
    }
    let inks = [ink::blue(), ink::orange(), ink::green()];
    let mut ends = Vec::new();
    for (k, &s) in POWERS.iter().enumerate() {
        let mut running = 0.0f64;
        let mut path = Vec::with_capacity(TERMS);
        for n in 1..=TERMS {
            running += 1.0 / (n as f64).powi(s as i32);
            path.push(place(n, running));
        }
        assert_eq!(path.len(), TERMS);
        board.polyline(&path, 4.0, inks[k]);
        board.disc(path[TERMS - 1].0, path[TERMS - 1].1, 9.0, inks[k]);
        ends.push(running);
    }
    assert_eq!(ends.len(), 3);
    assert!((ends[0] - limits[0]).abs() < 0.026);
    assert!((ends[1] - limits[1]).abs() < 0.0004);
    assert!((ends[2] - limits[2]).abs() < 0.00002);
    save("wiki-riemann-zeta-function", &board)?;
    Ok(())
}
