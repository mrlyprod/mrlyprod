use mrlycore::errors::Result;
use mrlyfig::board::{Board, Frame};
use mrlyfig::ink::Ramp;
use mrlyfig::{field, ink, save};
use mrlylab::moire::pairs::correlation;

const SCALES: usize = 31;
const CELL: f64 = 28.0;

fn main() -> Result<()> {
    let odds: Vec<usize> = (3..=63).step_by(2).collect();
    assert_eq!(odds.len(), SCALES);
    assert_eq!(correlation(3, 5), 0.0);
    assert!((correlation(9, 9) - 1.0).abs() < 1e-12);
    let mut values = Vec::with_capacity(SCALES * SCALES);
    for &m in &odds {
        for &n in &odds {
            let r = correlation(m, n);
            values.push(if r <= 0.0 { 0.0 } else { 0.5 + 0.5 * r.sqrt() });
        }
    }
    assert_eq!(values.iter().filter(|&&v| v == 0.0).count(), 762);

    let ramp = Ramp::new(vec![ink::GROUND, ink::BLUE, ink::GOLD]);
    let mut board = Board::square();
    let side = CELL * SCALES as f64;
    let edge = (board.width as f64 - side) / 2.0;
    let frame = Frame::new(edge, edge, side, side);
    field::draw_range(
        &mut board,
        frame,
        SCALES,
        SCALES,
        &values,
        (0.0, 1.0),
        &ramp,
    );
    save("paper-moire-correlation-laws", &board)?;
    Ok(())
}
