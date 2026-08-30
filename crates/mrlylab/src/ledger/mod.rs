/// The markdown page the ledger renders.
pub mod markdown;
/// The measures and their cost classes.
pub mod measure;
/// The curated OEIS records and the identification against them.
pub mod records;
/// The term generators and the closed forms.
pub mod terms;

pub use markdown::markdown;
pub use measure::{Cost, Measure};
pub use records::{identify, Record, RECORDS};
pub use terms::{closed, fill_polynomial, terms};

use mrlycore::errors::{value_error, Result};
use mrlymath::bang::{baseq, Code};
use mrlymath::name::{Bang, Named};
use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

type Walked = BTreeMap<(usize, usize), &'static [Code]>;

/// The dimension and base pairs the ledger walks.
pub const SPACES: [(usize, usize); 9] = [
    (1, 2),
    (2, 2),
    (3, 2),
    (4, 2),
    (1, 3),
    (2, 3),
    (1, 4),
    (2, 4),
    (1, 5),
];

/// The cells a catalog row may render for one term.
pub const BUDGET: u128 = 500_000;

/// The terms a catalog row holds.
pub const TERMS: usize = 8;

/// The index a sequence runs along.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Axis {
    /// The fractal level `L` from 1, at side `n = max(q, 3)`.
    Level,
    /// The odd side `n = 2k - 1` from `k = 2`, at level 1.
    Side,
}

impl Axis {
    /// Both axes, level first.
    pub const ALL: [Axis; 2] = [Axis::Level, Axis::Side];
    /// Returns the axis's one-word name.
    pub fn slug(self) -> &'static str {
        match self {
            Axis::Level => "level",
            Axis::Side => "side",
        }
    }
    /// Parses a one-word name back into its axis, or an error for any other word.
    pub fn parse(slug: &str) -> Result<Axis> {
        match slug {
            "level" => Ok(Axis::Level),
            "side" => Ok(Axis::Side),
            other => value_error(format!("unknown axis {other:?}.")),
        }
    }
    /// Returns the ledger index of the first term: the level 1 or the `k` of side 3.
    pub fn start(self) -> i32 {
        match self {
            Axis::Level => 1,
            Axis::Side => 2,
        }
    }
    /// Returns the side number and level of the term at the index.
    pub fn place(self, index: usize, number: usize) -> (usize, u32) {
        match self {
            Axis::Level => (number, index as u32 + 1),
            Axis::Side => (2 * index + 3, 1),
        }
    }
}

/// The status a claim carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tag {
    /// Proved on this tree.
    Proved,
    /// Checked against a record or a second generator.
    Verified,
    /// Stated and not yet checked.
    Conjecture,
    /// Checked and found false.
    Refuted,
}

impl Tag {
    /// Returns the tag's capitalised word.
    pub fn text(self) -> &'static str {
        match self {
            Tag::Proved => "Proved",
            Tag::Verified => "Verified",
            Tag::Conjecture => "Conjecture",
            Tag::Refuted => "Refuted",
        }
    }
}

/// A design sequence's address: the design, the measure and the axis.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Key {
    /// The design's code.
    pub code: Code,
    /// The design's dimension.
    pub dimension: usize,
    /// The numeral base of the corners.
    pub base: usize,
    /// The reading taken.
    pub measure: Measure,
    /// The index the reading runs along.
    pub axis: Axis,
}

impl Key {
    /// Pins a design sequence to its address.
    pub const fn new(
        code: Code,
        dimension: usize,
        base: usize,
        measure: Measure,
        axis: Axis,
    ) -> Key {
        Key {
            code,
            dimension,
            base,
            measure,
            axis,
        }
    }
    /// Returns the sequence's name, the design's name dotted with the measure and the axis.
    ///
    /// ```
    /// use mrlylab::ledger::{Axis, Key, Measure};
    /// let key = Key::new(23, 3, 2, Measure::Surface, Axis::Level);
    /// assert_eq!(key.name(), "mrly_bang_d3_23.surface.level");
    /// ```
    pub fn name(&self) -> String {
        format!(
            "{}.{}.{}",
            self.design().to_str(),
            self.measure.slug(),
            self.axis.slug()
        )
    }
    /// Returns the design pinned to its dimension and base.
    pub fn design(&self) -> Bang {
        Bang::new(self.code, self.dimension, self.base)
    }
    /// Returns the side number the level axis runs at.
    pub fn number(&self) -> usize {
        self.base.max(3)
    }
}

