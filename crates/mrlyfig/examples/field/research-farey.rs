use mrlycore::errors::Result;
use mrlyfig::board::Board;
use mrlyfig::ink::Ramp;
use mrlyfig::{ink, save};
use mrlynum::factor::lcm;
use mrlynum::lattice;

const ORDER: usize = 60;

fn main() -> Result<()> {
    let nodes = lattice::farey(ORDER);
    assert_eq!(nodes.len(), 1103);
    let mut stack: Vec<(f64, f64, usize)> = Vec::new();
    for across in &nodes {
        for down in &nodes {
            let grid = lcm(across.den as usize, down.den as usize);
            if grid > ORDER {
                continue;
            }
            stack.push((
                across.num as f64 / across.den as f64,
                down.num as f64 / down.den as f64,
                ORDER / grid,
            ));
        }
    }
    assert_eq!(stack.len(), 63261);
    stack.sort_by_key(|node| node.2);

    let ramp = Ramp::new(vec![ink::GROUND, ink::BLUE, ink::GOLD]);
    let mut board = Board::square();
    let frame = board.frame(0.08);
    for (x, y, weight) in stack {
        let share = weight as f64 / ORDER as f64;
        let (px, py) = frame.at(x, 1.0 - y);
        board.disc(
            px,
            py,
            1.0 + 3.0 * share.powf(0.7),
            ramp.at(share.powf(0.30)),
        );
    }
    save("research-farey", &board)?;
    Ok(())
}
