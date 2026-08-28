use super::models::CellNd;
use mrlycore::errors::Result;
use mrlycore::tensor::Tensor;

/// Grows a seed pattern into a cell, deepened to its fractal past level one.
pub fn grow<const N: usize>(pattern: Tensor, level: usize) -> Result<CellNd<N>> {
    let mut cell = CellNd::<N>::new(pattern);
    if level > 1 {
        cell = cell.fractal(level)?;
    }
    Ok(cell)
}
