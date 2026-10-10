use crate::Fault;
use mrlyrs::core::json;
use mrlyrs::num::kummer::{carry, columns, count, valuation, witness, Carry};
use mrlyrs::num::prime::primes;
use wasm_bindgen::prelude::*;

const PRIMES: usize = 101;
const TOP: u32 = 9;
const BOARD: u128 = 1 << 14;
const EXACT: u128 = 1 << 53;
const DEEPEST: u32 = 12;
const LARGEST: u64 = 1_000_000_000_000_000;

fn built(prime: u32, top: u32, bottom: u32) -> Result<Carry, Fault> {
    if prime == 2 || prime as usize > PRIMES {
        return Err(Fault::new(format!(
            "the page reads the odd primes up to {PRIMES}, not {prime}."
        )));
    }
    if top > TOP {
        return Err(Fault::new(format!(
            "the page reads a up to {TOP}, not {top}."
        )));
    }
    Ok(carry(u64::from(prime), u64::from(top), u64::from(bottom))?)
}

fn deepest(prime: u32, cells: u128) -> u32 {
    (1..=DEEPEST)
        .take_while(|&l| u128::from(prime).checked_pow(l).is_some_and(|n| n <= cells))
        .last()
        .unwrap_or(1)
}

fn digits(mut n: u128, prime: u64) -> Vec<u64> {
    let p = u128::from(prime);
    let mut out = vec![(n % p) as u64];
    n /= p;
    while n > 0 {
        out.push((n % p) as u64);
        n /= p;
    }
    out
}

fn runs(row: &[Option<usize>]) -> Vec<[usize; 3]> {
    let mut out: Vec<[usize; 3]> = Vec::new();
    for (d, to) in row.iter().enumerate() {
        let Some(to) = *to else { continue };
        match out.last_mut() {
            Some(run) if run[1] + 1 == d && run[2] == to => run[1] = d,
            _ => out.push([d, d, to]),
        }
    }
    out
}

/// Returns the odd primes the page offers, `3` to `101`.
#[wasm_bindgen]
pub fn kummer_primes() -> Vec<u32> {
    primes(PRIMES)
        .into_iter()
        .filter(|&p| p > 2)
        .map(|p| p as u32)
        .collect()
}

/// Returns the largest top `a` the page offers.
#[wasm_bindgen]
pub fn kummer_top() -> u32 {
    TOP
}

/// Returns the deepest level the board draws at the prime, at least one: at most `2^14` cells in its deepest row.
#[wasm_bindgen]
pub fn kummer_cap(prime: u32) -> u32 {
    deepest(prime, BOARD)
}

/// Reads the Kummer set `K_(a,b)` at the prime as JSON: its carry states, the moves and the runs of digits each state keeps with the state each run moves to, which states close, the count below `p^level` at every level to 12 against `((p + 1)/2)^level`, whether `p > a`, and the least witness `u`, `v p`, `u + v p` that the set is no digit design.
#[wasm_bindgen]
pub fn kummer_read(prime: u32, top: u32, bottom: u32) -> Result<String, Fault> {
    let k = built(prime, top, bottom)?;
    let p = u64::from(prime);
    let fill = p.div_ceil(2);
    let deep = deepest(prime, EXACT);
    let counts = (1..=deep)
        .map(|l| count(p, k.top, k.bottom, l).map(|n| n as u64))
        .collect::<Result<Vec<u64>, _>>()?;
    let law: Vec<u64> = (1..=deep).map(|l| fill.pow(l)).collect();
    let witness = witness(p, k.top, k.bottom)?.map(|(u, high)| {
        json!({
            "u": u,
            "high": high,
            "sum": u + high,
            "digits": [digits(u.into(), p), digits(high.into(), p), digits((u + high).into(), p)],
        })
    });
    Ok(json!({
        "prime": p,
        "top": k.top,
        "bottom": k.bottom,
        "fill": fill,
        "holds": p > k.top,
        "states": k.states,
        "moves": k.moves,
        "runs": k.moves.iter().map(|row| runs(row)).collect::<Vec<_>>(),
        "kept": k.moves.iter().map(|row| row.iter().flatten().count()).collect::<Vec<_>>(),
        "closes": k.closes,
        "cap": deepest(prime, BOARD),
        "counts": counts,
        "law": law,
        "witness": witness,
    })
    .to_string())
}

/// Returns the board of `K_(a,b)` at the prime: the rows of levels `1` to `level` one after another, row `r` holding `p^r` cells with the word `d_0 d_1 .. d_(r-1)`, units digit first, at cell `sum_i d_i p^(r - 1 - i)`, so every cell splits into the `p` cells below it; a cell holds `0` when the automaton refuses a digit of its word and `1 + s` when the word ends in state `s`.
#[wasm_bindgen]
pub fn kummer_board(prime: u32, top: u32, bottom: u32, level: u32) -> Result<Vec<u8>, Fault> {
    let k = built(prime, top, bottom)?;
    let cap = deepest(prime, BOARD);
    if !(1..=cap).contains(&level) {
        return Err(Fault::new(format!(
            "the board runs from level 1 to {cap} at {prime}, not {level}."
        )));
    }
    let p = prime as usize;
    let moves = &k.moves;
    let mut out: Vec<u8> = Vec::new();
    let mut row: Vec<u8> = vec![1];
    for _ in 0..level {
        let next: Vec<u8> = row
            .iter()
            .flat_map(|&cell| {
                (0..p).map(move |d| match cell {
                    0 => 0,
                    _ => moves[cell as usize - 1][d].map_or(0, |s| s as u8 + 1),
                })
            })
            .collect();
        out.extend_from_slice(&next);
        row = next;
    }
    Ok(out)
}

/// Reads one `k`, typed as a decimal, through the automaton of `K_(a,b)` at the prime as JSON: its digits units first, the columns read with the state each is read in and moves to, the digits of `b k`, `(a - b) k` and `a k`, the values `a k` and `b k`, the verdict and `v_p(C(a k, b k))` by Legendre's digit sums.
#[wasm_bindgen]
pub fn kummer_trace(k: &str, prime: u32, top: u32, bottom: u32) -> Result<String, Fault> {
    let set = built(prime, top, bottom)?;
    let text = k.trim();
    let n: u64 = text.parse().ok().filter(|&n| n <= LARGEST).ok_or_else(|| {
        Fault::new(format!(
            "k is a whole number up to {LARGEST}, not {text:?}."
        ))
    })?;
    let (p, a, b) = (u64::from(prime), set.top, set.bottom);
    let index = |(c, e): (u64, u64)| c * (a - b) + e;
    let read: Vec<_> = columns(n, p, a, b)?
        .into_iter()
        .map(|c| {
            json!({
                "digit": c.digit,
                "state": index(c.state),
                "left": c.left,
                "right": c.right,
                "next": c.next.map(index),
            })
        })
        .collect();
    let member = read.last().is_some_and(|c| !c["next"].is_null());
    let n = u128::from(n);
    Ok(json!({
        "k": n.to_string(),
        "digits": digits(n, p),
        "columns": read,
        "member": member,
        "valuation": valuation(n as u64, p, a, b)?,
        "low": digits(u128::from(b) * n, p),
        "high": digits(u128::from(a - b) * n, p),
        "whole": digits(u128::from(a) * n, p),
        "product": (u128::from(a) * n).to_string(),
        "chosen": (u128::from(b) * n).to_string(),
    })
    .to_string())
}
