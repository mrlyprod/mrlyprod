use super::Boundary;
use crate::core::error::Result;
use crate::core::tensor::Tensor;
use crate::math::cell::models::counting_dtype;
use crate::math::two::Cell2d;

/// Advances a grid one generation under birth and survive counts, a neighbor mask and a boundary.
///
/// ```
/// use mrlyrs::core::tensor::Tensor;
/// use mrlyrs::life::{moore, next_grid, Boundary};
/// use mrlyrs::math::two::Cell2d;
/// let bar = Cell2d::new(Tensor::of(vec![0, 0, 0, 1, 1, 1, 0, 0, 0], vec![3, 3])?)?;
/// let mask = moore()?.types().clone();
/// let next = next_grid(&bar, &[3], &[2, 3], &mask, Boundary::Constant)?;
/// assert_eq!(next.types().get(&[0, 1])?, 1);
/// # Ok::<(), mrlyrs::Error>(())
/// ```
///
/// # Errors
///
/// Errs when the mask will not count the grid's neighbors, an even side for one.
pub fn next_grid(
    cell: &Cell2d,
    birth: &[usize],
    survive: &[usize],
    mask: &Tensor,
    boundary: Boundary,
) -> Result<Cell2d> {
    let types = cell.types();
    let neighbors = types.neighbors(mask, 1, boundary.wrap(), counting_dtype(mask))?;
    let side = mask.size() + 1;
    let mut rule = vec![0u8; 2 * side];
    for (row, counts) in [birth, survive].into_iter().enumerate() {
        for &n in counts.iter().filter(|&&n| n < side) {
            rule[row * side + n] = 1;
        }
    }
    let next = (0..types.size())
        .map(|i| rule[usize::from(types.at(i) == 1) * side + neighbors.at(i) as usize])
        .collect();
    Cell2d::new(Tensor::of(next, types.shape.clone())?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::life::{blinker, moore};
    use crate::math::two::designs;
    #[test]
    fn conway_blinker_oscillates() {
        let cell = blinker();
        let mask = moore().unwrap().types().clone();
        let next = next_grid(&cell, &[3], &[2, 3], &mask, Boundary::Constant).unwrap();
        assert_eq!(next.types().get(&[2, 1]).unwrap(), 1);
        assert_eq!(next.types().get(&[2, 2]).unwrap(), 1);
        assert_eq!(next.types().get(&[2, 3]).unwrap(), 1);
        assert_eq!(next.types().get(&[1, 2]).unwrap(), 0);
        assert_eq!(next.types().get(&[3, 2]).unwrap(), 0);
        let back = next_grid(&next, &[3], &[2, 3], &mask, Boundary::Constant).unwrap();
        assert_eq!(back.types(), cell.types());
    }
    #[test]
    fn empty_stays_empty() {
        let cell = designs::zeros(3, 1).unwrap();
        let mask = moore().unwrap().types().clone();
        let next = next_grid(&cell, &[3], &[2, 3], &mask, Boundary::Constant).unwrap();
        assert_eq!(next.types().sum(), 0);
    }
    #[test]
    fn wide_masks_count_beyond_a_byte() {
        use crate::core::tensor::Tensor;
        let mut mask = Tensor::full(vec![17, 17], 1);
        mask.set(&[8, 8], 0).unwrap();
        let full = 17 * 17 - 1;
        let cell = Cell2d::new(Tensor::full(vec![21, 21], 1)).unwrap();
        let kept = next_grid(&cell, &[], &[full], &mask, Boundary::Wrap).unwrap();
        assert_eq!(kept.types().sum() as usize, 21 * 21);
        let gone = next_grid(&cell, &[], &[full - 1], &mask, Boundary::Wrap).unwrap();
        assert_eq!(gone.types().sum(), 0);
    }
}
