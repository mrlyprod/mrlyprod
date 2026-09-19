use mrlycore::errors::Result;
use mrlyfig::{ink, plot, save, Board};
use mrlynum::classics::primes;
use mrlynum::formulas;

const TOP: usize = 200;
const SAMPLES: usize = 1200;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let left = 2.0;
    let right = (TOP + 1) as f64;
    let count = formulas::prime_count(TOP);
    let peak = count as f64;
    let across = |x: f64| frame.x + frame.w * (x - left) / (right - left);
    let up = |v: f64| frame.y + frame.h * (1.0 - v / peak);

    let roll = primes(TOP);
    let hair = ink::fade(ink::dim(), 0.30);
    for &p in &roll {
        let x = across(p as f64);
        board.segment((x, frame.y), (x, frame.y + frame.h), 1.4, hair);
    }

    let mut guess = Vec::with_capacity(SAMPLES + 1);
    for k in 0..=SAMPLES {
        let x = left + (right - left) * k as f64 / SAMPLES as f64;
        guess.push((across(x), up(x / x.ln())));
    }
    board.polyline(&guess, 3.4, ink::blue());

    let mut stair = Vec::with_capacity(2 * (TOP - 1));
    let mut seen = 0usize;
    for n in 2..=TOP {
        seen += roll.binary_search(&n).is_ok() as usize;
        stair.push((across(n as f64), up(seen as f64)));
        stair.push((across((n + 1) as f64), up(seen as f64)));
    }
    board.polyline(&stair, 3.4, ink::yellow());

    plot::axis(&mut board, frame, ink::line());
    assert_eq!(count, 46);
    assert_eq!(roll.len(), count);
    assert_eq!(seen, count);
    save("wiki-prime-counting-function", &board)?;
    Ok(())
}
