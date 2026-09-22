use super::Boundary;
use crate::core::error::Result;
use crate::core::tensor::Tensor;
use crate::math::cell::models::counting_dtype;
use crate::math::two::Cell2d;

/// Advances a grid one generation under birth and survive counts, a neighbor mask and a boundary.
pub fn next_grid(
    cell: &Cell2d,
    birth: &[usize],
    survive: &[usize],
    mask: &Tensor,
    boundary: Boundary,
) -> Result<Cell2d> {
    let types = cell.types();
    let neighbors = types.neighbors(mask, 1, boundary.wrap(), counting_dtype(mask))?;
    let mut next = Tensor::new(types.shape.clone());
    for i in 0..types.size() {
        let n = neighbors.at(i) as usize;
        let lives = if types.at(i) == 1 {
            survive.contains(&n)
        } else {
            birth.contains(&n)
        };
        next.put(i, i64::from(lives));
    }
    Cell2d::new(next)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::rng::Rng;
    use crate::life::{blinker, design_mask, moore};
    use crate::math::bang::Code;
    use crate::math::two::designs;
    use crate::num::fft::{convolve_with, embed_kernel, transform};
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
    #[test]
    fn the_fft_step_is_the_crate_step_on_a_design_mask() {
        let size = 32;
        for (code, side, level, birth, survive) in [
            (7u128, 3usize, 1usize, (0.375, 0.375), (0.25, 0.375)),
            (7, 3, 2, (0.28, 0.38), (0.28, 0.48)),
        ] {
            let mask = design_mask(2, Code::from(code), side, level).unwrap();
            let span = mask.shape[0];
            let budget = mask.sum() as usize;
            let window = |lo: f64, hi: f64| -> Vec<usize> {
                (0..=budget)
                    .filter(|&k| {
                        let share = k as f64 / budget as f64;
                        lo <= share && share <= hi
                    })
                    .collect()
            };
            let born = window(birth.0, birth.1);
            let kept = window(survive.0, survive.1);
            let (kernel_re, kernel_im) = transform(
                &embed_kernel(mask.bytes().unwrap(), span, size).unwrap(),
                size,
            )
            .unwrap();
            let mut rng = Rng::new(11);
            let mut fast: Vec<u8> = (0..size * size)
                .map(|_| u8::from(rng.chance(0.5)))
                .collect();
            let mut slow =
                Cell2d::new(Tensor::of(fast.clone(), vec![size, size]).unwrap()).unwrap();
            for step in 0..8 {
                let field: Vec<f64> = fast
                    .iter()
                    .map(|&t| if t != 0 { 1.0 } else { 0.0 })
                    .collect();
                let counts = convolve_with(&field, &kernel_re, &kernel_im, size).unwrap();
                for (slot, &count) in fast.iter_mut().zip(&counts) {
                    let n = (count.round().max(0.0) as usize).min(budget);
                    let lives = if *slot != 0 {
                        kept.contains(&n)
                    } else {
                        born.contains(&n)
                    };
                    *slot = u8::from(lives);
                }
                slow = next_grid(&slow, &born, &kept, &mask, Boundary::Wrap).unwrap();
                assert_eq!(
                    fast,
                    slow.types().bytes().unwrap(),
                    "code {code} level {level} step {step}"
                );
            }
            assert!(fast.iter().any(|&t| t != 0));
        }
    }
}
