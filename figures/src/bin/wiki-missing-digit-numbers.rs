use figures::{ink, save, Board, Grid};
use mrlyrs::core::error::Result;
use mrlyrs::num::prime::flags;

const SIDE: usize = 100;
const DIGIT: usize = 7;
const KEPT: usize = 6561;
const PRIMES: usize = 680;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let grid = Grid::new(frame, SIDE, SIDE, 0.0);
    let top = SIDE * SIDE;
    let prime = flags(top);
    let mut kept = 0usize;
    let mut lit = 0usize;
    for (n, &is_prime) in prime.iter().enumerate().take(top) {
        if has(n, DIGIT) {
            continue;
        }
        kept += 1;
        let color = if is_prime {
            lit += 1;
            ink::yellow()
        } else {
            ink::blue()
        };
        grid.fill(&mut board, n % SIDE, n / SIDE, color);
    }
    assert_eq!(kept, 9usize.pow(4));
    assert_eq!(kept, KEPT);
    assert_eq!(lit, PRIMES);
    save("wiki-missing-digit-numbers", &board)?;
    Ok(())
}

fn has(mut n: usize, digit: usize) -> bool {
    for _ in 0..4 {
        if n % 10 == digit {
            return true;
        }
        n /= 10;
    }
    false
}
