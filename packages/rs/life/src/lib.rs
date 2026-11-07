pub mod catalog;
pub mod engine;
pub mod mask;
pub mod metrics;
pub mod recipe;
pub mod render;
pub mod source;

use mrly_tensor::Tensor;

pub fn moore() -> Tensor {
    mrly_math::cell::base::moore(2)
}

pub use catalog::Catalog;
pub use engine::{Mode, Reel, Run, Stop, evolve, tick};
pub use mask::{design_mask, lattice_index, mask_offsets};
pub use metrics::{churn, entropy};
pub use render::{frames, gif, heatmap};
pub use source::{Counts, Source, counts};
