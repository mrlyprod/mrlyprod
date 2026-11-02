//! The stored rows every binding replays, against what the crate returns today.

use mrly_num::{factor, prime, series};
use serde_json::{Value, json};
use std::path::Path;

fn row(path: &str, input: Value, out: Value, note: &str) -> Value {
    json!({"fn": path, "in": input, "out": out, "note": note})
}

fn big(number: u128) -> Value {
    json!(number.to_string())
}

fn f12(value: f64) -> Value {
    json!(format!("{value:.12}").parse::<f64>().unwrap() + 0.0)
}

// ROWS

fn rows() -> Vec<Value> {
    let pairs = [(1071u128, 462u128), (1u128 << 100, 1u128 << 60)];
    let named: Vec<Vec<String>> = pairs
        .iter()
        .map(|&(a, b)| vec![a.to_string(), b.to_string()])
        .collect();
    let gcds: Vec<String> = pairs
        .iter()
        .map(|&(a, b)| factor::gcd(a, b).to_string())
        .collect();
    let primes: Vec<usize> = (1..=100).filter(|&n| prime::is_prime(n)).collect();
    vec![
        row(
            "num::factor::factorial",
            json!({"number": 25}),
            big(factor::factorial(25).unwrap()),
            "the u128 crossing: 25! as a decimal string",
        ),
        row(
            "num::factor::gcd",
            json!({"pairs": named}),
            json!(gcds),
            "two calls, (1071, 462) and (2^100, 2^60), u128 in and out as decimal strings",
        ),
        row(
            "num::factor::divisors",
            json!({"number": 360}),
            json!(factor::divisors(360)),
            "the 24 divisors of 360 in ascending order",
        ),
        row(
            "num::factor::mobius_sieve",
            json!({"limit": 30}),
            json!(factor::mobius_sieve(30)),
            "OEIS A008683 with a leading pad: 31 entries, mu(n) at index n",
        ),
        row(
            "num::prime::is_prime",
            json!({"from": 1, "to": 100}),
            json!(primes),
            "the 25 numbers in 1..=100 the test keeps",
        ),
        row(
            "num::series::zeta",
            json!({"s": 2.0, "terms": 1000}),
            f12(series::zeta(2.0, 1000).unwrap()),
            "the f64 crossing: pi squared over six, rounded to 12 decimals",
        ),
    ]
}

// REPLAY

#[test]
fn every_row_replays() {
    let file = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures.json");
    let fresh = rows();
    if std::env::var_os("MRLY_FIXTURES").is_some() {
        let mut text = serde_json::to_string_pretty(&fresh).unwrap();
        text.push('\n');
        std::fs::write(&file, text).unwrap();
    }
    let saved: Vec<Value> = serde_json::from_str(&std::fs::read_to_string(&file).unwrap()).unwrap();
    assert_eq!(saved.len(), fresh.len(), "{} rows stored", saved.len());
    for (was, now) in saved.iter().zip(&fresh) {
        assert_eq!(was, now, "{} drifted; MRLY_FIXTURES=1 rewrites", was["fn"]);
    }
}
