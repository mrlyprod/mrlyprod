use super::Boundary;
use crate::two::Cell2d;
use mrlycore::cell::Cell;
use mrlycore::errors::Result;
use mrlycore::tensor::{Dtype, Tensor};

pub fn next_grid(
    cell: &Cell2d,
    birth: &[usize],
    survive: &[usize],
    mask: &Tensor,
    boundary: Boundary,
) -> Result<Cell2d> {
    let counted = Cell::new(cell.types().clone()).neighbors(mask, 1, boundary.wrap(), Dtype::U8)?;
    let neighbors = counted.tags.expect("neighbors sets tags");
    let types = &counted.types;
    let mut next = Tensor::new(types.shape.clone());
    for (i, slot) in next.bytes_mut().iter_mut().enumerate() {
        let n = neighbors.bytes()[i] as usize;
        let alive = types.bytes()[i] == 1;
        let lives = if alive {
            survive.contains(&n)
        } else {
            birth.contains(&n)
        };
        *slot = if lives { 1 } else { 0 };
    }
    Ok(Cell2d::new(next))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::two::designs;
    use mrlycore::atoms;
    fn moore() -> Tensor {
        let mut m = atoms::carpet_2d(3);
        m.set(&[1, 1], 0);
        m
    }
    #[test]
    fn conway_blinker_oscillates() {
        let mut t = Tensor::new(vec![5, 5]);
        t.set(&[1, 2], 1);
        t.set(&[2, 2], 1);
        t.set(&[3, 2], 1);
        let cell = Cell2d::new(t);
        let mask = moore();
        let next = next_grid(&cell, &[3], &[2, 3], &mask, Boundary::Constant).unwrap();
        assert_eq!(next.types().get(&[2, 1]), 1);
        assert_eq!(next.types().get(&[2, 2]), 1);
        assert_eq!(next.types().get(&[2, 3]), 1);
        assert_eq!(next.types().get(&[1, 2]), 0);
        assert_eq!(next.types().get(&[3, 2]), 0);
        let back = next_grid(&next, &[3], &[2, 3], &mask, Boundary::Constant).unwrap();
        assert_eq!(back.types(), cell.types());
    }
    #[test]
    fn empty_stays_empty() {
        let cell = designs::zeros(3, 1).unwrap();
        let mask = moore();
        let next = next_grid(&cell, &[3], &[2, 3], &mask, Boundary::Constant).unwrap();
        assert_eq!(next.types().sum(), 0);
    }
}
