use crate::core::errors::{value_error, Result};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub position: Vec<f64>,
    pub index: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Branch {
    pub parent: usize,
    pub child: usize,
    pub radius: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Network {
    pub dim: usize,
    pub nodes: Vec<Node>,
    pub branches: Vec<Branch>,
}

impl Network {
    pub fn new(dim: usize) -> Network {
        Network {
            dim,
            nodes: Vec::new(),
            branches: Vec::new(),
        }
    }
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
    pub fn degree(&self) -> Vec<usize> {
        let mut deg = vec![0; self.nodes.len()];
        for b in &self.branches {
            deg[b.parent] += 1;
            deg[b.child] += 1;
        }
        deg
    }
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
