use super::Cell2d;
use super::edge_graph;
use mrly_error::Result;
use mrly_math::cell::census;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Census {
    pub fills: usize,
    pub voids: usize,
    pub perimeter: u128,
    pub vertices: usize,
    pub edges: usize,
    pub euler: i64,
}

pub fn fills(cell: &Cell2d) -> usize {
    census::fills(cell)
}

pub fn voids(cell: &Cell2d) -> usize {
    census::voids(cell)
}

pub fn perimeter(cell: &Cell2d) -> u128 {
    census::exposure(cell)
}

pub fn euler(cell: &Cell2d) -> Result<i64> {
    let outline = edge_graph(cell)?;
    Ok(outline.nodes.len() as i64 - outline.branches.len() as i64 + fills(cell) as i64)
}

pub fn census(cell: &Cell2d) -> Result<Census> {
    let outline = edge_graph(cell)?;
    let (vertices, edges) = (outline.nodes.len(), outline.branches.len());
    let faces = fills(cell);
    Ok(Census {
        fills: faces,
        voids: voids(cell),
        perimeter: perimeter(cell),
        vertices,
        edges,
        euler: vertices as i64 - edges as i64 + faces as i64,
    })
}
