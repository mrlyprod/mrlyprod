use mrlycore::errors::Result;
use mrlyfig::{ink, plot, save, Board, Frame};
use mrlynum::design;

const BASE: u64 = 10;
const DIGITS: [u64; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
const DEPTH: usize = 5;
const SAMPLES: usize = 4096;
const FLOOR: usize = 101;
const BAND: (f64, f64) = (4.0, 60.0);
const THRESHOLD: f64 = 8.0;

fn column(board: &mut Board, frame: Frame, x: f64, thick: f64, color: mrlycore::Color, dash: f64) {
    if dash <= 0.0 {
        board.segment((x, frame.y), (x, frame.y + frame.h), thick, color);
        return;
    }
    let mut y = frame.y;
    while y < frame.y + frame.h {
        let end = (y + dash).min(frame.y + frame.h);
        board.segment((x, y), (x, end), thick, color);
        y = end + dash * 0.8;
    }
}

fn main() -> Result<()> {
    let values = design::elements(BASE, &DIGITS, DEPTH);
    let mu = design::mobius_of(&values);
    let running = design::meter(&mu);
    assert_eq!(values.len(), 59048);
    assert_eq!(running[values.len() - 1], 201);

    let alpha = (DIGITS.len() as f64).ln() / (BASE as f64).ln();
    let logx = design::log_grid(&values, SAMPLES);
    let series = design::resample(&values, &running, alpha / 2.0, &logx);
    let (gamma, power) = design::spectrum(&logx, &series);
    let score = design::score(&power, FLOOR);
    let found = design::peaks(&gamma, &score, BAND, THRESHOLD);
    let bin = gamma[1];
    let zeros: Vec<f64> = design::ZETA_ORDINATES
        .iter()
        .copied()
        .filter(|g| *g > BAND.0 && *g < BAND.1)
        .collect();
    let lattice: Vec<f64> = design::pole_lattice(BASE, BAND.1)
        .into_iter()
        .filter(|g| *g > BAND.0)
        .collect();
    assert_eq!(found.len(), 5);
    assert_eq!(zeros.len(), 13);
    assert_eq!(lattice.len(), 20);
    assert_eq!(
        found
            .iter()
            .filter(|&&at| design::nearest(gamma[at], &zeros) <= bin)
            .count(),
        5
    );

    let mut board = Board::square();
    let frame = board.frame(0.08);
    let peak = found
        .iter()
        .map(|&at| score[at])
        .fold(0.0f64, |top, value| top.max(value));
    let x = |g: f64| frame.x + frame.w * (g - BAND.0) / (BAND.1 - BAND.0);
    let y = |v: f64| frame.y + frame.h * (1.0 - (v / peak).clamp(0.0, 1.0).sqrt());

    for line in &lattice {
        column(
            &mut board,
            frame,
            x(*line),
            3.0,
            ink::fade(ink::pink(), 0.34),
            13.0,
        );
    }
    for zero in &zeros {
        column(
            &mut board,
            frame,
            x(*zero),
            7.0,
            ink::fade(ink::yellow(), 0.46),
            0.0,
        );
    }
    plot::baseline(&mut board, frame, ink::line());
    for at in 0..gamma.len() {
        if gamma[at] <= BAND.0 || gamma[at] >= BAND.1 {
            continue;
        }
        board.segment(
            (x(gamma[at]), frame.y + frame.h),
            (x(gamma[at]), y(score[at])),
            4.0,
            ink::blue(),
        );
    }
    let crowns: Vec<(f64, f64)> = found
        .iter()
        .map(|&at| (x(gamma[at]), y(score[at])))
        .collect();
    plot::dots(&mut board, &crowns, 11.0, ink::yellow());
    save("demo-echo", &board)?;
    Ok(())
}
