use mrlycore::colors::{INDIGO, INDIGO_DARK, INDIGO_LIGHT};
use mrlycore::errors::Result;
use mrlyfig::{ink, iso, save, Board, Color};
use mrlymath::bang::bang;
use mrlymath::three::designs;
use mrlymath::three::faces::quads;
use mrlymath::three::Cell3d;

const NUMBER: usize = 3;
const ROWS: [usize; 5] = [4, 5, 4, 5, 4];
const HALF: f64 = 1.5;

fn cage(board: &mut Board, cx: f64, cy: f64, s: f64) {
    let corner = |i: usize| {
        let bit = |b: usize| ((i >> b) & 1) as f64 * 2.0 - 1.0;
        let p = iso::project(bit(0) * HALF, bit(1) * HALF, bit(2) * HALF);
        (cx + p.0 * s, cy + p.1 * s)
    };
    for a in 0usize..8 {
        for b in (a + 1)..8 {
            if (a ^ b).count_ones() == 1 {
                board.segment(
                    corner(a),
                    corner(b),
                    s / 24.0,
                    ink::mix(ink::line(), ink::dim(), 0.3),
                );
            }
        }
    }
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
                let p = iso::project(
                    v.x as f64 - HALF,
                    v.y as f64 - HALF,
                    v.z as f64 - HALF,
                );
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
        board.polyline(&ring, s / 18.0, ink::ground());
    }
}

fn weight(corner: &[u8]) -> u64 {
    corner.iter().map(|bit| if *bit == 0 { 2 } else { 1 }).product()
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let gallery = bang(3).canonical();
    assert_eq!(gallery.len(), 22);
    assert_eq!(ROWS.iter().sum::<usize>(), gallery.len());
    let shade = [INDIGO_LIGHT, INDIGO, INDIGO_DARK];
    let (mx, _) = frame.center();
    let pitch = (frame.w / 5.0, frame.h / 5.0);
    let s = (pitch.0 * 0.46 / (3.0 * 0.866_025_403_784_438_6)).min(pitch.1 * 0.46 / 3.0);
    let mut taken = 0usize;
    for (r, count) in ROWS.iter().enumerate() {
        let cy = frame.y + (r as f64 + 0.5) * pitch.1;
        for j in 0..*count {
            let design = &gallery[taken];
            let cx = mx + (j as f64 - (*count as f64 - 1.0) / 2.0) * pitch.0;
            let solid = designs::create(design.i, NUMBER, 1, 2)?;
            assert_eq!((solid.width(), solid.height(), solid.depth()), (3, 3, 3));
            assert_eq!(
                solid.types().sum(),
                design.rule().iter().map(|c| weight(c.as_slice())).sum::<u64>()
            );
            cage(&mut board, cx, cy, s);
            stamp(&mut board, &solid, cx, cy, s, shade);
            taken += 1;
        }
    }
    assert_eq!(taken, 22);
    save("demo-universe", &board)?;
    Ok(())
}
