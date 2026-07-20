pub mod animate;
pub mod crop;
pub mod heatmap;
pub mod metrics;
pub mod models;
pub mod render;
pub mod sequence;
pub mod step;
pub mod story;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Boundary {
    Constant,
    Wrap,
}

impl Boundary {
    pub fn wrap(self) -> bool {
        matches!(self, Boundary::Wrap)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fate {
    Dead,
    Alive,
    Loop,
    Timeout,
}

impl Fate {
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

pub use crate::core::ramp::Colorizer;
