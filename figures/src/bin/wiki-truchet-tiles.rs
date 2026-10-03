use figures::{ink, save, Board, Color};
use mrlyrs::core::error::Result;
use mrlyrs::core::Rng;
use std::f64::consts::{FRAC_PI_2, PI};

const SIDE: usize = 16;
const SEED: u64 = 7;
const LOOPS: usize = 19;
const SAMPLES: usize = 3;

fn top(row: usize, col: usize) -> usize {
    row * SIDE + col
}

fn left(row: usize, col: usize) -> usize {
    (SIDE + 1) * SIDE + row * (SIDE + 1) + col
}

fn arcs(turn: bool, row: usize, col: usize) -> [(usize, usize); 2] {
    let (t, b) = (top(row, col), top(row + 1, col));
    let (l, r) = (left(row, col), left(row, col + 1));
    if turn {
        [(t, r), (l, b)]
    } else {
        [(t, l), (r, b)]
    }
}

fn centres(turn: bool) -> [(f64, f64); 2] {
    if turn {
        [(1.0, 0.0), (0.0, 1.0)]
    } else {
        [(0.0, 0.0), (1.0, 1.0)]
    }
}

fn even(turns: &[bool], u: f64, v: f64) -> bool {
    let col = (u.floor() as usize).min(SIDE - 1);
    let row = (v.floor() as usize).min(SIDE - 1);
    let turn = turns[row * SIDE + col];
    let (x, y) = (u - col as f64, v - row as f64);
    let corner = |c: (f64, f64)| (row + col + c.0 as usize + c.1 as usize).is_multiple_of(2);
    let [a, b] = centres(turn);
    for c in [a, b] {
        if (x - c.0).hypot(y - c.1) < 0.5 {
            return corner(c);
        }
    }
    !corner(a)
}

fn main() -> Result<()> {
    let mut rng = Rng::new(SEED);
    let turns: Vec<bool> = (0..SIDE * SIDE).map(|_| rng.boolean()).collect();
    let nodes = 2 * SIDE * (SIDE + 1);
    let mut links: Vec<Vec<usize>> = vec![Vec::new(); nodes];
    let mut ends = Vec::new();
    for row in 0..SIDE {
        for col in 0..SIDE {
            for (a, b) in arcs(turns[row * SIDE + col], row, col) {
                let arc = ends.len();
                ends.push((a, b));
                links[a].push(arc);
                links[b].push(arc);
            }
        }
    }
    assert_eq!(ends.len(), 2 * SIDE * SIDE);
    let border = links.iter().filter(|l| l.len() == 1).count();
    assert_eq!(border, 4 * SIDE);

    let mut curve = vec![usize::MAX; ends.len()];
    let mut open = Vec::new();
    for start in 0..ends.len() {
        if curve[start] != usize::MAX {
            continue;
        }
        let id = open.len();
        let mut stack = vec![start];
        let mut touches = false;
        while let Some(arc) = stack.pop() {
            if curve[arc] != usize::MAX {
                continue;
            }
            curve[arc] = id;
            let (a, b) = ends[arc];
            for node in [a, b] {
                touches |= links[node].len() == 1;
                stack.extend(links[node].iter().filter(|&&n| curve[n] == usize::MAX));
            }
        }
        open.push(touches);
    }
    let strands = open.iter().filter(|o| **o).count();
    assert_eq!(strands, 2 * SIDE);
    assert_eq!(open.len() - strands, LOOPS);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let cell = frame.w / SIDE as f64;
    let tint = ink::fade(ink::dim(), 0.25);
    let x0 = frame.x.floor() as usize;
    let x1 = (frame.x + frame.w).ceil() as usize;
    for py in x0..x1 {
        for px in x0..x1 {
            let mut hits = 0;
            for sy in 0..SAMPLES {
                for sx in 0..SAMPLES {
                    let x = px as f64 + (sx as f64 + 0.5) / SAMPLES as f64;
                    let y = py as f64 + (sy as f64 + 0.5) / SAMPLES as f64;
                    let (u, v) = ((x - frame.x) / cell, (y - frame.y) / cell);
                    let inside = (0.0..SIDE as f64).contains(&u) && (0.0..SIDE as f64).contains(&v);
                    if inside && even(&turns, u, v) {
                        hits += 1;
                    }
                }
            }
            board.blend(px, py, tint, hits as f64 / (SAMPLES * SAMPLES) as f64);
        }
    }

    let thick = cell * 0.15;
    let radius = cell / 2.0;
    let mut arc = 0;
    for row in 0..SIDE {
        for col in 0..SIDE {
            let turn = turns[row * SIDE + col];
            let (x, y) = (frame.x + col as f64 * cell, frame.y + row as f64 * cell);
            let sweeps: [((f64, f64), (f64, f64)); 2] = if turn {
                [
                    ((x + cell, y), (FRAC_PI_2, PI)),
                    ((x, y + cell), (-FRAC_PI_2, 0.0)),
                ]
            } else {
                [
                    ((x, y), (0.0, FRAC_PI_2)),
                    ((x + cell, y + cell), (PI, PI + FRAC_PI_2)),
                ]
            };
            for (centre, angles) in sweeps {
                let color: Color = if open[curve[arc]] {
                    ink::blue()
                } else {
                    ink::yellow()
                };
                board.arc(centre, radius, angles, thick, color);
                arc += 1;
            }
        }
    }
    let (w, end) = (board.width as f64, frame.x + frame.w);
    let ground = ink::ground();
    board.rect(0.0, 0.0, w, frame.y, ground);
    board.rect(0.0, end, w, w - end, ground);
    board.rect(0.0, 0.0, frame.x, w, ground);
    board.rect(end, 0.0, w - end, w, ground);
    save("wiki-truchet-tiles", &board)?;
    Ok(())
}
