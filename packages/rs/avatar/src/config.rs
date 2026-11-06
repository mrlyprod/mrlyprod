use mrly_error::{Result, value_error};
use mrly_name::named_enum;
use serde::{Deserialize, Serialize};

named_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Rule {
        Life => "life",
        Maze => "maze",
        Replicator => "replicator",
        Anneal => "anneal",
    }
}

impl Rule {
    pub(crate) fn counts(self) -> (Vec<usize>, Vec<usize>) {
        match self {
            Rule::Life => (vec![3], vec![2, 3]),
            Rule::Maze => (vec![3], vec![1, 2, 3, 4, 5]),
            Rule::Replicator => (vec![1, 3, 5, 7], vec![1, 3, 5, 7]),
            Rule::Anneal => (vec![4, 6, 7, 8], vec![3, 5, 6, 7, 8]),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Config {
    pub side: usize,
    pub rounds: usize,
    pub rule: Rule,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            side: 32,
            rounds: 16,
            rule: Rule::Replicator,
        }
    }
}

impl Config {
    pub(crate) fn bits(&self) -> usize {
        self.side * self.side
    }
    pub(crate) fn rate(&self) -> usize {
        self.bits() - self.bits() / 2
    }
    pub(crate) fn check(&self) -> Result<()> {
        if self.side < 2 || !self.side.is_multiple_of(2) || self.side > 4096 {
            return value_error("side must be even, from 2 to 4096.");
        }
        if !(1..=64).contains(&self.rounds) {
            return value_error("rounds must be from 1 to 64.");
        }
        Ok(())
    }
}
