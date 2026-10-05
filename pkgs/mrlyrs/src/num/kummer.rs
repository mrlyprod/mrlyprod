use crate::core::error::{value_error, Result};
use crate::num::prime::is_prime;
use serde::{Deserialize, Serialize};

/// The largest top `a` a Kummer set takes here, so an automaton holds at most `32 * 32` carry states.
pub const MOST: u64 = 64;

/// The most moves a [`Carry`] table holds, its states times the prime, `2^24`.
pub const TABLE: u64 = 1 << 24;

// THE AUTOMATON

/// The carry automaton of the Kummer set `K_(a,b) = {k : p does not divide C(a k, b k)}` at a prime `p`.
///
/// By Kummer's theorem `v_p(C(a k, b k))` counts the carries of the addition `b k + (a - b) k` in base `p`, so `k` lies in the set exactly when that addition carries nowhere. The digits of `k` are read from the units, and the state is the pair `(c_1, c_2)` of carries of the multiplications by `b` and by `a - b`, `c_1 < b` and `c_2 < a - b`. In state `(c_1, c_2)` the digit `d` meets the digits `x = (b d + c_1) mod p` of `b k` and `y = ((a - b) d + c_2) mod p` of `(a - b) k`; it is allowed when `x + y <= p - 1` and moves to `(floor((b d + c_1)/p), floor(((a - b) d + c_2)/p))`. Past the top digit the zeros spend the carries, and `k` lies in the set when every digit and every one of those zeros is allowed.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Carry {
    /// The prime `p`, the base the digits are read in.
    pub prime: u64,
    /// The top `a` of `C(a k, b k)`.
    pub top: u64,
    /// The bottom `b` of `C(a k, b k)`, at least `1` and below the top.
    pub bottom: u64,
    /// The carry states `(c_1, c_2)` in reading order, `(0, 0)` first: state `c_1 (a - b) + c_2` sits at that index.
    pub states: Vec<(u64, u64)>,
    /// The moves: entry `[s][d]` is the state the digit `d` moves state `s` to, `None` when the digit is refused there.
    pub moves: Vec<Vec<Option<usize>>>,
    /// Whether each state closes: whether the zeros read from it are allowed until its carries are spent, so that a word ending there is an element of the set.
    pub closes: Vec<bool>,
}

/// Builds the carry automaton of `K_(a,b)` at the prime, `a` the top and `b` the bottom.
///
/// At `(3, 1)` and `p = 7` state `0` keeps the digits `0..2` to itself and `4` to state `1`, and state `1` keeps `0, 1` to state `0` and `3, 4` to itself.
///
/// ```
/// let k = mrlyrs::num::kummer::carry(7, 3, 1).unwrap();
/// assert_eq!(k.states, vec![(0, 0), (0, 1)]);
/// assert_eq!(k.moves[0], vec![Some(0), Some(0), Some(0), None, Some(1), None, None]);
/// ```
///
/// # Errors
///
/// Errs when `p` is not a prime, when `1 <= b < a <= MOST` fails, or when the table passes [`TABLE`] moves.
pub fn carry(prime: u64, top: u64, bottom: u64) -> Result<Carry> {
    checked(prime, top, bottom)?;
    let rest = top - bottom;
    let states: Vec<(u64, u64)> = (0..bottom)
        .flat_map(|c| (0..rest).map(move |e| (c, e)))
        .collect();
    if (states.len() as u64).saturating_mul(prime) > TABLE {
        return value_error(format!(
            "{} states at the prime {prime} pass the {TABLE} moves a table holds.",
            states.len()
        ));
    }
    let index = |(c, e): (u64, u64)| (c * rest + e) as usize;
    let moves = states
        .iter()
        .map(|&s| {
            (0..prime)
                .map(|d| column(prime, top, bottom, s, d).2.map(index))
                .collect()
        })
        .collect();
    let closes = states
        .iter()
        .map(|&s| spent(prime, top, bottom, s))
        .collect();
    Ok(Carry {
        prime,
        top,
        bottom,
        states,
        moves,
        closes,
    })
}

// THE READING

