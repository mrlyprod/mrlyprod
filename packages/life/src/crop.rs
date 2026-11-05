use mrly_core::error::{Result, shape_error};
use mrly_core::tensor::Tensor;
use mrly_math::two::Cell2d;

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
