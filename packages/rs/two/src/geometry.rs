use super::Cell2d;
use mrly_error::{Result, value_error};
use mrly_math::cell::base::{Cell, remap};
use mrly_tensor::{Dtype, Tensor};
use mrly_three::Cell3d;

pub use mrly_math::cell::geometry::{magic, mosaic, perforate};

fn widest(cells: &[Cell2d]) -> Option<Dtype> {
    cells
        .iter()
        .filter_map(|c| c.cell.tags.as_ref())
        .map(|t| t.dtype())
        .max_by_key(|d| d.max())
}

fn stacked(cells: &[Cell2d]) -> Cell {
    let inner = cells[0].types().size();
    let shape = vec![cells.len() * inner];
    let mut types = Tensor::typed(shape.clone(), cells[0].types().dtype());
    let mut colors = vec![[0u8; 4]; cells.len() * inner];
    let mut tags = widest(cells).map(|dtype| Tensor::typed(shape, dtype));
    for (block, cell) in cells.iter().enumerate() {
        let base = block * inner;
        for local in 0..inner {
            types.put(base + local, cell.types().at(local));
            if let Some(source) = &cell.cell.colors {
                colors[base + local] = source[local];
            }
            if let (Some(layer), Some(source)) = (tags.as_mut(), &cell.cell.tags) {
                layer.put(base + local, source.at(local));
            }
        }
    }
    Cell {
        types,
        colors: cells
            .iter()
            .any(|c| c.cell.colors.is_some())
            .then_some(colors),
        tags,
    }
}

pub fn merge(cells: &[Cell2d], width: usize, height: usize) -> Result<Cell2d> {
    if cells.is_empty() {
        return value_error("Cannot merge an empty list of cells.");
    }
    let count = width * height;
    if cells.len() != count {
        return value_error(format!("Expected {count} cells, got {}", cells.len()));
    }
    let (inner_w, inner_h) = (cells[0].width(), cells[0].height());
    if cells
        .iter()
        .any(|c| c.types().shape != cells[0].types().shape)
    {
        return value_error("All cells in a merge operation must have the same dimensions.");
    }
    let shape = vec![inner_h * height, inner_w * width];
    let inner = inner_w * inner_h;
    let mut map = Vec::with_capacity(shape[0] * shape[1]);
    for y in 0..shape[0] {
        for x in 0..shape[1] {
            let block = (y / inner_h) * width + x / inner_w;
            map.push(block * inner + (y % inner_h) * inner_w + x % inner_w);
        }
    }
    Ok(Cell2d {
        cell: remap(&stacked(cells), &map, &shape)?,
    })
}

pub fn special(mask: &Tensor, cell: &Cell2d) -> Result<Cell2d> {
    if mask.shape.len() != 2 {
        return value_error("special mask must be 2d.");
    }
    if mask.bytes()?.iter().any(|&v| v > 3) {
        return value_error("Invalid rotation value. Must be 0, 1, 2, or 3.");
    }
    let rotated: Vec<Cell2d> = mask
        .bytes()?
        .iter()
        .map(|&k| cell.clone().rotate(k as usize))
        .collect::<Result<_>>()?;
    merge(&rotated, mask.shape[1], mask.shape[0])
}

pub fn to_3d(cell: &Cell2d) -> Cell3d {
    mrly_three::extrude(cell, 0, 1).expect("a one-deep lift on the leading axis always holds")
}

pub fn mask(mask: &Tensor, shape: &[usize]) -> Result<Tensor> {
    if mask.shape.len() != 2 || shape.len() != 2 {
        return value_error("a flat mask tiles a two-axis shape.");
    }
    let (mh, mw) = (mask.shape[0], mask.shape[1]);
    if mh == 0 || mw == 0 {
        return value_error("cannot tile an empty mask.");
    }
    let (h, w) = (shape[0], shape[1]);
    let mut out = Tensor::typed(vec![h, w], mask.dtype());
    for y in 0..h {
        for x in 0..w {
            out.put(y * w + x, mask.at((y % mh) * mw + x % mw));
        }
    }
    Ok(out)
}
