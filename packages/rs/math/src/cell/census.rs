use super::graph::edge_graph;
use super::models::CellNd;
use mrly_core::error::Result;

pub fn fills<const N: usize>(cell: &CellNd<N>) -> usize {
    cell.types().count(1)
}

pub fn voids<const N: usize>(cell: &CellNd<N>) -> usize {
    cell.types().count(0)
}

pub fn exposure<const N: usize>(cell: &CellNd<N>) -> u128 {
    cell.types().exposed()
}

pub fn vertices<const N: usize>(cell: &CellNd<N>) -> Result<usize> {
    Ok(edge_graph(cell)?.nodes.len())
}

pub fn edges<const N: usize>(cell: &CellNd<N>) -> Result<usize> {
    Ok(edge_graph(cell)?.branches.len())
}
