use mrlycore::errors::Result;
use mrlyfig::{ink, iso, save, Board, Frame};
use mrlymath::three::designs;
use mrlymath::three::faces::quads;

const SIDE: usize = 3;
const HALF: f64 = SIDE as f64 / 2.0;

enum Mark {
    Face(Vec<(f64, f64)>, usize),
    Cage(Vec<[(f64, f64); 2]>),
}

fn even(c: &[u8]) -> usize {
    c.iter().filter(|&&v| v == 1).count()
}

fn corners() -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    for i in 0..SIDE {
        for j in 0..SIDE {
            for k in 0..SIDE {
                out.push(vec![i as u8, j as u8, k as u8]);
            }
        }
    }
    out
}

fn node(i: f64, j: f64, k: f64) -> (f64, f64) {
    iso::project((i - HALF) / HALF, (j - HALF) / HALF, (k - HALF) / HALF)
}

fn cage(c: &[u8]) -> Vec<[(f64, f64); 2]> {
    let (x, y, z) = (c[0] as f64, c[1] as f64, c[2] as f64);
    let corner = |b: usize| {
        node(
            x + (b & 1) as f64,
            y + ((b >> 1) & 1) as f64,
            z + ((b >> 2) & 1) as f64,
        )
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

fn place(marks: &[(f64, Mark)], frame: Frame) -> impl Fn((f64, f64)) -> (f64, f64) {
    let mut lo = (f64::MAX, f64::MAX);
    let mut hi = (f64::MIN, f64::MIN);
    for (_, mark) in marks {
        if let Mark::Face(pts, _) = mark {
            for p in pts {
                lo.0 = lo.0.min(p.0);
                lo.1 = lo.1.min(p.1);
                hi.0 = hi.0.max(p.0);
                hi.1 = hi.1.max(p.1);
            }
        }
    }
    let (sx, sy) = (hi.0 - lo.0, hi.1 - lo.1);
    let scale = (frame.w / sx).min(frame.h / sy);
    let ox = frame.x + (frame.w - sx * scale) / 2.0;
    let oy = frame.y + (frame.h - sy * scale) / 2.0;
    move |p: (f64, f64)| (ox + (p.0 - lo.0) * scale, oy + (p.1 - lo.1) * scale)
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let all = corners();
    let kept: Vec<Vec<u8>> = all.iter().filter(|c| even(c) <= 1).cloned().collect();
    let drilled: Vec<Vec<u8>> = all.iter().filter(|c| even(c) >= 2).cloned().collect();
    assert_eq!(kept.len(), 20);
    assert_eq!(drilled.len(), 7);
    let solid = designs::from_corners(&kept, SIDE, 1, SIDE)?;
    assert_eq!(solid.types().sum(), 20);
    let mut marks: Vec<(f64, Mark)> = Vec::new();
    for quad in quads(&solid) {
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
            .map(|v| iso::project(v.x as f64, v.y as f64, v.z as f64))
            .collect();
        let depth = quad
            .verts
            .iter()
            .map(|v| (v.x + v.y + v.z) as f64)
            .sum::<f64>()
            / 4.0;
        marks.push((depth, Mark::Face(pts, tone)));
    }
    for cell in &drilled {
        let mid = cell.iter().map(|&v| v as f64 + 0.5).sum::<f64>();
        let depth = (mid - 3.0 * HALF) / HALF;
        marks.push((depth, Mark::Cage(cage(cell))));
    }
    marks.sort_by(|a, b| a.0.total_cmp(&b.0));
    let put = place(&marks, frame);
    let gold = [
        ink::GOLD,
        ink::mix(ink::GOLD, ink::GROUND, 0.4),
        ink::mix(ink::GOLD, ink::GROUND, 0.66),
    ];
    let unit = frame.h / (SIDE as f64 * 2.0);
    for (_, mark) in &marks {
        match mark {
            Mark::Face(pts, tone) => {
                let screen: Vec<(f64, f64)> = pts.iter().map(|p| put(*p)).collect();
                board.polygon(&screen, gold[*tone]);
                let mut ring = screen.clone();
                ring.push(screen[0]);
                board.polyline(&ring, unit / 70.0, ink::LINE);
            }
            Mark::Cage(edges) => {
                for edge in edges {
                    board.segment(put(edge[0]), put(edge[1]), unit / 34.0, ink::DIM);
                }
            }
        }
    }
    save("paper-divisor-avatars", &board)?;
    Ok(())
}
