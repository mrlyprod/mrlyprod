use super::models::Cell2d;
use crate::three::Cell3d;
use mrlycore::cell::{remap, Cell};
use mrlycore::errors::{value_error, Result};
use mrlycore::tensor::{Dtype, Tensor};

pub use crate::dim::geometry::{magic, mosaic, perforate};

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

/// Merges same-shaped cells into one block of the given width and height in cells, colors and tags kept.
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
        cell: remap(&stacked(cells), &map, &shape),
    })
}

/// Tiles quarter-turned copies of the cell as the 2d mask directs, or an error at a rotation past 3.
pub fn special(mask: &Tensor, cell: &Cell2d) -> Result<Cell2d> {
    if mask.shape.len() != 2 {
        return value_error("special mask must be 2d.");
    }
    if mask.bytes().iter().any(|&v| v > 3) {
        return value_error("Invalid rotation value. Must be 0, 1, 2, or 3.");
    }
    let rotated: Vec<Cell2d> = mask
        .bytes()
        .iter()
        .map(|&k| cell.clone().rotate(k as usize))
        .collect();
    merge(&rotated, mask.shape[1], mask.shape[0])
}

/// Lifts the flat cell into a cube one site deep, colors and tags with it.
///
/// ```
/// let flat = mrlymath::two::carpet(3, 2).unwrap();
/// let solid = mrlymath::two::to_3d(&flat);
/// assert_eq!(mrlymath::three::slice(&solid, 2, 0).unwrap(), flat);
/// ```
pub fn to_3d(cell: &Cell2d) -> Cell3d {
    crate::three::extrude(cell, 2, 1).expect("a one-deep lift on the last axis always holds")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::two::designs;
    use mrlycore::cell::mapping;
    use mrlycore::enums::Mode;
    #[test]
    fn special_rotations_preserve_sum() {
        let tree = designs::htree(3, 1).unwrap();
        let mask = Tensor::of(vec![0, 1, 3, 2], vec![2, 2]);
        let s = special(&mask, &tree).unwrap();
        assert_eq!(s.width(), 6);
        assert_eq!(s.types().sum(), 4 * tree.types().sum());
        assert!(special(&Tensor::of(vec![4], vec![1, 1]), &tree).is_err());
    }
    #[test]
    fn special_identity_mask_is_tile() {
        let c = designs::carpet(3, 1).unwrap();
        let mask = Tensor::new(vec![2, 3]);
        let s = special(&mask, &c).unwrap();
        assert_eq!(s, c.clone().tile(3, 2));
    }
    #[test]
    fn perforate_zero_mask_is_identity() {
        let c = designs::carpet(3, 1).unwrap();
        let mask = Tensor::new(c.types().shape.clone());
        let p = perforate(&mask, &c, 9).unwrap();
        assert_eq!(p, c);
    }
    #[test]
    fn merge_carries_colors_and_tags() {
        let painted = designs::carpet(3, 1)
            .unwrap()
            .layers()
            .paint(&mapping(), Mode::Type);
        let plain = designs::void(3, 1).unwrap();
        let block = merge(
            &[painted.clone(), plain, painted.clone(), painted.clone()],
            2,
            2,
        )
        .unwrap();
        assert_eq!(block.width(), 6);
        let colors = block.cell.colors.as_ref().unwrap();
        let source = painted.cell.colors.as_ref().unwrap();
        assert_eq!(colors.len(), 36);
        assert_eq!(colors[0], source[0]);
        assert_eq!(colors[3], [0, 0, 0, 0]);
        assert_eq!(colors[18], source[0]);
        assert_eq!(colors[21], source[0]);
        let tags = block.cell.tags.as_ref().unwrap();
        let seed = painted.cell.tags.as_ref().unwrap();
        assert_eq!(tags.shape, vec![6, 6]);
        assert_eq!(tags.at(7), seed.at(4));
        assert_eq!(tags.at(10), 0);
    }
    #[test]
    fn merge_without_paint_stays_bare() {
        let c = designs::carpet(3, 1).unwrap();
        let block = merge(&[c.clone(), c.clone(), c.clone(), c], 2, 2).unwrap();
        assert!(block.cell.colors.is_none());
        assert!(block.cell.tags.is_none());
        assert!(merge(&[], 1, 1).is_err());
    }
    #[test]
    fn to_3d_is_a_one_deep_lift() {
        let flat = designs::htree(5, 1)
            .unwrap()
            .layers()
            .paint(&mapping(), Mode::Index);
        let solid = to_3d(&flat);
        assert_eq!(solid.types().shape, vec![5, 5, 1]);
        assert_eq!(solid.types().sum(), flat.types().sum());
        assert_eq!(solid.cell.colors, flat.cell.colors);
        assert_eq!(crate::three::slice(&solid, 2, 0).unwrap(), flat);
    }
}
