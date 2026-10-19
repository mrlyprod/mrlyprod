//! The stored fixture rows against what each unit returns today.

#[path = "common/rows.rs"]
mod rows;

use serde_json::Value;
use std::path::Path;

fn stored(module: &str) -> Vec<Value> {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join(format!("{module}.json"));
    let text = std::fs::read_to_string(&file)
        .unwrap_or_else(|_| panic!("{} is missing; run the fixtures example", file.display()));
    serde_json::from_str(&text).unwrap()
}

fn replays(module: &str) {
    let saved = stored(module);
    let fresh = rows::rows(module);
    assert_eq!(
        saved.len(),
        fresh.len(),
        "{module}.json holds {} rows, the example writes {}",
        saved.len(),
        fresh.len()
    );
    for (index, (was, now)) in saved.iter().zip(&fresh).enumerate() {
        let name = was["fn"].as_str().unwrap_or("?");
        assert_eq!(
            was["fn"], now["fn"],
            "{module}.json row {index} is not {name}"
        );
        assert_eq!(
            was["in"], now["in"],
            "{module}.json row {index} {name}: the arguments drifted"
        );
        assert_eq!(
            was["note"], now["note"],
            "{module}.json row {index} {name}: the note drifted"
        );
        assert_eq!(
            was["out"], now["out"],
            "{module}.json row {index} {name}: the stored value is not what the crate returns"
        );
    }
}

#[test]
fn core_replays() {
    replays("core");
}

#[test]
fn num_replays() {
    replays("num");
}

#[test]
fn math_replays() {
    replays("math");
}

#[test]
fn tile_replays() {
    replays("tile");
}

#[test]
fn life_replays() {
    replays("life");
}

#[test]
fn font_replays() {
    replays("font");
}
