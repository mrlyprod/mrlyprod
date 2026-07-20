use super::models::CellNd;
use crate::core::errors::Result;
use crate::math::graph::extract;
use crate::math::graph::models::Network;

pub fn core_graph<const N: usize>(cell: &CellNd<N>) -> Result<Network> {
    extract::core_graph(cell.types())
}

pub fn edge_graph<const N: usize>(cell: &CellNd<N>) -> Result<Network> {
    extract::edge_graph(cell.types())
}

pub fn tunnel_graph<const N: usize>(cell: &CellNd<N>) -> Result<Network> {
    extract::tunnel_graph(cell.types())
}
