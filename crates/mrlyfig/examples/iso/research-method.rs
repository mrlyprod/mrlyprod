use mrlycore::errors::Result;
use mrlyfig::{ink, iso, save, Board, Color};
use mrlymath::bang::bang;
use mrlymath::three::designs;
use mrlymath::three::faces::quads;
use mrlymath::three::Cell3d;

const ROWS: usize = 9;
const WIDEST: f64 = 6.5;

fn cage(cx: f64, cy: f64, s: f64) -> Vec<[(f64, f64); 2]> {
    let corner = |i: usize| {
        let bit = |b: usize| ((i >> b) & 1) as f64 * 2.0 - 1.0;
        let p = iso::project(bit(0), bit(1), bit(2));
        (cx + p.0 * s, cy + p.1 * s)
    };
    let mut out = Vec::new();
    for a in 0usize..8 {
        for b in (a + 1)..8 {
            if (a ^ b).count_ones() == 1 {
                out.push([corner(a), corner(b)]);
            }
        }
    }
    out
}

fn stamp(board: &mut Board, cell: &Cell3d, cx: f64, cy: f64, s: f64, shade: [Color; 3]) {
    let mut faces = Vec::new();
    for quad in quads(cell) {
        let n = (
            quad.normal.x as f64,
            quad.normal.y as f64,
            quad.normal.z as f64,
        );
        if n.0 + n.1 + n.2 <= 0.0 {
            continue;
        }
        let tone = if n.2 > 0.0 {
            0
        } else if n.1 > 0.0 {
            1
        } else {
            2
        };
        let pts: Vec<(f64, f64)> = quad
            .verts
            .iter()
            .map(|v| {
                let p = iso::project(v.x as f64, v.y as f64, v.z as f64);
                (cx + p.0 * s, cy + p.1 * s)
            })
            .collect();
        let depth = quad
            .verts
            .iter()
            .map(|v| (v.x + v.y + v.z) as f64)
            .sum::<f64>();
        faces.push((depth, tone, pts));
    }
    faces.sort_by(|a, b| a.0.total_cmp(&b.0));
    for (_, tone, pts) in &faces {
        board.polygon(pts, shade[*tone]);
        let mut ring = pts.clone();
        ring.push(pts[0]);
        board.polyline(&ring, s / 14.0, ink::GROUND);
    }
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let mut rows: Vec<Vec<u128>> = vec![Vec::new(); ROWS];
    for design in bang(3).canonical() {
        rows[design.i.count_ones() as usize].push(design.i);
    }
    assert_eq!(
        rows.iter().map(|r| r.len()).collect::<Vec<usize>>(),
        vec![1, 1, 3, 3, 6, 3, 3, 1, 1]
    );
    assert_eq!(rows.iter().map(|r| r.len()).sum::<usize>(), 22);
    let shade = [
        ink::BLUE,
        ink::mix(ink::BLUE, ink::GROUND, 0.4),
        ink::mix(ink::BLUE, ink::GROUND, 0.65),
    ];
    let (mx, my) = frame.center();
    let pitch = frame.h / ROWS as f64;
    let step = frame.w / WIDEST;
    let s = pitch * 0.225;
    for (r, codes) in rows.iter().enumerate() {
        let cy = my + (r as f64 - (ROWS as f64 - 1.0) / 2.0) * pitch;
        for (j, code) in codes.iter().enumerate() {
            let cx = mx + (j as f64 - (codes.len() as f64 - 1.0) / 2.0) * step;
            for edge in cage(cx, cy, s) {
                board.segment(
                    edge[0],
                    edge[1],
                    s / 20.0,
                    ink::mix(ink::LINE, ink::DIM, 0.3),
                );
            }
            let cube = designs::create(*code, 2, 1, 2)?;
            assert_eq!(cube.types().sum(), r as u64);
            stamp(&mut board, &cube, cx, cy, s, shade);
        }
    }
    save("research-method", &board)?;
    Ok(())
}
