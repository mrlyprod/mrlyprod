use crate::rng::Rng;
use crate::Json;

/// A dataset source: a named, seeded generator of JSON rows.
///
/// Pouring obeys the determinism law: the same seed pours the same rows in
/// the same order, and a shorter pour is a prefix of a longer one. A well
/// may tag a row `{"split": "eval"}` to claim it for evaluation; untagged
/// rows are training rows.
///
/// ```
/// use mrlycore::data::Well;
/// use mrlycore::{json, Json};
///
/// struct Squares;
///
/// impl Well for Squares {
///     fn name(&self) -> &str {
///         "squares"
///     }
///     fn about(&self) -> &str {
///         "The squares of the naturals."
///     }
///     fn pour(&self, _seed: u64, count: usize) -> Vec<Json> {
///         (0..count).map(|n| json!({ "n": n, "square": n * n })).collect()
///     }
/// }
///
/// let well: Box<dyn Well> = Box::new(Squares);
/// assert_eq!(well.pour(7, 2)[..], well.pour(7, 3)[..2]);
/// ```
pub trait Well {
    /// Returns the dataset's name.
    fn name(&self) -> &str;
    /// Returns the dataset's one-line description.
    fn about(&self) -> &str;
    /// Pours count seeded rows, identical for identical seeds, earlier rows first.
    fn pour(&self, seed: u64, count: usize) -> Vec<Json>;
}

/// Shuffles rows into the seed's order, so a short pour samples an enumeration fairly.
pub fn shuffle(rows: &mut [Json], seed: u64) {
    let mut rng = Rng::new(seed);
    for i in (1..rows.len()).rev() {
        let j = rng.below(i + 1);
        rows.swap(i, j);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::json;

    #[test]
    fn shuffle_replays_and_reorders() {
        let fresh = || (0..32).map(|n| json!({ "n": n })).collect::<Vec<Json>>();
        let (mut a, mut b, straight) = (fresh(), fresh(), fresh());
        shuffle(&mut a, 7);
        shuffle(&mut b, 7);
        assert_eq!(a, b);
        assert_ne!(a, straight);
        let mut sorted = a.clone();
        sorted.sort_by_key(|row| row["n"].as_i64());
        assert_eq!(sorted, straight);
    }

    #[test]
    fn different_seeds_deal_different_orders() {
        let fresh = || (0..32).map(|n| json!({ "n": n })).collect::<Vec<Json>>();
        let (mut a, mut b) = (fresh(), fresh());
        shuffle(&mut a, 1);
        shuffle(&mut b, 2);
        assert_ne!(a, b);
    }
}
