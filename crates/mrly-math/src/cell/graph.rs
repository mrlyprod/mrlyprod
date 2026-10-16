use super::models::CellNd;
use crate::graph::extract;
use crate::graph::models::Network;
use mrly_core::error::Result;

/// Extracts the network of filled sites joined to their axis neighbors.
///
/// # Errors
///
/// Errors when the cell is neither two- nor three-dimensional.
pub fn core_graph<const N: usize>(cell: &CellNd<N>) -> Result<Network> {
    extract::core_graph(cell.types())
}

/// Extracts the network of corners and edges outlining every filled site.
///
/// # Errors
///
/// Errors when the cell is neither two- nor three-dimensional.
pub fn edge_graph<const N: usize>(cell: &CellNd<N>) -> Result<Network> {
    extract::edge_graph(cell.types())
}

/// Extracts the network of empty sites joined to their axis neighbors.
///
/// # Errors
///
/// Errors when the cell is neither two- nor three-dimensional.
pub fn tunnel_graph<const N: usize>(cell: &CellNd<N>) -> Result<Network> {
    extract::tunnel_graph(cell.types())
}
