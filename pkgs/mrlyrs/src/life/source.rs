use crate::core::error::{value_error, Result};
use crate::core::rng::Rng;
use crate::math::bang::Code;
use crate::math::counts;
use crate::math::two::{self, census};
use crate::num::prime;
use crate::num::series;

const DIM: usize = 2;
const BASE: usize = 2;

/// A named source of neighbor-count values.
///
/// | Source | OEIS |
/// |---|---|
/// | Evens | A005843 |
/// | Odds | A005408 |
/// | Primes | A000040 |
/// | Binary | A000079 |
/// | Fibonacci | A000045 |
/// | GridSquares | A016754 |
///
/// Random, the other mrly families and the code families carry no OEIS id.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Source {
    /// The even numbers.
    Evens,
    /// The odd numbers.
    Odds,
    /// The random subset its seed draws.
    Random(u64),
    /// The primes.
    Primes,
    /// The powers of two.
    Binary,
    /// The Fibonacci numbers.
    Fibonacci,
    /// The squares of the odd numbers.
    GridSquares,
    /// The carpet fill counts.
    CarpetFills,
    /// The carpet void counts.
    CarpetVoids,
    /// The net fill counts.
    NetFills,
    /// The net void counts.
    NetVoids,
    /// The H-tree fill counts.
    TreeFills,
    /// The H-tree void counts.
    TreeVoids,
    /// The void design fill counts.
    VoidFills,
    /// The void design void counts.
    VoidVoids,
    /// The point fill counts.
    PointFills,
    /// The point void counts.
    PointVoids,
    /// The dust fill counts.
    DustFills,
    /// The dust void counts.
    DustVoids,
    /// The H-line fill counts.
    LineFills,
    /// The H-line void counts.
    LineVoids,
    /// The star fill counts.
    StarFills,
    /// The star void counts.
    StarVoids,
    /// The fill counts of a coded design.
    CodeFills(u128),
    /// The void counts of a coded design.
    CodeVoids(u128),
}

