//! The design names: the bang, sequence and word kinds of the canonical JSON object.
//!
//! The `Named` trait, the `kind!` marker and the text views live in [`mrly_core::name`].

/// The bang name: a design code pinned to its dimension, lattice and base.
pub mod bang;
/// The sequence name: a design's reading pinned to its measure and axis.
pub mod sequence;
/// The word name: an ordered list of design letters, each at its own side.
pub mod word;

pub use bang::{Bang, Lattice, of, words};
pub use sequence::Sequence;
pub use word::Word;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::round_trip;

    #[test]
    fn serde_round_trips() {
        round_trip(Bang::new(7, 2, 2));
        round_trip(Word::new(2, &[(7, 3), (14, 7)]).unwrap());
        round_trip(Sequence::new(7, 2, 2, "fills", "side"));
        round_trip(Lattice::Hex);
    }
}
