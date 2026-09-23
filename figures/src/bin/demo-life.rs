use figures::{ink, save, Board, Ramp};
use mrlyrs::core::error::Result;
use mrlyrs::core::tensor::Tensor;
use mrlyrs::life::{design_mask, next_grid, Boundary};
use mrlyrs::math::bang::Code;
use mrlyrs::math::two::Cell2d;

const NAME: &str = "demo-life";
const SIDE: usize = 96;
const STEPS: usize = 512;
const SEED: [(usize, usize); 5] = [(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)];

// LIFE

fn life() -> Result<(Vec<i32>, Vec<u8>)> {
    let mask = design_mask(2, Code::from(7u128), 3, 1)?;
    assert_eq!(mask.shape, vec![3, 3]);
    assert_eq!((0..mask.size()).filter(|&i| mask.at(i) == 1).count(), 8);

    let mut types = Tensor::new(vec![SIDE, SIDE]);
    let corner = SIDE / 2 - 1;
    for (x, y) in SEED {
        types.set(&[corner + y, corner + x], 1)?;
    }
    assert_eq!(types.bytes()?.iter().filter(|&&on| on == 1).count(), 5);

    let mut cell = Cell2d::new(types)?;
    let mut first = vec![-1i32; SIDE * SIDE];
    for step in 0..=STEPS {
        let live = cell.types();
        for (slot, mark) in first.iter_mut().enumerate() {
            if *mark < 0 && live.at(slot) != 0 {
                *mark = step as i32;
            }
        }
        if step < STEPS {
            cell = next_grid(&cell, &[3], &[2, 3], &mask, Boundary::Constant)?;
        }
    }
    let ever = first.iter().filter(|mark| **mark >= 0).count();
    let standing: Vec<u8> = (0..SIDE * SIDE)
        .map(|slot| u8::from(cell.types().at(slot) != 0))
        .collect();
    let alive = standing.iter().filter(|&&bit| bit != 0).count();
    assert!(alive > 0);
    assert!(ever > alive);
    Ok((first, standing))
}

// PRESS

fn main() -> Result<()> {
    let (first, live) = life()?;
    let mut board = Board::square();
    let area = board.frame(0.08);
    let scale = (area.w / SIDE as f64).floor().max(1.0);
    let block = SIDE as f64 * scale;
    let ox = ((board.width as f64 - block) / 2.0).round();
    let oy = ((board.height as f64 - block) / 2.0).round();
    let ramp = Ramp::tone(ink::green(), ink::line());
    for row in 0..SIDE {
        for col in 0..SIDE {
            let slot = row * SIDE + col;
            let x = ox + col as f64 * scale;
            let y = oy + row as f64 * scale;
            if first[slot] >= 0 {
                let t = first[slot] as f64 / STEPS as f64;
                board.rect(x, y, scale, scale, ramp.at(t));
            }
            if live[slot] != 0 {
                board.rect(x, y, scale, scale, ink::yellow());
            }
        }
    }
    save(NAME, &board)?;
    Ok(())
}
