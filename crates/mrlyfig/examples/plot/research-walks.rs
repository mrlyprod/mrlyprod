use mrlycore::errors::Result;
use mrlycore::{Color, Rng};
use mrlyfig::{ink, plot, save, Board, Frame};
use mrlymath::two::designs;
use mrlymath::two::Cell2d;
use std::collections::VecDeque;

const LEVEL: usize = 4;
const STEPS: usize = 20000;
const SEED: u64 = 20260902;

fn sites(cell: &Cell2d) -> (usize, Vec<bool>) {
    let side = cell.width();
    let types = cell.types();
    let mut mask = vec![false; side * side];
    for row in 0..side {
        for col in 0..side {
            mask[row * side + col] = types.get(&[row, col]) != 0;
        }
    }
    (side, mask)
}

fn around(side: usize, at: usize) -> Vec<usize> {
    let (row, col) = (at / side, at % side);
    let mut out = Vec::with_capacity(4);
    if row > 0 {
        out.push(at - side);
    }
    if row + 1 < side {
        out.push(at + side);
    }
    if col > 0 {
        out.push(at - 1);
    }
    if col + 1 < side {
        out.push(at + 1);
    }
    out
}

fn giant(side: usize, mask: &[bool]) -> Vec<usize> {
    let mut seen = vec![false; mask.len()];
    let mut best = Vec::new();
    for start in 0..mask.len() {
        if !mask[start] || seen[start] {
            continue;
        }
        let mut part = Vec::new();
        let mut queue = VecDeque::from([start]);
        seen[start] = true;
        while let Some(at) = queue.pop_front() {
            part.push(at);
            for next in around(side, at) {
                if mask[next] && !seen[next] {
                    seen[next] = true;
                    queue.push_back(next);
                }
            }
        }
        if part.len() > best.len() {
            best = part;
        }
    }
    best
}

fn middle(side: usize, part: &[usize]) -> usize {
    let mid = (side as f64 - 1.0) / 2.0;
    *part
        .iter()
        .min_by(|a, b| {
            let reach = |at: &usize| {
                let (row, col) = ((at / side) as f64, (at % side) as f64);
                (row - mid).hypot(col - mid)
            };
            reach(a).partial_cmp(&reach(b)).unwrap()
        })
        .unwrap()
}

fn walk(side: usize, mask: &[bool], start: usize, seed: u64) -> Vec<usize> {
    let mut rng = Rng::new(seed);
    let mut at = start;
    let mut path = vec![at];
    for _ in 0..STEPS {
        let (row, col) = (at / side, at % side);
        let next = match rng.below(4) {
            0 if row > 0 => at - side,
            1 if row + 1 < side => at + side,
            2 if col > 0 => at - 1,
            3 if col + 1 < side => at + 1,
            _ => at,
        };
        if next != at && mask[next] {
            at = next;
            path.push(at);
        }
    }
    path
}

fn panel(board: &mut Board, area: Frame, code: u128, color: Color) -> Result<usize> {
    let cell = designs::create(code, 3, LEVEL, 0, 3)?;
    let (side, mask) = sites(&cell);
    let step = area.w / side as f64;
    let dot = step * 0.7;
    for (at, on) in mask.iter().enumerate() {
        if *on {
            let (row, col) = (at / side, at % side);
            board.rect(
                area.x + col as f64 * step + (step - dot) / 2.0,
                area.y + row as f64 * step + (step - dot) / 2.0,
                dot,
                dot,
                ink::fade(ink::DIM, 0.55),
            );
        }
    }
    let part = giant(side, &mask);
    let trail = walk(side, &mask, middle(side, &part), SEED);
    let mut seen = vec![false; mask.len()];
    for at in trail {
        if seen[at] {
            continue;
        }
        seen[at] = true;
        let (row, col) = (at / side, at % side);
        board.rect(
            area.x + col as f64 * step,
            area.y + row as f64 * step,
            step,
            step,
            color,
        );
    }
    Ok(mask.iter().filter(|f| **f).count())
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let half = frame.w / 2.0;
    let first = Frame::new(frame.x, frame.y, half, half).inset(8.0);
    let second = Frame::new(frame.x + half, frame.y + half, half, half).inset(8.0);
    for area in [first, second] {
        plot::axis(&mut board, area, ink::LINE);
    }
    let a = panel(&mut board, first.inset(10.0), 127, ink::BLUE)?;
    let b = panel(&mut board, second.inset(10.0), 239, ink::ORANGE)?;
    assert_eq!(a, 7usize.pow(LEVEL as u32));
    assert_eq!(b, a);
    save("research-walks", &board)?;
    Ok(())
}
