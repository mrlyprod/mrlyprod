use super::models::CellNd;
use mrlycore::cell;
use mrlycore::errors::Result;
use mrlycore::tensor::Tensor;

/// Merges same-shaped cells into one block laid out by the per-axis repetition counts.
pub fn merge_reps<const N: usize>(cells: &[CellNd<N>], reps: &[usize]) -> Result<CellNd<N>> {
    let inner: Vec<cell::Cell> = cells.iter().map(|c| c.cell.clone()).collect();
    Ok(CellNd {
        cell: cell::merge(&inner, reps)?,
    })
}

/// Folds two or more cells into one by chained Kronecker combination.
pub fn magic<const N: usize>(cells: &[CellNd<N>]) -> Result<CellNd<N>> {
    let inner: Vec<cell::Cell> = cells.iter().map(|c| c.cell.clone()).collect();
    Ok(CellNd {
        cell: cell::magic(&inner)?,
    })
}

/// Builds a cell by placing at each mask site the cell its value indexes.
pub fn mosaic<const N: usize>(mask: &Tensor, cells: &[CellNd<N>]) -> Result<CellNd<N>> {
    let inner: Vec<cell::Cell> = cells.iter().map(|c| c.cell.clone()).collect();
    Ok(CellNd {
        cell: cell::mosaic(mask, &inner)?,
    })
}

/// Writes the value into the cell wherever the tiled mask is nonzero.
pub fn perforate<const N: usize>(mask: &Tensor, cell: &CellNd<N>, value: u8) -> Result<CellNd<N>> {
    cell.clone().perforate(mask, value)
}
