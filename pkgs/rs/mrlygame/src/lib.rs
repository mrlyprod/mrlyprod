#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

/// The editorial constants: ladders, caps, tempos and freezes.
pub mod config;
/// The press: one quest emitted as frames, heatmap, audio and record.
pub mod emit;
/// The palettes: frame colors and the heatmap ramp.
pub mod frames;
/// The soundtrack: scores, tracks and the two-part quest audio.
pub mod music;
/// The quest: chained runs retried until a chapter settles alive.
pub mod quest;
/// The rule draw: sequences, reflection and the zero and one rolls.
pub mod sequence;
/// The samplers: boards up the ladders and the three mask paths.
pub mod variations;

use mrlycore::errors::{value_error, Result};

/// The two rulebooks a segment may run.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Way {
    /// The classic B3/S23 rule on the Moore ring.
    Conway,
    /// The sequence-driven rule on a drawn mask.
    Mrly,
}

impl Way {
    /// Returns the other way.
    pub fn flip(self) -> Way {
        match self {
            Way::Conway => Way::Mrly,
            Way::Mrly => Way::Conway,
        }
    }
    /// Returns the way's lowercase name.
    pub fn name(self) -> &'static str {
        match self {
            Way::Conway => "conway",
            Way::Mrly => "mrly",
        }
    }
    /// Parses a way name, or an error for an unknown one.
    pub fn parse(name: &str) -> Result<Way> {
        match name {
            "conway" => Ok(Way::Conway),
            "mrly" => Ok(Way::Mrly),
            other => value_error(format!("unknown way {other:?}.")),
        }
    }
}

/// The three ways a segment draws its mask.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Path {
    /// A three by three classic design.
    Simple,
    /// A fresh tile drawn under the mask caps.
    Basic,
    /// The segment's own board, coin-inverted.
    Copy,
}

impl Path {
    /// Returns the path's lowercase name.
    pub fn name(self) -> &'static str {
        match self {
            Path::Simple => "simple",
            Path::Basic => "basic",
            Path::Copy => "copy",
        }
    }
    /// Parses a path name, or an error for an unknown one.
    pub fn parse(name: &str) -> Result<Path> {
        match name {
            "simple" => Ok(Path::Simple),
            "basic" => Ok(Path::Basic),
            "copy" => Ok(Path::Copy),
            other => value_error(format!("unknown path {other:?}.")),
        }
    }
}

pub use config::Config;
pub use emit::{emit, emit_with, SEED};
pub use quest::{pivot_options, quest, Quest, Segment};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ways_flip_back_and_forth() {
        assert_eq!(Way::Conway.flip(), Way::Mrly);
        assert_eq!(Way::Mrly.flip(), Way::Conway);
        assert_eq!(Way::Conway.flip().flip(), Way::Conway);
    }
    #[test]
    fn names_round_trip() {
        for way in [Way::Conway, Way::Mrly] {
            assert_eq!(Way::parse(way.name()).unwrap(), way);
        }
        for path in [Path::Simple, Path::Basic, Path::Copy] {
            assert_eq!(Path::parse(path.name()).unwrap(), path);
        }
        assert!(Way::parse("sideways").is_err());
        assert!(Path::parse("scenic").is_err());
    }
}
