use mrlycore::errors::Result;
use mrlyfig::{ink, plot, save, Board};
use mrlymath::two::designs;
use mrlymath::two::graph::core_graph;
use mrlynum::spectrum::{laplacian_spectrum, multiplicity};

const LEVEL: usize = 6;

fn main() -> Result<()> {
    let cell = designs::create(7, 2, LEVEL, 0, 2)?;
    let network = core_graph(&cell)?;
    let mut values = laplacian_spectrum(&network, true)?;
    values.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let count = values.len();
    let ones = multiplicity(&values, 1.0, 1e-9);
    assert_eq!(count, 729);
    assert_eq!(ones, 243);

    let mut board = Board::square();
    let frame = board.frame(0.08);
    plot::axis(&mut board, frame, ink::LINE);
    let total = count as f64;
    let at = |lambda: f64, rank: f64| {
        (
            frame.x + frame.w * lambda / 2.0,
            frame.y + frame.h * (1.0 - rank / total),
        )
    };
    let mut steps = Vec::with_capacity(2 * count + 2);
    steps.push(at(0.0, 0.0));
    for (index, value) in values.iter().enumerate() {
        steps.push(at(*value, index as f64));
        steps.push(at(*value, index as f64 + 1.0));
    }
    steps.push(at(2.0, total));
    board.polyline(&steps, 3.0, ink::BLUE);

    let below = values.iter().filter(|v| **v < 1.0 - 1e-9).count() as f64;
    board.segment(at(1.0, below), at(1.0, below + ones as f64), 7.0, ink::GOLD);
    save("research-complexity", &board)?;
    Ok(())
}
