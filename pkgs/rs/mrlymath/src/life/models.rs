use super::sequence::Counts;
use super::{Boundary, Fate};
use crate::two::Cell2d;
use mrlycore::errors::Result;

/// The rulebook of a life run.
#[derive(Clone, Debug)]
pub struct Config {
    /// The neighborhood mask.
    pub mask: Cell2d,
    /// The neighbor counts that create a cell.
    pub birth: Counts,
    /// The neighbor counts that keep a cell.
    pub survive: Counts,
    /// The edge policy.
    pub boundary: Boundary,
    /// The generation cap.
    pub max_generations: usize,
    /// The tiling factor applied to the seed.
    pub grid_size: usize,
    /// The dead border added around the seed.
    pub padding: usize,
}

impl Config {
    /// Builds a config with a constant boundary, a 64-generation cap, no tiling and no padding.
    pub fn new(mask: Cell2d, birth: impl Into<Counts>, survive: impl Into<Counts>) -> Config {
        Config {
            mask,
            birth: birth.into(),
            survive: survive.into(),
            boundary: Boundary::Constant,
            max_generations: 64,
            grid_size: 1,
            padding: 0,
        }
    }
    /// Returns the largest neighbor count the mask can reach.
    pub fn budget(&self) -> usize {
        self.mask.types().sum() as usize
    }
    /// Resolves the birth and survive counts against the mask's budget.
    pub fn counts(&self) -> Result<(Vec<usize>, Vec<usize>)> {
        let budget = self.budget();
        Ok((self.birth.values(budget)?, self.survive.values(budget)?))
    }
}

/// The recorded run of one seed.
#[derive(Clone, Debug)]
pub struct Life {
    /// Every generation in order.
    pub grids: Vec<Cell2d>,
    /// The run's ending.
    pub fate: Fate,
    /// The number of recorded generations.
    pub count: usize,
    /// The cycle length when the fate is a loop, else zero.
    pub loop_length: usize,
}

impl Life {
    /// Returns the final grid, or None when the run is empty.
    pub fn last(&self) -> Option<&Cell2d> {
        self.grids.last()
    }
    /// Returns the index of the first frame.
    pub fn first_frame_idx(&self) -> usize {
        0
    }
    /// Returns the index of the last frame.
    pub fn last_frame_idx(&self) -> usize {
        self.grids.len().saturating_sub(1)
    }
}