/// One column of the addition `b k + (a - b) k` as the automaton reads it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Column {
    /// The digit of `k` in this column, `0` above its top digit.
    pub digit: u64,
    /// The state `(c_1, c_2)` the column is read in.
    pub state: (u64, u64),
    /// The digit of `b k` in this column, `(b d + c_1) mod p`.
    pub left: u64,
    /// The digit of `(a - b) k` in this column, `((a - b) d + c_2) mod p`.
    pub right: u64,
    /// The state the column moves to, `None` when `left + right >= p` and the addition carries here.
    pub next: Option<(u64, u64)>,
}

/// Returns the columns the automaton reads for `k`, units first: one per digit of `k`, then the zeros that spend the carries, ending at the first column that carries or once every digit is read and the state is `(0, 0)` again.
///
/// ```
/// let read = mrlyrs::num::kummer::columns(18, 7, 3, 1).unwrap();
/// assert_eq!((read[0].digit, read[0].next), (4, Some((0, 1))));
/// assert_eq!((read[1].left, read[1].right, read[1].next), (2, 5, None));
/// ```
///
/// # Errors
///
/// Errs as [`carry`] does on the prime and the pair.
pub fn columns(k: u64, prime: u64, top: u64, bottom: u64) -> Result<Vec<Column>> {
    checked(prime, top, bottom)?;
    let mut out = Vec::new();
    let (mut rest, mut state) = (k, (0, 0));
    loop {
        let digit = rest % prime;
        rest /= prime;
        let (left, right, next) = column(prime, top, bottom, state, digit);
        out.push(Column {
            digit,
            state,
            left,
            right,
            next,
        });
        match next {
            Some(s) if rest > 0 || s != (0, 0) => state = s,
            _ => return Ok(out),
        }
    }
}

/// Returns whether `k` lies in `K_(a,b)`, read by the carry automaton: whether the prime does not divide `C(a k, b k)`.
///
/// ```
/// use mrlyrs::num::kummer::member;
/// assert!(member(4, 7, 3, 1).unwrap() && member(14, 7, 3, 1).unwrap());
/// assert!(!member(18, 7, 3, 1).unwrap());
/// ```
///
/// # Errors
///
/// Errs as [`carry`] does on the prime and the pair.
pub fn member(k: u64, prime: u64, top: u64, bottom: u64) -> Result<bool> {
    Ok(columns(k, prime, top, bottom)?
        .last()
        .is_some_and(|c| c.next.is_some()))
}

/// Returns `v_p(C(a k, b k))` by Legendre's digit sums, `(s_p(b k) + s_p((a - b) k) - s_p(a k))/(p - 1)` with `s_p` the digit sum in base `p`: the carries of `b k + (a - b) k`.
///
/// ```
/// assert_eq!(mrlyrs::num::kummer::valuation(18, 7, 3, 1).unwrap(), 1);
/// ```
///
/// # Errors
///
/// Errs as [`carry`] does on the prime and the pair.
pub fn valuation(k: u64, prime: u64, top: u64, bottom: u64) -> Result<u32> {
    checked(prime, top, bottom)?;
    let p = u128::from(prime);
    let sum = |mut n: u128| {
        let mut s = 0;
        while n > 0 {
            s += n % p;
            n /= p;
        }
        s
    };
    let k = u128::from(k);
    let carries =
        sum(u128::from(bottom) * k) + sum(u128::from(top - bottom) * k) - sum(u128::from(top) * k);
    Ok((carries / (p - 1)) as u32)
}

// THE MASS

