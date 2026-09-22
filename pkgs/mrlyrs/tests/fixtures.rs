#[path = "common/rows.rs"]
mod rows;

use rows::sha256;
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
        assert_eq!(was["fn"], now["fn"], "{module}.json row {index} is not {name}");
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
fn the_sha256_holds_its_standard_vectors() {
    assert_eq!(
        sha256::hex(b""),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    assert_eq!(
        sha256::hex(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
}

#[test]
fn the_six_files_hold_thirty_three_rows() {
    let counted: Vec<usize> = rows::MODULES.iter().map(|m| stored(m).len()).collect();
    assert_eq!(counted, vec![4, 6, 7, 5, 6, 5]);
    assert_eq!(counted.iter().sum::<usize>(), 33);
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
fn gen_replays() {
    replays("gen");
}

#[test]
fn life_replays() {
    replays("life");
}

#[test]
fn font_replays() {
    replays("font");
}