impl Source {
    /// Returns the sequence's parseable name, the one string that regenerates it.
    pub fn name(self) -> String {
        let fixed = match self {
            Source::Evens => "evens",
            Source::Odds => "odds",
            Source::Random(seed) => return format!("random_{seed}"),
            Source::Primes => "primes",
            Source::Binary => "binary",
            Source::Fibonacci => "fibonacci",
            Source::GridSquares => "grid_squares",
            Source::CarpetFills => "carpet_fills",
            Source::CarpetVoids => "carpet_voids",
            Source::NetFills => "net_fills",
            Source::NetVoids => "net_voids",
            Source::TreeFills => "tree_fills",
            Source::TreeVoids => "tree_voids",
            Source::VoidFills => "void_fills",
            Source::VoidVoids => "void_voids",
            Source::PointFills => "point_fills",
            Source::PointVoids => "point_voids",
            Source::DustFills => "dust_fills",
            Source::DustVoids => "dust_voids",
            Source::LineFills => "line_fills",
            Source::LineVoids => "line_voids",
            Source::StarFills => "star_fills",
            Source::StarVoids => "star_voids",
            Source::CodeFills(code) => return format!("code_fills_{code}"),
            Source::CodeVoids(code) => return format!("code_voids_{code}"),
        };
        fixed.to_string()
    }
    /// Parses a sequence name, or an error for an unknown one.
    pub fn parse(name: &str) -> Result<Source> {
        let lower = name.to_lowercase();
        if HEADS.iter().any(|head| lower.starts_with(head)) {
            return match Source::read(&lower) {
                Some((seq, "")) => Ok(seq),
                _ => value_error(format!("sequence {lower:?} wants a plain number.")),
            };
        }
        let seq = match lower.as_str() {
            "evens" => Source::Evens,
            "odds" => Source::Odds,
            "primes" | "prime" => Source::Primes,
            "binary" => Source::Binary,
            "fibonacci" | "fib" => Source::Fibonacci,
            "grid_squares" | "grid" => Source::GridSquares,
            "carpet_fills" => Source::CarpetFills,
            "carpet_voids" => Source::CarpetVoids,
            "net_fills" => Source::NetFills,
            "net_voids" => Source::NetVoids,
            "tree_fills" => Source::TreeFills,
            "tree_voids" => Source::TreeVoids,
            "void_fills" => Source::VoidFills,
            "void_voids" => Source::VoidVoids,
            "point_fills" => Source::PointFills,
            "point_voids" => Source::PointVoids,
            "dust_fills" => Source::DustFills,
            "dust_voids" => Source::DustVoids,
            "line_fills" => Source::LineFills,
            "line_voids" => Source::LineVoids,
            "star_fills" => Source::StarFills,
            "star_voids" => Source::StarVoids,
            other => {
                let alias = Source::all()
                    .into_iter()
                    .find(|s| s.oeis().is_some_and(|id| id.eq_ignore_ascii_case(other)));
                match alias {
                    Some(seq) => seq,
                    None => return value_error(format!("unknown sequence {other:?}.")),
                }
            }
        };
        Ok(seq)
    }
    /// Reads a canonical name off the front of the text, returning the tail left over.
    pub fn read(text: &str) -> Option<(Source, &str)> {
        if let Some(rest) = text.strip_prefix("random_") {
            let (seed, tail) = seed_of(rest)?;
            return Some((Source::Random(u64::try_from(seed).ok()?), tail));
        }
        if let Some(rest) = text.strip_prefix("code_fills_") {
            let (code, tail) = seed_of(rest)?;
            return Some((Source::CodeFills(code), tail));
        }
        if let Some(rest) = text.strip_prefix("code_voids_") {
            let (code, tail) = seed_of(rest)?;
            return Some((Source::CodeVoids(code), tail));
        }
        let fixed: Vec<Source> = Source::all()
            .into_iter()
            .filter(|seq| !seq.is_random())
            .collect();
        let names: Vec<String> = fixed.iter().map(|seq| seq.name()).collect();
        let (i, rest) = crate::math::name::text::longest(text, &names)?;
        Some((fixed[i], rest))
    }
    /// Returns every fixed sequence, the seeded and coded families excluded.
    pub fn all() -> [Source; 23] {
        [
            Source::Evens,
            Source::Odds,
            Source::Random(0),
            Source::Primes,
            Source::Binary,
            Source::Fibonacci,
            Source::GridSquares,
            Source::CarpetFills,
            Source::CarpetVoids,
            Source::NetFills,
            Source::NetVoids,
            Source::TreeFills,
            Source::TreeVoids,
            Source::VoidFills,
            Source::VoidVoids,
            Source::PointFills,
            Source::PointVoids,
            Source::DustFills,
            Source::DustVoids,
            Source::LineFills,
            Source::LineVoids,
            Source::StarFills,
            Source::StarVoids,
        ]
    }
    /// Returns the six number sequences, the random one listed under seed zero.
    pub fn numbers() -> [Source; 6] {
        [
            Source::Evens,
            Source::Odds,
            Source::Random(0),
            Source::Primes,
            Source::Binary,
            Source::Fibonacci,
        ]
    }
    /// Returns the seventeen mrly design families: the grid, the four classics and their antis.
    pub fn designs() -> [Source; 17] {
        [
            Source::GridSquares,
            Source::CarpetFills,
            Source::CarpetVoids,
            Source::NetFills,
            Source::NetVoids,
            Source::TreeFills,
            Source::TreeVoids,
            Source::VoidFills,
            Source::VoidVoids,
            Source::PointFills,
            Source::PointVoids,
            Source::DustFills,
            Source::DustVoids,
            Source::LineFills,
            Source::LineVoids,
            Source::StarFills,
            Source::StarVoids,
        ]
    }
    /// Returns the sequence's OEIS id, or None off the encyclopedia.
    pub fn oeis(self) -> Option<&'static str> {
        match self {
            Source::Evens => Some("A005843"),
            Source::Odds => Some("A005408"),
            Source::Primes => Some("A000040"),
            Source::Binary => Some("A000079"),
            Source::Fibonacci => Some("A000045"),
            Source::GridSquares => Some("A016754"),
            _ => None,
        }
    }
    /// Returns whether the sequence is a seeded random draw.
    pub fn is_random(self) -> bool {
        matches!(self, Source::Random(_))
    }
}

const HEADS: [&str; 3] = ["random_", "code_fills_", "code_voids_"];

fn seed_of(text: &str) -> Option<(u128, &str)> {
    let end = text.bytes().take_while(u8::is_ascii_digit).count();
    if end == 0 || (end > 1 && text.starts_with('0')) {
        return None;
    }
    Some((text[..end].parse().ok()?, &text[end..]))
}

