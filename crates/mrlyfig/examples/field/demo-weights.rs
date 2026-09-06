use mrlycore::errors::Result;
use mrlyfig::board::{Board, Frame};
use mrlyfig::ink::Ramp;
use mrlyfig::{field, ink, save};
use mrlymath::two;

const CODE: u128 = 495;
const SIDE: usize = 3;
const LEVEL: usize = 5;
const SPAN: usize = 243;
const MARGIN: f64 = 0.08;
const GAMMA: f64 = 0.12;
const SHARES: [f64; 8] = [5.0, 4.0, 3.0, 4.0, 2.0, 3.0, 2.0, 1.0];
const WHOLE: f64 = 24.0;

fn corners() -> Result<Vec<(usize, usize)>> {
    let tile = two::designs::create(CODE, SIDE, 1, 0, SIDE)?;
    let types = tile.types();
    Ok((0..SIDE)
        .flat_map(|a| (0..SIDE).map(move |b| (a, b)))
        .filter(|&(a, b)| types.get(&[a, b]) != 0)
        .collect())
}

fn mass(corners: &[(usize, usize)]) -> Vec<f64> {
    let mut wide = 1usize;
    let mut field = vec![1.0f64];
    for _ in 0..LEVEL {
        let next = wide * SIDE;
        let mut grown = vec![0.0f64; next * next];
        for row in 0..wide {
            for col in 0..wide {
                let held = field[row * wide + col];
                if held == 0.0 {
                    continue;
                }
                for (at, &(a, b)) in corners.iter().enumerate() {
                    grown[(row * SIDE + a) * next + col * SIDE + b] = held * SHARES[at] / WHOLE;
                }
            }
        }
        field = grown;
        wide = next;
    }
    field
}

fn main() -> Result<()> {
    let corners = corners()?;
    assert_eq!(
        corners,
        vec![
            (0, 0),
            (0, 1),
            (0, 2),
            (1, 0),
            (1, 2),
            (2, 0),
            (2, 1),
            (2, 2)
        ]
    );
    let field = mass(&corners);
    assert_eq!(field.len(), SPAN * SPAN);
    assert_eq!(field.iter().filter(|&&m| m > 0.0).count(), 32768);
    assert!((field.iter().sum::<f64>() - 1.0).abs() < 1e-12);
    let peak = field.iter().copied().fold(f64::MIN, f64::max);
    let least = field
        .iter()
        .copied()
        .filter(|&m| m > 0.0)
        .fold(f64::MAX, f64::min);
    assert!((peak - (5.0f64 / WHOLE).powi(LEVEL as i32)).abs() < 1e-15);
    assert!((least - (1.0f64 / WHOLE).powi(LEVEL as i32)).abs() < 1e-15);
    assert!((peak / least - 3125.0).abs() < 1e-9);
    let reads: Vec<f64> = field
        .iter()
        .map(|&m| if m > 0.0 { (m / peak).powf(GAMMA) } else { 0.0 })
        .collect();

    let ramp = Ramp::new(vec![ink::GROUND, ink::BLUE, ink::GOLD]);
    let mut board = Board::square();
    let edge = (board.width as f64 * MARGIN).round();
    let plate = board.width as f64 - 2.0 * edge;
    let frame = Frame::new(edge, edge, plate, plate);
    field::draw_range(&mut board, frame, SPAN, SPAN, &reads, (0.0, 1.0), &ramp);
    save("demo-weights", &board)?;
    Ok(())
}
