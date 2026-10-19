#![doc = include_str!("NAMES.md")]

use crate::error::Result;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// The sha256 digest the ids are cut from.
pub mod sha;
mod text;

pub use text::longest;

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

/// Declares `Kind`, the unit marker that serializes as one kind word and refuses any other.
///
/// A [`Named`] struct holds a `kind: Kind` field first, so its JSON object opens with the kind and
/// a decoder fed another kind fails on the first key.
///
/// ```
/// mod rule {
///     mrly_core::kind!("rule");
/// }
/// assert_eq!(serde_json::to_string(&rule::Kind).unwrap(), "\"rule\"");
/// assert!(serde_json::from_str::<rule::Kind>("\"tile\"").is_err());
/// ```
#[macro_export]
macro_rules! kind {
    ($word:literal) => {
        #[doc = ::std::concat!("The kind marker that spells `", $word, "`.")]
        #[derive(
            ::std::clone::Clone,
            ::std::marker::Copy,
            ::std::fmt::Debug,
            ::std::default::Default,
            ::std::cmp::PartialEq,
            ::std::cmp::Eq,
            ::std::hash::Hash,
        )]
        pub struct Kind;

        impl $crate::__private::serde::Serialize for Kind {
            fn serialize<S: $crate::__private::serde::Serializer>(
                &self,
                serializer: S,
            ) -> ::std::result::Result<S::Ok, S::Error> {
                serializer.serialize_str($word)
            }
        }

        impl<'de> $crate::__private::serde::Deserialize<'de> for Kind {
            fn deserialize<D: $crate::__private::serde::Deserializer<'de>>(
                deserializer: D,
            ) -> ::std::result::Result<Kind, D::Error> {
                let word =
                    <::std::string::String as $crate::__private::serde::Deserialize>::deserialize(
                        deserializer,
                    )?;
                if word == $word {
                    ::std::result::Result::Ok(Kind)
                } else {
                    ::std::result::Result::Err(
                        <D::Error as $crate::__private::serde::de::Error>::custom(::std::format!(
                            "kind {word:?} is not {:?}",
                            $word
                        )),
                    )
                }
            }
        }
    };
}
