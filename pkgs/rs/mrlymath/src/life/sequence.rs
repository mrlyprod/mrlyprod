use crate::formulas::{self, classics};
use crate::two::{self, census};
use mrlycore::errors::{value_error, Result};
use mrlycore::rng::Rng;

const DIM: usize = 2;
const BASE: usize = 2;

/// A named source of neighbor-count values.
///
/// | Sequence | OEIS |
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
pub enum Sequence {
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
    /// The fill counts of a coded design.
    CodeFills(u128),
    /// The void counts of a coded design.
    CodeVoids(u128),
}

impl Sequence {
    /// Returns the sequence's parseable name, the one string that regenerates it.
    pub fn name(self) -> String {
        let fixed = match self {
            Sequence::Evens => "evens",
            Sequence::Odds => "odds",
            Sequence::Random(seed) => return format!("random_{seed}"),
            Sequence::Primes => "primes",
            Sequence::Binary => "binary",
            Sequence::Fibonacci => "fibonacci",
            Sequence::GridSquares => "grid_squares",
            Sequence::CarpetFills => "carpet_fills",
            Sequence::CarpetVoids => "carpet_voids",
            Sequence::NetFills => "net_fills",
            Sequence::NetVoids => "net_voids",
            Sequence::TreeFills => "tree_fills",
            Sequence::TreeVoids => "tree_voids",
            Sequence::VoidFills => "void_fills",
            Sequence::VoidVoids => "void_voids",
            Sequence::CodeFills(code) => return format!("code_fills_{code}"),
            Sequence::CodeVoids(code) => return format!("code_voids_{code}"),
        };
        fixed.to_string()
    }
    /// Parses a sequence name, or an error for an unknown one.
    pub fn parse(name: &str) -> Result<Sequence> {
        let lower = name.to_lowercase();
        if HEADS.iter().any(|head| lower.starts_with(head)) {
            return match Sequence::read(&lower) {
                Some((seq, "")) => Ok(seq),
                _ => value_error(format!("sequence {lower:?} wants a plain number.")),
            };
        }
        let seq = match lower.as_str() {
            "evens" => Sequence::Evens,
            "odds" => Sequence::Odds,
            "primes" | "prime" => Sequence::Primes,
            "binary" => Sequence::Binary,
            "fibonacci" | "fib" => Sequence::Fibonacci,
            "grid_squares" | "grid" => Sequence::GridSquares,
            "carpet_fills" => Sequence::CarpetFills,
            "carpet_voids" => Sequence::CarpetVoids,
            "net_fills" => Sequence::NetFills,
            "net_voids" => Sequence::NetVoids,
            "tree_fills" => Sequence::TreeFills,
            "tree_voids" => Sequence::TreeVoids,
            "void_fills" => Sequence::VoidFills,
            "void_voids" => Sequence::VoidVoids,
            other => {
                let alias = Sequence::all()
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
    pub fn read(text: &str) -> Option<(Sequence, &str)> {
        if let Some(rest) = text.strip_prefix("random_") {
            let (seed, tail) = seed_of(rest)?;
            return Some((Sequence::Random(u64::try_from(seed).ok()?), tail));
        }
        if let Some(rest) = text.strip_prefix("code_fills_") {
            let (code, tail) = seed_of(rest)?;
            return Some((Sequence::CodeFills(code), tail));
        }
        if let Some(rest) = text.strip_prefix("code_voids_") {
            let (code, tail) = seed_of(rest)?;
            return Some((Sequence::CodeVoids(code), tail));
        }
        let fixed: Vec<Sequence> = Sequence::all()
            .into_iter()
            .filter(|seq| !seq.is_random())
            .collect();
        let names: Vec<String> = fixed.iter().map(|seq| seq.name()).collect();
        let (i, rest) = crate::name::text::longest(text, &names)?;
        Some((fixed[i], rest))
    }
    /// Returns every fixed sequence, the seeded and coded families excluded.
    pub fn all() -> [Sequence; 15] {
        [
            Sequence::Evens,
            Sequence::Odds,
            Sequence::Random(0),
            Sequence::Primes,
            Sequence::Binary,
            Sequence::Fibonacci,
            Sequence::GridSquares,
            Sequence::CarpetFills,
            Sequence::CarpetVoids,
            Sequence::NetFills,
            Sequence::NetVoids,
            Sequence::TreeFills,
            Sequence::TreeVoids,
            Sequence::VoidFills,
            Sequence::VoidVoids,
        ]
    }
    /// Returns the six number sequences, the random one listed under seed zero.
    pub fn numbers() -> [Sequence; 6] {
        [
            Sequence::Evens,
            Sequence::Odds,
            Sequence::Random(0),
            Sequence::Primes,
            Sequence::Binary,
            Sequence::Fibonacci,
        ]
    }
    /// Returns the nine mrly design families.
    pub fn designs() -> [Sequence; 9] {
        [
            Sequence::GridSquares,
            Sequence::CarpetFills,
            Sequence::CarpetVoids,
            Sequence::NetFills,
            Sequence::NetVoids,
            Sequence::TreeFills,
            Sequence::TreeVoids,
            Sequence::VoidFills,
            Sequence::VoidVoids,
        ]
    }
    /// Returns the sequence's OEIS id, or None off the encyclopedia.
    pub fn oeis(self) -> Option<&'static str> {
        match self {
            Sequence::Evens => Some("A005843"),
            Sequence::Odds => Some("A005408"),
            Sequence::Primes => Some("A000040"),
            Sequence::Binary => Some("A000079"),
            Sequence::Fibonacci => Some("A000045"),
            Sequence::GridSquares => Some("A016754"),
            _ => None,
        }
    }
    /// Returns whether the sequence is a seeded random draw.
    pub fn is_random(self) -> bool {
        matches!(self, Sequence::Random(_))
    }
    fn is_number(self) -> bool {
        matches!(
            self,
            Sequence::Evens
                | Sequence::Odds
                | Sequence::Random(_)
                | Sequence::Primes
                | Sequence::Binary
                | Sequence::Fibonacci
        )
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
pub fn sequence(seq: Sequence, limit: usize) -> Result<Vec<usize>> {
    if seq.is_number() {
        return Ok(match seq {
            Sequence::Evens => classics::evens(limit),
            Sequence::Odds => classics::odds(limit),
            Sequence::Random(seed) => random_subset(seed, limit),
            Sequence::Primes => classics::primes(limit),
            Sequence::Binary => classics::binary(limit),
            Sequence::Fibonacci => classics::fibonacci(limit),
            _ => unreachable!(),
        });
    }
    match seq {
        Sequence::GridSquares => mrly_sequence(limit, |n| Ok(n * n)),
        Sequence::CarpetFills => mrly_sequence(limit, |n| Ok(census::fills(&two::carpet(n, 1)?))),
        Sequence::CarpetVoids => mrly_sequence(limit, |n| Ok(census::voids(&two::carpet(n, 1)?))),
        Sequence::NetFills => mrly_sequence(limit, |n| Ok(census::fills(&two::net(n, 1)?))),
        Sequence::NetVoids => mrly_sequence(limit, |n| Ok(census::voids(&two::net(n, 1)?))),
        Sequence::TreeFills => mrly_sequence(limit, |n| Ok(census::fills(&two::htree(n, 1)?))),
        Sequence::TreeVoids => mrly_sequence(limit, |n| Ok(census::voids(&two::htree(n, 1)?))),
        Sequence::VoidFills => mrly_sequence(limit, |n| Ok(census::fills(&two::void(n, 1)?))),
        Sequence::VoidVoids => mrly_sequence(limit, |n| Ok(census::voids(&two::void(n, 1)?))),
        Sequence::CodeFills(code) => mrly_sequence(limit, |n| {
            Ok(formulas::fill(code, n, DIM, 1, BASE)? as usize)
        }),
        Sequence::CodeVoids(code) => mrly_sequence(limit, |n| {
            Ok(formulas::void(code, n, DIM, 1, BASE)? as usize)
        }),
        _ => unreachable!(),
    }
}

/// Returns the sequence up to max_neighbors, keeping zeros and ones only on request.
pub fn counts(
    seq: Sequence,
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
        seq: Sequence,
        /// Whether zero stays in the counts.
        zeros: bool,
        /// Whether one stays in the counts.
        ones: bool,
    },
}

impl Counts {
    /// Builds the counts a sequence lays down, keeping zeros and ones on request.
    pub fn drawn(seq: Sequence, zeros: bool, ones: bool) -> Counts {
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
        assert_eq!(sequence(Sequence::Primes, 8).unwrap(), vec![2, 3, 5, 7]);
        assert_eq!(sequence(Sequence::Binary, 8).unwrap(), vec![1, 2, 4, 8]);
    }
    #[test]
    fn grid_squares_walk() {
        assert_eq!(sequence(Sequence::GridSquares, 8).unwrap(), vec![1]);
        assert_eq!(sequence(Sequence::GridSquares, 30).unwrap(), vec![1, 9, 25]);
    }
    #[test]
    fn counts_can_drop_zero_and_one() {
        let c = counts(Sequence::Evens, 8, false, true).unwrap();
        assert_eq!(c, vec![2, 4, 6, 8]);
        let c = counts(Sequence::Evens, 8, true, true).unwrap();
        assert_eq!(c, vec![0, 2, 4, 6, 8]);
    }
    #[test]
    fn parse_roundtrips() {
        for s in Sequence::all() {
            assert_eq!(Sequence::parse(&s.name()).unwrap(), s);
        }
    }
    #[test]
    fn random_draws_a_seeded_sorted_subset() {
        let a = sequence(Sequence::Random(42), 8).unwrap();
        let b = sequence(Sequence::Random(42), 8).unwrap();
        assert_eq!(a, b);
        assert!(!a.is_empty() && a.len() <= 9);
        assert!(a.windows(2).all(|w| w[0] < w[1]));
        assert!(a.iter().all(|&x| x <= 8));
        assert_ne!(sequence(Sequence::Random(43), 64).unwrap(), a);
    }
    #[test]
    fn a_random_name_regenerates_its_counts() {
        let seq = Sequence::Random(4848495);
        assert_eq!(seq.name(), "random_4848495");
        let back = Sequence::parse(&seq.name()).unwrap();
        assert_eq!(back, seq);
        assert_eq!(sequence(back, 48).unwrap(), sequence(seq, 48).unwrap());
        assert!(Sequence::parse("random").is_err());
        assert!(Sequence::parse("random_").is_err());
        assert!(Sequence::parse("random_007").is_err());
    }
    #[test]
    fn read_leaves_the_tail_behind() {
        assert_eq!(
            Sequence::read("fibonacciz_s3"),
            Some((Sequence::Fibonacci, "z_s3"))
        );
        assert_eq!(
            Sequence::read("grid_squares_sgrid_squares"),
            Some((Sequence::GridSquares, "_sgrid_squares"))
        );
        assert_eq!(
            Sequence::read("random_12_s3"),
            Some((Sequence::Random(12), "_s3"))
        );
        assert_eq!(Sequence::read("fib"), None);
        assert_eq!(Sequence::read("3"), None);
    }
    #[test]
    fn read_takes_the_longest_name_not_the_first() {
        for short in Sequence::all() {
            for long in Sequence::all() {
                if short == long || short.is_random() || long.is_random() {
                    continue;
                }
                if !long.name().starts_with(&short.name()) {
                    continue;
                }
                let name = long.name();
                assert_eq!(Sequence::read(&name), Some((long, "")), "{name}");
            }
        }
        assert_eq!(
            Sequence::read("code_fills_12"),
            Some((Sequence::CodeFills(12), ""))
        );
        assert_eq!(
            Sequence::read("random_4848495z_s3"),
            Some((Sequence::Random(4848495), "z_s3"))
        );
    }
    #[test]
    fn every_fixed_name_reads_back_whole() {
        for seq in Sequence::all() {
            if seq.is_random() {
                continue;
            }
            let name = seq.name();
            assert_eq!(Sequence::read(&name), Some((seq, "")), "{name}");
        }
    }
    #[test]
    fn tiers_split_the_fixed_sequences() {
        assert!(Sequence::numbers().iter().any(|s| s.is_random()));
        assert!(Sequence::designs().contains(&Sequence::GridSquares));
        let mut both = Sequence::numbers().to_vec();
        both.extend(Sequence::designs());
        assert_eq!(both, Sequence::all().to_vec());
    }
    #[test]
    fn oeis_aliases_parse_either_case() {
        assert_eq!(Sequence::parse("A005843").unwrap(), Sequence::Evens);
        assert_eq!(Sequence::parse("a000045").unwrap(), Sequence::Fibonacci);
        assert_eq!(Sequence::parse("A016754").unwrap(), Sequence::GridSquares);
        assert!(Sequence::parse("A999999").is_err());
    }
    #[test]
    fn oeis_ids_roundtrip_through_parse() {
        let mut listed = 0;
        for s in Sequence::all() {
            if let Some(id) = s.oeis() {
                assert_eq!(Sequence::parse(id).unwrap(), s);
                listed += 1;
            }
        }
        assert_eq!(listed, 6);
        assert_eq!(Sequence::Random(0).oeis(), None);
        assert_eq!(Sequence::CarpetFills.oeis(), None);
        assert_eq!(Sequence::CodeFills(7).oeis(), None);
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
        for (s, want) in Sequence::all().into_iter().zip(expected) {
            assert_eq!(s.name(), want);
        }
    }
    #[test]
    fn code_sequences_match_formulas() {
        use crate::formulas;
        for code in [1u128, 7, 14, 15] {
            let seq = sequence(Sequence::CodeFills(code), 50).unwrap();
            let expected: Vec<usize> = {
                let mut v = Vec::new();
                let mut n = 1;
                while n <= 53 {
                    let f = formulas::fill(code, n, 2, 1, 2).unwrap() as usize;
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
        let s = Sequence::CodeVoids(9);
        assert_eq!(s.name(), "code_voids_9");
        assert_eq!(Sequence::parse(&s.name()).unwrap(), s);
        assert_eq!(
            Sequence::parse("code_fills_7").unwrap(),
            Sequence::CodeFills(7)
        );
        assert!(Sequence::parse("code_fills_x").is_err());
    }
    #[test]
    fn counts_carry_a_list_or_a_sequence() {
        let listed = Counts::from(vec![3, 3, 1]);
        assert_eq!(listed.values(8).unwrap(), vec![1, 3]);
        let drawn = Counts::drawn(Sequence::Fibonacci, false, false);
        assert_eq!(drawn.values(8).unwrap(), vec![2, 3, 5, 8]);
        let wide = Counts::drawn(Sequence::Fibonacci, true, true);
        assert_eq!(wide.values(24).unwrap(), vec![0, 1, 2, 3, 5, 8, 13, 21]);
    }
}
