use super::models::CellNd;
use crate::core::cell;
use crate::core::errors::Result;
use crate::core::tensor::Tensor;

pub fn merge_reps<const N: usize>(cells: &[CellNd<N>], reps: &[usize]) -> Result<CellNd<N>> {
    let inner: Vec<cell::Cell> = cells.iter().map(|c| c.cell.clone()).collect();
    Ok(CellNd {
        cell: cell::merge(&inner, reps)?,
    })
}

pub fn magic<const N: usize>(cells: &[CellNd<N>]) -> Result<CellNd<N>> {
    let inner: Vec<cell::Cell> = cells.iter().map(|c| c.cell.clone()).collect();
    Ok(CellNd {
        cell: cell::magic(&inner)?,
    })
}

pub fn mosaic<const N: usize>(mask: &Tensor, cells: &[CellNd<N>]) -> Result<CellNd<N>> {
    let inner: Vec<cell::Cell> = cells.iter().map(|c| c.cell.clone()).collect();
    Ok(CellNd {
        cell: cell::mosaic(mask, &inner)?,
    })
}

pub fn perforate<const N: usize>(mask: &Tensor, cell: &CellNd<N>, value: u8) -> Result<CellNd<N>> {
    cell.clone().perforate(mask, value)
}
