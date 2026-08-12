/// The run of a seed until it fixes, loops or times out.
pub mod animate;
/// The centred cropping of a run to its ever-live bounds.
pub mod crop;
/// The cumulative-visit heatmap frames of a run.
pub mod heatmap;
/// The entropy reading of a grid.
pub mod metrics;
/// The run config and the recorded life.
pub mod models;
/// The PNG frames of grids and runs.
pub mod render;
/// The named sources of neighbor-count values.
pub mod sequence;
/// The one-generation advance of a grid.
pub mod step;
/// The chaptered chain of runs, each seeding the next.
pub mod story;

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
}

pub use animate::animate;
pub use crop::crop;
pub use heatmap::{heatmap, heatmap_range};
pub use metrics::entropy;
pub use models::{Config, Life};
pub use render::{frames, frames_of};
pub use sequence::{counts, Sequence};
pub use step::next_grid;
pub use story::{tell, Chapter, Story};

pub use mrlycore::ramp::Colorizer;
