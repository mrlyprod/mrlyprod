use mrlycore::errors::Result;

pub(crate) mod text;

/// The bang name: a design code pinned to its dimension and base.
pub mod bang;
/// The rule name: a life rule's birth and survival counts and edge policy.
pub mod rule;
/// The tile name: a full tile recipe folded to its one canonical spelling.
pub mod tile;
/// The word name: an ordered list of plane letters at base two.
pub mod word;

/// One canonical string per mathematical thing.
pub trait Named: Sized {
    /// Prints the thing's canonical mrly name.
    fn to_str(&self) -> String;
    /// Parses a canonical mrly name back into the thing, or an error for any other spelling.
    fn from_str(text: &str) -> Result<Self>;
}

pub use bang::Bang;
pub use rule::Rule;
pub use tile::classic_code;
pub use word::Word;
