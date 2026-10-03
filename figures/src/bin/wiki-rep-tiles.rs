use figures::{ink, save, Board};
use mrlyrs::core::error::Result;
use std::collections::HashSet;

const DEPTH: usize = 3;
const GAPS: [f64; DEPTH] = [16.0, 8.0, 3.0];
const SPHINX: [[(i64, i64); 3]; 6] = [
    [(0, 0), (1, 0), (0, 1)],
    [(1, 0), (2, 0), (1, 1)],
    [(2, 0), (3, 0), (2, 1)],
    [(1, 0), (0, 1), (1, 1)],
    [(2, 0), (1, 1), (2, 1)],
    [(0, 1), (1, 1), (0, 2)],
];
const OUTLINE: [(i64, i64); 5] = [(0, 0), (3, 0), (2, 1), (1, 1), (0, 2)];
const ID: Mat = [[1, 0], [0, 1]];
const TURN: Mat = [[0, -1], [1, 1]];
const MIRROR: Mat = [[1, 1], [0, -1]];

type Mat = [[i64; 2]; 2];
type Point = (i64, i64);

#[derive(Clone, Copy)]
struct Tile {
    m: Mat,
    t: Point,
}

fn mul(a: Mat, b: Mat) -> Mat {
    let cell = |i: usize, j: usize| a[i][0] * b[0][j] + a[i][1] * b[1][j];
    [[cell(0, 0), cell(0, 1)], [cell(1, 0), cell(1, 1)]]
}

fn apply(m: Mat, p: Point) -> Point {
    (m[0][0] * p.0 + m[0][1] * p.1, m[1][0] * p.0 + m[1][1] * p.1)
}

fn det(m: Mat) -> i64 {
    m[0][0] * m[1][1] - m[0][1] * m[1][0]
}

fn place(tile: Tile, p: Point) -> Point {
    let q = apply(tile.m, p);
    (q.0 + tile.t.0, q.1 + tile.t.1)
}

fn key(tri: [Point; 3]) -> Point {
    (
        tri[0].0 + tri[1].0 + tri[2].0,
        tri[0].1 + tri[1].1 + tri[2].1,
    )
}

fn split(tri: [Point; 3]) -> [[Point; 3]; 4] {
    let [p, q, r] = tri;
    let two = |a: Point| (2 * a.0, 2 * a.1);
    let sum = |a: Point, b: Point| (a.0 + b.0, a.1 + b.1);
    let (pq, qr, pr) = (sum(p, q), sum(q, r), sum(p, r));
    [
        [two(p), pq, pr],
        [two(q), pq, qr],
        [two(r), pr, qr],
        [pq, qr, pr],
    ]
}

fn cells(tile: Tile) -> Vec<Point> {
    SPHINX
        .iter()
        .map(|tri| key(tri.map(|p| place(tile, p))))
        .collect()
}

fn cover(
    target: &HashSet<Point>,
    pieces: &[(Tile, Vec<Point>)],
    used: &mut Vec<usize>,
    found: &mut Vec<Vec<usize>>,
) {
    let covered: HashSet<Point> = used
        .iter()
        .flat_map(|&i| pieces[i].1.iter().copied())
        .collect();
    let Some(&first) = target.iter().filter(|c| !covered.contains(c)).min() else {
        found.push(used.clone());
        return;
    };
    for (i, (_, keys)) in pieces.iter().enumerate() {
        if keys.contains(&first) && keys.iter().all(|k| !covered.contains(k)) {
            used.push(i);
            cover(target, pieces, used, found);
            used.pop();
        }
    }
}

fn main() -> Result<()> {
    let doubled: HashSet<Point> = SPHINX.iter().flat_map(|&tri| split(tri)).map(key).collect();
    assert_eq!(doubled.len(), 24);
    let mut pieces = Vec::new();
    for flip in [ID, MIRROR] {
        let mut m = flip;
        for _ in 0..6 {
            for a in -8..=8 {
                for b in -8..=8 {
                    let tile = Tile { m, t: (a, b) };
                    let keys = cells(tile);
                    if keys.iter().all(|k| doubled.contains(k)) {
                        pieces.push((tile, keys));
                    }
                }
            }
            m = mul(TURN, m);
        }
    }
    let mut found = Vec::new();
    cover(&doubled, &pieces, &mut Vec::new(), &mut found);
    assert_eq!(found.len(), 1);
    let rule: Vec<Tile> = found[0].iter().map(|&i| pieces[i].0).collect();
    assert_eq!(rule.len(), 4);
    assert_eq!(rule.iter().filter(|c| det(c.m) < 0).count(), 3);

    let mut levels = vec![vec![Tile { m: ID, t: (0, 0) }]];
    for _ in 0..DEPTH {
        let next: Vec<Tile> = levels
            .last()
            .unwrap()
            .iter()
            .flat_map(|p| {
                rule.iter().map(move |c| {
                    let shift = apply(p.m, c.t);
                    Tile {
                        m: mul(p.m, c.m),
                        t: (shift.0 + 2 * p.t.0, shift.1 + 2 * p.t.1),
                    }
                })
            })
            .collect();
        levels.push(next);
    }
    let counts: Vec<usize> = levels.iter().map(Vec::len).collect();
    assert_eq!(counts, vec![1, 4, 16, 64]);
    let leaves = &levels[DEPTH];
    let direct = leaves.iter().filter(|c| det(c.m) > 0).count();
    assert_eq!((direct, leaves.len() - direct), (28, 36));
    let mut fine: Vec<[Point; 3]> = SPHINX.to_vec();
    for _ in 0..DEPTH {
        fine = fine.into_iter().flat_map(split).collect();
    }
    let fine: HashSet<Point> = fine.into_iter().map(key).collect();
    let mut laid = HashSet::new();
    for c in leaves {
        for k in cells(*c) {
            assert!(laid.insert(k));
        }
    }
    assert_eq!(laid.len(), 6 * 64);
    assert_eq!(laid, fine);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let rise = 3f64.sqrt() / 2.0;
    let px = frame.w / 3.0;
    let top = frame.y + (frame.h - 2.0 * rise * px) / 2.0;
    let outline = |tile: &Tile, depth: usize| -> Vec<(f64, f64)> {
        let scale = (1 << depth) as f64;
        OUTLINE
            .iter()
            .map(|&p| {
                let (a, b) = place(*tile, p);
                let (a, b) = (a as f64 / scale, b as f64 / scale);
                (
                    frame.x + (a + b / 2.0) * px,
                    top + (2.0 * rise - b * rise) * px,
                )
            })
            .collect()
    };
    for c in leaves {
        let color = if det(c.m) > 0 {
            ink::blue()
        } else {
            ink::orange()
        };
        board.polygon(&outline(c, DEPTH), color);
    }
    for depth in (1..=DEPTH).rev() {
        for c in &levels[depth] {
            let mut ring = outline(c, depth);
            ring.push(ring[0]);
            board.polyline(&ring, GAPS[depth - 1], ink::ground());
        }
    }
    save("wiki-rep-tiles", &board)?;
    Ok(())
}
