//! The mrly-math binary, driven end to end.

use serde_json::Value;
use std::process::Command;

#[test]
fn the_binary_replays_its_fixture_row() {
    let out = Command::new(env!("CARGO_BIN_EXE_mrly-math"))
        .args(["math.counts.fill", r#"["23",3,3,2,2]"#])
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let seen: Value = serde_json::from_slice(&out.stdout).unwrap();
    let text = include_str!("../../tests/fixtures.json");
    let rows: Vec<Value> = serde_json::from_str(text).unwrap();
    let row = rows
        .iter()
        .find(|row| row["fn"] == "math::counts::fill")
        .unwrap();
    assert_eq!(seen, row["out"]);
}