/// A closed form of a sequence.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Closed {
    /// `f^L`.
    Power(u128),
    /// `g^L - f^L`.
    Difference(u128, u128),
    /// A polynomial in `k` by rising power, at side `n = 2k - 1`.
    Polynomial(Vec<i128>),
    /// `a(L) = c[0] a(L-1) + c[1] a(L-2) + ...`.
    Recurrence(Vec<i128>),
}

fn signed(text: &mut String, coefficient: i128, first: bool) {
    if first {
        if coefficient < 0 {
            text.push('-');
        }
    } else {
        text.push_str(if coefficient < 0 { " - " } else { " + " });
    }
}

impl Closed {
    /// Spells the closed form.
    pub fn text(&self) -> String {
        match self {
            Closed::Power(f) => format!("{f}^L"),
            Closed::Difference(g, f) => format!("{g}^L - {f}^L"),
            Closed::Polynomial(coefficients) => {
                let mut text = String::new();
                for (power, &c) in coefficients.iter().enumerate().rev() {
                    if c == 0 {
                        continue;
                    }
                    let first = text.is_empty();
                    signed(&mut text, c, first);
                    if c.abs() != 1 || power == 0 {
                        text.push_str(&c.abs().to_string());
                    }
                    match power {
                        0 => {}
                        1 => text.push('k'),
                        _ => text.push_str(&format!("k^{power}")),
                    }
                }
                if text.is_empty() {
                    text.push('0');
                }
                text
            }
            Closed::Recurrence(coefficients) => {
                let mut text = String::new();
                for (back, &c) in coefficients.iter().enumerate() {
                    if c == 0 {
                        continue;
                    }
                    let first = text.is_empty();
                    signed(&mut text, c, first);
                    if c.abs() != 1 {
                        text.push_str(&c.abs().to_string());
                        text.push(' ');
                    }
                    text.push_str(&format!("a(L-{})", back + 1));
                }
                if text.is_empty() {
                    text.push('0');
                }
                format!("a(L) = {text}")
            }
        }
    }
}

/// A cost tier of the catalog.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tier {
    /// The closed measures on both axes.
    Closed,
    /// The profile measures on both axes.
    Convolved,
    /// The grid measures on the side axis.
    SideGrid,
    /// The grid measures on the level axis.
    LevelGrid,
}

impl Tier {
    /// Every tier, cheapest first.
    pub const ALL: [Tier; 4] = [
        Tier::Closed,
        Tier::Convolved,
        Tier::SideGrid,
        Tier::LevelGrid,
    ];
    /// Returns the tier's one-word name.
    pub fn slug(self) -> &'static str {
        match self {
            Tier::Closed => "closed",
            Tier::Convolved => "convolved",
            Tier::SideGrid => "side",
            Tier::LevelGrid => "level",
        }
    }
    /// Parses a one-word name back into its tier, or an error for any other word.
    pub fn parse(slug: &str) -> Result<Tier> {
        Tier::ALL
            .into_iter()
            .find(|tier| tier.slug() == slug)
            .map_or_else(|| value_error(format!("unknown tier {slug:?}.")), Ok)
    }
    fn cost(self) -> Cost {
        match self {
            Tier::Closed => Cost::Closed,
            Tier::Convolved => Cost::Convolved,
            _ => Cost::Grid,
        }
    }
    fn axes(self) -> &'static [Axis] {
        match self {
            Tier::SideGrid => &[Axis::Side],
            Tier::LevelGrid => &[Axis::Level],
            _ => &Axis::ALL,
        }
    }
}

/// One row of the catalog: a design sequence with its terms, its closed form and its record.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sequence {
    /// The sequence's address.
    pub key: Key,
    /// The first terms.
    pub terms: Vec<i128>,
    /// Whether the cell budget or a u128 stopped the terms short.
    pub capped: bool,
    /// The closed form, when one is known.
    pub closed: Option<Closed>,
    /// The record the terms match, with the record's index less the ledger's.
    pub record: Option<(&'static Record, i32)>,
    /// The status of the match: the record's when the record names this key, else a collision to explain.
    pub tag: Option<Tag>,
}

impl Sequence {
    fn mentions(&self, needle: &str) -> bool {
        self.key.name().contains(needle)
            || self.record.is_some_and(|(record, _)| {
                record.id.to_lowercase().contains(needle)
                    || record.name.to_lowercase().contains(needle)
            })
    }
}

