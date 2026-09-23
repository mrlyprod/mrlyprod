use figures::{ink, save, Board, Color, Frame};
use mrlyrs::core::error::Result;
use mrlyrs::math::three::sponge::{profile, reading, COVER, EDGE};

const PERIODS: usize = 8;
const STEPS: usize = 144;
const OPEN: usize = 45;
const TWELFTH: (f64, f64) = (2.122718, 2.122723);
const SIXTH: (f64, f64) = (2.135019, 2.136794);

fn runs(
    values: &[Option<f64>],
    place: impl Fn(usize, f64) -> Option<(f64, f64)>,
) -> Vec<Vec<(f64, f64)>> {
    let mut out = Vec::new();
    let mut run = Vec::new();
    for (i, value) in values.iter().enumerate() {
        match value.and_then(|v| place(i, v)) {
            Some(point) => run.push(point),
            None => {
                if run.len() > 1 {
                    out.push(std::mem::take(&mut run));
                }
                run.clear();
            }
        }
    }
    if run.len() > 1 {
        out.push(run);
    }
    out
}

fn stroke(board: &mut Board, lines: &[Vec<(f64, f64)>], thick: f64, color: Color) {
    for line in lines {
        board.polyline(line, thick, color);
    }
}

fn main() -> Result<()> {
    let start = (1.0 / COVER).ln();
    let period = 3f64.ln();
    let count = PERIODS * STEPS;
    let us: Vec<f64> = (0..count)
        .map(|i| start + period * (i as f64 + 0.5) / STEPS as f64)
        .collect();
    let once: Vec<Option<f64>> = us[..STEPS].iter().map(|u| profile((-u).exp())).collect();
    let waves: Vec<Option<f64>> = (0..count).map(|i| once[i % STEPS]).collect();
    let reads: Vec<Option<f64>> = us.iter().map(|u| reading((-u).exp())).collect();

    assert_eq!(waves.iter().filter(|v| v.is_none()).count(), PERIODS * OPEN);
    assert!(waves
        .iter()
        .zip(&reads)
        .all(|(p, r)| p.is_none() == r.is_none()));
    assert!(waves.iter().zip(&reads).all(|(p, r)| p.is_none() || r < p));
    let twelfth = profile(1.0 / 12.0).unwrap_or(0.0);
    let sixth = profile(EDGE).unwrap_or(0.0);
    assert!(TWELFTH.0 <= twelfth && twelfth <= TWELFTH.1);
    assert!(SIXTH.0 <= sixth && sixth <= SIXTH.1);

    let seen: Vec<f64> = once.iter().flatten().copied().collect();
    let low = seen.iter().copied().fold(f64::INFINITY, f64::min);
    let high = seen.iter().copied().fold(f64::NEG_INFINITY, f64::max);
    let spread = high - low;
    let (floor, roof) = (low - 1.6 * spread, high + 0.35 * spread);

    let mut board = Board::square();
    let plot: Frame = board.frame(0.08);
    let (u0, u1) = (start, start + PERIODS as f64 * period);
    let x_of = |u: f64| plot.x + plot.w * (u - u0) / (u1 - u0);
    let y_of = |v: f64| plot.y + plot.h * (roof - v) / (roof - floor);
    let gap = (COVER / EDGE).ln();
    for k in 0..PERIODS {
        let left = x_of(u0 + k as f64 * period);
        let right = x_of(u0 + k as f64 * period + gap);
        board.rect(left, plot.y, right - left, plot.h, ink::panel());
    }
    let hair = (plot.w / 512.0).max(1.0);
    for k in 1..PERIODS {
        let x = x_of(u0 + k as f64 * period);
        board.segment((x, plot.y), (x, plot.y + plot.h), hair, ink::line());
    }
    let place = |i: usize, v: f64| (v >= floor).then(|| (x_of(us[i]), y_of(v.min(roof))));
    stroke(&mut board, &runs(&waves, place), 7.0, ink::yellow());
    stroke(&mut board, &runs(&reads, place), 5.0, ink::blue());
    save("demo-minkowski", &board)?;
    Ok(())
}
