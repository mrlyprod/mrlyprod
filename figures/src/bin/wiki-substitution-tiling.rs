use figures::{ink, save, Board, Frame};
use mrlyrs::core::error::Result;

const DEPTH: usize = 3;
const GAPS: [f64; DEPTH] = [22.0, 11.0, 4.0];

#[derive(Clone, Copy)]
struct Chair {
    x: f64,
    y: f64,
    side: f64,
    turn: usize,
}

fn spin(p: (f64, f64), centre: (f64, f64), turn: usize) -> (f64, f64) {
    let (mut dx, mut dy) = (p.0 - centre.0, p.1 - centre.1);
    for _ in 0..turn % 4 {
        (dx, dy) = (-dy, dx);
    }
    (centre.0 + dx, centre.1 + dy)
}

fn outline(c: Chair) -> Vec<(f64, f64)> {
    let (s, h) = (c.side, c.side / 2.0);
    let centre = (c.x + h, c.y + h);
    [(0.0, 0.0), (s, 0.0), (s, h), (h, h), (h, s), (0.0, s)]
        .iter()
        .map(|&(u, v)| spin((c.x + u, c.y + v), centre, c.turn))
        .collect()
}

fn inflate(c: Chair) -> [Chair; 4] {
    let h = c.side / 2.0;
    let q = c.side / 4.0;
    let centre = (c.x + h, c.y + h);
    let place = |x: f64, y: f64, turn: usize| {
        let mid = spin((c.x + x + q, c.y + y + q), centre, c.turn);
        Chair {
            x: mid.0 - q,
            y: mid.1 - q,
            side: h,
            turn: (c.turn + turn) % 4,
        }
    };
    [
        place(0.0, 0.0, 0),
        place(q, q, 0),
        place(h, 0.0, 1),
        place(0.0, h, 3),
    ]
}

fn area(c: Chair) -> f64 {
    0.75 * c.side * c.side
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame: Frame = board.frame(0.08);
    let root = Chair {
        x: 0.0,
        y: 0.0,
        side: 1.0,
        turn: 0,
    };
    let mut levels = vec![vec![root]];
    for _ in 0..DEPTH {
        let next: Vec<Chair> = levels
            .last()
            .unwrap()
            .iter()
            .flat_map(|c| inflate(*c))
            .collect();
        levels.push(next);
    }
    let counts: Vec<usize> = levels.iter().map(Vec::len).collect();
    assert_eq!(counts, vec![1, 4, 16, 64]);
    let leaves = levels.last().unwrap();
    let total: f64 = leaves.iter().map(|c| area(*c)).sum();
    assert!((total - area(root)).abs() < 1e-12);
    let mut cells = std::collections::HashSet::new();
    let grain = 2.0 / leaves[0].side;
    for c in leaves {
        let h = c.side / 2.0;
        for (u, v) in [(0.25, 0.25), (0.75, 0.25), (0.25, 0.75), (0.75, 0.75)] {
            let p = spin(
                (c.x + u * c.side, c.y + v * c.side),
                (c.x + h, c.y + h),
                c.turn,
            );
            let inside = !(u > 0.5 && v > 0.5);
            if inside {
                assert!(cells.insert(((p.0 * grain).floor() as i64, (p.1 * grain).floor() as i64)));
            }
        }
    }
    assert_eq!(cells.len(), 3 * 64);
    assert!(cells
        .iter()
        .all(|&(i, j)| (0..16).contains(&i) && (0..16).contains(&j) && (i < 8 || j < 8)));
    let even = leaves.iter().filter(|c| c.turn.is_multiple_of(2)).count();
    assert_eq!((even, leaves.len() - even), (32, 32));

    let map = |p: (f64, f64)| (frame.x + p.0 * frame.w, frame.y + (1.0 - p.1) * frame.h);
    for c in leaves {
        let pts: Vec<(f64, f64)> = outline(*c).into_iter().map(map).collect();
        let color = if c.turn.is_multiple_of(2) {
            ink::blue()
        } else {
            ink::yellow()
        };
        board.polygon(&pts, color);
    }
    for depth in (1..=DEPTH).rev() {
        for c in &levels[depth] {
            let pts: Vec<(f64, f64)> = outline(*c).into_iter().map(map).collect();
            let gap = GAPS[depth - 1];
            for (i, a) in pts.iter().enumerate() {
                let b = pts[(i + 1) % pts.len()];
                let (x, y) = (a.0.min(b.0) - gap / 2.0, a.1.min(b.1) - gap / 2.0);
                let (w, h) = ((a.0 - b.0).abs() + gap, (a.1 - b.1).abs() + gap);
                board.rect(x, y, w, h, ink::ground());
            }
        }
    }
    save("wiki-substitution-tiling", &board)?;
    Ok(())
}
