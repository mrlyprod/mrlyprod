use mrlycore::errors::Result;
use mrlyfig::board::Board;
use mrlyfig::ink::Ramp;
use mrlyfig::{ink, save};
use mrlynum::gauss::Ring;
use std::collections::HashSet;

const RING: Ring = Ring::Gaussian;
const BOUND: i64 = 60;

fn norm(z: (i64, i64)) -> i64 {
    RING.norm(z.0, z.1) as i64
}

fn round_quotient(z: (i64, i64), w: (i64, i64)) -> (i64, i64) {
    let p = RING.mul(z, RING.conjugate(w.0, w.1));
    let n = norm(w);
    (
        (2 * p.0 + n).div_euclid(2 * n),
        (2 * p.1 + n).div_euclid(2 * n),
    )
}

fn gcd(mut z: (i64, i64), mut w: (i64, i64)) -> (i64, i64) {
    while w != (0, 0) {
        let step = RING.mul(round_quotient(z, w), w);
        let rest = (z.0 - step.0, z.1 - step.1);
        z = w;
        w = rest;
    }
    z
}

fn layers(bound: i64) -> Vec<(i64, i64)> {
    let mut out = Vec::new();
    for a in 1.. {
        if a * a > bound {
            break;
        }
        for b in 0.. {
            if a * a + b * b > bound {
                break;
            }
            out.push((a, b));
        }
    }
    out
}

fn classes(bound: i64) -> i64 {
    let mut sum = 0;
    let mut j = 0;
    while 4 * j < bound {
        sum += bound / (4 * j + 1) - bound / (4 * j + 3);
        j += 1;
    }
    sum
}

fn nodes(bound: i64) -> Vec<(f64, f64, i64)> {
    let mut seen = HashSet::new();
    let mut out = Vec::new();
    for den in layers(bound) {
        let n = norm(den);
        let lit = classes(bound / n);
        for p in 0..n {
            for q in 0..n {
                if norm(gcd((p, q), den)) != 1 {
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

fn wrapped(stack: Vec<(f64, f64, i64)>) -> Vec<(f64, f64, i64)> {
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
    assert_eq!(layers(BOUND).len(), 46);
    let top = classes(BOUND);
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
