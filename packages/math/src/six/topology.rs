use super::census::{corners, edges_of, fills_only};
use super::graph::slice_core_graph;
use super::models::Cell6d;
use super::{FILL, VOID};
use crate::graph::largest_component;
use crate::graph::models::Network;
use crate::spectrum::{laplacian_spectrum, spectral_exponent as slope};
use mrly_core::error::{Result, value_error};
use std::collections::BTreeMap;

type Point = (i64, i64);
type Edge = (Point, Point);

fn pieces(network: &Network) -> Result<Vec<usize>> {
    let n = network.nodes.len();
    let adjacency = network.adjacency()?;
    let mut seen = vec![false; n];
    let mut sizes = Vec::new();
    for start in 0..n {
        if seen[start] {
            continue;
        }
        let mut size = 0;
        let mut stack = vec![start];
        seen[start] = true;
        while let Some(current) = stack.pop() {
            size += 1;
            for &neighbor in &adjacency[&current] {
                if !seen[neighbor] {
                    seen[neighbor] = true;
                    stack.push(neighbor);
                }
            }
        }
        sizes.push(size);
    }
    Ok(sizes)
}

pub fn components(cell: &Cell6d) -> Result<usize> {
    Ok(pieces(&slice_core_graph(cell)?)?.len())
}

pub fn giant(cell: &Cell6d) -> Result<usize> {
    Ok(pieces(&slice_core_graph(cell)?)?
        .into_iter()
        .max()
        .unwrap_or(0))
}

pub fn giant_network(cell: &Cell6d) -> Result<Network> {
    largest_component(&slice_core_graph(cell)?)
}

pub fn spectral_exponent(cell: &Cell6d, window: f64) -> Result<f64> {
    let spectrum = laplacian_spectrum(&giant_network(cell)?, true)?;
    match slope(&spectrum, window) {
        Some(value) => Ok(value),
        None => value_error("The giant piece is too small to fit an exponent."),
    }
}

pub fn holes(cell: &Cell6d) -> Result<usize> {
    let count = components(cell)? as i64;
    Ok((count - fills_only(cell).euler).max(0) as usize)
}

pub fn rim_holes(cell: &Cell6d) -> Result<usize> {
    let inner = &cell.cell;
    let start = cell.start as i64;
    let (height, width) = (inner.height(), inner.width());
    let mut sites = Vec::new();
    let mut mesh: BTreeMap<Edge, usize> = BTreeMap::new();
    for y in 0..height {
        for x in 0..width {
            let v = inner.types().get(&[y, x])?;
            if v != FILL && v != VOID {
                continue;
            }
            for edge in edges_of(&corners(x as i64, y as i64, start)) {
                *mesh.entry(edge).or_insert(0) += 1;
            }
            if v == VOID {
                sites.push((x as i64, y as i64));
            }
        }
    }
    let mut owners: BTreeMap<Edge, Vec<usize>> = BTreeMap::new();
    for (index, &(x, y)) in sites.iter().enumerate() {
        for edge in edges_of(&corners(x, y, start)) {
            owners.entry(edge).or_default().push(index);
        }
    }
    let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); sites.len()];
    let mut on_rim = vec![false; sites.len()];
    for (edge, shared) in &owners {
        if mesh[edge] == 1 {
            for &index in shared {
                on_rim[index] = true;
            }
        }
        if shared.len() == 2 {
            adjacency[shared[0]].push(shared[1]);
            adjacency[shared[1]].push(shared[0]);
        }
    }
    let mut seen = vec![false; sites.len()];
    let mut enclosed = 0;
    for start in 0..sites.len() {
        if seen[start] {
            continue;
        }
        let mut open = false;
        let mut stack = vec![start];
        seen[start] = true;
        while let Some(current) = stack.pop() {
            open |= on_rim[current];
            for &neighbor in &adjacency[current] {
                if !seen[neighbor] {
                    seen[neighbor] = true;
                    stack.push(neighbor);
                }
            }
        }
        if !open {
            enclosed += 1;
        }
    }
    Ok(enclosed)
}
