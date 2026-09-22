use super::graph::edge_graph;
use super::models::CellNd;
use crate::core::error::Result;

/// Counts the filled sites of the cell.
pub fn fills<const N: usize>(cell: &CellNd<N>) -> usize {
    cell.types().count(1)
}

/// Counts the empty sites of the cell.
pub fn voids<const N: usize>(cell: &CellNd<N>) -> usize {
    cell.types().count(0)
}

/// Counts the faces of filled sites open to emptiness or the border.
pub fn exposure<const N: usize>(cell: &CellNd<N>) -> u128 {
    cell.types().exposed()
}

/// Counts the distinct corners the filled sites touch, the edge graph's nodes.
///
/// # Errors
///
/// Errors when the cell is neither two- nor three-dimensional.
pub fn vertices<const N: usize>(cell: &CellNd<N>) -> Result<usize> {
    Ok(edge_graph(cell)?.nodes.len())
}

/// Counts the distinct unit edges the filled sites carry, the edge graph's branches.
///
/// # Errors
///
/// Errors when the cell is neither two- nor three-dimensional.
pub fn edges<const N: usize>(cell: &CellNd<N>) -> Result<usize> {
    Ok(edge_graph(cell)?.branches.len())
}
