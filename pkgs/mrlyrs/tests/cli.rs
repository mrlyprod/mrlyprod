#[path = "common/sha256.rs"]
mod sha256;

use serde_json::Value;
use std::path::Path;
use std::process::{Command, Output};

fn stored(module: &str, path: &str) -> Value {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures")
        .join(format!("{module}.json"));
    let text = std::fs::read_to_string(&file)
        .unwrap_or_else(|_| panic!("{} is missing; run the fixtures example", file.display()));
    let rows: Vec<Value> = serde_json::from_str(&text).unwrap();
    rows.into_iter()
        .find(|row| row["fn"] == path)
        .unwrap_or_else(|| panic!("{module}.json holds no {path} row"))["out"]
        .clone()
}

fn mrly(words: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_mrly"))
        .args(words)
        .output()
        .expect("the mrly binary runs")
}

fn printed(door: &str, args: &str) -> Value {
    let out = mrly(&[door, args]);
    assert!(
        out.status.success(),
        "{door} {args}: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).expect("the door prints one json value")
}

fn hashed(value: &Value) -> Value {
    let bytes: Vec<u8> = serde_json::from_value(value.clone()).expect("a list of bytes");
    Value::String(sha256::hex(&bytes))
}

#[test]
fn core_replays_its_png() {
    let colors = "[[[255,0,0,255],[0,255,0,255],[0,0,255,255],[255,255,255,255]],2,2,3]";
    assert_eq!(
        hashed(&printed("core.codec.png", colors)),
        stored("core", "core::png")
    );
}

#[test]
fn num_replays_its_factorial() {
    assert_eq!(
        printed("num.factor.factorial", "[25]"),
        stored("num", "num::factor::factorial")
    );
}

#[test]
fn math_replays_its_fill() {
    assert_eq!(
        printed("math.counts.fill", r#"["23",3,3,2,2]"#),
        stored("math", "math::counts::fill")
    );
}

#[test]
fn gen_replays_its_background() {
    assert_eq!(
        hashed(&printed("gen.background", "[1,2,2]")),
        stored("gen", "gen::background")
    );
}

#[test]
fn life_replays_its_counts() {
    assert_eq!(
        printed("life.counts", r#"["Primes",8,false,false]"#),
        stored("life", "life::counts")
    );
}

#[test]
fn font_replays_its_path() {
    assert_eq!(printed("font.path", r#"["M"]"#), stored("font", "font::path"));
}

#[test]
fn a_rust_error_exits_one_on_stderr() {
    let out = mrly(&["num.factor.factorial", "[40]"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty());
    assert_eq!(
        String::from_utf8_lossy(&out.stderr).trim(),
        "a factorial of 40 passes u128"
    );
}

#[test]
fn list_names_every_door_with_its_types() {
    let out = mrly(&["list"]);
    assert!(out.status.success());
    let text = String::from_utf8(out.stdout).unwrap();
    assert!(
        text.lines()
            .any(|line| line == "num.factor.gcd(a: u128, b: u128) -> u128"),
        "list does not name num.factor.gcd"
    );
}
