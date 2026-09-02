use mrlycore::errors::Result;
use mrlycore::tensor::Tensor;
use mrlycore::Rng;
use mrlyfig::{ink, save, Board};
use mrlymath::life::{design_mask, lattice_index, next_grid, Boundary};
use mrlymath::two::Cell2d;

const SIDE: usize = 192;
const GENERATIONS: usize = 64;
const SEED: u64 = 1729;
const DENSITY: f64 = 0.05;
const CELL: f64 = 4.0;
const STAMP: f64 = 8.0;
const MASK: usize = 9;
const SITES: usize = 64;

fn soup(seed: u64) -> Cell2d {
    let mut rng = Rng::new(seed);
    let mut types = Tensor::new(vec![SIDE, SIDE]);
    for slot in types.bytes_mut().iter_mut() {
        *slot = u8::from(rng.chance(DENSITY));
    }
    Cell2d::new(types)
}

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let mask = design_mask(2, 7, 3, 2)?;
    assert_eq!(mask.shape, vec![MASK, MASK]);
    assert_eq!((0..mask.size()).filter(|&i| mask.at(i) == 1).count(), SITES);
    assert_eq!(mask.get(&[MASK / 2, MASK / 2]), 0);
    assert_eq!(lattice_index(&mask), 1);

    let mut cell = soup(SEED);
    for _ in 0..GENERATIONS {
        cell = next_grid(&cell, &[3], &[2, 3], &mask, Boundary::Wrap)?;
    }
    assert_eq!(cell.width(), SIDE);
    assert_eq!(cell.height(), SIDE);
    let types = cell.types();
    let live = (0..SIDE * SIDE).filter(|&i| types.at(i) != 0).count();
    assert!(live > 0);

    let block = SIDE as f64 * CELL;
    let ox = (area.x + area.w - block).round();
    let oy = (area.y + area.h - block).round();
    for row in 0..SIDE {
        for col in 0..SIDE {
            if types.at(row * SIDE + col) != 0 {
                let x = ox + col as f64 * CELL;
                let y = oy + row as f64 * CELL;
                board.rect(x, y, CELL, CELL, ink::BLUE);
            }
        }
    }

    let sx = area.x.round();
    let sy = area.y.round();
    let mut stamped = 0usize;
    for row in 0..MASK {
        for col in 0..MASK {
            if mask.get(&[row, col]) == 1 {
                let x = sx + col as f64 * STAMP;
                let y = sy + row as f64 * STAMP;
                board.rect(x, y, STAMP, STAMP, ink::GOLD);
                stamped += 1;
            }
        }
    }
    assert_eq!(stamped, SITES);
    save("demo-mrlylife", &board)?;
    Ok(())
}
