use figures::{ink, plot, save, Board, Frame};
use mrlyrs::core::error::Result;
use mrlyrs::num::prime::{prime_count, primes};

const TOP: usize = 100_000;
const SAMPLES: usize = 1000;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let gap = frame.h * 0.08;
    let half = (frame.h - gap) / 2.0;
    let upper = Frame::new(frame.x, frame.y, frame.w, half);
    let lower = Frame::new(frame.x, frame.y + half + gap, frame.w, half);

    let least = least_factor(TOP);
    let mut member = vec![false; TOP + 1];
    member[1] = true;
    for n in 2..=TOP {
        let p = least[n];
        member[n] = p % 4 == 1 && member[n / p];
    }
    let all_integers = counts(TOP, |_| true);
    let toy_integers = counts(TOP, |n| member[n]);
    let all_primes = counts(TOP, |n| n > 1 && least[n] == n);
    let toy_primes = counts(TOP, |n| n > 1 && least[n] == n && n % 4 == 1);

    assert_eq!(all_integers[TOP], TOP);
    assert_eq!(toy_integers[TOP], 9623);
    assert_eq!(all_primes[TOP], prime_count(TOP));
    assert_eq!(all_primes[TOP], 9592);
    assert_eq!(toy_primes[TOP], 4783);
    assert_eq!(primes(TOP).len(), 9592);

    trace(&mut board, upper, &all_integers, TOP as f64, ink::blue());
    trace(&mut board, upper, &toy_integers, TOP as f64, ink::yellow());
    let peak = all_primes[TOP] as f64;
    trace(&mut board, lower, &all_primes, peak, ink::blue());
    trace(&mut board, lower, &toy_primes, peak, ink::yellow());
    plot::axis(&mut board, upper, ink::line());
    plot::axis(&mut board, lower, ink::line());
    save("wiki-beurling-generalized-primes", &board)?;
    Ok(())
}

fn least_factor(top: usize) -> Vec<usize> {
    let mut least: Vec<usize> = (0..=top).collect();
    let mut p = 2;
    while p * p <= top {
        if least[p] == p {
            for m in (p * p..=top).step_by(p) {
                if least[m] == m {
                    least[m] = p;
                }
            }
        }
        p += 1;
    }
    least
}

fn counts(top: usize, keep: impl Fn(usize) -> bool) -> Vec<usize> {
    let mut out = vec![0; top + 1];
    for n in 1..=top {
        out[n] = out[n - 1] + keep(n) as usize;
    }
    out
}

fn trace(board: &mut Board, frame: Frame, count: &[usize], peak: f64, color: figures::Color) {
    let top = count.len() - 1;
    let pts: Vec<(f64, f64)> = (0..=SAMPLES)
        .map(|k| {
            let x = top * k / SAMPLES;
            (
                frame.x + frame.w * x as f64 / top as f64,
                frame.y + frame.h * (1.0 - count[x] as f64 / peak),
            )
        })
        .collect();
    board.polyline(&pts, 3.4, color);
}
