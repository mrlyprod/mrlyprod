//! The stored rows every binding replays, against what the crate returns today.

use mrly_core::Image;
use mrly_core::name::sha;
use mrly_core::tensor::Tensor;
use mrly_life::{self as life, Config, Source};
use mrly_math::two::{self, Cell2d};
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

fn f12(value: f64) -> Value {
    json!(format!("{value:.12}").parse::<f64>().unwrap() + 0.0)
}

fn pixels(image: &Image) -> Value {
    json!({
        "shape": [image.height, image.width],
        "sha256": sha::hex(&image.colors.concat()),
    })
}

fn glider() -> Cell2d {
    let mut grid = Tensor::new(vec![8, 8]);
    for (y, x) in [(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)] {
        grid.set(&[y, x], 1).unwrap();
    }
    Cell2d::new(grid).unwrap()
}

fn glider_in() -> Value {
    json!({
        "seed": tensor(glider().types()),
        "config": {
            "mask": "life::moore",
            "birth": [3],
            "survive": [2, 3],
            "boundary": "Constant",
            "max_generations": 64,
            "grid_size": 1,
            "padding": 0,
        },
    })
}

// ROWS

fn rows() -> Vec<Value> {
    let mut seed = vec![0u8; 31];
    seed[15] = 1;
    let space = life::history(&seed, 30, 16, true).unwrap();
    let config = Config::new(life::moore().unwrap(), vec![3].into(), vec![2, 3].into());
    let run = life::animate::animate(&glider(), &config).unwrap();
    let small = two::carpet(2, 2).unwrap();
    let blank = Cell2d::new(Tensor::new(vec![4, 4])).unwrap();
    let pair = [blank, small.clone()];
    let heat = life::heatmap(&run.grids, 1).unwrap();
    vec![
        row(
            "life::history",
            json!({"row": seed, "rule": 30, "steps": 16, "wrap": true}),
            tensor(&space),
            "the space-time diagram, row 0 the seed and one row per step",
        ),
        row(
            "life::animate::animate",
            glider_in(),
            json!({
                "fate": run.fate.name(),
                "count": run.count,
                "loop_length": run.loop_length,
                "last": cell(run.last().unwrap()),
            }),
            "the glider walks into the dead border and settles",
        ),
        row(
            "life::entropy",
            json!({"grid": {"fn": "math::two::carpet", "in": {"number": 2, "level": 2}}}),
            json!(life::entropy(&small)),
            "the binary Shannon entropy of the grid, in millibits",
        ),
        row(
            "life::churn",
            json!({
                "grids": [
                    {"zeros": {"shape": [4, 4]}},
                    {"fn": "math::two::carpet", "in": {"number": 2, "level": 2}},
                ],
            }),
            f12(life::churn(&pair)),
            "the mean fraction of sites changed, to 12 decimals",
        ),
        row(
            "life::counts",
            json!({
                "seq": "Primes",
                "max_neighbors": 8,
                "include_zeros": false,
                "include_ones": false,
            }),
            json!(life::counts(Source::Primes, 8, false, false).unwrap()),
            "the primes up to eight, laid down as neighbor counts",
        ),
        row(
            "life::heatmap",
            json!({"grids": {"fn": "life::animate::animate", "in": glider_in()}, "scale": 1}),
            Value::Array(heat.iter().map(pixels).collect()),
            "the row crossing: the shape and the sha256 of the rgba bytes of one heat frame per generation",
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
