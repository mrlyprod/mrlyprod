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
    use mrly_core::name::Named;

    #[test]
    fn serde_round_trips() {
        round_trip(Bang::new(7, 2, 2));
        round_trip(Word::new(2, &[(7, 3), (14, 7)]).unwrap());
        round_trip(Sequence::new(7, 2, 2, "fills", "side"));
        round_trip(Lattice::Hex);
    }
    #[test]
    fn every_bang_sequence_and_word_on_names_md_is_canonical() {
        let mut seen = 0;
        for piece in include_str!("../../../core/src/name/NAMES.md").split('`') {
            let Some(rest) = piece.strip_prefix(r#"{"kind":""#) else {
                continue;
            };
            let printed = match rest.split('"').next() {
                Some("bang") => Bang::from_json(piece).map(|v| v.to_json()),
                Some("sequence") => Sequence::from_json(piece).map(|v| v.to_json()),
                Some("word") => Word::from_json(piece).map(|v| v.to_json()),
                _ => continue,
            };
            assert_eq!(printed.unwrap(), piece);
            seen += 1;
        }
        assert!(seen >= 6, "NAMES.md kept only {seen} examples");
    }
}
