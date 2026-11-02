//! The stored rows every binding replays, against what the crate returns today.

use mrly_core::tensor::Tensor;
use mrly_math::bang::Code;
use mrly_math::graph::Network;
use mrly_math::two::Cell2d;
use mrly_math::{atoms, bang, counts, spectrum, three, two};
use serde_json::{Value, json};
use std::path::Path;

fn row(path: &str, input: Value, out: Value, note: &str) -> Value {
    json!({"fn": path, "in": input, "out": out, "note": note})
}

fn tensor(grid: &Tensor) -> Value {
    json!({"shape": grid.shape, "data": grid.bytes().unwrap()})
}

fn cell(flat: &Cell2d) -> Value {
    serde_json::from_str(&two::to_json(flat)).unwrap()
}

fn big(number: u128) -> Value {
    json!(number.to_string())
}

fn f12(value: f64) -> Value {
    json!(format!("{value:.12}").parse::<f64>().unwrap() + 0.0)
}

// ROWS

fn rows() -> Vec<Value> {
    let atom = atoms::carpet_2d(3);
    let flat = two::carpet(3, 2).unwrap();
    let survey = two::census::census(&flat).unwrap();
    let filled = counts::fill(Code::from(23u64), 3, 3, 2, 2).unwrap();
    let universe = bang::bang(3).unwrap();
    let cube = three::carpet(3, 1).unwrap();
    let solid = three::census::census(&cube).unwrap();
    let mut network = Network::new(1);
    for node in 0..5 {
        network.add_node(vec![node as f64]).unwrap();
    }
    for branch in 0..4 {
        network.add_branch(branch, branch + 1, 1.0).unwrap();
    }
    let spread = spectrum::laplacian_spectrum(&network, false).unwrap();
    vec![
        row(
            "math::atoms::carpet_2d",
            json!({"n": 3}),
            tensor(&atom),
            "the 3 by 3 seed every carpet design rests on",
        ),
        row(
            "math::two::carpet",
            json!({"number": 3, "level": 2}),
            cell(&flat),
            "the byte-grid crossing: the cell as math::two::to_json writes it",
        ),
        row(
            "math::two::census::census",
            json!({"cell": {"fn": "math::two::carpet", "in": {"number": 3, "level": 2}}}),
            serde_json::to_value(&survey).unwrap(),
            "a struct crossing: the Census through its serde derive",
        ),
        row(
            "math::counts::fill",
            json!({"code": "23", "number": 3, "dimension": 3, "level": 2, "base": 2}),
            big(filled),
            "the sponge's filled cells in closed form, u128 as a decimal string",
        ),
        row(
            "math::bang::bang",
            json!({"dimension": 3}),
            json!(universe.distinct()),
            "Universe::distinct() of the dimension-3 universe",
        ),
        row(
            "math::three::census::census",
            json!({"cell": {"fn": "math::three::carpet", "in": {"number": 3, "level": 1}}}),
            big(solid.surface),
            "the .surface field only, u128 as a decimal string",
        ),
        row(
            "math::spectrum::laplacian_spectrum",
            json!({
                "network": {
                    "dim": 1,
                    "nodes": [[0.0], [1.0], [2.0], [3.0], [4.0]],
                    "branches": [[0, 1, 1.0], [1, 2, 1.0], [2, 3, 1.0], [3, 4, 1.0]],
                },
                "normalised": false,
            }),
            Value::Array(spread.iter().map(|&value| f12(value)).collect()),
            "the five eigenvalues of a 5-node path, ascending, to 12 decimals",
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