fn random_subset(seed: u64, limit: usize) -> Vec<usize> {
    let mut rng = Rng::new(seed);
    let mut options: Vec<usize> = (0..=limit).collect();
    let count = 1 + rng.below(options.len());
    for i in 0..count {
        let j = i + rng.below(options.len() - i);
        options.swap(i, j);
    }
    let mut out = options[..count].to_vec();
    out.sort_unstable();
    out
}

fn mrly_sequence(limit: usize, count_of: impl Fn(usize) -> Result<usize>) -> Result<Vec<usize>> {
    let mut out = Vec::new();
    let mut number = 1;
    loop {
        let value = count_of(number)?;
        if value > limit {
            break;
        }
        if !out.contains(&value) {
            out.push(value);
        }
        number += 2;
        if number > limit + 3 {
            break;
        }
    }
    out.sort_unstable();
    Ok(out)
}

/// Generates the sequence's values up to the limit.
pub fn sequence(seq: Source, limit: usize) -> Result<Vec<usize>> {
    match seq {
        Source::Evens => Ok(series::evens(limit)),
        Source::Odds => Ok(series::odds(limit)),
        Source::Random(seed) => Ok(random_subset(seed, limit)),
        Source::Primes => Ok(prime::primes(limit)),
        Source::Binary => Ok(series::binary(limit)),
        Source::Fibonacci => Ok(series::fibonacci(limit)),
        Source::GridSquares => mrly_sequence(limit, |n| Ok(n * n)),
        Source::CarpetFills => mrly_sequence(limit, |n| Ok(census::fills(&two::carpet(n, 1)?))),
        Source::CarpetVoids => mrly_sequence(limit, |n| Ok(census::voids(&two::carpet(n, 1)?))),
        Source::NetFills => mrly_sequence(limit, |n| Ok(census::fills(&two::net(n, 1)?))),
        Source::NetVoids => mrly_sequence(limit, |n| Ok(census::voids(&two::net(n, 1)?))),
        Source::TreeFills => mrly_sequence(limit, |n| Ok(census::fills(&two::htree(n, 1)?))),
        Source::TreeVoids => mrly_sequence(limit, |n| Ok(census::voids(&two::htree(n, 1)?))),
        Source::VoidFills => mrly_sequence(limit, |n| Ok(census::fills(&two::void(n, 1)?))),
        Source::VoidVoids => mrly_sequence(limit, |n| Ok(census::voids(&two::void(n, 1)?))),
        Source::PointFills => mrly_sequence(limit, |n| Ok(census::fills(&two::point(n, 1)?))),
        Source::PointVoids => mrly_sequence(limit, |n| Ok(census::voids(&two::point(n, 1)?))),
        Source::DustFills => mrly_sequence(limit, |n| Ok(census::fills(&two::dust(n, 1)?))),
        Source::DustVoids => mrly_sequence(limit, |n| Ok(census::voids(&two::dust(n, 1)?))),
        Source::LineFills => mrly_sequence(limit, |n| Ok(census::fills(&two::hline(n, 1)?))),
        Source::LineVoids => mrly_sequence(limit, |n| Ok(census::voids(&two::hline(n, 1)?))),
        Source::StarFills => mrly_sequence(limit, |n| Ok(census::fills(&two::star(n, 1)?))),
        Source::StarVoids => mrly_sequence(limit, |n| Ok(census::voids(&two::star(n, 1)?))),
        Source::CodeFills(code) => mrly_sequence(limit, |n| {
            Ok(counts::fill(Code::from(code), n, DIM, 1, BASE)? as usize)
        }),
        Source::CodeVoids(code) => mrly_sequence(limit, |n| {
            Ok(counts::void(Code::from(code), n, DIM, 1, BASE)? as usize)
        }),
    }
}

/// Returns the sequence up to max_neighbors, keeping zeros and ones only on request.
pub fn counts(
    seq: Source,
    max_neighbors: usize,
    include_zeros: bool,
    include_ones: bool,
) -> Result<Vec<usize>> {
    let raw = sequence(seq, max_neighbors)?;
    Ok(raw
        .into_iter()
        .filter(|&x| (x != 0 || include_zeros) && (x != 1 || include_ones))
        .collect())
}

