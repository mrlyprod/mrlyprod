use crate::tensor::Tensor;
use crate::Result;

/// The number of one-hot planes a grid embeds into.
pub const COLORS: usize = 10;

/// The largest side a grid may have.
pub const MAX_SIDE: usize = 30;

fn check(rows: &[Vec<u8>]) -> Result<(usize, usize)> {
    if rows.is_empty() || rows[0].is_empty() {
        return Err("grid is empty");
    }
    let width = rows[0].len();
    if rows.len() > MAX_SIDE || width > MAX_SIDE {
        return Err("grid exceeds the side limit");
    }
    for row in rows {
        if row.len() != width {
            return Err("grid rows have uneven lengths");
        }
        if row.iter().any(|&v| v as usize >= COLORS) {
            return Err("grid value past the colors");
        }
    }
    Ok((rows.len(), width))
}

/// Embeds a small u8 grid as one-hot color planes, or an error for a bad grid.
pub fn embed(rows: &[Vec<u8>]) -> Result<Tensor> {
    let (h, w) = check(rows)?;
    let mut planes = Tensor::zeros(&[COLORS, h, w]);
    for (y, row) in rows.iter().enumerate() {
        for (x, &v) in row.iter().enumerate() {
            planes.data[v as usize * h * w + y * w + x] = 1.0;
        }
    }
    Ok(planes)
}

/// Reads one-hot planes back into a u8 grid by plane argmax, or an error for a bad shape.
pub fn unembed(planes: &Tensor) -> Result<Vec<Vec<u8>>> {
    if planes.shape.len() != 3 || planes.shape[0] != COLORS {
        return Err("planes need shape colors by height by width");
    }
    let (h, w) = (planes.shape[1], planes.shape[2]);
    if h == 0 || w == 0 || h > MAX_SIDE || w > MAX_SIDE {
        return Err("grid exceeds the side limit");
    }
    let mut rows = vec![vec![0u8; w]; h];
    for (y, row) in rows.iter_mut().enumerate() {
        for (x, cell) in row.iter_mut().enumerate() {
            let mut best = 0;
            for plane in 1..COLORS {
                if planes.data[plane * h * w + y * w + x] > planes.data[best * h * w + y * w + x] {
                    best = plane;
                }
            }
            *cell = best as u8;
        }
    }
    Ok(rows)
}

/// Pads a grid into a larger frame with a fill color, or an error when the frame does not hold it.
pub fn pad(rows: &[Vec<u8>], height: usize, width: usize, fill: u8) -> Result<Vec<Vec<u8>>> {
    let (h, w) = check(rows)?;
    if fill as usize >= COLORS {
        return Err("grid value past the colors");
    }
    if height < h || width < w {
        return Err("frame smaller than the grid");
    }
    if height > MAX_SIDE || width > MAX_SIDE {
        return Err("grid exceeds the side limit");
    }
    let mut out = vec![vec![fill; width]; height];
    for (y, row) in rows.iter().enumerate() {
        out[y][..w].copy_from_slice(row);
    }
    Ok(out)
}

