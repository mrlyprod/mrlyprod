/// The knobs of the sponge hash.
pub mod config;
/// The identicon rendering of a digest.
pub mod fingerprint;
/// The digest and its hex forms.
pub mod hasher;
/// The avalanche, balance, entropy and collision measures of the hash.
pub mod metrics;
/// The round permutation that stirs the grid.
pub mod permute;
/// The 4-bit S-box and the Mrly codes that spell it.
pub mod sbox;
/// The absorb-and-squeeze core of the hash.
pub mod sponge;

/// The edge treatment of the automaton grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boundary {
    /// The toroidal edge.
    Wrap,
    /// The dead border.
    Constant,
}

/// The cellular automaton rule that stirs the hash state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rule {
    /// The Conway Life rule.
    Life,
    /// The maze-growing rule.
    Maze,
    /// The self-replicating rule.
    Replicator,
    /// The majority-annealing rule.
    Anneal,
}

impl Rule {
    /// Returns the rule's birth and survival neighbor counts.
    pub fn counts(self) -> (Vec<usize>, Vec<usize>) {
        match self {
            Rule::Life => (vec![3], vec![2, 3]),
            Rule::Maze => (vec![3], vec![1, 2, 3, 4, 5]),
            Rule::Replicator => (vec![1, 3, 5, 7], vec![1, 3, 5, 7]),
            Rule::Anneal => (vec![4, 6, 7, 8], vec![3, 5, 6, 7, 8]),
        }
    }
    /// Returns the rule's lowercase name.
    pub fn name(self) -> &'static str {
        match self {
            Rule::Life => "life",
            Rule::Maze => "maze",
            Rule::Replicator => "replicator",
            Rule::Anneal => "anneal",
        }
    }
    /// Returns the rule with the given name in any case, or None for an unknown name.
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
pub use hasher::{digest, hexdigest, keyed_hexdigest, quick_hexdigest, Digest};
pub use permute::permute;
pub use sponge::sponge_hash;
