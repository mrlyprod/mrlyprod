//! The engine: a rule over a grid, stepped, recorded, measured and rendered.
//!
//! - `step`: one generation of a grid under birth and survive counts.
//! - `animate`: a seed run until it fixes, loops or times out.
//! - `models`: the run config and the recorded life.
//! - `metrics`: the entropy and churn readings of a run.
//! - `crop`: the centred cropping and tiling of a run's frames.
//! - `mask`: the design masks a rule reads and the lattice they generate.
//! - `source`: the named sources of neighbor counts, and the counts they lay down.
//! - `elementary`: the one-line automata and the card of one rule.
//! - `render`: the PNG frames, the visit heatmap and the gif movie.
//! - `rule`: the canonical name of a rule.
//!
//! The doors: [`next_grid`](crate::life::next_grid), [`animate`](crate::life::animate()),
//! [`entropy`](crate::life::entropy), [`churn`](crate::life::churn),
//! [`counts`](crate::life::counts), [`frames`](crate::life::frames),
//! [`heatmap`](crate::life::heatmap), [`history`](crate::life::history) and
//! [`Rule`](crate::life::Rule).

/// The run of a seed until it fixes, loops or times out.
pub mod animate;
/// The centred cropping and tiling of a run's frames.
pub mod crop;
/// The elementary automata: their stepping, their space-time diagrams and the card of one rule.
pub mod elementary;
/// The design masks a rule reads and the lattice they generate.
pub mod mask;
/// The entropy and churn readings of a run.
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

use crate::core::error::Result;
use crate::core::named_enum;
use crate::math::two::Cell2d;
use serde::{Deserialize, Serialize};

/// Builds the 3 by 3 Moore mask, every site on but the center.
///
/// # Errors
///
/// Errs when the mask tensor will not make a flat cell.
pub fn moore() -> Result<Cell2d> {
    Cell2d::new(crate::core::cell::moore(2))
}

named_enum! {
    /// The edge policy of a life grid.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum Boundary {
        /// The fixed dead border.
        Constant => "Constant",
        /// The toroidal edge.
        Wrap => "Wrap",
    }
}

impl Boundary {
    /// Returns whether the edges wrap.
    pub fn wrap(self) -> bool {
        matches!(self, Boundary::Wrap)
    }
}

named_enum! {
    /// The ending of a life run.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    t.set(&[1, 2], 1).unwrap();
    t.set(&[2, 2], 1).unwrap();
    t.set(&[3, 2], 1).unwrap();
    Cell2d::new(t).unwrap()
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
    #[test]
    fn boundary_names_parse_back() {
        assert_eq!(Boundary::all().len(), 2);
        for boundary in Boundary::all() {
            assert_eq!(boundary, boundary.name().parse().unwrap());
        }
    }
    #[test]
    fn serde_round_trips() {
        let counts = Counts::drawn(Source::Random(7), true, false);
        assert_eq!(
            counts,
            serde_json::from_str(&serde_json::to_string(&counts).unwrap()).unwrap()
        );
        let listed = Counts::List(vec![2, 3]);
        assert_eq!(
            listed,
            serde_json::from_str(&serde_json::to_string(&listed).unwrap()).unwrap()
        );
        assert_eq!(
            Boundary::Wrap,
            serde_json::from_str(&serde_json::to_string(&Boundary::Wrap).unwrap()).unwrap()
        );
        assert_eq!(
            Fate::Loop,
            serde_json::from_str(&serde_json::to_string(&Fate::Loop).unwrap()).unwrap()
        );
    }
}
