use demos::kummer::*;
use mrlyrs::core::error::parse;

const PRESETS: [(u32, u32); 4] = [(2, 1), (3, 1), (4, 1), (5, 2)];

fn column(read: &mrlyrs::core::Json, key: &str) -> String {
    read[key]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

#[test]
fn the_verdict_is_the_valuation() {
    for p in [5, 7, 11] {
        for (a, b) in PRESETS {
            for k in 0..p * p * p * p {
                let read = parse(&kummer_trace(&k.to_string(), p, a, b).unwrap()).unwrap();
                assert_eq!(
                    read["member"].as_bool().unwrap(),
                    read["valuation"] == 0,
                    "p {p}, ({a}, {b}), k {k}"
                );
            }
        }
    }
}

#[test]
fn the_kummer_exports_answer() {
    let seven = parse(&kummer_read(7, 3, 1).unwrap()).unwrap();
    assert!(column(&seven, "counts").starts_with("4,16,64,256,1024,4096,"));
    assert_eq!(column(&seven, "kept"), "4,4");
    assert_eq!(
        seven["runs"].to_string(),
        "[[[0,2,0],[4,4,1]],[[0,1,0],[3,4,1]]]"
    );
    assert_eq!(seven["witness"]["u"], 4);
    assert_eq!(seven["witness"]["high"], 14);
    assert_eq!(seven["witness"]["sum"], 18);
    assert_eq!(seven["witness"]["digits"].to_string(), "[[4],[0,2],[4,2]]");
    assert!(parse(&kummer_read(7, 2, 1).unwrap()).unwrap()["witness"].is_null());

    let wide = parse(&kummer_read(101, 3, 1).unwrap()).unwrap();
    assert_eq!(wide["counts"][4], 345_025_251);
    let three = parse(&kummer_read(3, 3, 1).unwrap()).unwrap();
    let four = parse(&kummer_read(3, 4, 1).unwrap()).unwrap();
    assert_eq!(three["counts"][10], 1);
    assert_eq!(four["counts"][10], 8997);
    assert_eq!(four["law"][10], 2048);
    assert_eq!(three["holds"], false);

    for (p, a, b) in [(7, 3, 1), (3, 4, 1), (5, 5, 2), (13, 2, 1)] {
        let read = parse(&kummer_read(p, a, b).unwrap()).unwrap();
        let level = kummer_cap(p).min(4);
        let board = kummer_board(p, a, b, level).unwrap();
        let last = &board[board.len() - (p as usize).pow(level)..];
        let members = last
            .iter()
            .filter(|&&c| c > 0 && read["closes"][c as usize - 1] == true)
            .count();
        assert_eq!(
            read["counts"][level as usize - 1],
            members as u64,
            "p {p}, ({a}, {b})"
        );
    }

    let out = parse(&kummer_trace("18", 7, 3, 1).unwrap()).unwrap();
    assert_eq!(column(&out, "digits"), "4,2");
    assert_eq!(out["columns"][0]["next"], 1);
    assert!(out["columns"][1]["next"].is_null());
    assert_eq!(out["member"], false);
    assert_eq!(out["valuation"], 1);
    assert_eq!(column(&out, "whole"), "5,0,1");

    assert_eq!(kummer_cap(7), 4);
    assert_eq!(kummer_cap(101), 2);
    assert_eq!(kummer_primes().first(), Some(&3));
    assert_eq!(kummer_primes().last(), Some(&101));
    assert!(kummer_read(9, 3, 1).is_err());
    assert!(kummer_read(2, 3, 1).is_err());
    assert!(kummer_board(7, 3, 1, 5).is_err());
    assert!(kummer_trace("x", 7, 3, 1).is_err());
}
