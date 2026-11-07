use mrly_error::{Result, value_error};
use mrly_math::code::Code;
use mrly_num::prime;
use mrly_num::series;
use mrly_rand::Rng;
use mrly_sequences::counts;
use mrly_two as two;
use mrly_two::census;
use serde::{Deserialize, Serialize};

const DIM: usize = 2;
const BASE: usize = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Source {
    Evens,
    Odds,
    Random(u64),
    Primes,
    Binary,
    Fibonacci,
    GridSquares,
    CarpetFills,
    CarpetVoids,
    NetFills,
    NetVoids,
    TreeFills,
    TreeVoids,
    VoidFills,
    VoidVoids,
    PointFills,
    PointVoids,
    DustFills,
    DustVoids,
    LineFills,
    LineVoids,
    StarFills,
    StarVoids,
    CodeFills(u128),
    CodeVoids(u128),
}

impl Source {
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
        let (i, rest) = mrly_name::longest(text, &names)?;
        Some((fixed[i], rest))
    }
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
    pub fn oeis(self) -> Option<String> {
        match self {
            Source::Evens => Some("A005843".to_string()),
            Source::Odds => Some("A005408".to_string()),
            Source::Primes => Some("A000040".to_string()),
            Source::Binary => Some("A000079".to_string()),
            Source::Fibonacci => Some("A000045".to_string()),
            Source::GridSquares => Some("A016754".to_string()),
            Source::CarpetFills => Some("A000567".to_string()),
            Source::CarpetVoids => Some("A000290".to_string()),
            Source::NetFills => Some("A045944".to_string()),
            Source::NetVoids => Some("A000290".to_string()),
            Source::TreeFills => Some("A000384".to_string()),
            Source::TreeVoids => Some("A014105".to_string()),
            Source::VoidFills => Some("A001844".to_string()),
            Source::VoidVoids => Some("A046092".to_string()),
            Source::PointFills => Some("A000290".to_string()),
            Source::PointVoids => Some("A000567".to_string()),
            Source::DustFills => Some("A000290".to_string()),
            Source::DustVoids => Some("A045944".to_string()),
            Source::LineFills => Some("A014105".to_string()),
            Source::LineVoids => Some("A000384".to_string()),
            Source::StarFills => Some("A046092".to_string()),
            Source::StarVoids => Some("A001844".to_string()),
            Source::Random(_) | Source::CodeFills(_) | Source::CodeVoids(_) => None,
        }
    }
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Counts {
    List(Vec<usize>),
    Drawn {
        seq: Source,
        zeros: bool,
        ones: bool,
    },
}

impl Counts {
    pub fn list(counts: Vec<usize>) -> Counts {
        Counts::List(counts)
    }
    pub fn drawn(seq: Source, zeros: bool, ones: bool) -> Counts {
        Counts::Drawn { seq, zeros, ones }
    }
    pub fn parse(token: &str) -> Result<Counts> {
        if token.is_empty() {
            return Ok(Counts::list(Vec::new()));
        }
        if token.starts_with(|c: char| c.is_ascii_digit()) {
            let mut list = Vec::new();
            for part in token.split(',') {
                match part.parse::<usize>() {
                    Ok(n) => list.push(n),
                    Err(_) => return value_error(format!("count {part:?} is not a number.")),
                }
            }
            return Ok(Counts::list(list));
        }
        let Some((seq, tail)) = Source::read(token) else {
            return value_error(format!("no source starts {token:?}."));
        };
        let (zeros, tail) = match tail.strip_prefix("_zeros") {
            Some(rest) => (true, rest),
            None => (false, tail),
        };
        let (ones, tail) = match tail.strip_prefix("_ones") {
            Some(rest) => (true, rest),
            None => (false, tail),
        };
        if !tail.is_empty() {
            return value_error(format!("counts {token:?} have a tail {tail:?}."));
        }
        Ok(Counts::drawn(seq, zeros, ones))
    }
    pub fn token(&self) -> String {
        match self {
            Counts::List(list) => sorted(list)
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(","),
            Counts::Drawn { seq, zeros, ones } => format!("{}{}", seq.name(), roll(*zeros, *ones)),
        }
    }
    pub fn values(&self, budget: usize) -> Result<Vec<usize>> {
        match self {
            Counts::List(list) => Ok(sorted(list).into_iter().filter(|&n| n <= budget).collect()),
            Counts::Drawn { seq, zeros, ones } => counts(*seq, budget, *zeros, *ones),
        }
    }
}

pub fn roll(zeros: bool, ones: bool) -> &'static str {
    match (zeros, ones) {
        (false, false) => "",
        (true, false) => "_zeros",
        (false, true) => "_ones",
        (true, true) => "_zeros_ones",
    }
}

fn sorted(list: &[usize]) -> Vec<usize> {
    let mut out = list.to_vec();
    out.sort_unstable();
    out.dedup();
    out
}

impl From<Vec<usize>> for Counts {
    fn from(list: Vec<usize>) -> Counts {
        Counts::list(list)
    }
}
