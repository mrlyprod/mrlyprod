use mrlyfig::{ink, plot, save, Board};
use mrlyrs::core::errors::Result;
use mrlyrs::num::factor::mobius_sieve;

const TOP: usize = 1000;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let reach = (TOP as f64).sqrt() * 1.1;
    let across = |n: f64| frame.x + frame.w * (n - 1.0) / (TOP as f64 - 1.0);
    let up = |v: f64| frame.y + frame.h * (1.0 - (v + reach) / (2.0 * reach));

    let mu = mobius_sieve(TOP);
    let mut walk: Vec<i64> = Vec::with_capacity(TOP);
    let mut sum = 0i64;
    for n in 1..=TOP {
        sum += i64::from(mu[n]);
        walk.push(sum);
    }

    let mut over = Vec::with_capacity(TOP);
    let mut under = Vec::with_capacity(TOP);
    for n in 1..=TOP {
        let root = (n as f64).sqrt();
        over.push((across(n as f64), up(root)));
        under.push((across(n as f64), up(-root)));
    }
    board.polyline(&over, 2.6, ink::dim());
    board.polyline(&under, 2.6, ink::dim());
    board.segment(
        (frame.x, up(0.0)),
        (frame.x + frame.w, up(0.0)),
        1.6,
        ink::line(),
    );

    let mut steps = Vec::with_capacity(2 * TOP);
    for (i, &m) in walk.iter().enumerate() {
        let n = (i + 1) as f64;
        steps.push((across(n), up(m as f64)));
        steps.push((across(n + 1.0), up(m as f64)));
    }
    board.polyline(&steps, 2.6, ink::blue());

    plot::axis(&mut board, frame, ink::line());
    assert_eq!(walk.len(), TOP);
    assert_eq!(walk[999], 2);
    assert_eq!(walk.iter().copied().min(), Some(-12));
    assert!(walk
        .iter()
        .enumerate()
        .all(|(i, &m)| (m as f64).abs() <= ((i + 1) as f64).sqrt()));
    save("wiki-mertens-function", &board)?;
    Ok(())
}
