use super::{Boundary, Fate};
use crate::math::two::Cell2d;

#[derive(Clone, Debug)]
pub struct Config {
    pub mask: Cell2d,
    pub birth: Vec<usize>,
    pub survive: Vec<usize>,
    pub boundary: Boundary,
    pub max_generations: usize,
    pub grid_size: usize,
    pub padding: usize,
}

impl Config {
    pub fn new(mask: Cell2d, birth: Vec<usize>, survive: Vec<usize>) -> Config {
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
}

#[derive(Clone, Debug)]
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
    pub fn first_frame_idx(&self) -> usize {
        0
    }
    pub fn last_frame_idx(&self) -> usize {
        self.grids.len().saturating_sub(1)
    }
}
