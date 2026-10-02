use figures::board::{Board, Frame};
use figures::{ink, plot, save, Color, Grid};
use mrlyrs::core::error::Result;
use mrlyrs::math::bang::{magic, MagicLayer};
use mrlyrs::math::name::Bang;
use mrlyrs::num::sieve;

const CODE: u128 = 7;
const SIDES: [usize; 3] = [3, 5, 7];
const SPAN: usize = 105;
const CELL: f64 = 5.0;
const REACH: f64 = 5.0;
const STOPS: usize = 400;
const TALL: f64 = 300.0;

fn mapped(frame: Frame, x: f64, y: f64, low: f64, high: f64) -> (f64, f64) {
    frame.at(x, 1.0 - (y - low) / (high - low))
}

fn rule(board: &mut Board, frame: Frame, y: f64, low: f64, high: f64, color: Color) {
    let left = mapped(frame, 0.0, y, low, high);
    let right = mapped(frame, 1.0, y, low, high);
    board.segment(left, right, 2.0, color);
}

fn main() -> Result<()> {
    let layers: Vec<MagicLayer> = SIDES
        .iter()
        .map(|&side| MagicLayer::new(Bang::new(CODE, 2, 2), side))
        .collect();
    let word = magic(&layers)?;
    assert_eq!(word.shape, vec![SPAN, SPAN]);
    let types = word.bytes()?.to_vec();
    assert_eq!(types.iter().filter(|&&b| b != 0).count(), 6720);

    let profile = sieve::row_profile(CODE, 2)?;
    let row = sieve::row_law(&profile)?;
    assert_eq!(row.closed, "3 pi/(4 Gamma(1/3))");
    assert!((row.constant - 0.879_525_401_447_625_3).abs() < 1e-14);
    let mut stops: Vec<usize> = (0..STOPS)
        .map(|i| 10f64.powf(REACH * i as f64 / (STOPS - 1) as f64).round() as usize)
        .collect();
    stops.dedup();
    let walk = sieve::row_settle(&profile, &stops, false)?;
    let last = walk[walk.len() - 1];
    assert!((last - row.constant).abs() < 1e-5);
    let high = walk.iter().copied().fold(f64::MIN, f64::max);
    let low = row.constant - 0.06 * (high - row.constant);

    let mut board = Board::square();
    let margin = (board.width as f64 * 0.08).round();
    let plate = CELL * SPAN as f64;
    let sheet = Frame::new(margin, margin, plate, plate);
    let lattice = Grid::new(sheet, SPAN, SPAN, 0.0);
    for row in 0..SPAN {
        for col in 0..SPAN {
            let color = if types[row * SPAN + col] != 0 {
                ink::blue()
            } else {
                ink::panel()
            };
            lattice.fill(&mut board, col, row, color);
        }
    }

    let panel = Frame::new(
        board.width as f64 - margin - plate,
        board.height as f64 - margin - TALL,
        plate,
        TALL,
    );
    plot::axis(&mut board, panel, ink::line());
    let inner = panel.inset(24.0);
    rule(&mut board, inner, row.constant, low, high, ink::dim());
    let path: Vec<(f64, f64)> = stops
        .iter()
        .zip(&walk)
        .map(|(&level, &value)| mapped(inner, (level as f64).log10() / REACH, value, low, high))
        .collect();
    board.polyline(&path, 4.0, ink::yellow());
    save("demo-staircase", &board)?;
    Ok(())
}
