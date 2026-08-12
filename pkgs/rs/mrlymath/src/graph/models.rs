use mrlycore::errors::{value_error, Result};
use std::collections::HashMap;

/// A point of the network.
#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    /// The coordinates, one per dimension.
    pub position: Vec<f64>,
    /// The node's place in the network's list.
    pub index: usize,
}

/// A link between two nodes.
#[derive(Clone, Debug, PartialEq)]
pub struct Branch {
    /// The index of the node the branch leaves.
    pub parent: usize,
    /// The index of the node the branch reaches.
    pub child: usize,
    /// The thickness of the branch.
    pub radius: f64,
}

/// A spatial graph of nodes and branches.
#[derive(Clone, Debug, PartialEq)]
pub struct Network {
    /// The dimension every position must match.
    pub dim: usize,
    /// The nodes in insertion order.
    pub nodes: Vec<Node>,
    /// The branches in insertion order.
    pub branches: Vec<Branch>,
}

impl Network {
    /// Builds an empty network of the given dimension.
    pub fn new(dim: usize) -> Network {
        Network {
            dim,
            nodes: Vec::new(),
            branches: Vec::new(),
        }
    }
    /// Appends a node at the position and returns its index, or an error on a dimension mismatch.
    pub fn add_node(&mut self, position: Vec<f64>) -> Result<usize> {
        if position.len() != self.dim {
            return value_error(format!(
                "Expected {}D position, got {}D",
                self.dim,
                position.len()
            ));
        }
        let index = self.nodes.len();
        self.nodes.push(Node { position, index });
        Ok(index)
    }
    /// Appends a branch between two node indices, or an error when either is out of range.
    pub fn add_branch(&mut self, parent: usize, child: usize, radius: f64) -> Result<()> {
        let n = self.nodes.len();
        if parent >= n || child >= n {
            return value_error(format!("Branch endpoints out of range: {parent}, {child}"));
        }
        self.branches.push(Branch {
            parent,
            child,
            radius,
        });
        Ok(())
    }
    /// Returns each node's branch count, indexed like the node list.
    pub fn degree(&self) -> Vec<usize> {
        let mut deg = vec![0; self.nodes.len()];
        for b in &self.branches {
            deg[b.parent] += 1;
            deg[b.child] += 1;
        }
        deg
    }
    /// Returns the undirected neighbor lists of every node.
    pub fn adjacency(&self) -> HashMap<usize, Vec<usize>> {
        let mut adj: HashMap<usize, Vec<usize>> =
            (0..self.nodes.len()).map(|i| (i, Vec::new())).collect();
        for b in &self.branches {
            adj.get_mut(&b.parent).unwrap().push(b.child);
            adj.get_mut(&b.child).unwrap().push(b.parent);
        }
        adj
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn build_and_measure() {
        let mut n = Network::new(2);
        let a = n.add_node(vec![0.0, 0.0]).unwrap();
        let b = n.add_node(vec![1.0, 0.0]).unwrap();
        n.add_branch(a, b, 1.0).unwrap();
        assert_eq!(n.degree(), vec![1, 1]);
        assert!(n.add_node(vec![0.0]).is_err());
        assert!(n.add_branch(0, 5, 1.0).is_err());
    }
}
