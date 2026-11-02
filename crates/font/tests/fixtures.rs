//! The stored rows every binding replays, against what the crate returns today.

use mrly_core::name::sha;
use mrly_font as font;
use serde_json::{Value, json};
use std::path::Path;

fn row(path: &str, input: Value, out: Value, note: &str) -> Value {
    json!({"fn": path, "in": input, "out": out, "note": note})
}

// ROWS

fn rows() -> Vec<Value> {
    let a = font::glyph('a').unwrap();
    let write = font::animate::animate("MRLY", 1);
    let book = font::map();
    let text = serde_json::to_string(&book).unwrap();
    vec![
        row(
            "font::glyph",
            json!({"c": "a"}),
            json!(a.rows),
            "the five bitmap rows of the rounded lowercase a",
        ),
        row(
            "font::raster::raster",
            json!({"text": "42"}),
            json!(font::raster::raster("42")),
            "the text as one 0/1 grid, its trimmed glyphs a blank column apart",
        ),
        row(
            "font::path",
            json!({"c": "M"}),
            json!(font::path('M')),
            "the tuple-list crossing: the stroke order as (row, col) pairs",
        ),
        row(
            "font::animate::animate",
            json!({"text": "MRLY", "pad": 1}),
            json!({
                "rows": write.rows,
                "cols": write.cols,
                "fps": write.fps,
                "frames": write.frames.len(),
                "last": write.frames.last().unwrap(),
            }),
            "the board, the rate, the frame count and the finished frame",
        ),
        row(
            "font::map",
            json!({}),
            json!({"sha256": sha::hex(text.as_bytes()), "len": book.len()}),
            "sha256 of serde's json of the whole table, which pins all 108 glyphs",
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
