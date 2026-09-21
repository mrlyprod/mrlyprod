use mrlyfig::{ink, save, Board};
use mrlyrs::core::error::Result;
use mrlyrs::math::spirograph::{frame as bounds, point, track, Kind, Pencil};

const RING: usize = 7;
const WHEEL: usize = 3;
const REACH: f64 = 0.8;
const SAMPLES: usize = 4000;

fn main() -> Result<()> {
    let path = track("in", RING, WHEEL, 4, 1)?;
    assert_eq!((path.ratio, path.orbits, path.fold), ((RING, WHEEL), 3, 7));
    let pen = Pencil {
        x: REACH,
        y: 0.0,
        seat: (0, 0),
        kind: Kind::Fill,
    };
    let curve: Vec<(f64, f64)> = (0..SAMPLES)
        .map(|k| {
            let s = path.total * k as f64 / (SAMPLES - 1) as f64;
            point(&path, &pen, s)
        })
        .collect();
    assert_eq!(curve.len(), SAMPLES);
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let [x0, y0, x1, y1] = bounds(&path, &[pen]);
    let scale = (frame.w / (x1 - x0)).min(frame.h / (y1 - y0));
    let (cx, cy) = frame.center();
    board.ring(cx, cy, RING as f64 * scale, 1.5, ink::dim());
    let pts: Vec<(f64, f64)> = curve
        .iter()
        .map(|&(x, y)| (cx + x * scale, cy - y * scale))
        .collect();
    board.polyline(&pts, 2.4, ink::blue());
    save("wiki-spirograph", &board)?;
    Ok(())
}
