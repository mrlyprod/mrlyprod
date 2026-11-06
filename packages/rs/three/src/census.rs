use super::Cell3d;
use super::edge_graph;
use mrly_error::Result;
use mrly_math::cell::census;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Census {
    pub fills: usize,
    pub voids: usize,
    pub surface: u128,
    pub vertices: usize,
    pub edges: usize,
    pub faces: usize,
    pub euler: i64,
}

pub fn fills(cell: &Cell3d) -> usize {
    census::fills(cell)
}

pub fn voids(cell: &Cell3d) -> usize {
    census::voids(cell)
}

pub fn volume(cell: &Cell3d) -> usize {
    fills(cell)
}

pub fn surface(cell: &Cell3d) -> u128 {
    census::exposure(cell)
}

pub fn hidden(cell: &Cell3d) -> u128 {
    6 * fills(cell) as u128 - surface(cell)
}

pub fn faces(cell: &Cell3d) -> usize {
    let grid = cell.types();
    let (dx, dy, dz) = (grid.shape[0], grid.shape[1], grid.shape[2]);
    let mut planes = [
        vec![false; (dx + 1) * dy * dz],
        vec![false; dx * (dy + 1) * dz],
        vec![false; dx * dy * (dz + 1)],
    ];
    for i in 0..dx {
        for j in 0..dy {
            for k in 0..dz {
                if grid.at(grid.index(&[i, j, k])) == 0 {
                    continue;
                }
                for step in 0..2 {
                    planes[0][((i + step) * dy + j) * dz + k] = true;
                    planes[1][(i * (dy + 1) + j + step) * dz + k] = true;
                    planes[2][(i * dy + j) * (dz + 1) + k + step] = true;
                }
            }
        }
    }
    planes.iter().flatten().filter(|&&on| on).count()
}

pub fn euler(cell: &Cell3d) -> Result<i64> {
    let net = edge_graph(cell)?;
    let (v, e) = (net.nodes.len() as i64, net.branches.len() as i64);
    Ok(v - e + faces(cell) as i64 - fills(cell) as i64)
}

pub fn census(cell: &Cell3d) -> Result<Census> {
    let net = edge_graph(cell)?;
    let (vertices, edges) = (net.nodes.len(), net.branches.len());
    let (fills, faces) = (fills(cell), faces(cell));
    Ok(Census {
        fills,
        voids: voids(cell),
        surface: surface(cell),
        vertices,
        edges,
        faces,
        euler: vertices as i64 - edges as i64 + faces as i64 - fills as i64,
    })
}
