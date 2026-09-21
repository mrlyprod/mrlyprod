use super::models::CellNd;
use crate::core::error::Result;
use crate::math::graph::extract;
use crate::math::graph::models::Network;

/// Extracts the network of filled sites joined to their axis neighbors.
pub fn core_graph<const N: usize>(cell: &CellNd<N>) -> Result<Network> {
    extract::core_graph(cell.types())
}

/// Extracts the network of corners and edges outlining every filled site.
pub fn edge_graph<const N: usize>(cell: &CellNd<N>) -> Result<Network> {
    extract::edge_graph(cell.types())
}

/// Extracts the network of empty sites joined to their axis neighbors.
pub fn tunnel_graph<const N: usize>(cell: &CellNd<N>) -> Result<Network> {
    extract::tunnel_graph(cell.types())
}