/// Returns the count of `k < p^level` in `K_(a,b)`: the words of length `level` the automaton allows, summed by the state they end in over the states that close.
///
/// At every prime `p > a` every state keeps `(p + 1)/2` digits and every state closes, so the count is `((p + 1)/2)^level`; at `p <= a` it can fail, as at `(3, 1)` and `p = 3`.
///
/// ```
/// use mrlyrs::num::kummer::count;
/// assert_eq!(count(7, 3, 1, 5).unwrap(), 1024);
/// assert_eq!(count(3, 3, 1, 11).unwrap(), 1);
/// ```
///
/// # Errors
///
/// Errs as [`carry`] does, and when `p^level` passes a `u128`.
pub fn count(prime: u64, top: u64, bottom: u64, level: u32) -> Result<u128> {
    let carry = carry(prime, top, bottom)?;
    if u128::from(prime).checked_pow(level).is_none() {
        return value_error(format!("{prime}^{level} passes a u128."));
    }
    let mut words = vec![0u128; carry.states.len()];
    words[0] = 1;
    for _ in 0..level {
        let mut next = vec![0u128; words.len()];
        for (s, &n) in words.iter().enumerate() {
            for &to in carry.moves[s].iter().flatten() {
                next[to] += n;
            }
        }
        words = next;
    }
    Ok(words
        .iter()
        .zip(&carry.closes)
        .filter(|&(_, &closes)| closes)
        .map(|(&n, _)| n)
        .sum())
}

/// Returns the least witness that `K_(a,b)` is no digit design, as the pair `(u, v p)` of elements: digits `u, v < p` with `u` and `v p` in the set and `u + v p` outside it, least `u + v p` first; a digit design holding `u` and `v p` holds `u + v p`. `None` when no two such digits exist, as on the half interval `K_(2,1)`.
///
/// ```
/// use mrlyrs::num::kummer::witness;
/// assert_eq!(witness(7, 3, 1).unwrap(), Some((4, 14)));
/// assert_eq!(witness(7, 2, 1).unwrap(), None);
/// ```
///
/// # Errors
///
/// Errs as [`carry`] does.
pub fn witness(prime: u64, top: u64, bottom: u64) -> Result<Option<(u64, u64)>> {
    let carry = carry(prime, top, bottom)?;
    let holds = |s: Option<usize>| s.is_some_and(|s| carry.closes[s]);
    for v in 0..prime as usize {
        if !holds(carry.moves[0][v]) {
            continue;
        }
        for u in 0..prime as usize {
            let Some(s) = carry.moves[0][u].filter(|&s| carry.closes[s]) else {
                continue;
            };
            if !holds(carry.moves[s][v]) {
                return Ok(Some((u as u64, v as u64 * prime)));
            }
        }
    }
    Ok(None)
}

fn checked(prime: u64, top: u64, bottom: u64) -> Result<()> {
    if !usize::try_from(prime).is_ok_and(is_prime) {
        return value_error(format!("{prime} is not a prime."));
    }
    if bottom == 0 || bottom >= top || top > MOST {
        return value_error(format!(
            "a Kummer set takes 1 <= b < a <= {MOST}, not a = {top} and b = {bottom}."
        ));
    }
    Ok(())
}

fn column(
    prime: u64,
    top: u64,
    bottom: u64,
    (c, e): (u64, u64),
    digit: u64,
) -> (u64, u64, Option<(u64, u64)>) {
    let p = u128::from(prime);
    let x = u128::from(bottom) * u128::from(digit) + u128::from(c);
    let y = u128::from(top - bottom) * u128::from(digit) + u128::from(e);
    let (left, right) = (x % p, y % p);
    let next = (left + right < p).then_some(((x / p) as u64, (y / p) as u64));
    (left as u64, right as u64, next)
}