/// The neighbor counts one side of a rule fires on.
///
/// A list spells its counts outright and holds them all; a drawn side names the
/// sequence behind them, so any budget of neighbors rebuilds the same counts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Counts {
    /// The counts listed outright.
    List(Vec<usize>),
    /// The counts a named sequence lays down inside the budget.
    Drawn {
        /// The sequence behind the counts.
        seq: Source,
        /// Whether zero stays in the counts.
        zeros: bool,
        /// Whether one stays in the counts.
        ones: bool,
    },
}

impl Counts {
    /// Builds the counts a sequence lays down, keeping zeros and ones on request.
    pub fn drawn(seq: Source, zeros: bool, ones: bool) -> Counts {
        Counts::Drawn { seq, zeros, ones }
    }
    /// Returns the counts, a drawn side resolved against the mask's neighbor budget.
    pub fn values(&self, budget: usize) -> Result<Vec<usize>> {
        match self {
            Counts::List(list) => {
                let mut out = list.clone();
                out.sort_unstable();
                out.dedup();
                Ok(out)
            }
            Counts::Drawn { seq, zeros, ones } => counts(*seq, budget, *zeros, *ones),
        }
    }
}

impl From<Vec<usize>> for Counts {
    fn from(list: Vec<usize>) -> Counts {
        Counts::List(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn number_sequences_clip_to_limit() {
        assert_eq!(sequence(Source::Primes, 8).unwrap(), vec![2, 3, 5, 7]);
        assert_eq!(sequence(Source::Binary, 8).unwrap(), vec![1, 2, 4, 8]);
    }
    #[test]
    fn grid_squares_walk() {
        assert_eq!(sequence(Source::GridSquares, 8).unwrap(), vec![1]);
        assert_eq!(sequence(Source::GridSquares, 30).unwrap(), vec![1, 9, 25]);
    }
    #[test]
    fn counts_can_drop_zero_and_one() {
        let c = counts(Source::Evens, 8, false, true).unwrap();
        assert_eq!(c, vec![2, 4, 6, 8]);
        let c = counts(Source::Evens, 8, true, true).unwrap();
        assert_eq!(c, vec![0, 2, 4, 6, 8]);
    }
    #[test]
    fn parse_roundtrips() {
        for s in Source::all() {
            assert_eq!(Source::parse(&s.name()).unwrap(), s);
        }
    }
    #[test]
    fn random_draws_a_seeded_sorted_subset() {
        let a = sequence(Source::Random(42), 8).unwrap();
        let b = sequence(Source::Random(42), 8).unwrap();
        assert_eq!(a, b);
        assert!(!a.is_empty() && a.len() <= 9);
        assert!(a.windows(2).all(|w| w[0] < w[1]));
        assert!(a.iter().all(|&x| x <= 8));
        assert_ne!(sequence(Source::Random(43), 64).unwrap(), a);
    }
    #[test]
    fn a_random_name_regenerates_its_counts() {
        let seq = Source::Random(4848495);
        assert_eq!(seq.name(), "random_4848495");
        let back = Source::parse(&seq.name()).unwrap();
        assert_eq!(back, seq);
        assert_eq!(sequence(back, 48).unwrap(), sequence(seq, 48).unwrap());
        assert!(Source::parse("random").is_err());
        assert!(Source::parse("random_").is_err());
        assert!(Source::parse("random_007").is_err());
    }
    #[test]
    fn read_leaves_the_tail_behind() {
        assert_eq!(
            Source::read("fibonacciz_s3"),
            Some((Source::Fibonacci, "z_s3"))
        );
        assert_eq!(
            Source::read("grid_squares_sgrid_squares"),
            Some((Source::GridSquares, "_sgrid_squares"))
        );
        assert_eq!(
            Source::read("random_12_s3"),
            Some((Source::Random(12), "_s3"))
        );
        assert_eq!(Source::read("fib"), None);
        assert_eq!(Source::read("3"), None);
    }
    #[test]
    fn read_takes_the_longest_name_not_the_first() {
        for short in Source::all() {
            for long in Source::all() {
                if short == long || short.is_random() || long.is_random() {
                    continue;
                }
                if !long.name().starts_with(&short.name()) {
                    continue;
                }
                let name = long.name();
                assert_eq!(Source::read(&name), Some((long, "")), "{name}");
            }
        }
        assert_eq!(
            Source::read("code_fills_12"),
            Some((Source::CodeFills(12), ""))
        );
        assert_eq!(
            Source::read("random_4848495z_s3"),
            Some((Source::Random(4848495), "z_s3"))
        );
    }
    #[test]
    fn every_fixed_name_reads_back_whole() {
        for seq in Source::all() {
            if seq.is_random() {
                continue;
            }
            let name = seq.name();
            assert_eq!(Source::read(&name), Some((seq, "")), "{name}");
        }
    }
    #[test]
    fn tiers_split_the_fixed_sequences() {
        assert!(Source::numbers().iter().any(|s| s.is_random()));
        assert!(Source::designs().contains(&Source::GridSquares));
        let mut both = Source::numbers().to_vec();
        both.extend(Source::designs());
        assert_eq!(both, Source::all().to_vec());
    }
    #[test]
    fn oeis_aliases_parse_either_case() {
        assert_eq!(Source::parse("A005843").unwrap(), Source::Evens);
        assert_eq!(Source::parse("a000045").unwrap(), Source::Fibonacci);
        assert_eq!(Source::parse("A016754").unwrap(), Source::GridSquares);
        assert!(Source::parse("A999999").is_err());
    }
    #[test]
    fn oeis_ids_roundtrip_through_parse() {
        let mut listed = 0;
        for s in Source::all() {
            if let Some(id) = s.oeis() {
                assert_eq!(Source::parse(id).unwrap(), s);
                listed += 1;
            }
        }
        assert_eq!(listed, 6);
        assert_eq!(Source::Random(0).oeis(), None);
        assert_eq!(Source::CarpetFills.oeis(), None);
        assert_eq!(Source::CodeFills(7).oeis(), None);
    }
    #[test]
    fn names_stay_canonical() {
        let expected = [
            "evens",
            "odds",
            "random_0",
            "primes",
            "binary",
            "fibonacci",
            "grid_squares",
            "carpet_fills",
            "carpet_voids",
            "net_fills",
            "net_voids",
            "tree_fills",
            "tree_voids",
            "void_fills",
            "void_voids",
        ];
        for (s, want) in Source::all().into_iter().zip(expected) {
            assert_eq!(s.name(), want);
        }
    }
    #[test]
    fn code_sequences_match_formulas() {
        use crate::math::counts;
        for code in [1u128, 7, 14, 15] {
            let seq = sequence(Source::CodeFills(code), 50).unwrap();
            let expected: Vec<usize> = {
                let mut v = Vec::new();
                let mut n = 1;
                while n <= 53 {
                    let f = counts::fill(Code::from(code), n, 2, 1, 2).unwrap() as usize;
                    if f <= 50 && !v.contains(&f) {
                        v.push(f);
                    }
                    n += 2;
                }
                v.sort_unstable();
                v
            };
            assert_eq!(seq, expected, "code {code}");
        }
    }
    #[test]
    fn code_name_roundtrips() {
        let s = Source::CodeVoids(9);
        assert_eq!(s.name(), "code_voids_9");
        assert_eq!(Source::parse(&s.name()).unwrap(), s);
        assert_eq!(Source::parse("code_fills_7").unwrap(), Source::CodeFills(7));
        assert!(Source::parse("code_fills_x").is_err());
    }
    #[test]
    fn counts_carry_a_list_or_a_sequence() {
        let listed = Counts::from(vec![3, 3, 1]);
        assert_eq!(listed.values(8).unwrap(), vec![1, 3]);
        let drawn = Counts::drawn(Source::Fibonacci, false, false);
        assert_eq!(drawn.values(8).unwrap(), vec![2, 3, 5, 8]);
        let wide = Counts::drawn(Source::Fibonacci, true, true);
        assert_eq!(wide.values(24).unwrap(), vec![0, 1, 2, 3, 5, 8, 13, 21]);
    }
    #[test]
    fn refuses_counts() {
        assert!(counts(Source::CodeFills(1 << 20), 8, true, true).is_err());
        assert!(counts(Source::CodeVoids(1 << 20), 8, true, true).is_err());
        assert!(Counts::drawn(Source::CodeFills(1 << 20), false, false)
            .values(8)
            .is_err());
        assert_eq!(
            counts(Source::Evens, 0, false, false).unwrap(),
            Vec::<usize>::new()
        );
    }
}
