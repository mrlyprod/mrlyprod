use super::models::CellNd;
use mrly_core::error::Result;
use mrly_core::tensor::Tensor;

pub fn grow<const N: usize>(pattern: Tensor, level: usize) -> Result<CellNd<N>> {
    let mut cell = CellNd::<N>::new(pattern)?;
    if level > 1 {
        cell = cell.fractal(level)?;
    }
    Ok(cell)
}
