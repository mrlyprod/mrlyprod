use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};
use mrlymath::two;
use mrlynum::spirograph::{distinct, frame, pencils, trace, track, Kind};

const CODE: u128 = 495;
const SIDE: usize = 3;
const LEVEL: usize = 1;
const BASE: usize = 3;
const RING: usize = 13;
const WHEEL: usize = 5;
const REACH: f64 = 1.2;
const SAMPLES: usize = 6000;
const PENCILS: usize = 9;
const CURVES: usize = 9;
const ORBITS: usize = 5;
const MARGIN: f64 = 0.08;
const THICK: f64 = 2.2;
const HAIR: f64 = 1.5;

fn main() -> Result<()> {
    let cell = two::create(CODE, SIDE, LEVEL, 0, BASE)?;
    let types = cell.types().bytes().to_vec();
    let path = track("in", RING, WHEEL, 4, 1)?;
    let pens = pencils(&types, cell.width(), cell.height(), "both", REACH, 0.0, 1)?;
    assert_eq!(pens.len(), PENCILS);
    assert_eq!(distinct(&path, &pens, true), CURVES);
    assert_eq!(path.orbits, ORBITS);
    let points = trace(&path, &pens, SAMPLES)?;
    let [x0, y0, x1, y1] = frame(&path, &pens);

    let mut board = Board::square();
    let area = board.frame(MARGIN);
    let scale = (area.w / (x1 - x0)).min(area.h / (y1 - y0));
    let (cx, cy) = area.center();
    let at = |x: f64, y: f64| {
        (
            cx + (x - (x0 + x1) / 2.0) * scale,
            cy - (y - (y0 + y1) / 2.0) * scale,
        )
    };
    board.ring(cx, cy, RING as f64 * scale, HAIR, ink::dim());
    for (k, pencil) in pens.iter().enumerate() {
        let pts: Vec<(f64, f64)> = (0..SAMPLES)
            .map(|i| {
                let j = 2 * (k * SAMPLES + i);
                at(f64::from(points[j]), f64::from(points[j + 1]))
            })
            .collect();
        let color = if pencil.kind == Kind::Void {
            ink::orange()
        } else {
            ink::blue()
        };
        board.polyline(&pts, THICK, color);
    }
    save("demo-spirograph", &board)?;
    Ok(())
}
