use super::source::Counts;
use super::{Boundary, Fate};
use mrly_core::error::Result;
use mrly_math::two::Cell2d;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub mask: Cell2d,
    pub birth: Counts,
    pub survive: Counts,
    pub boundary: Boundary,
    pub max_generations: usize,
    pub grid_size: usize,
    pub padding: usize,
}

impl Config {
    pub fn new(mask: Cell2d, birth: Counts, survive: Counts) -> Config {
        Config {
            mask,
            birth,
            survive,
            boundary: Boundary::Constant,
            max_generations: 64,
            grid_size: 1,
            padding: 0,
        }
    }
    pub fn budget(&self) -> usize {
        self.mask.types().sum() as usize
    }
    pub fn counts(&self) -> Result<(Vec<usize>, Vec<usize>)> {
        let budget = self.budget();
        Ok((self.birth.values(budget)?, self.survive.values(budget)?))
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Life {
    pub grids: Vec<Cell2d>,
    pub fate: Fate,
    pub count: usize,
    pub loop_length: usize,
}

impl Life {
    pub fn last(&self) -> Option<&Cell2d> {
        self.grids.last()
    }
}
