pub mod animate;
pub mod mask;
pub mod metrics;
pub mod models;
pub mod recipe;
pub mod render;
pub mod rule;
pub mod source;
pub mod step;

use mrly_error::Result;
use mrly_name::named_enum;
use mrly_two::Cell2d;
use serde::{Deserialize, Serialize};

pub fn moore() -> Result<Cell2d> {
    Cell2d::new(mrly_math::cell::base::moore(2))
}

named_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Boundary {
        Constant => "Constant",
        Wrap => "Wrap",
    }
}

impl Boundary {
    pub fn wrap(self) -> bool {
        matches!(self, Boundary::Wrap)
    }
}

named_enum! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Fate {
        Dead => "dead",
        Alive => "alive",
        Loop => "loop",
        Timeout => "timeout",
    }
}

pub use animate::animate;
pub use mask::{design_mask, lattice_index, mask_offsets};
pub use metrics::{churn, entropy};
pub use models::{Config, Life};
pub use render::{frames, heatmap};
pub use rule::Rule;
pub use source::{Counts, Source, counts};
pub use step::next_grid;
