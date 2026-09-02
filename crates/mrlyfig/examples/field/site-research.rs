use mrlycore::errors::Result;
use mrlycore::Color;
use mrlyfig::board::{Board, Frame};
use mrlyfig::ink::Ramp;
use mrlyfig::{field, ink, save};
use mrlylab::moire::stack::stack;
use mrlylab::moire::{Combine, Field, Lattice, Spec};

const SCALES: usize = 20;
const SHEET: usize = 1075;
const START: usize = 215;
const SIZE: usize = 860;

fn window(sheet: &Field, start: usize, size: usize) -> Vec<f64> {
    let mut out = Vec::with_capacity(size * size);
    for row in 0..size {
        let base = (start + row) * sheet.size + start;
        for col in 0..size {
            out.push(sheet.data[base + col] as f64);
        }
    }
    out
}

fn main() -> Result<()> {
    let numbers: Vec<usize> = (1..=SCALES).collect();
    let sheet = stack(
        Spec::new(495, 3, 2),
        &numbers,
        Combine::Sum,
        1,
        Lattice::Square,
        SHEET,
        &[],
    )?;
    assert_eq!(sheet.max() as usize, SCALES);

    let values = window(&sheet, START, SIZE);
    assert_eq!(values.len(), SIZE * SIZE);
    let hi = SCALES as f64;
    let lo = values.iter().copied().fold(f64::MAX, f64::min);
    assert!(values.contains(&hi));
    let levels = (hi - lo) as usize + 1;

    let mut stops: Vec<Color> = (0..levels - 1)
        .map(|i| {
            let t = i as f64 / (levels - 2) as f64;
            ink::mix(ink::GROUND, ink::BLUE, 0.20 + 0.80 * t.powf(2.1))
        })
        .collect();
    stops.push(ink::GOLD);
    let ramp = Ramp::new(stops);

    let mut board = Board::square();
    let edge = (board.width as f64 - SIZE as f64) / 2.0;
    let frame = Frame::new(edge, edge, SIZE as f64, SIZE as f64);
    field::draw_range(&mut board, frame, SIZE, SIZE, &values, (lo, hi), &ramp);
    save("site-research", &board)?;
    Ok(())
}
