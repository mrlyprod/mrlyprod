use mrlycore::errors::Result;
use mrlyfig::{ink, save, Board};

const ARROWS: usize = 8;
const MATRIX: [[f64; 2]; 2] = [[2.0, 1.0], [1.0, 3.0]];
const START: (f64, f64) = (1.0, -0.6);

fn push(v: (f64, f64)) -> (f64, f64) {
    (
        MATRIX[0][0] * v.0 + MATRIX[0][1] * v.1,
        MATRIX[1][0] * v.0 + MATRIX[1][1] * v.1,
    )
}

fn main() -> Result<()> {
    let mut v = START;
    let mut angles = Vec::with_capacity(ARROWS);
    for _ in 0..ARROWS {
        angles.push(v.1.atan2(v.0));
        v = push(v);
    }
    let golden = (1.0 + 5f64.sqrt()) / 2.0;
    let settled = golden.atan2(1.0);
    assert_eq!(angles.len(), ARROWS);
    assert!(angles.windows(2).all(|pair| pair[1] > pair[0]));
    assert!((settled - angles[ARROWS - 1]).abs() < 0.10);
    assert!(angles[0] < 0.0);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let (cx, cy) = frame.at(0.30, 0.59);
    let reach = frame.w * 0.62;
    let far = frame.w * 0.70;
    board.segment(
        (cx - far * settled.cos(), cy + far * settled.sin()),
        (cx + far * settled.cos(), cy - far * settled.sin()),
        3.0,
        ink::fade(ink::dim(), 0.8),
    );
    board.arc(
        (cx, cy),
        reach * 1.05,
        (-settled - 0.12, -angles[0] + 0.12),
        3.0,
        ink::fade(ink::dim(), 0.8),
    );
    for (index, angle) in angles.iter().enumerate() {
        let heat = index as f64 / (ARROWS - 1) as f64;
        let paint = ink::mix(ink::blue(), ink::yellow(), heat);
        let tip = (cx + reach * angle.cos(), cy - reach * angle.sin());
        let way = (angle.cos(), -angle.sin());
        let side = (-way.1, way.0);
        let head = frame.w * 0.045;
        let base = (tip.0 - way.0 * head, tip.1 - way.1 * head);
        board.segment((cx, cy), base, 7.0, paint);
        board.triangle(
            tip,
            (base.0 + side.0 * head * 0.5, base.1 + side.1 * head * 0.5),
            (base.0 - side.0 * head * 0.5, base.1 - side.1 * head * 0.5),
            paint,
        );
    }
    board.disc(cx, cy, frame.w * 0.018, ink::fg());
    save("wiki-spectral-radius", &board)?;
    Ok(())
}
