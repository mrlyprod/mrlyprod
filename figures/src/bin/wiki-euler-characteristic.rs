use mrlyfig::{ink, iso, save, Board, Frame, Grid};
use mrlyrs::core::errors::Result;
use mrlyrs::math::two::{census, designs};

const SIDE: usize = 9;

fn corner(i: usize) -> (f64, f64, f64) {
    ((i & 1) as f64, ((i >> 1) & 1) as f64, ((i >> 2) & 1) as f64)
}

fn cube(board: &mut Board, frame: Frame) -> (usize, usize, usize) {
    let flat: Vec<(f64, f64)> = (0..8)
        .map(|i| {
            let (x, y, z) = corner(i);
            iso::project(x, y, z)
        })
        .collect();
    let mut lo = (f64::MAX, f64::MAX);
    let mut hi = (f64::MIN, f64::MIN);
    for p in &flat {
        lo.0 = lo.0.min(p.0);
        lo.1 = lo.1.min(p.1);
        hi.0 = hi.0.max(p.0);
        hi.1 = hi.1.max(p.1);
    }
    let span = (hi.0 - lo.0).max(hi.1 - lo.1);
    let scale = frame.w.min(frame.h) * 0.95 / span;
    let (cx, cy) = frame.center();
    let place = |p: (f64, f64)| {
        (
            cx + (p.0 - (lo.0 + hi.0) / 2.0) * scale,
            cy + (p.1 - (lo.1 + hi.1) / 2.0) * scale,
        )
    };

    let faces = [[4usize, 5, 7, 6], [1, 3, 7, 5], [2, 3, 7, 6]];
    let tones = [
        ink::blue(),
        ink::mix(ink::blue(), ink::ground(), 0.3),
        ink::mix(ink::blue(), ink::ground(), 0.55),
    ];
    for (quad, tone) in faces.iter().zip(tones) {
        let pts: Vec<(f64, f64)> = quad.iter().map(|i| place(flat[*i])).collect();
        board.polygon(&pts, tone);
    }

    let mut edges = 0usize;
    for a in 0..8usize {
        for b in a + 1..8usize {
            if (a ^ b).count_ones() != 1 {
                continue;
            }
            let hidden = a == 0 || b == 0;
            board.segment(
                place(flat[a]),
                place(flat[b]),
                if hidden { 4.0 } else { 7.0 },
                if hidden { ink::dim() } else { ink::fg() },
            );
            edges += 1;
        }
    }
    for p in &flat {
        let (x, y) = place(*p);
        board.disc(x, y, 15.0, ink::yellow());
    }
    (8, edges, faces.len() * 2)
}

fn holes(board: &mut Board, frame: Frame) -> Result<usize> {
    let carpet = designs::carpet(3, 2)?;
    assert_eq!(carpet.width(), SIDE);
    let grid = Grid::new(frame, SIDE, SIDE, 0.0);
    grid.paint(board, &carpet, |kind| (kind != 0).then_some(ink::blue()));

    let mut seen = [[false; SIDE]; SIDE];
    let mut found = 0usize;
    for row in 0..SIDE {
        for col in 0..SIDE {
            if seen[row][col] || carpet.types().get(&[row, col]) != 0 {
                continue;
            }
            let mut stack = vec![(row, col)];
            let mut cells = Vec::new();
            seen[row][col] = true;
            while let Some((r, c)) = stack.pop() {
                cells.push((r, c));
                let step = |r: usize,
                            c: usize,
                            stack: &mut Vec<(usize, usize)>,
                            seen: &mut [[bool; SIDE]; SIDE]| {
                    if !seen[r][c] && carpet.types().get(&[r, c]) == 0 {
                        seen[r][c] = true;
                        stack.push((r, c));
                    }
                };
                if r > 0 {
                    step(r - 1, c, &mut stack, &mut seen);
                }
                if r + 1 < SIDE {
                    step(r + 1, c, &mut stack, &mut seen);
                }
                if c > 0 {
                    step(r, c - 1, &mut stack, &mut seen);
                }
                if c + 1 < SIDE {
                    step(r, c + 1, &mut stack, &mut seen);
                }
            }
            let rows: Vec<usize> = cells.iter().map(|(r, _)| *r).collect();
            let cols: Vec<usize> = cells.iter().map(|(_, c)| *c).collect();
            let (r0, r1) = (*rows.iter().min().unwrap(), *rows.iter().max().unwrap());
            let (c0, c1) = (*cols.iter().min().unwrap(), *cols.iter().max().unwrap());
            let (x0, y0, _, _) = grid.cell(c0, r0);
            let (x1, y1, w, h) = grid.cell(c1, r1);
            let cx = (x0 + x1 + w) / 2.0;
            let cy = (y0 + y1 + h) / 2.0;
            let radius = ((x1 + w - x0).min(y1 + h - y0)) * 0.62;
            board.ring(cx, cy, radius, 6.0, ink::orange());
            found += 1;
        }
    }
    Ok(found)
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.06);
    let panes = area.cols(2);
    let (vertices, edges, faces) = cube(&mut board, panes[0].square().inset(18.0));
    assert_eq!((vertices, edges, faces), (8, 12, 6));
    assert_eq!(vertices as i64 - edges as i64 + faces as i64, 2);
    let found = holes(&mut board, panes[1].square().inset(18.0))?;
    assert_eq!(found, 9);
    assert_eq!(census::euler(&designs::carpet(3, 2)?)?, 1 - found as i64);
    assert_eq!(census::euler(&designs::carpet(3, 3)?)?, -72);
    save("wiki-euler-characteristic", &board)?;
    Ok(())
}
