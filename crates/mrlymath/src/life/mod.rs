/// The run of a seed until it fixes, loops or times out.
pub mod animate;
/// The centred cropping and tiling of a run's frames.
pub mod crop;
/// The elementary automata: their stepping, their space-time diagrams and the card of one rule.
pub mod elementary;
/// The cumulative-visit heatmap frames of a run.
pub mod heatmap;
/// The design masks a rule reads and the lattice they generate.
pub mod mask;
/// The entropy, churn and chaos readings of a run.
pub mod metrics;
/// The run config and the recorded life.
pub mod models;
/// The PNG frames and the gif movie of grids and runs.
pub mod render;
/// The named sources of neighbor-count values, and the counts they lay down.
pub mod sequence;
/// The one-generation advance of a grid.
pub mod step;
/// The chaptered chain of runs, each seeding the next.
pub mod story;

use crate::two::Cell2d;
use mrlycore::errors::{value_error, Result};

/// Builds the 3 by 3 Moore mask, every site on but the center.
pub fn moore() -> Cell2d {
    Cell2d::new(mrlycore::cell::moore(2))
}

/// The edge policy of a life grid.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boundary {
    /// The fixed dead border.
    Constant,
    /// The toroidal edge.
    Wrap,
}

impl Boundary {
    /// Returns whether the edges wrap.
    pub fn wrap(self) -> bool {
        matches!(self, Boundary::Wrap)
    }
}

/// The ending of a life run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fate {
    /// The empty fixed point.
    Dead,
    /// The living fixed point.
    Alive,
    /// The periodic cycle.
    Loop,
    /// The generation cap reached before settling.
    Timeout,
}

impl Fate {
    /// Returns the fate's lowercase name.
    pub fn name(self) -> &'static str {
        match self {
            Fate::Dead => "dead",
            Fate::Alive => "alive",
            Fate::Loop => "loop",
            Fate::Timeout => "timeout",
        }
    }
    /// Parses a fate name, or an error for an unknown one.
    pub fn parse(name: &str) -> Result<Fate> {
        match name {
            "dead" => Ok(Fate::Dead),
            "alive" => Ok(Fate::Alive),
            "loop" => Ok(Fate::Loop),
            "timeout" => Ok(Fate::Timeout),
            other => value_error(format!("unknown fate {other:?}.")),
        }
    }
}

pub use animate::animate;
pub use crop::{crop, tessellate};
pub use elementary::{
    affine, corner_bits, cube_orbit, gasket, genus, history, lambda, npn_class, outer_totalistic,
    popcount, reversible, rule_degree, rule_name, single_seed, step, surjective, wolfram_class,
};
pub use heatmap::{heatmap, heatmap_range};
pub use mask::{design_mask, lattice_index, mask_offsets};
pub use metrics::{churn, entropy};
pub use models::{Config, Life};
pub use render::{frames, frames_of, frames_with, movie};
pub use sequence::{counts, Counts, Sequence};
pub use step::next_grid;
pub use story::{tell, Chapter, Story};

pub use mrlycore::ramp::Colorizer;
