use super::models::{Branch, Network, Node};
use mrlycore::logs;
use std::collections::HashSet;
use std::f64::consts::LN_2;

const RUNGS: u32 = 32;

/// Sums the straight-line lengths of every branch.
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

/// What a node's degree makes it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Role {
    /// A node no branch touches.
    Alone,
    /// A node exactly one branch touches.
    Tip,
    /// A node exactly two branches pass through.
    Through,
    /// A node three or more branches meet at.
    Junction,
}

/// Tags every node by its degree, indexed like the node list.
///
/// ```
/// let mut net = mrlynum::graph::Network::new(2);
/// net.add_node(vec![0.0, 0.0]).unwrap();
/// net.add_node(vec![1.0, 0.0]).unwrap();
/// net.add_branch(0, 1, 1.0).unwrap();
/// assert_eq!(mrlynum::graph::roles(&net), vec![mrlynum::graph::Role::Tip; 2]);
/// ```
pub fn roles(network: &Network) -> Vec<Role> {
    network
        .degree()
        .iter()
        .map(|&d| match d {
            0 => Role::Alone,
            1 => Role::Tip,
            2 => Role::Through,
            _ => Role::Junction,
        })
        .collect()
}

/// Counts the nodes of degree one.
pub fn tips(network: &Network) -> usize {
    roles(network).iter().filter(|&&r| r == Role::Tip).count()
}

/// Counts the nodes of degree three or more.
pub fn junctions(network: &Network) -> usize {
    roles(network)
        .iter()
        .filter(|&&r| r == Role::Junction)
        .count()
}

/// Counts the connected components of the network.
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

/// Extracts the largest connected piece as a network of its own, branches re-indexed.
///
/// Ties go to the piece whose lowest node index comes first. An empty network comes back empty.
///
/// ```
/// let mut net = mrlynum::graph::Network::new(1);
/// for i in 0..3 { net.add_node(vec![i as f64]).unwrap(); }
/// net.add_branch(0, 1, 1.0).unwrap();
/// assert_eq!(mrlynum::graph::largest_component(&net).nodes.len(), 2);
/// ```
pub fn largest_component(network: &Network) -> Network {
    let n = network.nodes.len();
    let adjacency = network.adjacency();
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
        return giant;
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
    giant
}

/// Estimates the box-counting dimension of the node cloud over a ladder of halving boxes.
pub fn fractal_dimension(network: &Network) -> f64 {
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
    for k in 0..=RUNGS {
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
        log_count.push(logs::ln(boxes.len() as f64));
        if boxes.len() == distinct.len() {
            break;
        }
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

/// The measurements of one network.
#[derive(Clone, Debug, PartialEq)]
pub struct Census {
    /// The node count.
    pub nodes: usize,
    /// The branch count.
    pub branches: usize,
    /// The count of degree-one nodes.
    pub tips: usize,
    /// The count of nodes of degree three or more.
    pub junctions: usize,
    /// The connected component count.
    pub components: usize,
    /// The summed branch length.
    pub total_length: f64,
    /// The box-counting dimension estimate.
    pub fractal_dimension: f64,
}

/// Takes the full census of a network.
///
/// ```
/// let mut net = mrlynum::graph::Network::new(2);
/// net.add_node(vec![0.0, 0.0]).unwrap();
/// net.add_node(vec![3.0, 4.0]).unwrap();
/// assert_eq!(mrlynum::graph::census(&net).components, 2);
/// ```
pub fn census(network: &Network) -> Census {
    Census {
        nodes: network.nodes.len(),
        branches: network.branches.len(),
        tips: tips(network),
        junctions: junctions(network),
        components: components(network),
        total_length: total_length(network),
        fractal_dimension: fractal_dimension(network),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::extract::core_graph;
    use mrlycore::atoms;
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
    fn carpet_census_holds_its_pinned_counts() {
        let network = core_graph(&atoms::carpet_2d(3).fractal(4)).unwrap();
        let c = census(&network);
        assert_eq!(c.nodes, 4096);
        assert_eq!(c.branches, 6424);
        assert_eq!(c.tips, 0);
        assert_eq!(c.junctions, 3596);
        assert_eq!(c.components, 1);
        assert!((c.total_length - 6424.0).abs() < 1e-9);
    }
    #[test]
    fn the_two_node_ladder_reads_exactly_one() {
        let mut net = Network::new(2);
        net.add_node(vec![0.0, 0.0]).unwrap();
        net.add_node(vec![3.0, 4.0]).unwrap();
        assert_eq!(fractal_dimension(&net), 1.0);
        net.add_node(vec![0.0, 0.0]).unwrap();
        assert_eq!(fractal_dimension(&net), 1.0);
    }
    #[test]
    fn the_carpet_dimension_holds_its_pinned_value() {
        let network = core_graph(&atoms::carpet_2d(3).fractal(4)).unwrap();
        let d = census(&network).fractal_dimension;
        assert!((0.0..=3.0).contains(&d), "dimension {d}");
        assert_eq!(d, 1.787589465914211);
    }
}
