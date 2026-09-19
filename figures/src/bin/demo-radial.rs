use mrlycore::errors::Result;
use mrlyfig::board::{Board, Frame};
use mrlyfig::ink::Ramp;
use mrlyfig::{field, ink, save};
use mrlymath::two;
use mrlynum::spin;

const LEVEL: usize = 4;
const SIDE: usize = 81;
const COPIES: usize = 6;
const ORDER: usize = 4;
const OUT: usize = 860;
const RINGS: usize = 160;
const ORDERS: usize = 48;
const SAMPLES: usize = 3;

fn main() -> Result<()> {
    let carpet = two::designs::create(495, 3, LEVEL, 0, 3)?;
    assert_eq!(carpet.width(), SIDE);
    assert_eq!(carpet.types().sum(), 8u64.pow(LEVEL as u32));
    let source: Vec<f32> = carpet.types().bytes().iter().map(|&b| b as f32).collect();
    assert_eq!(source.len(), SIDE * SIDE);

    let power = spin::harmonics(&source, SIDE, RINGS, ORDERS);
    assert_eq!(spin::turns(&power), ORDER);
    assert_eq!(spin::petals(COPIES, ORDER), 12);

    let stack = spin::radial(
        &source,
        SIDE,
        OUT,
        COPIES,
        1.0 / COPIES as f64,
        spin::Blend::Mean,
        SAMPLES,
    );
    assert_eq!(stack.len(), OUT * OUT);
    let values: Vec<f64> = stack.iter().map(|&v| v as f64).collect();
    let high = values.iter().copied().fold(f64::MIN, f64::max);
    assert!(high > 0.9);

    let ramp = Ramp::new(vec![ink::ground(), ink::blue(), ink::yellow()]);
    let mut board = Board::square();
    let edge = (board.width as f64 - OUT as f64) / 2.0;
    let frame = Frame::new(edge, edge, OUT as f64, OUT as f64);
    field::draw_range(&mut board, frame, OUT, OUT, &values, (0.0, high), &ramp);
    save("demo-radial", &board)?;
    Ok(())
}
