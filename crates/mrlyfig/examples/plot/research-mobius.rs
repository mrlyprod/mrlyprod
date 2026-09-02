use mrlycore::errors::Result;
use mrlyfig::{ink, plot, save, Board};
use mrlynum::factor::mobius;

const LENGTH: u32 = 16;
const STEPS: usize = 1 << LENGTH;
const DRAWN: usize = 4096;

fn spread(index: usize) -> usize {
    let mut value = 0usize;
    for bit in 0..LENGTH {
        if index >> bit & 1 == 1 {
            value += 4usize.pow(bit);
        }
    }
    value
}

fn walks() -> (Vec<f64>, Vec<f64>) {
    let mut low = Vec::with_capacity(STEPS + 1);
    let mut high = Vec::with_capacity(STEPS + 1);
    let (mut a, mut b) = (0i64, 0i64);
    low.push(0.0);
    high.push(0.0);
    for index in 0..STEPS {
        let value = spread(index);
        a += mobius(value) as i64;
        b += mobius(2 * value) as i64;
        assert_eq!(b, -a);
        low.push(a as f64);
        high.push(b as f64);
    }
    (low, high)
}

fn main() -> Result<()> {
    let (low, high) = walks();
    let reach = (STEPS as f64).sqrt();
    let peak = low.iter().fold(0.0f64, |m, v| m.max(v.abs()));
    assert!(peak < reach);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let at = |index: f64, meter: f64| {
        (
            frame.x + frame.w * index / STEPS as f64,
            frame.y + frame.h * (0.5 - meter / (2.0 * reach)),
        )
    };
    board.segment(at(0.0, 0.0), at(STEPS as f64, 0.0), 1.6, ink::LINE);
    plot::axis(&mut board, frame, ink::LINE);
    for sign in [1.0f64, -1.0] {
        let envelope: Vec<(f64, f64)> = (0..=DRAWN)
            .map(|k| {
                let index = STEPS as f64 * k as f64 / DRAWN as f64;
                at(index, sign * index.sqrt())
            })
            .collect();
        board.polyline(&envelope, 2.0, ink::fade(ink::DIM, 0.75));
    }
    let stride = STEPS / DRAWN;
    for (walk, color) in [(&low, ink::BLUE), (&high, ink::ORANGE)] {
        let trace: Vec<(f64, f64)> = (0..=DRAWN)
            .map(|k| at((k * stride) as f64, walk[k * stride]))
            .collect();
        board.polyline(&trace, 2.4, color);
    }
    save("research-mobius", &board)?;
    Ok(())
}
