use super::{Boundary, Rule};

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub side: usize,
    pub rounds: usize,
    pub rule: Rule,
    pub boundary: Boundary,
    pub digest_bits: usize,
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
    pub fn state_bits(&self) -> usize {
        self.side * self.side
    }
    pub fn capacity_bits(&self) -> usize {
        self.state_bits() / 2
    }
    pub fn rate_bits(&self) -> usize {
        self.state_bits() - self.capacity_bits()
    }
}
