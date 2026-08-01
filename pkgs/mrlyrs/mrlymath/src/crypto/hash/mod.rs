pub mod config;
pub mod fingerprint;
pub mod hasher;
pub mod metrics;
pub mod permute;
pub mod sbox;
pub mod sponge;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boundary {
    Wrap,
    Constant,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rule {
    Life,
    Maze,
    Replicator,
    Anneal,
}

impl Rule {
    pub fn counts(self) -> (Vec<usize>, Vec<usize>) {
        match self {
            Rule::Life => (vec![3], vec![2, 3]),
            Rule::Maze => (vec![3], vec![1, 2, 3, 4, 5]),
            Rule::Replicator => (vec![1, 3, 5, 7], vec![1, 3, 5, 7]),
            Rule::Anneal => (vec![4, 6, 7, 8], vec![3, 5, 6, 7, 8]),
        }
    }
    pub fn name(self) -> &'static str {
        match self {
            Rule::Life => "life",
            Rule::Maze => "maze",
            Rule::Replicator => "replicator",
            Rule::Anneal => "anneal",
        }
    }
    pub fn parse(name: &str) -> Option<Rule> {
        match name.to_lowercase().as_str() {
            "life" => Some(Rule::Life),
            "maze" => Some(Rule::Maze),
            "replicator" => Some(Rule::Replicator),
            "anneal" => Some(Rule::Anneal),
            _ => None,
        }
    }
}

pub use config::Config;
pub use fingerprint::{fingerprint, fingerprint_cell};
pub use hasher::{digest, hexdigest, keyed_hexdigest, Digest};
pub use permute::permute;
pub use sponge::sponge_hash;
