use crate::core::error::{shape_error, Result};
use crate::core::tensor::Tensor;
use crate::math::two::Cell2d;

/// Crops a frame sequence to the centred square bounding every cell ever alive.
///
/// # Errors
///
/// Errs when the frames do not share one shape.
pub fn crop(grids: &[Cell2d]) -> Result<Vec<Cell2d>> {
    let Some(first) = grids.first() else {
        return Ok(Vec::new());
    };
    let shape = &first.types().shape;
    if grids.iter().any(|grid| &grid.types().shape != shape) {
        return shape_error("every frame must share one shape.");
    }
    let (h, w) = (shape[0], shape[1]);
    let mut any = vec![false; h * w];
    let mut found = false;
    for grid in grids {
        let types = grid.types();
        for (slot, i) in any.iter_mut().zip(0..types.size()) {
            if types.at(i) != 0 {
                *slot = true;
                found = true;
            }
        }
    }
    if !found {
        return Ok(grids.to_vec());
    }
    let (mut rmin, mut rmax, mut cmin, mut cmax) = (h, 0usize, w, 0usize);
    for r in 0..h {
        for c in 0..w {
            if any[r * w + c] {
                rmin = rmin.min(r);
                rmax = rmax.max(r);
                cmin = cmin.min(c);
                cmax = cmax.max(c);
            }
        }
    }
    let bh = rmax - rmin + 1;
    let bw = cmax - cmin + 1;
    if bh == h && bw == w {
        return Ok(grids.to_vec());
    }
    let side = bh.max(bw);
    let pad_top = (side - bh) / 2;
    let pad_left = (side - bw) / 2;
    grids
        .iter()
        .map(|grid| {
            let src = grid.types();
            let mut out = Tensor::new(vec![side, side]);
            for r in 0..bh {
                for c in 0..bw {
                    let v = src.at(src.index(&[rmin + r, cmin + c]));
                    if v != 0 {
                        out.put(out.index(&[pad_top + r, pad_left + c]), v);
                    }
                }
            }
            Cell2d::new(out)
        })
        .collect()
}

/// Tiles every frame n by n to reach at least min_canvas a side, unchanged when already there.
///
/// # Errors
///
/// Errs when a frame will not tile to the canvas.
pub fn tessellate(grids: &[Cell2d], min_canvas: usize) -> Result<Vec<Cell2d>> {
    let Some(first) = grids.first() else {
        return Ok(Vec::new());
    };
    let shape = &first.types().shape;
    let side = shape[0].max(shape[1]);
    if min_canvas == 0 || side == 0 || side >= min_canvas {
        return Ok(grids.to_vec());
    }
    let n = min_canvas.div_ceil(side);
    grids.iter().map(|grid| grid.clone().tile(n, n)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn crops_to_bounding_square() {
        let mut t = Tensor::new(vec![5, 5]);
        t.set(&[2, 3], 1).unwrap();
        let cropped = crop(&[Cell2d::new(t).unwrap()]).unwrap();
        assert_eq!(cropped[0].types().shape, vec![1, 1]);
        assert_eq!(cropped[0].types().get(&[0, 0]).unwrap(), 1);
    }
    #[test]
    fn crops_on_any_nonzero_type() {
        let mut t = Tensor::new(vec![5, 5]);
        t.set(&[2, 3], 2).unwrap();
        let cropped = crop(&[Cell2d::new(t).unwrap()]).unwrap();
        assert_eq!(cropped[0].types().shape, vec![1, 1]);
        assert_eq!(cropped[0].types().sum(), 2);
    }
    #[test]
    fn empty_is_returned_unchanged() {
        let t = Tensor::new(vec![4, 4]);
        let cropped = crop(&[Cell2d::new(t).unwrap()]).unwrap();
        assert_eq!(cropped[0].types().shape, vec![4, 4]);
    }
    #[test]
    fn full_grid_is_unchanged() {
        let mut t = Tensor::new(vec![3, 3]);
        t.set(&[0, 0], 1).unwrap();
        t.set(&[2, 2], 1).unwrap();
        let cropped = crop(&[Cell2d::new(t).unwrap()]).unwrap();
        assert_eq!(cropped[0].types().shape, vec![3, 3]);
    }
    #[test]
    fn tessellate_tiles_small_runs_up_to_the_canvas() {
        let mut t = Tensor::new(vec![5, 5]);
        t.set(&[1, 2], 1).unwrap();
        let grids = vec![Cell2d::new(t).unwrap()];
        let tiled = tessellate(&grids, 12).unwrap();
        assert_eq!(tiled[0].types().shape, vec![15, 15]);
        assert_eq!(tiled[0].types().get(&[1, 2]).unwrap(), 1);
        assert_eq!(tiled[0].types().get(&[6, 7]).unwrap(), 1);
        assert_eq!(tiled[0].types().sum(), 9);
    }
    #[test]
    fn tessellate_leaves_big_runs_alone() {
        let grids = vec![Cell2d::new(Tensor::new(vec![5, 5])).unwrap()];
        assert_eq!(tessellate(&grids, 5).unwrap()[0].types().shape, vec![5, 5]);
        assert_eq!(tessellate(&grids, 0).unwrap()[0].types().shape, vec![5, 5]);
        assert!(tessellate(&[], 12).unwrap().is_empty());
    }
}
