use super::{Boundary, Rule};

/// The knobs of the sponge hash.
#[derive(Clone, Copy, Debug)]
pub struct Config {
    /// The side length of the square automaton grid.
    pub side: usize,
    /// The rounds one permutation runs.
    pub rounds: usize,
    /// The automaton rule stirring the state.
    pub rule: Rule,
    /// The grid's edge treatment.
    pub boundary: Boundary,
    /// The digest length in bits.
    pub digest_bits: usize,
    /// Whether the starting state is woven from carpet and net tiles.
    pub seed_tile: bool,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            side: 32,
            rounds: 16,
            rule: Rule::Replicator,
            boundary: Boundary::Wrap,
            digest_bits: 256,
            seed_tile: true,
        }
    }
}

impl Config {
    /// Returns the state size in bits, one per grid cell.
    pub fn state_bits(&self) -> usize {
        self.side * self.side
    }
    /// Returns the hidden half of the state in bits.
    pub fn capacity_bits(&self) -> usize {
        self.state_bits() / 2
    }
    /// Returns the absorbing half of the state in bits.
    pub fn rate_bits(&self) -> usize {
        self.state_bits() - self.capacity_bits()
    }
}
