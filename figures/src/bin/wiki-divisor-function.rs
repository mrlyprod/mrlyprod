use mrlycore::errors::Result;
use mrlyfig::{ink, plot, save, Board};

const X: usize = 36;
const ROOT: usize = 6;
const WINDOW: f64 = 38.0;
const SAMPLES: usize = 800;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let unit = frame.w / WINDOW;
    let at = |a: f64, b: f64| (frame.x + a * unit, frame.y + frame.h - b * unit);

    let divisors: Vec<usize> = (0..=X)
        .map(|n| (1..=n).filter(|d| n % d == 0).count())
        .collect();
    let total: usize = divisors[1..].iter().sum();
    let folded = 2 * (1..=ROOT).map(|a| X / a).sum::<usize>() - ROOT * ROOT;
    assert_eq!(ROOT * ROOT, X);
    assert_eq!(total, 140);
    assert_eq!(folded, total);
    assert_eq!(divisors[36], 9);
    assert_eq!(divisors[31], 2);

    let mut inner = Vec::new();
    let mut arms = Vec::new();
    for a in 1..=X {
        for b in 1..=X / a {
            let p = at(a as f64, b as f64);
            if a <= ROOT && b <= ROOT {
                inner.push(p);
            } else {
                arms.push(p);
            }
        }
    }
    assert_eq!(inner.len(), ROOT * ROOT);
    assert_eq!(arms.len(), total - ROOT * ROOT);

    let curve: Vec<(f64, f64)> = (0..=SAMPLES)
        .map(|i| {
            let a = (X as f64).powf(i as f64 / SAMPLES as f64);
            at(a, X as f64 / a)
        })
        .collect();
    let hair = (frame.w / 512.0).max(1.0);
    board.segment(at(0.0, 0.0), at(WINDOW, 0.0), hair, ink::line());
    board.segment(at(0.0, 0.0), at(0.0, WINDOW), hair, ink::line());
    board.polyline(&curve, 3.0, ink::dim());
    let r = unit * 0.36;
    plot::dots(&mut board, &arms, r, ink::blue());
    plot::dots(&mut board, &inner, r, ink::yellow());
    save("wiki-divisor-function", &board)?;
    Ok(())
}
