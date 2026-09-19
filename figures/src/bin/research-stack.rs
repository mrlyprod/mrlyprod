use mrlycore::errors::Result;
use mrlyfig::board::Board;
use mrlyfig::ink::Ramp;
use mrlyfig::{ink, save};
use mrlynum::gauss::{classes, Ring};
use std::collections::HashSet;

const RING: Ring = Ring::Gaussian;
const BOUND: u64 = 60;

fn nodes(bound: u64) -> Vec<(f64, f64, usize)> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for den in classes(RING, bound) {
        let n = RING.norm(den.0, den.1) as i64;
        let lit = classes(RING, bound / n as u64).len();
        for p in 0..n {
            for q in 0..n {
                let g = RING.gcd((p, q), den);
                if RING.norm(g.0, g.1) != 1 {
                    continue;
                }
                let x = (p * den.0 + q * den.1).rem_euclid(n);
                let y = (q * den.0 - p * den.1).rem_euclid(n);
                if !seen.insert((x, y, n)) {
                    continue;
                }
                out.push((x as f64 / n as f64, y as f64 / n as f64, lit));
            }
        }
    }
    out
}

fn wrapped(stack: Vec<(f64, f64, usize)>) -> Vec<(f64, f64, usize)> {
    let mut out = Vec::new();
    for (x, y, lit) in stack {
        out.push((x, y, lit));
        if x == 0.0 {
            out.push((1.0, y, lit));
        }
        if y == 0.0 {
            out.push((x, 1.0, lit));
        }
        if x == 0.0 && y == 0.0 {
            out.push((1.0, 1.0, lit));
        }
    }
    out
}

fn main() -> Result<()> {
    assert_eq!(nodes(50).len(), 672);
    let mut stack = nodes(BOUND);
    assert_eq!(stack.len(), 880);
    let top = classes(RING, BOUND).len();
    assert_eq!(top, 46);
    stack = wrapped(stack);
    assert_eq!(stack.len(), 917);
    stack.sort_by_key(|node| node.2);

    let ramp = Ramp::new(vec![ink::ground(), ink::blue(), ink::yellow()]);
    let mut board = Board::square();
    let frame = board.frame(0.08);
    for (x, y, lit) in stack {
        let share = lit as f64 / top as f64;
        let (px, py) = frame.at(x, 1.0 - y);
        board.disc(
            px,
            py,
            3.5 + 15.0 * share.powf(0.7),
            ramp.at(share.powf(0.30)),
        );
    }
    save("research-stack", &board)?;
    Ok(())
}
