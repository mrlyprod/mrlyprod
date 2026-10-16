//! The mrly names.
//!
//! Every mathematical thing prints one canonical JSON object, and the url, filename, prose and id
//! views are functions of that one string.

use mrly_core::error::Result;
use serde::Serialize;
use serde::de::DeserializeOwned;

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
/// filename, `to_prose` in a line of prose, and `to_id` hashes it. The law is
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
    fn to_prose(&self) -> Result<String> {
        text::prose(&self.to_json(), Self::BARE)
    }
}

#[macro_export]
#[doc(hidden)]
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

pub use bang::{Bang, Lattice, of, words};
pub use sequence::Sequence;
#[doc(hidden)]
pub use text::longest;
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
