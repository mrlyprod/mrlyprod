use mrlycore::errors::Result;
use mrlyfig::{ink, plot, save, Board};
use mrlylab::roulette;
use mrlymath::two;
use mrlynum::spirograph::{
    cover, disc, distinct, nodes as law, pencils, trace, track, Cover, Pencil, Track,
};
use std::collections::HashSet;

const CODE: u128 = 495;
const SIDE: usize = 3;
const LEVEL: usize = 1;
const BASE: usize = 3;
const RING: usize = 7;
const WHEEL: usize = 3;
const REACH: f64 = 0.9;
const SAMPLES: usize = 8000;
const CURVES: usize = 8;
const CROSSINGS: usize = 1288;
const RASTER: usize = 1024;
const TOL: f64 = 4e-4;
const MARGIN: f64 = 0.08;
const GRID: usize = 256;
const SUB: usize = 4;
const WASH: f64 = 0.3;
const THIN: f64 = 1.4;
const HAIR: f64 = 1.1;
const DOT: f64 = 2.6;

fn main() -> Result<()> {
    let tile = two::create(CODE, SIDE, LEVEL, 0, BASE)?;
    let types = tile.types().bytes().to_vec();
    let path = track("in", RING, WHEEL, 4, 1)?;
    let pens = pencils(&types, tile.width(), tile.height(), "fill", REACH, 0.0, 1)?;
    assert_eq!(distinct(&path, &pens, true), CURVES);
    assert_eq!(law(&path, &pens, true), Some(CROSSINGS as u64));
    assert_eq!(
        roulette::nodes(&path, &pens, SAMPLES, TOL)?.total(),
        CROSSINGS
    );
    let marks = crossings(&path, &pens, SAMPLES)?;
    assert_eq!(marks.len(), CROSSINGS);
    let bounds = disc(&path, &pens)?;
    let shape = cover(&path, &pens, true, 2, RASTER)?;
    let points = trace(&path, &pens, SAMPLES)?;

    let mut board = Board::square();
    let area = board.frame(MARGIN);
    let (cx, cy) = area.center();
    let scale = area.radius() / RING as f64;
    let at = |x: f64, y: f64| (cx + (x - bounds.x) * scale, cy - (y - bounds.y) * scale);
    let span = 2.0 * bounds.radius * scale;
    wash(&mut board, &shape, (cx - span / 2.0, cy - span / 2.0), span);
    board.ring(cx, cy, RING as f64 * scale, HAIR, ink::line());
    for k in 0..CURVES {
        let curve: Vec<(f64, f64)> = (0..SAMPLES)
            .map(|i| {
                let j = 2 * (k * SAMPLES + i);
                at(f64::from(points[j]), f64::from(points[j + 1]))
            })
            .collect();
        board.polyline(&curve, THIN, ink::blue());
    }
    let dots: Vec<(f64, f64)> = marks.iter().map(|&(x, y)| at(x, y)).collect();
    plot::dots(&mut board, &dots, DOT, ink::orange());
    save("research-spirograph", &board)?;
    Ok(())
}

// THE SHAPE

fn wash(board: &mut Board, shape: &Cover, corner: (f64, f64), span: f64) {
    let n = shape.side;
    let cell = span / n as f64;
    let x0 = corner.0.floor().max(0.0) as usize;
    let y0 = corner.1.floor().max(0.0) as usize;
    let x1 = (corner.0 + span).ceil().max(0.0) as usize;
    let y1 = (corner.1 + span).ceil().max(0.0) as usize;
    for py in y0..y1 {
        for px in x0..x1 {
            let mut hit = 0;
            for sy in 0..SUB {
                for sx in 0..SUB {
                    let u = (px as f64 + (sx as f64 + 0.5) / SUB as f64 - corner.0) / cell;
                    let v = (py as f64 + (sy as f64 + 0.5) / SUB as f64 - corner.1) / cell;
                    if u < 0.0 || v < 0.0 {
                        continue;
                    }
                    let (column, row) = (u as usize, v as usize);
                    if column < n && row < n && shape.mask[row * n + column] == 3 {
                        hit += 1;
                    }
                }
            }
            if hit > 0 {
                board.blend(
                    px,
                    py,
                    ink::dim(),
                    WASH * f64::from(hit) / (SUB * SUB) as f64,
                );
            }
        }
    }
}

