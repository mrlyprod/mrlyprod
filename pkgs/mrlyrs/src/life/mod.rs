/// The run of a seed until it fixes, loops or times out.
pub mod animate;
/// The centred cropping and tiling of a run's frames.
pub mod crop;
/// The elementary automata: their stepping, their space-time diagrams and the card of one rule.
pub mod elementary;
/// The design masks a rule reads and the lattice they generate.
pub mod mask;
/// The entropy, churn and chaos readings of a run.
pub mod metrics;
/// The run config and the recorded life.
pub mod models;
/// The PNG frames, the cumulative-visit heatmap and the gif movie of grids.
pub mod render;
/// The rule name: a life rule's birth and survival counts and whether the edge wraps.
pub mod rule;
/// The named sources of neighbor-count values, and the counts they lay down.
pub mod source;
/// The one-generation advance of a grid.
pub mod step;

use crate::core::named_enum;
use crate::math::two::Cell2d;

/// Builds the 3 by 3 Moore mask, every site on but the center.
pub fn moore() -> Cell2d {
    Cell2d::new(crate::core::cell::moore(2))
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

named_enum! {
    /// The ending of a life run.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Fate {
        /// The empty fixed point.
        Dead => "dead",
        /// The living fixed point.
        Alive => "alive",
        /// The periodic cycle.
        Loop => "loop",
        /// The generation cap reached before settling.
        Timeout => "timeout",
    }
}

pub use animate::animate;
pub use crop::{crop, tessellate};
pub use elementary::{
    affine, corner_bits, cube_orbit, gasket, genus, history, lambda, npn_class, outer_totalistic,
    popcount, reversible, rule_degree, rule_name, single_seed, step, surjective, wolfram_class,
};
pub use mask::{design_mask, lattice_index, mask_offsets};
pub use metrics::{churn, entropy};
pub use models::{Config, Life};
pub use render::{frames, heatmap, movie};
pub use rule::Rule;
pub use source::{counts, Counts, Source};
pub use step::next_grid;

#[cfg(test)]
pub(crate) fn blinker() -> Cell2d {
    let mut t = crate::core::tensor::Tensor::new(vec![5, 5]);
    t.set(&[1, 2], 1);
    t.set(&[2, 2], 1);
    t.set(&[3, 2], 1);
    Cell2d::new(t)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_parse_back() {
        for fate in Fate::all() {
            assert_eq!(fate, fate.name().parse().unwrap());
        }
    }
}
