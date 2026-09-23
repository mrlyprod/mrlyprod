use figures::{ink, save, Board};
use mrlyrs::core::error::Result;
use mrlyrs::core::tensor::Tensor;
use mrlyrs::core::Rng;
use mrlyrs::life::{design_mask, lattice_index, next_grid, Boundary};
use mrlyrs::math::bang::Code;
use mrlyrs::math::two::Cell2d;

const NAME: &str = "demo-mrlylife";
const SIDE: usize = 192;
const GENERATIONS: usize = 64;
const SEED: u64 = 1729;
const DENSITY: f64 = 0.05;
const CELL: f64 = 4.0;
const STAMP: f64 = 8.0;
const MASK: usize = 9;
const SITES: usize = 64;

// LIFE

fn soup(seed: u64) -> Result<Cell2d> {
    let mut rng = Rng::new(seed);
    let mut types = Tensor::new(vec![SIDE, SIDE]);
    for flat in 0..types.size() {
        types.put(flat, i64::from(rng.chance(DENSITY)));
    }
    Cell2d::new(types)
}

fn life() -> Result<(Vec<u8>, Vec<u8>)> {
    let mask = design_mask(2, Code::from(7u128), 3, 2)?;
    assert_eq!(mask.shape, vec![MASK, MASK]);
    assert_eq!((0..mask.size()).filter(|&i| mask.at(i) == 1).count(), SITES);
    assert_eq!(mask.get(&[MASK / 2, MASK / 2])?, 0);
    assert_eq!(lattice_index(&mask), 1);

    let mut cell = soup(SEED)?;
    for _ in 0..GENERATIONS {
        cell = next_grid(&cell, &[3], &[2, 3], &mask, Boundary::Wrap)?;
    }
    assert_eq!(cell.width(), SIDE);
    assert_eq!(cell.height(), SIDE);
    let types = cell.types();
    let stamp: Vec<u8> = (0..MASK * MASK).map(|i| mask.at(i) as u8).collect();
    let cells: Vec<u8> = (0..SIDE * SIDE)
        .map(|i| u8::from(types.at(i) != 0))
        .collect();
    assert!(cells.iter().any(|&bit| bit != 0));
    Ok((stamp, cells))
}

// PRESS

fn main() -> Result<()> {
    let (mask, cells) = life()?;
    let mut board = Board::square();
    let area = board.frame(0.08);
    let block = SIDE as f64 * CELL;
    let ox = (area.x + area.w - block).round();
    let oy = (area.y + area.h - block).round();
    for row in 0..SIDE {
        for col in 0..SIDE {
            if cells[row * SIDE + col] != 0 {
                let x = ox + col as f64 * CELL;
                let y = oy + row as f64 * CELL;
                board.rect(x, y, CELL, CELL, ink::blue());
            }
        }
    }

    let sx = area.x.round();
    let sy = area.y.round();
    let mut stamped = 0usize;
    for row in 0..MASK {
        for col in 0..MASK {
            if mask[row * MASK + col] == 1 {
                let x = sx + col as f64 * STAMP;
                let y = sy + row as f64 * STAMP;
                board.rect(x, y, STAMP, STAMP, ink::yellow());
                stamped += 1;
            }
        }
    }
    assert_eq!(stamped, SITES);
    save(NAME, &board)?;
    Ok(())
}
