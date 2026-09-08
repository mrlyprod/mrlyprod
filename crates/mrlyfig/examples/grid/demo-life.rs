use mrlycore::errors::Result;
use mrlycore::tensor::Tensor;
use mrlyfig::{ink, save, Board, Ramp};
use mrlymath::life::{design_mask, next_grid, Boundary};
use mrlymath::two::Cell2d;

const SIDE: usize = 96;
const STEPS: usize = 512;
const SEED: [(usize, usize); 5] = [(1, 0), (2, 0), (0, 1), (1, 1), (1, 2)];

fn main() -> Result<()> {
    let mut board = Board::square();
    let area = board.frame(0.08);
    let mask = design_mask(2, 7, 3, 1)?;
    assert_eq!(mask.shape, vec![3, 3]);
    assert_eq!((0..mask.size()).filter(|&i| mask.at(i) == 1).count(), 8);

    let mut types = Tensor::new(vec![SIDE, SIDE]);
    let corner = SIDE / 2 - 1;
    for (x, y) in SEED {
        types.set(&[corner + y, corner + x], 1);
    }
    assert_eq!(types.bytes().iter().filter(|&&on| on == 1).count(), 5);

    let mut cell = Cell2d::new(types);
    let mut first = vec![usize::MAX; SIDE * SIDE];
    for step in 0..=STEPS {
        let live = cell.types();
        for (slot, mark) in first.iter_mut().enumerate() {
            if *mark == usize::MAX && live.at(slot) != 0 {
                *mark = step;
            }
        }
        if step < STEPS {
            cell = next_grid(&cell, &[3], &[2, 3], &mask, Boundary::Constant)?;
        }
    }
    let ever = first.iter().filter(|mark| **mark != usize::MAX).count();
    let standing = (0..SIDE * SIDE)
        .filter(|&slot| cell.types().at(slot) != 0)
        .count();
    assert!(standing > 0);
    assert!(ever > standing);

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
            if first[slot] != usize::MAX {
                let t = first[slot] as f64 / STEPS as f64;
                board.rect(x, y, scale, scale, ramp.at(t));
            }
            if cell.types().at(slot) != 0 {
                board.rect(x, y, scale, scale, ink::yellow());
            }
        }
    }
    save("demo-life", &board)?;
    Ok(())
}
