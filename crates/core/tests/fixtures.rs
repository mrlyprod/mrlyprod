//! The stored rows every binding replays, against what the crate returns today.

use mrly_core::name::sha;
use mrly_core::tensor::Tensor;
use mrly_core::{Color, Colorizer, Image};
use serde_json::{Value, json};
use std::path::Path;

fn row(path: &str, input: Value, out: Value, note: &str) -> Value {
    json!({"fn": path, "in": input, "out": out, "note": note})
}

fn tensor(grid: &Tensor) -> Value {
    json!({"shape": grid.shape, "data": grid.bytes().unwrap()})
}

fn pixels(image: &Image) -> Value {
    json!({
        "shape": [image.height, image.width],
        "sha256": sha::hex(&image.colors.concat()),
    })
}

// ROWS

fn rows() -> Vec<Value> {
    let source: Vec<u8> = (0..12).collect();
    let turned = Tensor::of(source.clone(), vec![3, 4])
        .unwrap()
        .rot90(1, (0, 1))
        .unwrap();
    let red = Color::from_hex("#ff3d40").unwrap();
    let corners = [
        [255u8, 0, 0, 255],
        [0, 255, 0, 255],
        [0, 0, 255, 255],
        [255, 255, 255, 255],
    ];
    let scaled = Image::new(2, 2, corners.to_vec())
        .unwrap()
        .scale(3)
        .unwrap();
    let ramp = Colorizer::gradient_bins(
        Color::from_hex("#000000").unwrap(),
        &[
            Color::from_hex("#ff3d40").unwrap(),
            Color::from_hex("#3d40ff").unwrap(),
        ],
        4,
    )
    .unwrap();
    let swatches: Vec<String> = (0..4)
        .map(|value| mrly_core::ramp::color(&ramp, value, 9).to_hex())
        .collect();
    vec![
        row(
            "core::Tensor::rot90",
            json!({"data": source, "shape": [3, 4], "k": 1, "axes": [0, 1]}),
            tensor(&turned),
            "one quarter turn in the (0, 1) plane, the numpy law",
        ),
        row(
            "core::Color::from_hex",
            json!({"hex": "#ff3d40"}),
            json!({"rgba": [red.r, red.g, red.b, red.a], "hex": red.to_hex()}),
            "the house red parsed, then printed back by to_hex",
        ),
        row(
            "core::Image::scale",
            json!({"colors": corners, "width": 2, "height": 2, "scale": 3}),
            pixels(&scaled),
            "the image crossing: the shape and the sha256 of the rgba bytes, which pins the scale",
        ),
        row(
            "core::Colorizer::color",
            json!({
                "background": "#000000",
                "ramp": ["#ff3d40", "#3d40ff"],
                "shades": 4,
                "values": [0, 1, 2, 3],
                "max": 9,
            }),
            json!(swatches),
            "gradient_bins first; four shades against a max of 9 bin every live value at the ramp head",
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