fn spent(prime: u64, top: u64, bottom: u64, mut state: (u64, u64)) -> bool {
    while state != (0, 0) {
        match column(prime, top, bottom, state, 0).2 {
            Some(next) => state = next,
            None => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const PAIRS: [(u64, u64); 6] = [(2, 1), (3, 1), (3, 2), (4, 1), (5, 2), (7, 3)];

    fn binomial(n: u128, m: u128) -> u128 {
        (0..m).fold(1, |c, i| c * (n - i) / (i + 1))
    }

    fn power(mut n: u128, p: u128) -> u32 {
        let mut v = 0;
        while n.is_multiple_of(p) {
            n /= p;
            v += 1;
        }
        v
    }

    fn whole(digits: impl Iterator<Item = u64>, p: u64) -> u64 {
        digits.fold((0, 1), |(n, w), d| (n + d * w, w * p)).0
    }

    #[test]
    fn the_valuation_is_the_definition() {
        for p in [2, 3, 5, 7, 11] {
            for (a, b) in PAIRS {
                for k in 0..=100 / a {
                    let c = binomial(u128::from(a * k), u128::from(b * k));
                    assert_eq!(
                        valuation(k, p, a, b).unwrap(),
                        power(c, p.into()),
                        "p {p}, ({a}, {b}), k {k}"
                    );
                }
            }
        }
    }

    #[test]
    fn the_automaton_reads_kummer() {
        for p in [2u64, 3, 5, 7] {
            for (a, b) in PAIRS {
                for k in 0..p.pow(4) {
                    let read = columns(k, p, a, b).unwrap();
                    let verdict = read.last().unwrap().next.is_some();
                    assert_eq!(
                        verdict,
                        valuation(k, p, a, b).unwrap() == 0,
                        "p {p}, ({a}, {b}), k {k}"
                    );
                    assert_eq!(member(k, p, a, b).unwrap(), verdict);
                    if verdict {
                        assert_eq!(whole(read.iter().map(|c| c.left), p), b * k);
                        assert_eq!(whole(read.iter().map(|c| c.right), p), (a - b) * k);
                        assert_eq!(whole(read.iter().map(|c| c.digit), p), k);
                    }
                }
            }
        }
    }

    #[test]
    fn the_count_is_the_mass() {
        for p in [3u64, 5, 7, 11] {
            for (a, b) in PAIRS {
                for level in 1..=3 {
                    let census = (0..p.pow(level))
                        .filter(|&k| member(k, p, a, b).unwrap())
                        .count() as u128;
                    assert_eq!(count(p, a, b, level).unwrap(), census);
                    if p > a {
                        assert_eq!(census, u128::from(p.div_ceil(2)).pow(level));
                    }
                }
            }
        }
        assert_eq!(count(3, 3, 1, 11).unwrap(), 1);
        assert_eq!(count(3, 4, 1, 11).unwrap(), 8997);
        assert_eq!(count(101, 3, 1, 5).unwrap(), 345_025_251);
    }

    #[test]
    fn the_digit_sets_read_as_printed() {
        for p in [5, 7, 11, 13, 101] {
            let k = carry(p, 3, 1).unwrap();
            let kept = |s: usize, to: usize| -> Vec<u64> {
                (0..p)
                    .filter(|&d| k.moves[s][d as usize] == Some(to))
                    .collect()
            };
            assert_eq!(kept(0, 0), (0..=(p - 1) / 3).collect::<Vec<u64>>());
            assert_eq!(
                kept(0, 1),
                (p.div_ceil(2)..=(2 * p - 1) / 3).collect::<Vec<u64>>()
            );
            assert_eq!(kept(1, 0), (0..=(p - 2) / 3).collect::<Vec<u64>>());
            assert_eq!(
                kept(1, 1),
                ((p - 1) / 2..=(2 * p - 2) / 3).collect::<Vec<u64>>()
            );
            assert_eq!(k.states, vec![(0, 0), (0, 1)]);
            assert!(k.closes.iter().all(|&c| c));
            let v = if p % 6 == 1 {
                (p - 1) / 3
            } else {
                (2 * p - 1) / 3
            };
            assert_eq!(witness(p, 3, 1).unwrap(), Some((p.div_ceil(2), v * p)));
        }
        for (a, b) in PAIRS {
            for p in [11, 13] {
                let k = carry(p, a, b).unwrap();
                assert!(k
                    .moves
                    .iter()
                    .all(|row| row.iter().flatten().count() as u64 == p.div_ceil(2)));
            }
        }
        assert_eq!(witness(11, 2, 1).unwrap(), None);
    }

    #[test]
    fn a_bad_pair_errs() {
        assert!(carry(9, 3, 1).is_err());
        assert!(count(7, 3, 3, 2).is_err());
        assert!(member(4, 7, 3, 0).is_err());
        assert!(valuation(4, 7, MOST + 1, 1).is_err());
        assert!(count(101, 3, 1, 20).is_err());
    }
}