/// Crops a window out of a grid, or an error when the window leaves it.
pub fn crop(
    rows: &[Vec<u8>],
    top: usize,
    left: usize,
    height: usize,
    width: usize,
) -> Result<Vec<Vec<u8>>> {
    let (h, w) = check(rows)?;
    if height == 0 || width == 0 {
        return Err("window is empty");
    }
    if top + height > h || left + width > w {
        return Err("window leaves the grid");
    }
    let mut out = Vec::with_capacity(height);
    for row in rows.iter().skip(top).take(height) {
        out.push(row[left..left + width].to_vec());
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Tape;
    use crate::nn::{Act, Mlp};
    use crate::optim::Adam;
    use crate::rng::Rng;

    #[test]
    fn embed_unembed_roundtrips() {
        let grid = vec![vec![0, 1, 9], vec![3, 3, 0]];
        let planes = embed(&grid).unwrap();
        assert_eq!(planes.shape, vec![COLORS, 2, 3]);
        assert_eq!(planes.sum(), 6.0);
        assert_eq!(planes.at(&[1, 0, 1]), 1.0);
        assert_eq!(planes.at(&[9, 0, 2]), 1.0);
        assert_eq!(unembed(&planes).unwrap(), grid);
    }

    #[test]
    fn bad_grids_fail_to_embed() {
        assert_eq!(embed(&[]), Err("grid is empty"));
        assert_eq!(embed(&[vec![]]), Err("grid is empty"));
        assert_eq!(
            embed(&[vec![0], vec![0, 1]]),
            Err("grid rows have uneven lengths")
        );
        assert_eq!(embed(&[vec![10]]), Err("grid value past the colors"));
        assert_eq!(embed(&[vec![0; 31]]), Err("grid exceeds the side limit"));
        assert_eq!(
            unembed(&Tensor::zeros(&[2, 2, 2])),
            Err("planes need shape colors by height by width")
        );
    }

    #[test]
    fn pad_and_crop_frame_the_grid() {
        let grid = vec![vec![1, 2], vec![3, 4]];
        let framed = pad(&grid, 3, 4, 0).unwrap();
        assert_eq!(
            framed,
            vec![vec![1, 2, 0, 0], vec![3, 4, 0, 0], vec![0, 0, 0, 0]]
        );
        assert_eq!(crop(&framed, 0, 0, 2, 2).unwrap(), grid);
        assert_eq!(
            crop(&framed, 1, 1, 2, 2).unwrap(),
            vec![vec![4, 0], vec![0, 0]]
        );
        assert_eq!(pad(&grid, 1, 4, 0), Err("frame smaller than the grid"));
        assert_eq!(pad(&grid, 31, 4, 0), Err("grid exceeds the side limit"));
        assert_eq!(pad(&grid, 3, 4, 10), Err("grid value past the colors"));
        assert_eq!(crop(&grid, 1, 1, 2, 2), Err("window leaves the grid"));
        assert_eq!(crop(&grid, 0, 0, 0, 1), Err("window is empty"));
    }

    #[test]
    fn a_grid_task_trains_past_ninety_percent() {
        let mut rng = Rng::new(crate::seed(21, 0));
        let mut grids = Vec::new();
        let mut labels = Vec::new();
        for _ in 0..64 {
            let rows: Vec<Vec<u8>> = (0..4)
                .map(|_| (0..4).map(|_| rng.below(2) as u8).collect())
                .collect();
            let full_row = rows.iter().any(|r| r.iter().all(|&v| v == 1));
            labels.push(full_row as usize);
            grids.push(rows);
        }
        let features: Vec<Vec<f32>> = grids.iter().map(|g| embed(g).unwrap().data).collect();
        let inputs = Tensor::from_rows(&features).unwrap();
        let mut mlp = Mlp::new(&[COLORS * 16, 32, 2], Act::Relu, &mut rng);
        let mut adam = Adam::new(0.01);
        let mut first = 0.0f32;
        let mut last = 0.0f32;
        for epoch in 0..200 {
            let mut tape = Tape::new();
            let x = tape.leaf(inputs.clone());
            let (logits, held) = mlp.forward(&mut tape, x).unwrap();
            let loss = tape.softmax_xent(logits, &labels).unwrap();
            tape.backward(loss).unwrap();
            let grads: Vec<Tensor> = held.iter().map(|h| tape.grad(*h).clone()).collect();
            let mut params = mlp.params_mut();
            adam.step(&mut params, &grads).unwrap();
            last = tape.value(loss).data[0];
            if epoch == 0 {
                first = last;
            }
        }
        assert!(last < first, "loss never fell: {first} to {last}");
        let mut tape = Tape::new();
        let x = tape.leaf(inputs.clone());
        let (logits, _) = mlp.forward(&mut tape, x).unwrap();
        let out = tape.value(logits);
        let mut hits = 0;
        for (r, &label) in labels.iter().enumerate() {
            let row = &out.data[r * 2..(r + 1) * 2];
            let pick = if row[1] > row[0] { 1 } else { 0 };
            if pick == label {
                hits += 1;
            }
        }
        let accuracy = hits as f32 / labels.len() as f32;
        assert!(accuracy > 0.9, "accuracy {accuracy}");
    }
}
