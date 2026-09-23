use figures::{field, ink, save, Board, Frame, Ramp};
use mrlyrs::core::error::Result;
use std::f64::consts::PI;

const RES: usize = 256;
const REACH: usize = 2;

type Wave = Box<dyn Fn(f64, f64) -> f64>;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let gap = frame.w * 0.06;
    let side = (frame.w - gap) / 2.0;
    let panels = [
        Frame::new(frame.x, frame.y, side, side),
        Frame::new(frame.x + side + gap, frame.y, side, side),
        Frame::new(frame.x, frame.y + side + gap, side, side),
        Frame::new(frame.x + side + gap, frame.y + side + gap, side, side),
    ];
    let modes: [(Wave, u32, usize); 4] = [
        (Box::new(|x, y| mode(1, 1, x, y)), 2, 1),
        (Box::new(|x, y| mode(2, 1, x, y)), 5, 2),
        (Box::new(|x, y| mode(2, 2, x, y)), 8, 4),
        (Box::new(|x, y| mode(1, 3, x, y) + mode(3, 1, x, y)), 10, 2),
    ];
    assert_eq!([index(2), index(5), index(8), index(10)], [1, 2, 4, 5]);
    let ramp = Ramp::new(vec![ink::orange(), ink::ground(), ink::blue()]);
    for (panel, (f, lambda, want)) in panels.iter().zip(modes.iter()) {
        let signs = signs(f);
        let domains = domains(&signs);
        assert_eq!(domains, *want);
        assert!(domains <= index(*lambda));
        let values = nodal(&signs);
        field::draw_range(&mut board, *panel, RES, RES, &values, (-1.0, 1.0), &ramp);
    }
    save("wiki-nodal-domains", &board)?;
    Ok(())
}

fn mode(m: u32, n: u32, x: f64, y: f64) -> f64 {
    (m as f64 * PI * x).sin() * (n as f64 * PI * y).sin()
}

fn index(lambda: u32) -> usize {
    let mut below = 0;
    for m in 1..=10 {
        for n in 1..=10 {
            if m * m + n * n < lambda {
                below += 1;
            }
        }
    }
    below + 1
}

fn signs(f: &dyn Fn(f64, f64) -> f64) -> Vec<i8> {
    let mut out = Vec::with_capacity(RES * RES);
    for row in 0..RES {
        for col in 0..RES {
            let x = (col as f64 + 0.5) / RES as f64;
            let y = (row as f64 + 0.5) / RES as f64;
            out.push(if f(x, y) >= 0.0 { 1 } else { -1 });
        }
    }
    out
}

fn neighbours(i: usize) -> impl Iterator<Item = usize> {
    let (row, col) = (i / RES, i % RES);
    let mut out = Vec::with_capacity(4);
    if row > 0 {
        out.push(i - RES);
    }
    if row + 1 < RES {
        out.push(i + RES);
    }
    if col > 0 {
        out.push(i - 1);
    }
    if col + 1 < RES {
        out.push(i + 1);
    }
    out.into_iter()
}

fn domains(signs: &[i8]) -> usize {
    let mut seen = vec![false; signs.len()];
    let mut count = 0;
    for start in 0..signs.len() {
        if seen[start] {
            continue;
        }
        count += 1;
        let mut stack = vec![start];
        seen[start] = true;
        while let Some(i) = stack.pop() {
            for j in neighbours(i) {
                if !seen[j] && signs[j] == signs[i] {
                    seen[j] = true;
                    stack.push(j);
                }
            }
        }
    }
    count
}

fn nodal(signs: &[i8]) -> Vec<f64> {
    (0..signs.len())
        .map(|i| {
            let (row, col) = (i / RES, i % RES);
            let rows = row.saturating_sub(REACH)..(row + REACH + 1).min(RES);
            let cols = col.saturating_sub(REACH)..(col + REACH + 1).min(RES);
            let crossed = rows
                .flat_map(|r| cols.clone().map(move |c| r * RES + c))
                .any(|j| signs[j] != signs[i]);
            if crossed {
                0.0
            } else {
                signs[i] as f64
            }
        })
        .collect()
}
