use figures::{ink, plot, save, Board};
use mrlyrs::core::error::Result;
use mrlyrs::num::series::{harmonic, EULER};

const TOP: usize = 30;
const SAMPLES: usize = 1600;

fn main() -> Result<()> {
    let mut board = Board::square();
    let frame = board.frame(0.08);
    let left = 1.0;
    let right = (TOP + 1) as f64;
    let peak = harmonic(TOP);
    let across = |x: f64| frame.x + frame.w * (x - left) / (right - left);
    let up = |v: f64| frame.y + frame.h * (1.0 - v / peak);

    let strip = ink::fade(ink::blue(), 0.28);
    let width = frame.w / SAMPLES as f64;
    for k in 0..SAMPLES {
        let x = left + (right - left) * (k as f64 + 0.5) / SAMPLES as f64;
        let crest = up(harmonic(x.floor() as usize));
        let foot = up(x.ln());
        board.rect(across(x) - width, crest, width * 2.0, foot - crest, strip);
    }

    let mut curve = Vec::with_capacity(SAMPLES + 1);
    for k in 0..=SAMPLES {
        let x = left + (right - left) * k as f64 / SAMPLES as f64;
        curve.push((across(x), up(x.ln())));
    }
    board.polyline(&curve, 3.4, ink::blue());

    let mut stair = Vec::with_capacity(2 * TOP);
    for n in 1..=TOP {
        let v = harmonic(n);
        stair.push((across(n as f64), up(v)));
        stair.push((across((n + 1) as f64), up(v)));
    }
    board.polyline(&stair, 3.4, ink::yellow());

    plot::axis(&mut board, frame, ink::line());
    assert_eq!(stair.len(), 2 * TOP);
    assert!((peak - (TOP as f64).ln() - EULER).abs() < 0.02);
    save("wiki-euler-mascheroni-constant", &board)?;
    Ok(())
}
