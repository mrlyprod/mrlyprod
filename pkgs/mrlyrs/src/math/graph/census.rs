use super::models::Network;
use std::collections::HashSet;

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

pub fn tips(network: &Network) -> usize {
    network.degree().iter().filter(|&&d| d == 1).count()
}

pub fn junctions(network: &Network) -> usize {
    network.degree().iter().filter(|&&d| d >= 3).count()
}

pub fn components(network: &Network) -> usize {
    let n = network.nodes.len();
    if n == 0 {
        return 0;
    }
    let adjacency = network.adjacency();
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
    count
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
    let mut log_inv = Vec::with_capacity(samples);
    let mut log_count = Vec::with_capacity(samples);
    for i in 0..samples {
        let t = i as f64 / (samples - 1) as f64;
        let size = (256f64).powf(-t);
        let mut boxes: HashSet<Vec<i64>> = HashSet::new();
        for p in &positions {
            let key: Vec<i64> = (0..dim)
                .map(|a| (((p[a] - mins[a]) / extent) / size).floor() as i64)
                .collect();
            boxes.insert(key);
        }
        log_inv.push((1.0 / size).ln());
        log_count.push((boxes.len() as f64).ln());
    }
    let n = samples as f64;
    let mean_x: f64 = log_inv.iter().sum::<f64>() / n;
    let mean_y: f64 = log_count.iter().sum::<f64>() / n;
    let cov: f64 = log_inv
        .iter()
        .zip(&log_count)
        .map(|(x, y)| (x - mean_x) * (y - mean_y))
        .sum();
    let var: f64 = log_inv.iter().map(|x| (x - mean_x) * (x - mean_x)).sum();
    if var == 0.0 {
        return 0.0;
    }
    cov / var
}

#[derive(Clone, Debug, PartialEq)]
pub struct Census {
    pub nodes: usize,
    pub branches: usize,
    pub tips: usize,
    pub junctions: usize,
    pub components: usize,
    pub total_length: f64,
    pub fractal_dimension: f64,
}

pub fn census(network: &Network) -> Census {
    Census {
        nodes: network.nodes.len(),
        branches: network.branches.len(),
        tips: tips(network),
        junctions: junctions(network),
        components: components(network),
        total_length: total_length(network),
        fractal_dimension: fractal_dimension(network, 12),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::atoms;
    use crate::math::graph::extract::core_graph;
    #[test]
    fn carpet_census() {
        let network = core_graph(&atoms::carpet_2d(3)).unwrap();
        let c = census(&network);
        assert_eq!(c.nodes, 8);
        assert_eq!(c.branches, 8);
        assert_eq!(c.components, 1);
        assert!((c.total_length - 8.0).abs() < 1e-9);
    }
    #[test]
    fn carpet_census_matches_python_exactly() {
        let network = core_graph(&atoms::carpet_2d(3).fractal(4)).unwrap();
        let c = census(&network);
        assert_eq!(c.nodes, 4096);
        assert_eq!(c.branches, 6424);
        assert_eq!(c.tips, 0);
        assert_eq!(c.junctions, 3596);
        assert_eq!(c.components, 1);
        assert!((c.total_length - 6424.0).abs() < 1e-9);
        assert!((c.fractal_dimension - 1.4816157537748447).abs() < 1e-9);
    }
}
