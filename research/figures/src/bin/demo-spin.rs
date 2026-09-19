use mrlycore::errors::Result;
use mrlyfig::board::{Board, Frame};
use mrlyfig::ink::Ramp;
use mrlyfig::{field, ink, save};
use mrlymath::six;
use mrlynum::spin;

const PANEL: usize = 430;
const TRIANGLES: usize = 486;
const STEPS: usize = 2048;

fn main() -> Result<()> {
    let cut = six::cut_design(23, 3, 2, 2)?;
    let tally = six::census(&cut, false);
    assert_eq!(tally.triangles, TRIANGLES);
    assert!(tally.fills > 0 && tally.fills < TRIANGLES);

    let source = six::raster(&cut, PANEL)?;
    assert_eq!(source.len(), PANEL * PANEL);
    let lit: f64 = source.iter().map(|&v| v as f64).sum();
    assert!(lit > 0.0);

    let profile = spin::profile(&source, PANEL, STEPS);
    assert_eq!(profile.len(), STEPS);
    let mass = spin::mass(&profile, PANEL);
    assert!((mass - lit).abs() < 0.03 * lit);

    let reach = profile.iter().rposition(|&v| v > 1e-6).unwrap_or(STEPS - 1);
    assert!(reach > STEPS / 2);
    let rings = spin::wheel(&profile[..=reach], PANEL);
    assert_eq!(rings.len(), PANEL * PANEL);
    let wheel: Vec<f64> = rings.iter().map(|&v| v as f64).collect();
    let peak = wheel.iter().copied().fold(f64::MIN, f64::max);
    assert!(peak > 0.0);

    let slice: Vec<f64> = source.iter().map(|&v| v as f64).collect();
    let flat = Ramp::tone(ink::ground(), ink::blue());
    let heat = Ramp::new(vec![ink::ground(), ink::blue(), ink::yellow()]);

    let mut board = Board::square();
    let edge = (board.width as f64 - 2.0 * PANEL as f64) / 2.0;
    let step = PANEL as f64;
    let first = Frame::new(edge, edge, step, step);
    let second = Frame::new(edge + step, edge + step, step, step);
    field::draw_range(&mut board, first, PANEL, PANEL, &slice, (0.0, 1.0), &flat);
    field::draw_range(&mut board, second, PANEL, PANEL, &wheel, (0.0, peak), &heat);
    save("demo-spin", &board)?;
    Ok(())
}
