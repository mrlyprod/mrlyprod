use crate::core::errors::{value_error, Result};
use crate::math::formulas::{self, classics};
use crate::math::two::{self, census};

const DIM: usize = 2;
const BASE: usize = 2;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sequence {
    Evens,
    Odds,
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
    CodeFills(u128),
    CodeVoids(u128),
}

impl Sequence {
    pub fn name(self) -> String {
        let fixed = match self {
            Sequence::Evens => "evens",
            Sequence::Odds => "odds",
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
            Sequence::CodeFills(code) => return format!("code_fills:{code}"),
            Sequence::CodeVoids(code) => return format!("code_voids:{code}"),
        };
        fixed.to_string()
    }
    pub fn parse(name: &str) -> Result<Sequence> {
        let lower = name.to_lowercase();
        if let Some(rest) = lower.strip_prefix("code_fills:") {
            return parse_code(rest).map(Sequence::CodeFills);
        }
        if let Some(rest) = lower.strip_prefix("code_voids:") {
            return parse_code(rest).map(Sequence::CodeVoids);
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
            other => return value_error(format!("unknown sequence {other:?}.")),
        };
        Ok(seq)
    }
    pub fn all() -> [Sequence; 14] {
        [
            Sequence::Evens,
            Sequence::Odds,
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
    fn is_number(self) -> bool {
        matches!(
            self,
            Sequence::Evens
                | Sequence::Odds
                | Sequence::Primes
                | Sequence::Binary
                | Sequence::Fibonacci
        )
    }
}

fn parse_code(s: &str) -> Result<u128> {
    s.trim().parse::<u128>().map_err(|_| {
        crate::core::MrlyError::Value(format!("invalid code {s:?} (expected an integer)."))
    })
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

pub fn sequence(seq: Sequence, limit: usize) -> Result<Vec<usize>> {
    if seq.is_number() {
        return Ok(match seq {
            Sequence::Evens => classics::evens(limit),
            Sequence::Odds => classics::odds(limit),
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
    fn code_sequences_match_formulas() {
        use crate::math::formulas;
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
        assert_eq!(s.name(), "code_voids:9");
        assert_eq!(Sequence::parse(&s.name()).unwrap(), s);
        assert_eq!(
            Sequence::parse("code_fills:7").unwrap(),
            Sequence::CodeFills(7)
        );
    }
}
