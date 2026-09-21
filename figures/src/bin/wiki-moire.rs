use figures::{ink, save, Board};
use mrlyrs::core::error::Result;
use mrlyrs::num::factor::gcd;

const SCALES: (usize, usize) = (5, 7);

fn marks(n: usize) -> Vec<f64> {
    (0..=n).map(|k| k as f64 / n as f64).collect()
}

fn main() -> Result<()> {
    let (small, large) = SCALES;
    let common = gcd(small as u128, large as u128) as usize;
    let shared = marks(common);
    let thin = marks(small);
    let thick = marks(large);
    let mut crossings: Vec<(f64, f64)> = Vec::new();
    for &x in &thin {
        for &y in &thick {
            crossings.push((x, y));
            if !thin.contains(&y) || !thick.contains(&x) {
                crossings.push((y, x));
            }
        }
    }
    assert_eq!(common, 1);
    assert_eq!(shared.len(), 2);
    assert_eq!(crossings.len(), 92);

    let mut board = Board::square();
    let frame = board.frame(0.10);
    let at = |u: f64, v: f64| (frame.x + u * frame.w, frame.y + v * frame.h);
    for &u in &thin {
        let (x, top) = at(u, 0.0);
        let (left, y) = at(0.0, u);
        board.segment((x, top), (x, top + frame.h), 5.0, ink::blue());
        board.segment((left, y), (left + frame.w, y), 5.0, ink::blue());
    }
    for &u in &thick {
        let (x, top) = at(u, 0.0);
        let (left, y) = at(0.0, u);
        board.segment((x, top), (x, top + frame.h), 5.0, ink::orange());
        board.segment((left, y), (left + frame.w, y), 5.0, ink::orange());
    }
    for &(u, v) in &crossings {
        let (x, y) = at(u, v);
        board.disc(x, y, frame.w * 0.010, ink::fade(ink::dim(), 0.9));
    }
    for &u in &shared {
        for &v in &shared {
            let (x, y) = at(u, v);
            board.disc(x, y, frame.w * 0.035, ink::yellow());
        }
    }
    save("wiki-moire", &board)?;
    Ok(())
}
