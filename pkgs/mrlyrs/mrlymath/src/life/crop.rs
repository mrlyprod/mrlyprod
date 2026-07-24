use crate::two::Cell2d;
use mrlycore::tensor::Tensor;

pub fn crop(grids: &[Cell2d]) -> Vec<Cell2d> {
    if grids.is_empty() {
        return Vec::new();
    }
    let shape = &grids[0].types().shape;
    let (h, w) = (shape[0], shape[1]);
    let mut any = vec![false; h * w];
    let mut found = false;
    for grid in grids {
        for (i, &v) in grid.types().bytes().iter().enumerate() {
            if v == 1 {
                any[i] = true;
                found = true;
            }
        }
    }
    if !found {
        return grids.to_vec();
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
        return grids.to_vec();
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
                    let v = src.get(&[rmin + r, cmin + c]);
                    if v != 0 {
                        out.set(&[pad_top + r, pad_left + c], v);
                    }
                }
            }
            Cell2d::new(out)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn crops_to_bounding_square() {
        let mut t = Tensor::new(vec![5, 5]);
        t.set(&[2, 3], 1);
        let cropped = crop(&[Cell2d::new(t)]);
        assert_eq!(cropped[0].types().shape, vec![1, 1]);
        assert_eq!(cropped[0].types().get(&[0, 0]), 1);
    }
    #[test]
    fn empty_is_returned_unchanged() {
        let t = Tensor::new(vec![4, 4]);
        let cropped = crop(&[Cell2d::new(t)]);
        assert_eq!(cropped[0].types().shape, vec![4, 4]);
    }
    #[test]
    fn full_grid_is_unchanged() {
        let mut t = Tensor::new(vec![3, 3]);
        t.set(&[0, 0], 1);
        t.set(&[2, 2], 1);
        let cropped = crop(&[Cell2d::new(t)]);
        assert_eq!(cropped[0].types().shape, vec![3, 3]);
    }
}
