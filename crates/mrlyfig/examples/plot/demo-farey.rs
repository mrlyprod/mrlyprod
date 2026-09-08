use mrlycore::errors::Result;
use mrlyfig::{ink, plot, save, Board};
use mrlynum::lattice::{farey, new_nodes, totients};

const ORDER: usize = 80;

fn main() -> Result<()> {
    let phi = totients(ORDER);
    let lit = farey(ORDER).len();
    let primes = (2..=ORDER).filter(|&n| phi[n] == n as u64 - 1).count();
    assert_eq!(lit, 1967);
    assert_eq!(1 + phi[1..].iter().sum::<u64>() as usize, lit);
    assert_eq!(primes, 22);
    assert_eq!(new_nodes(1), 2);
    for (n, count) in phi.iter().enumerate().skip(2) {
        assert_eq!(new_nodes(n), *count);
    }

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let peak = (ORDER - 1) as f64;
    let slot = frame.w / ORDER as f64;
    let foot = frame.y + frame.h;
    let wide = slot * 0.56;
    let mid = |n: usize| frame.x + (n as f64 - 0.5) * slot;
    for (n, count) in phi.iter().enumerate().skip(1) {
        let tall = frame.h * *count as f64 / peak;
        let fresh = n > 1 && *count == n as u64 - 1;
        board.rect(
            mid(n) - wide / 2.0,
            foot - tall,
            wide,
            tall,
            if fresh {
                ink::blue()
            } else {
                ink::fade(ink::dim(), 0.5)
            },
        );
    }
    let ramp = |n: usize| (mid(n), foot - frame.h * (n - 1) as f64 / peak);
    board.segment(ramp(1), ramp(ORDER), 3.0, ink::yellow());
    plot::baseline(&mut board, frame, ink::line());
    save("demo-farey", &board)?;
    Ok(())
}
