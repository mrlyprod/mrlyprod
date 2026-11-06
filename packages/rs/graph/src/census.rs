use super::models::{Branch, Network, Node};
use mrly_error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::f64::consts::LN_2;

const RUNGS: u32 = 32;

pub fn total_length(network: &Network) -> f64 {
    network
        .branches
        .iter()
        .map(|b| {
            let a = &network.nodes[b.parent].position;
            let c = &network.nodes[b.child].position;
            a.iter()
                .zip(c)
                .map(|(x, y)| (x - y) * (x - y))
                .sum::<f64>()
                .sqrt()
        })
        .sum()
}

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Role {
    Alone,
    Tip,
    Through,
    Junction,
}

pub fn roles(network: &Network) -> Result<Vec<Role>> {
    Ok(network
        .degree()?
        .iter()
        .map(|&d| match d {
            0 => Role::Alone,
            1 => Role::Tip,
            2 => Role::Through,
            _ => Role::Junction,
        })
        .collect())
}

pub fn tips(network: &Network) -> Result<usize> {
    Ok(roles(network)?.iter().filter(|&&r| r == Role::Tip).count())
}

pub fn junctions(network: &Network) -> Result<usize> {
    Ok(roles(network)?
        .iter()
        .filter(|&&r| r == Role::Junction)
        .count())
}

pub fn components(network: &Network) -> Result<usize> {
    let n = network.nodes.len();
    if n == 0 {
        return Ok(0);
    }
    let adjacency = network.adjacency()?;
    let mut seen = vec![false; n];
    let mut count = 0;
    for start in 0..n {
        if seen[start] {
            continue;
        }
        count += 1;
        let mut stack = vec![start];
        seen[start] = true;
        while let Some(current) = stack.pop() {
            for &neighbor in &adjacency[&current] {
                if !seen[neighbor] {
                    seen[neighbor] = true;
                    stack.push(neighbor);
                }
            }
        }
    }
    Ok(count)
}

pub fn largest_component(network: &Network) -> Result<Network> {
    let n = network.nodes.len();
    let adjacency = network.adjacency()?;
    let mut label = vec![usize::MAX; n];
    let mut sizes: Vec<usize> = Vec::new();
    for start in 0..n {
        if label[start] != usize::MAX {
            continue;
        }
        let piece = sizes.len();
        let mut size = 0;
        let mut stack = vec![start];
        label[start] = piece;
        while let Some(current) = stack.pop() {
            size += 1;
            for &neighbor in &adjacency[&current] {
                if label[neighbor] == usize::MAX {
                    label[neighbor] = piece;
                    stack.push(neighbor);
                }
            }
        }
        sizes.push(size);
    }
    let mut best = 0;
    for (piece, &size) in sizes.iter().enumerate() {
        if size > sizes[best] {
            best = piece;
        }
    }
    let mut giant = Network::new(network.dim);
    if sizes.is_empty() {
        return Ok(giant);
    }
    let mut index_of = vec![usize::MAX; n];
    for (old, node) in network.nodes.iter().enumerate() {
        if label[old] != best {
            continue;
        }
        let index = giant.nodes.len();
        index_of[old] = index;
        giant.nodes.push(Node {
            position: node.position.clone(),
            index,
        });
    }
    for branch in &network.branches {
        if label[branch.parent] != best {
            continue;
        }
        giant.branches.push(Branch {
            parent: index_of[branch.parent],
            child: index_of[branch.child],
            radius: branch.radius,
        });
    }
    Ok(giant)
}

pub fn fractal_dimension(network: &Network, samples: usize) -> f64 {
    let positions: Vec<&Vec<f64>> = network.nodes.iter().map(|n| &n.position).collect();
    if positions.len() < 2 {
        return 0.0;
    }
    let dim = network.dim;
    let mins: Vec<f64> = (0..dim)
        .map(|a| positions.iter().map(|p| p[a]).fold(f64::MAX, f64::min))
        .collect();
    let maxs: Vec<f64> = (0..dim)
        .map(|a| positions.iter().map(|p| p[a]).fold(f64::MIN, f64::max))
        .collect();
    let extent = mins
        .iter()
        .zip(&maxs)
        .map(|(lo, hi)| hi - lo)
        .fold(0.0, f64::max);
    if extent == 0.0 {
        return 0.0;
    }
    let distinct: HashSet<Vec<u64>> = positions
        .iter()
        .map(|p| p.iter().map(|v| v.to_bits()).collect())
        .collect();
    let mut scales: Vec<f64> = Vec::new();
    let mut log_count: Vec<f64> = Vec::new();
    for k in 0..samples.min(RUNGS as usize + 1) as u32 {
        let split = f64::from_bits(((1023 + k) as u64) << 52);
        let last = (1i64 << k) - 1;
        let mut boxes: HashSet<Vec<i64>> = HashSet::new();
        for p in &positions {
            let key: Vec<i64> = (0..dim)
                .map(|a| ((((p[a] - mins[a]) * split) / extent).floor() as i64).min(last))
                .collect();
            boxes.insert(key);
        }
        scales.push(k as f64);
        log_count.push(f64::ln(boxes.len() as f64));
        if boxes.len() == distinct.len() {
            break;
        }
    }
    if scales.len() < 2 {
        return 0.0;
    }
    let n = scales.len() as f64;
    let mean_x: f64 = scales.iter().sum::<f64>() / n;
    let mean_y: f64 = log_count.iter().sum::<f64>() / n;
    let cov: f64 = scales
        .iter()
        .zip(&log_count)
        .map(|(x, y)| (x - mean_x) * (y - mean_y))
        .sum();
    let var: f64 = scales.iter().map(|x| (x - mean_x) * (x - mean_x)).sum();
    cov / (var * LN_2)
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Census {
    pub nodes: usize,
    pub branches: usize,
    pub tips: usize,
    pub junctions: usize,
    pub components: usize,
    pub total_length: f64,
    pub fractal_dimension: f64,
}

pub fn census(network: &Network) -> Result<Census> {
    Ok(Census {
        nodes: network.nodes.len(),
        branches: network.branches.len(),
        tips: tips(network)?,
        junctions: junctions(network)?,
        components: components(network)?,
        total_length: total_length(network),
        fractal_dimension: fractal_dimension(network, 12),
    })
}
