use super::graph::edge_graph;
use super::models::CellNd;
use crate::census::{count, exposed};
use mrlycore::errors::Result;

/// Counts the filled sites of the cell.
pub fn fills<const N: usize>(cell: &CellNd<N>) -> usize {
    count(cell.types(), 1)
}

/// Counts the empty sites of the cell.
pub fn voids<const N: usize>(cell: &CellNd<N>) -> usize {
    count(cell.types(), 0)
}

/// Counts the faces of filled sites open to emptiness or the border.
pub fn exposure<const N: usize>(cell: &CellNd<N>) -> u128 {
    exposed(cell.types())
}

/// Counts the distinct corners the filled sites touch, the edge graph's nodes.
pub fn vertices<const N: usize>(cell: &CellNd<N>) -> Result<usize> {
    Ok(edge_graph(cell)?.nodes.len())
}

/// Counts the distinct unit edges the filled sites carry, the edge graph's branches.
pub fn edges<const N: usize>(cell: &CellNd<N>) -> Result<usize> {
    Ok(edge_graph(cell)?.branches.len())
}
