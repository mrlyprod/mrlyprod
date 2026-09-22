//! The mrly names.
//!
//! Every mathematical thing prints one canonical JSON object, and the url, filename, prose and id
//! views are functions of that one string.

use crate::core::error::Result;
use serde::de::DeserializeOwned;
use serde::Serialize;

mod sha;
pub(crate) mod text;

/// The bang name: a design code pinned to its dimension, lattice and base.
pub mod bang;
/// The sequence name: a design's reading pinned to its measure and axis.
pub mod sequence;
/// The word name: an ordered list of design letters, each at its own side.
pub mod word;

/// One canonical JSON object per mathematical thing, and the views cut from it.
///
/// The object holds `kind` first, then the glossary words as keys in a fixed order per kind, with
/// defaults elided and no whitespace, so equality of things is equality of strings. Every other
/// form is a function of that string: `to_url` puts the keys in a query string, `to_file` in a
/// filename, `to_mrly` in a line of prose, and `to_id` hashes it. The law is
/// `from_json(to_json(x)) == checked(x)` for every value `x`.
pub trait Named: Serialize + DeserializeOwned + Sized {
    /// The kind word, the first value of the object.
    const KIND: &'static str;
    /// The keys whose values are lists even when one item long, so the query string reads them back.
    const LISTS: &'static [&'static str] = &[];
    /// The keys whose word value prints alone in the prose form.
    const BARE: &'static [&'static str] = &[];
    /// Folds a decoded value to its canonical form, or an error for one outside the kind.
    fn checked(self) -> Result<Self>;
    /// Prints the canonical JSON object.
    fn to_json(&self) -> String {
        serde_json::to_string(self).expect("a name serializes")
    }
    /// Reads a JSON object into its canonical value, or an error naming the broken key.
    fn from_json(text: &str) -> Result<Self> {
        let value: Self = serde_json::from_str(text)?;
        value.checked()
    }
    /// Prints the first eight hex digits of the sha256 of the canonical JSON.
    fn to_id(&self) -> String {
        sha::short(self.to_json().as_bytes())
    }
    /// Prints the kind as a path and the keys as a query string, lists comma-joined, or an error when the name does not read back.
    fn to_url(&self) -> Result<String> {
        text::url(&self.to_json())
    }
    /// Reads a path and query string back into the value, or an error.
    fn from_url(text: &str) -> Result<Self> {
        Self::from_json(&text::url_to_json(text, Self::KIND, Self::LISTS)?)
    }
    /// Prints the kind and the `key=value` pairs joined by underscores, lists in brackets, or an error when the name does not read back.
    fn to_file(&self) -> Result<String> {
        text::file(&self.to_json())
    }
    /// Reads a filename back into the value, or an error.
    fn from_file(text: &str) -> Result<Self> {
        Self::from_json(&text::file_to_json(text, Self::KIND)?)
    }
    /// Prints the kind and the keys as a line of prose for pages, or an error when the name does not read back.
    fn to_mrly(&self) -> Result<String> {
        text::mrly(&self.to_json(), Self::BARE)
    }
}

macro_rules! kind {
    ($word:literal) => {
        #[doc = concat!("The kind marker that spells `", $word, "`.")]
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
        pub struct Kind;

        impl serde::Serialize for Kind {
            fn serialize<S: serde::Serializer>(
                &self,
                serializer: S,
            ) -> std::result::Result<S::Ok, S::Error> {
                serializer.serialize_str($word)
            }
        }

        impl<'de> serde::Deserialize<'de> for Kind {
            fn deserialize<D: serde::Deserializer<'de>>(
                deserializer: D,
            ) -> std::result::Result<Kind, D::Error> {
                let word = <String as serde::Deserialize>::deserialize(deserializer)?;
                if word == $word {
                    Ok(Kind)
                } else {
                    Err(serde::de::Error::custom(format!(
                        "kind {word:?} is not {:?}",
                        $word
                    )))
                }
            }
        }
    };
}

pub(crate) use kind;

pub use bang::{Bang, Lattice};
pub use sequence::Sequence;
pub use word::Word;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gen::name::Tile;
    use crate::life::Counts;
    use crate::life::Rule;
    use crate::math::round_trip;

    #[test]
    fn serde_round_trips() {
        round_trip(Bang::new(7, 2, 2));
        round_trip(Word::new(2, &[(7, 3), (14, 7)]).unwrap());
        round_trip(Sequence::new(7, 2, 2, "fills", "side"));
        round_trip(Lattice::Hex);
    }

    #[test]
    fn every_kind_has_one_canonical_string_and_one_id() {
        let bang = Bang::new(7, 2, 2);
        let rule = Rule::new(vec![3], vec![2, 3], false);
        let word = Word::new(2, &[(7, 3), (14, 7)]).unwrap();
        let tile = Tile::from_json(r#"{"kind":"tile","code":7,"side":3,"level":2}"#).unwrap();
        let sequence = Sequence::new(7, 2, 2, "fills", "side");
        let strings = [
            bang.to_json(),
            rule.to_json(),
            word.to_json(),
            tile.to_json(),
            sequence.to_json(),
        ];
        let ids = [
            bang.to_id(),
            rule.to_id(),
            word.to_id(),
            tile.to_id(),
            sequence.to_id(),
        ];
        for (text, id) in strings.iter().zip(&ids) {
            assert!(text.starts_with("{\"kind\":\""));
            assert!(!text.contains(' '));
            assert_eq!(id.len(), 8);
            assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
        }
        assert_eq!(Bang::from_json(&strings[0]).unwrap(), bang);
        assert_eq!(Rule::from_json(&strings[1]).unwrap(), rule);
        assert_eq!(Word::from_json(&strings[2]).unwrap(), word);
        assert_eq!(Tile::from_json(&strings[3]).unwrap(), tile);
        assert_eq!(Sequence::from_json(&strings[4]).unwrap(), sequence);
        assert!(Rule::from_json(&strings[0]).is_err());
        assert_eq!(rule.birth, Counts::List(vec![3]));
    }

    #[test]
    fn every_example_on_names_md_round_trips() {
        let doc = include_str!("../../../NAMES.md");
        let mut seen = 0;
        for piece in doc.split('`') {
            let Some(rest) = piece.strip_prefix("{\"kind\":\"") else {
                continue;
            };
            let kind = rest.split('"').next().unwrap();
            let printed = match kind {
                "bang" => Bang::from_json(piece).map(|v| v.to_json()),
                "rule" => Rule::from_json(piece).map(|v| v.to_json()),
                "sequence" => Sequence::from_json(piece).map(|v| v.to_json()),
                "tile" => Tile::from_json(piece).map(|v| v.to_json()),
                "word" => Word::from_json(piece).map(|v| v.to_json()),
                other => panic!("NAMES.md names no kind {other:?}"),
            };
            let printed = printed.unwrap_or_else(|e| panic!("{piece} does not read: {e}"));
            assert_eq!(printed, piece, "{piece} is not canonical");
            seen += 1;
        }
        assert!(seen >= 13, "NAMES.md kept only {seen} examples");
    }
}
