//! The values that cross units: letters into rows, and every name kind into one canonical string.

use mrly::core::name::Named;
use mrly::life::{Counts, Rule};
use mrly::math;
use mrly::math::name::{Bang, Sequence, Word};
use mrly::num::sieve;
use mrly::tile::name::Tile;

#[test]
fn the_row_fill_counts_every_rendered_letter() {
    for dimension in 1..=3usize {
        for code in 1..(1u128 << (1 << dimension)) {
            let profile = sieve::row_profile(code, dimension).unwrap();
            for side in 2..=7u64 {
                let tile = math::bang::factory::create(
                    math::bang::Code::from(code),
                    side as usize,
                    dimension,
                    2,
                    1,
                )
                .unwrap();
                assert_eq!(
                    sieve::row_fill(&profile, side).unwrap(),
                    u128::from(tile.sum()),
                    "dim {dimension} code {code} side {side}"
                );
            }
        }
    }
}

#[test]
fn every_kind_has_one_canonical_string_and_one_id() {
    let bang = Bang::new(7, 2, 2);
    let rule = Rule::new(vec![3].into(), vec![2, 3].into(), false);
    let word = Word::new(2, &[(7, 3), (14, 7)]).unwrap();
    let tile = Tile::from_json(r#"{"kind":"tile","code":7,"side":3,"level":2}"#).unwrap();
    let sequence = Sequence::new(7, 2, 2, "fills", "side");
    let strings = [
        bang.to_json(),
        rule.to_json(),
        word.to_json(),
        tile.to_json(),
        sequence.to_json(),
    ];
    let ids = [
        bang.to_id(),
        rule.to_id(),
        word.to_id(),
        tile.to_id(),
        sequence.to_id(),
    ];
    for (text, id) in strings.iter().zip(&ids) {
        assert!(text.starts_with("{\"kind\":\""));
        assert!(!text.contains(' '));
        assert_eq!(id.len(), 8);
        assert!(id.chars().all(|c| c.is_ascii_hexdigit()));
    }
    assert_eq!(Bang::from_json(&strings[0]).unwrap(), bang);
    assert_eq!(Rule::from_json(&strings[1]).unwrap(), rule);
    assert_eq!(Word::from_json(&strings[2]).unwrap(), word);
    assert_eq!(Tile::from_json(&strings[3]).unwrap(), tile);
    assert_eq!(Sequence::from_json(&strings[4]).unwrap(), sequence);
    assert!(Rule::from_json(&strings[0]).is_err());
    assert_eq!(rule.birth, Counts::List(vec![3]));
}

#[test]
fn every_example_on_names_md_round_trips() {
    let doc = include_str!("../../mrly-core/src/name/NAMES.md");
    let mut seen = 0;
    for piece in doc.split('`') {
        let Some(rest) = piece.strip_prefix("{\"kind\":\"") else {
            continue;
        };
        let kind = rest.split('"').next().unwrap();
        let printed = match kind {
            "bang" => Bang::from_json(piece).map(|v| v.to_json()),
            "rule" => Rule::from_json(piece).map(|v| v.to_json()),
            "sequence" => Sequence::from_json(piece).map(|v| v.to_json()),
            "tile" => Tile::from_json(piece).map(|v| v.to_json()),
            "word" => Word::from_json(piece).map(|v| v.to_json()),
            other => panic!("NAMES.md names no kind {other:?}"),
        };
        let printed = printed.unwrap_or_else(|e| panic!("{piece} does not read: {e}"));
        assert_eq!(printed, piece, "{piece} is not canonical");
        seen += 1;
    }
    assert!(seen >= 13, "NAMES.md kept only {seen} examples");
}