// THE NODES

fn crossings(path: &Track, pens: &[Pencil], samples: usize) -> Result<Vec<(f64, f64)>> {
    let points = trace(path, pens, samples)?;
    let steps = samples - 1;
    let at = |k: usize, i: usize| {
        let base = 2 * (k * samples + i);
        (f64::from(points[base]), f64::from(points[base + 1]))
    };
    let (mut low, mut high) = ((f64::MAX, f64::MAX), (f64::MIN, f64::MIN));
    for pair in points.chunks_exact(2) {
        let (x, y) = (f64::from(pair[0]), f64::from(pair[1]));
        low = (low.0.min(x), low.1.min(y));
        high = (high.0.max(x), high.1.max(y));
    }
    let size = (high.0 - low.0, high.1 - low.1);
    let cell = |p: (f64, f64)| {
        let column = ((p.0 - low.0) / size.0 * GRID as f64) as usize;
        let row = ((p.1 - low.1) / size.1 * GRID as f64) as usize;
        (column.min(GRID - 1), row.min(GRID - 1))
    };
    let mut buckets: Vec<Vec<u32>> = vec![Vec::new(); GRID * GRID];
    for k in 0..pens.len() {
        for i in 0..steps {
            let (a, b) = (at(k, i), at(k, i + 1));
            let (x0, y0) = cell((a.0.min(b.0), a.1.min(b.1)));
            let (x1, y1) = cell((a.0.max(b.0), a.1.max(b.1)));
            for x in x0..=x1 {
                for y in y0..=y1 {
                    buckets[x * GRID + y].push((k * steps + i) as u32);
                }
            }
        }
    }
    let mut seen: HashSet<(u32, u32)> = HashSet::new();
    let mut out = Vec::new();
    for bucket in &buckets {
        for (u, &left) in bucket.iter().enumerate() {
            for &right in &bucket[u + 1..] {
                let (p, q) = (left as usize, right as usize);
                let ((kp, ip), (kq, iq)) = ((p / steps, p % steps), (q / steps, q % steps));
                let gap = ip.abs_diff(iq);
                if kp == kq && (gap == 1 || gap == steps - 1) {
                    continue;
                }
                let (a, b) = (at(kp, ip), at(kp, ip + 1));
                let (c, d) = (at(kq, iq), at(kq, iq + 1));
                if side(a, b, c) * side(a, b, d) < 0
                    && side(c, d, a) * side(c, d, b) < 0
                    && seen.insert((left.min(right), left.max(right)))
                {
                    out.push(meet(a, b, c, d));
                }
            }
        }
    }
    Ok(out)
}

fn side(a: (f64, f64), b: (f64, f64), c: (f64, f64)) -> i32 {
    let turn = (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0);
    match turn.partial_cmp(&0.0) {
        Some(std::cmp::Ordering::Greater) => 1,
        Some(std::cmp::Ordering::Less) => -1,
        _ => 0,
    }
}

fn meet(a: (f64, f64), b: (f64, f64), c: (f64, f64), d: (f64, f64)) -> (f64, f64) {
    let run = (b.0 - a.0, b.1 - a.1);
    let other = (d.0 - c.0, d.1 - c.1);
    let denominator = run.0 * other.1 - run.1 * other.0;
    let step = ((c.0 - a.0) * other.1 - (c.1 - a.1) * other.0) / denominator;
    (a.0 + step * run.0, a.1 + step * run.1)
}
