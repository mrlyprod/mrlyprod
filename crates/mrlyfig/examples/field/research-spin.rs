use mrlycore::errors::Result;
use mrlyfig::board::Board;
use mrlyfig::ink::Ramp;
use mrlyfig::{ink, save};
use mrlymath::two;
use mrlynum::spin;

const LEVEL: usize = 5;
const SIDE: usize = 243;
const RINGS: usize = 1600;

fn mirror(cell: &two::Cell2d) -> Vec<f32> {
    let span = 2 * SIDE;
    let types = cell.types();
    let mut out = vec![0f32; span * span];
    for row in 0..span {
        let y = if row < SIDE {
            SIDE - 1 - row
        } else {
            row - SIDE
        };
        for col in 0..span {
            let x = if col < SIDE {
                SIDE - 1 - col
            } else {
                col - SIDE
            };
            out[row * span + col] = types.get(&[y, x]) as f32;
        }
    }
    out
}

fn main() -> Result<()> {
    let carpet = two::designs::create(495, 3, LEVEL, 0, 3)?;
    assert_eq!(carpet.width(), SIDE);
    assert_eq!(carpet.types().sum(), 8u64.pow(LEVEL as u32));
    let span = 2 * SIDE;
    let board_data = mirror(&carpet);
    let profile: Vec<f64> = (0..=RINGS)
        .map(|k| spin::ring(&board_data, span, SIDE as f64 * k as f64 / RINGS as f64))
        .collect();
    assert!((profile[0] - 1.0).abs() < 1e-9);
    let lo = profile.iter().copied().fold(f64::MAX, f64::min);
    let hi = profile.iter().copied().fold(f64::MIN, f64::max);
    assert!(hi - lo > 0.5);

    let ramp = Ramp::new(vec![ink::GROUND, ink::BLUE, ink::GOLD]);
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let (cx, cy) = frame.center();
    let reach = frame.radius();
    let x0 = (cx - reach - 1.0).max(0.0) as usize;
    let y0 = (cy - reach - 1.0).max(0.0) as usize;
    let x1 = ((cx + reach + 1.0) as usize).min(board.width);
    let y1 = ((cy + reach + 1.0) as usize).min(board.height);
    for py in y0..y1 {
        for px in x0..x1 {
            let d = ((px as f64 + 0.5 - cx).powi(2) + (py as f64 + 0.5 - cy).powi(2)).sqrt();
            let cover = (reach + 0.5 - d).clamp(0.0, 1.0);
            if cover <= 0.0 {
                continue;
            }
            let t = (d / reach) * RINGS as f64;
            let i = (t.floor() as usize).min(RINGS - 1);
            let f = t - i as f64;
            let value = profile[i] * (1.0 - f) + profile[i + 1] * f;
            board.blend(px, py, ramp.at((value - lo) / (hi - lo)), cover);
        }
    }
    save("research-spin", &board)?;
    Ok(())
}
