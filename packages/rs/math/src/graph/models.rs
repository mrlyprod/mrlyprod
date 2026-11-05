use mrly_core::error::{Result, value_error};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub position: Vec<f64>,
    pub index: usize,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Branch {
    pub parent: usize,
    pub child: usize,
    pub radius: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
    pub fn degree(&self) -> Result<Vec<usize>> {
        let n = self.nodes.len();
        let mut deg = vec![0; n];
        for b in &self.branches {
            if b.parent >= n || b.child >= n {
                return value_error(format!(
                    "Branch endpoints out of range: {}, {}",
                    b.parent, b.child
                ));
            }
            deg[b.parent] += 1;
            deg[b.child] += 1;
        }
        Ok(deg)
    }
    pub fn adjacency(&self) -> Result<HashMap<usize, Vec<usize>>> {
        let n = self.nodes.len();
        let mut adj: HashMap<usize, Vec<usize>> = (0..n).map(|i| (i, Vec::new())).collect();
        for b in &self.branches {
            if b.parent >= n || b.child >= n {
                return value_error(format!(
                    "Branch endpoints out of range: {}, {}",
                    b.parent, b.child
                ));
            }
            adj.entry(b.parent).or_default().push(b.child);
            adj.entry(b.child).or_default().push(b.parent);
        }
        Ok(adj)
    }
}