/// Returns the designs of a dimension and base, the least code of every orbit, walked once and cached, or an error past the walk limit.
///
/// ```
/// assert_eq!(mrlylab::ledger::designs(2, 2).unwrap(), [0, 1, 3, 6, 7, 15]);
/// assert_eq!(mrlylab::ledger::designs(2, 3).unwrap().len(), 26);
/// ```
pub fn designs(dimension: usize, base: usize) -> Result<&'static [Code]> {
    static CACHE: OnceLock<Mutex<Walked>> = OnceLock::new();
    let cache = CACHE.get_or_init(|| Mutex::new(BTreeMap::new()));
    let mut guard = cache.lock().expect("the design cache is not poisoned");
    if let Some(codes) = guard.get(&(dimension, base)) {
        return Ok(codes);
    }
    let codes: Vec<Code> = baseq::representatives(base, dimension)?
        .into_iter()
        .map(|(code, _)| code)
        .collect();
    let leaked: &'static [Code] = Box::leak(codes.into_boxed_slice());
    guard.insert((dimension, base), leaked);
    Ok(leaked)
}

/// Lists every key of a tier: the designs of every space, the measures of the tier's cost that apply, on the tier's axes.
pub fn keys(tier: Tier) -> Vec<Key> {
    let mut out = Vec::new();
    for (dimension, base) in SPACES {
        let codes = designs(dimension, base).expect("the ledger spaces are walkable");
        for &code in codes {
            for measure in Measure::ALL {
                if measure.cost() != tier.cost() || !measure.applies(dimension, base) {
                    continue;
                }
                for &axis in tier.axes() {
                    out.push(Key::new(code, dimension, base, measure, axis));
                }
            }
        }
    }
    out
}

fn attach(key: &Key, terms: &[i128]) -> Option<(&'static Record, i32)> {
    if terms.len() < 4 || terms.iter().all(|&term| term == terms[0]) {
        return None;
    }
    let found = identify(terms);
    found
        .iter()
        .find(|(record, _)| record.key == Some(*key))
        .or(found.first())
        .map(|&(record, shift)| (record, shift - key.axis.start()))
}

/// Reads one design sequence: its terms within the budget, its closed form and the record it matches.
pub fn sequence(key: &Key, count: usize, cells: u128) -> Result<Sequence> {
    let (terms, capped) = terms(key, count, cells)?;
    let closed = closed(key)?;
    let record = attach(key, &terms);
    let tag = record.map(|(record, _)| {
        if record.key == Some(*key) {
            record.status
        } else {
            Tag::Conjecture
        }
    });
    Ok(Sequence {
        key: *key,
        terms,
        capped,
        closed,
        record,
        tag,
    })
}

/// Reads every sequence of a tier at the count of terms, within the standing cell budget.
pub fn catalog(tier: Tier, count: usize) -> Vec<Sequence> {
    keys(tier)
        .iter()
        .filter_map(|key| sequence(key, count, BUDGET).ok())
        .collect()
}

/// Parses integers separated by commas or spaces, or none when a token is not one.
pub fn numbers(text: &str) -> Option<Vec<i128>> {
    let tokens: Vec<&str> = text
        .split(|c: char| c == ',' || c.is_whitespace())
        .filter(|token| !token.is_empty())
        .collect();
    if tokens.is_empty() {
        return None;
    }
    tokens.iter().map(|token| token.parse().ok()).collect()
}

