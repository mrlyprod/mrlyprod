//! The mrly-tile binary, driven end to end.

use mrly_core::Image;
use mrly_core::name::sha;
use serde_json::{Value, json};
use std::process::Command;

#[test]
fn the_binary_replays_its_fixture_row() {
    let out = Command::new(env!("CARGO_BIN_EXE_mrly-tile"))
        .args(["tile.background", r#"[1,2,2]"#])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let image: Image = serde_json::from_slice(&out.stdout).unwrap();
    let seen = json!({
        "shape": [image.height, image.width],
        "sha256": sha::hex(&image.colors.concat()),
    });
    let text = include_str!("../../tests/fixtures.json");
    let rows: Vec<Value> = serde_json::from_str(text).unwrap();
    let row = rows
        .iter()
        .find(|row| row["fn"] == "tile::background")
        .unwrap();
    assert_eq!(seen, row["out"]);
}
