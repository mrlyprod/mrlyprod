use super::{kind, Bang, Named};
use mrlycore::errors::{value_error, Result};
use serde::{Deserialize, Serialize};

kind!("sequence");

const MEASURES: [&str; 12] = [
    "fills",
    "voids",
    "surface",
    "peak",
    "heights",
    "vertices",
    "edges",
    "faces",
    "euler",
    "triangles",
    "holes",
    "pieces",
];

const AXES: [&str; 2] = ["level", "side"];

fn two() -> usize {
    2
}

fn is_two(base: &usize) -> bool {
    *base == 2
}

/// A design sequence's address: the design, the reading taken off it and the index it runs along.
///
/// ```
/// use mrlymath::name::{Named, Sequence};
/// let carpet = Sequence::new(7, 2, 2, "fills", "side");
/// assert_eq!(
///     carpet.to_json(),
///     r#"{"kind":"sequence","dim":2,"code":7,"measure":"fills","axis":"side"}"#
/// );
/// assert_eq!(carpet.to_file(), "sequence_dim=2_code=7_measure=fills_axis=side");
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sequence {
    /// The kind word.
    pub kind: Kind,
    /// The number of axes.
    pub dim: usize,
    /// The digits per axis, 2 unless said.
    #[serde(default = "two", skip_serializing_if = "is_two")]
    pub base: usize,
    /// The design as a number.
    pub code: u128,
    /// The reading taken.
    pub measure: String,
    /// The index the reading runs along.
    pub axis: String,
}

impl Sequence {
    /// Pins a design's reading to its measure and axis.
    pub fn new(code: u128, dim: usize, base: usize, measure: &str, axis: &str) -> Sequence {
        Sequence {
            kind: Kind,
            dim,
            base,
            code,
            measure: measure.to_string(),
            axis: axis.to_string(),
        }
    }
    /// Returns the design pinned to its dimension and base.
    pub fn design(&self) -> Bang {
        Bang::new(self.code, self.dim, self.base)
    }
}

impl Named for Sequence {
    const KIND: &'static str = "sequence";
    const BARE: &'static [&'static str] = &["measure", "axis"];
    fn checked(self) -> Result<Sequence> {
        self.design().checked()?;
        if !MEASURES.contains(&self.measure.as_str()) {
            return value_error(format!("unknown measure {:?}.", self.measure));
        }
        if !AXES.contains(&self.axis.as_str()) {
            return value_error(format!("unknown axis {:?}.", self.axis));
        }
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CARPET: &str = r#"{"kind":"sequence","dim":2,"code":7,"measure":"fills","axis":"side"}"#;

    fn carpet() -> Sequence {
        Sequence::new(7, 2, 2, "fills", "side")
    }

    #[test]
    fn defaults_elide() {
        assert_eq!(carpet().to_json(), CARPET);
        assert_eq!(
            Sequence::new(23, 3, 2, "surface", "level").to_json(),
            r#"{"kind":"sequence","dim":3,"code":23,"measure":"surface","axis":"level"}"#
        );
        assert_eq!(
            Sequence::new(4, 2, 3, "voids", "side").to_json(),
            r#"{"kind":"sequence","dim":2,"base":3,"code":4,"measure":"voids","axis":"side"}"#
        );
    }
    #[test]
    fn a_decoded_value_folds_to_the_checked_one() {
        assert_eq!(Sequence::from_json(&carpet().to_json()).unwrap(), carpet());
        assert_eq!(Sequence::from_json(CARPET).unwrap().to_json(), CARPET);
        let spelt =
            r#"{"axis":"side", "measure":"fills", "code":7, "base":2, "dim":2, "kind":"sequence"}"#;
        assert_eq!(Sequence::from_json(spelt).unwrap(), carpet());
    }
    #[test]
    fn the_carpet_row_holds_through_every_view() {
        assert_eq!(
            carpet().to_file(),
            "sequence_dim=2_code=7_measure=fills_axis=side"
        );
        assert_eq!(
            carpet().to_url(),
            "/sequence?dim=2&code=7&measure=fills&axis=side"
        );
        assert_eq!(carpet().to_mrly(), "sequence dim 2, code 7, fills, side");
        assert_eq!(Sequence::from_file(&carpet().to_file()).unwrap(), carpet());
        assert_eq!(Sequence::from_url(&carpet().to_url()).unwrap(), carpet());
    }
    #[test]
    fn the_id_is_eight_stable_hex_digits() {
        let id = carpet().to_id();
        assert_eq!(id, "8a9e4ce8");
        assert_eq!(id.len(), 8);
        assert!(id
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase()));
        assert_ne!(id, Sequence::new(7, 2, 2, "fills", "level").to_id());
    }
    #[test]
    fn only_a_fitting_sequence_parses() {
        for bad in [
            r#"{"kind":"bang","dim":2,"code":7}"#,
            r#"{"kind":"sequence","dim":2,"code":7,"measure":"area","axis":"side"}"#,
            r#"{"kind":"sequence","dim":2,"code":7,"measure":"fills","axis":"depth"}"#,
            r#"{"kind":"sequence","dim":2,"code":16,"measure":"fills","axis":"side"}"#,
            r#"{"kind":"sequence","dim":2,"code":7,"axis":"side"}"#,
            r#"{"kind":"sequence","dim":2,"code":7,"measure":"fills"}"#,
            r#"{"kind":"sequence","dim":2,"code":7,"measure":"fills","axis":"side","level":2}"#,
            "sequence_dim=2_code=7_measure=fills_axis=side",
        ] {
            assert!(Sequence::from_json(bad).is_err(), "{bad}");
        }
    }
}