/// Finds the catalog rows a query names: the rows holding the typed terms as a window, or the rows whose name or record holds the typed fragment.
pub fn search(catalog: &[Sequence], query: &str) -> Vec<usize> {
    let query = query.trim();
    if query.is_empty() {
        return (0..catalog.len()).collect();
    }
    if let Some(window) = numbers(query) {
        return catalog
            .iter()
            .enumerate()
            .filter(|(_, row)| {
                row.terms
                    .windows(window.len())
                    .any(|w| w == window.as_slice())
            })
            .map(|(index, _)| index)
            .collect();
    }
    let needle = query.to_lowercase();
    catalog
        .iter()
        .enumerate()
        .filter(|(_, row)| row.mentions(&needle))
        .map(|(index, _)| index)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_classics_read_their_records() {
        let carpet = sequence(&Key::new(7, 2, 2, Measure::Fills, Axis::Side), 4, BUDGET).unwrap();
        assert_eq!(carpet.terms, [8, 21, 40, 65]);
        assert_eq!(carpet.record.map(|(r, s)| (r.id, s)), Some(("A000567", 0)));
        assert_eq!(carpet.tag, Some(Tag::Proved));
        assert_eq!(carpet.closed.unwrap().text(), "3k^2 - 2k");
        let tree = sequence(&Key::new(3, 2, 2, Measure::Fills, Axis::Side), 4, BUDGET).unwrap();
        assert_eq!(tree.terms[..3], [6, 15, 28]);
        assert_eq!(tree.record.map(|(r, s)| (r.id, s)), Some(("A000384", 0)));
        let sponge = sequence(
            &Key::new(23, 3, 2, Measure::Surface, Axis::Level),
            3,
            BUDGET,
        )
        .unwrap();
        assert_eq!(sponge.terms, [72, 1056, 18048]);
        assert_eq!(
            sponge.closed.unwrap().text(),
            "a(L) = 28 a(L-1) - 160 a(L-2)"
        );
        let slice = sequence(
            &Key::new(23, 3, 2, Measure::Triangles, Axis::Level),
            8,
            BUDGET,
        )
        .unwrap();
        assert_eq!(slice.terms, [42, 306, 2250, 16578]);
        assert!(slice.capped);
        assert_eq!(slice.record.map(|(r, s)| (r.id, s)), Some(("A299916", 1)));
        let void = sequence(&Key::new(9, 2, 2, Measure::Voids, Axis::Side), 3, BUDGET).unwrap();
        assert_eq!(void.closed.unwrap().text(), "2k^2 - 2k");
        assert_eq!(void.terms, [4, 12, 24]);
    }

    #[test]
    fn the_keys_of_the_closed_tier_cover_every_space() {
        let closed = keys(Tier::Closed);
        assert_eq!(closed.len(), 1282 * 3 * 2);
        assert_eq!(keys(Tier::Convolved).len(), (1282 - 3 - 4 - 6 - 8) * 2 * 2);
        assert!(closed.iter().all(|key| key.measure.cost() == Cost::Closed));
        assert!(designs(3, 3).is_err());
    }

    #[test]
    fn the_search_finds_terms_and_names() {
        let rows: Vec<Sequence> = [
            Key::new(7, 2, 2, Measure::Fills, Axis::Level),
            Key::new(7, 2, 2, Measure::Surface, Axis::Level),
            Key::new(23, 3, 2, Measure::Fills, Axis::Level),
        ]
        .iter()
        .map(|key| sequence(key, 5, BUDGET).unwrap())
        .collect();
        assert_eq!(search(&rows, "64, 512"), [0]);
        assert_eq!(search(&rows, "80 496"), [1]);
        assert_eq!(search(&rows, "d3_23"), [2]);
        assert_eq!(search(&rows, "A381517"), [1]);
        assert_eq!(search(&rows, "surface"), [1]);
        assert_eq!(search(&rows, ""), [0, 1, 2]);
        assert!(search(&rows, "5, 6, 7").is_empty());
        assert_eq!(numbers("1, 2 3"), Some(vec![1, 2, 3]));
        assert_eq!(numbers("1, x"), None);
    }

    #[test]
    fn the_closed_forms_spell_themselves() {
        assert_eq!(Closed::Power(8).text(), "8^L");
        assert_eq!(Closed::Difference(9, 8).text(), "9^L - 8^L");
        assert_eq!(Closed::Polynomial(vec![1, -2, 2]).text(), "2k^2 - 2k + 1");
        assert_eq!(Closed::Polynomial(vec![0, 0, 1]).text(), "k^2");
        assert_eq!(Closed::Polynomial(vec![-1, 0, -1]).text(), "-k^2 - 1");
        assert_eq!(Closed::Polynomial(vec![0]).text(), "0");
        assert_eq!(
            Closed::Recurrence(vec![11, -24]).text(),
            "a(L) = 11 a(L-1) - 24 a(L-2)"
        );
        assert_eq!(Closed::Recurrence(vec![1]).text(), "a(L) = a(L-1)");
        assert_eq!(Closed::Recurrence(vec![0]).text(), "a(L) = 0");
        assert_eq!(Axis::Side.place(0, 3), (3, 1));
        assert_eq!(Axis::Level.place(2, 3), (3, 3));
        assert_eq!(Tier::parse("side").unwrap(), Tier::SideGrid);
        assert!(Axis::parse("depth").is_err());
    }
}
