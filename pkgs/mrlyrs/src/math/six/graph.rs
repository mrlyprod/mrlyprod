use super::census::{corners, edges_of};
use super::models::Cell6d;
use super::{FILL, VOID};
use crate::core::errors::Result;
use crate::math::graph::models::Network;
use std::collections::BTreeMap;

type Point = (i64, i64);
type Edge = (Point, Point);

fn centroid(c: &[Point; 3]) -> Vec<f64> {
    let xs: f64 = c.iter().map(|p| p.0 as f64).sum::<f64>() / 3.0;
    let ys: f64 = c.iter().map(|p| p.1 as f64).sum::<f64>() / 3.0;
    vec![xs, ys]
}

fn adjacency_graph(cell: &Cell6d, keep: impl Fn(u8) -> bool) -> Result<Network> {
    let inner = &cell.cell;
    let start = cell.start as i64;
    let (height, width) = (inner.height(), inner.width());
    let mut cells = Vec::new();
    for y in 0..height {
        for x in 0..width {
            if keep(inner.types().get(&[y, x])) {
                cells.push((x as i64, y as i64));
            }
        }
    }
    let mut network = Network::new(2);
    for &(x, y) in &cells {
        network.add_node(centroid(&corners(x, y, start)))?;
    }
    let mut edge_to_cells: BTreeMap<Edge, Vec<usize>> = BTreeMap::new();
    for (i, &(x, y)) in cells.iter().enumerate() {
        for edge in edges_of(&corners(x, y, start)) {
            edge_to_cells.entry(edge).or_default().push(i);
        }
    }
    for shared in edge_to_cells.values() {
        if shared.len() == 2 {
            network.add_branch(shared[0], shared[1], 1.0)?;
        }
    }
    Ok(network)
}

pub fn slice_core_graph(cell: &Cell6d) -> Result<Network> {
    adjacency_graph(cell, |v| v == FILL)
}

pub fn slice_dual_graph(cell: &Cell6d) -> Result<Network> {
    adjacency_graph(cell, |v| v == FILL || v == VOID)
}

pub fn slice_edge_graph(cell: &Cell6d, value: Option<u8>) -> Result<Network> {
    let inner = &cell.cell;
    let start = cell.start as i64;
    let (height, width) = (inner.height(), inner.width());
    let keep = |v: u8| match value {
        Some(target) => v == target,
        None => v == FILL || v == VOID,
    };
    let mut corner_index: BTreeMap<Point, usize> = BTreeMap::new();
    let mut network = Network::new(2);
    let mut seen: BTreeMap<Edge, bool> = BTreeMap::new();
    for y in 0..height {
        for x in 0..width {
            if !keep(inner.types().get(&[y, x])) {
                continue;
            }
            let c = corners(x as i64, y as i64, start);
            for edge in edges_of(&c) {
                if seen.contains_key(&edge) {
                    continue;
                }
                seen.insert(edge, true);
                let mut node = |p: Point, network: &mut Network| -> Result<usize> {
                    if let Some(&i) = corner_index.get(&p) {
                        return Ok(i);
                    }
                    let i = network.add_node(vec![p.0 as f64, p.1 as f64])?;
                    corner_index.insert(p, i);
                    Ok(i)
                };
                let a = node(edge.0, &mut network)?;
                let b = node(edge.1, &mut network)?;
                network.add_branch(a, b, 1.0)?;
            }
        }
    }
    Ok(network)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::formulas::six as formulas;
    use crate::math::six::geometry::blank;
    use crate::math::six::{Orientation, Projection};
    fn solid(radius: usize) -> Cell6d {
        Cell6d::new(
            blank(
                radius,
                Orientation::Horizontal,
                FILL,
                crate::math::six::GRID,
            ),
            Projection::Cut,
            Orientation::Horizontal,
            0,
        )
    }
    #[test]
    fn solid_slice_matches_python() {
        let expected = [(1, 6, 6), (2, 24, 27), (3, 54, 72)];
        for (radius, nodes, branches) in expected {
            let core = slice_core_graph(&solid(radius)).unwrap();
            assert_eq!(core.nodes.len(), nodes, "r={radius}");
            assert_eq!(core.branches.len(), branches, "r={radius}");
        }
        assert_eq!(
            slice_core_graph(&solid(3)).unwrap().branches.len() as u128,
            formulas::solid_slice_core_edges(3).unwrap()
        );
    }
    #[test]
    fn dual_contains_core() {
        let s = solid(2);
        let dual = slice_dual_graph(&s).unwrap();
        let core = slice_core_graph(&s).unwrap();
        assert!(dual.nodes.len() >= core.nodes.len());
        let edge = slice_edge_graph(&s, Some(FILL)).unwrap();
        assert!(!edge.nodes.is_empty());
    }
}
