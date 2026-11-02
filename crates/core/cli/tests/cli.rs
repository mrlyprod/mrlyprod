//! The mrly-core binary, driven end to end: its fixture row and the shell every leaf binary shares.

use mrly_core::Image;
use mrly_core::name::sha;
use serde_json::{Value, json};
use std::io::{Read, Write};
use std::process::{Command, Output, Stdio};

#[test]
fn the_binary_replays_its_fixture_row() {
    let out = Command::new(env!("CARGO_BIN_EXE_mrly-core"))
        .args(["core.image.scale", r#"[{"width":2,"height":2,"colors":[[255,0,0,255],[0,255,0,255],[0,0,255,255],[255,255,255,255]]},3]"#])
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
        .find(|row| row["fn"] == "core::Image::scale")
        .unwrap();
    assert_eq!(seen, row["out"]);
}

fn mrly(words: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_mrly-core"))
        .args(words)
        .output()
        .expect("the mrly-core binary runs")
}

fn piped(words: &[&str], input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mrly-core"))
        .args(words)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the mrly-core binary runs");
    child
        .stdin
        .take()
        .expect("a stdin")
        .write_all(input.as_bytes())
        .expect("stdin takes the input");
    child.wait_with_output().expect("mrly-core finishes")
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).trim().to_string()
}

#[test]
fn an_image_that_does_not_fill_its_size_is_a_usage_error() {
    let out = mrly(&[
        "core.image.scale",
        r#"[{"width":2,"height":2,"colors":[[0,0,0,255]]},3]"#,
    ]);
    assert_eq!(out.status.code(), Some(2));
    assert!(out.stdout.is_empty());
    assert!(text(&out.stderr).contains("colors length must equal width * height."));
}

#[test]
fn a_library_error_exits_one_on_stderr() {
    let out = mrly(&["core.colors.from_hex", r#""nope""#]);
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty());
    assert_eq!(
        text(&out.stderr),
        "mrly-core: Hex code must be in format #RRGGBB or #RRGGBBAA"
    );
}

#[test]
fn list_names_every_function_with_its_types() {
    let out = mrly(&["list"]);
    assert!(out.status.success());
    let listed = String::from_utf8(out.stdout).unwrap();
    assert!(
        listed
            .lines()
            .any(|line| line == "core.colors.from_hex(hex: str) -> Color"),
        "list does not name core.colors.from_hex"
    );
    assert_eq!(
        listed.lines().count(),
        include_str!("../src/doors.gen.rs")
            .matches("    Function {")
            .count()
    );
}

#[test]
fn list_takes_a_prefix() {
    let out = mrly(&["list", "core.colors."]);
    assert!(out.status.success());
    let listed = String::from_utf8(out.stdout).unwrap();
    assert!(listed.lines().all(|line| line.starts_with("core.colors.")));
    assert!(listed.lines().count() > 1);
}

#[test]
fn help_and_version_print_and_exit_zero() {
    for words in [&[][..], &["help"], &["-h"], &["--help"]] {
        let out = mrly(words);
        assert!(out.status.success(), "{words:?}");
        let page = text(&out.stdout);
        assert!(
            page.contains("exit codes:") && page.contains("mrly-core list"),
            "{words:?}"
        );
    }
    let out = mrly(&["--version"]);
    assert!(out.status.success());
    assert_eq!(
        text(&out.stdout),
        format!("mrly-core {}", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn help_names_one_function_with_its_signature_and_docs() {
    let out = mrly(&["help", "core.colors.from_hex"]);
    assert!(out.status.success());
    let page = text(&out.stdout);
    let mut lines = page.lines();
    assert_eq!(
        lines.next(),
        Some("core.colors.from_hex(hex: str) -> Color")
    );
    assert_eq!(lines.next(), Some(""));
    assert!(lines.next().is_some_and(|line| !line.is_empty()));
}

#[test]
fn an_unknown_name_suggests_close_ones() {
    let out = mrly(&["core.colors.from_hexx", r##""#ff3d40""##]);
    assert_eq!(out.status.code(), Some(2));
    let said = text(&out.stderr);
    assert!(
        said.contains("no function named \"core.colors.from_hexx\""),
        "{said}"
    );
    assert!(said.contains("core.colors.from_hex?"), "{said}");
    assert!(!said.contains("door"), "{said}");
}

#[test]
fn a_wrong_count_names_the_signature() {
    let out = mrly(&["core.colors.from_hex", r##"["#ff3d40", 1]"##]);
    assert_eq!(out.status.code(), Some(2));
    let said = text(&out.stderr);
    assert!(
        said.contains("core.colors.from_hex takes 1 argument, got 2"),
        "{said}"
    );
    assert!(said.contains("core.colors.from_hex("), "{said}");
}

#[test]
fn stdin_holds_the_arguments_and_a_lone_value_is_the_one_argument() {
    let listed = piped(&["core.cell.moore", "-"], "[2]");
    assert!(listed.status.success(), "{}", text(&listed.stderr));
    let chained = piped(&["core.tensor.invert"], &text(&listed.stdout));
    assert!(chained.status.success(), "{}", text(&chained.stderr));
    let inverted: Value = serde_json::from_slice(&chained.stdout).expect("one json value");
    assert_eq!(inverted["data"]["U8"], json!([0, 0, 0, 0, 1, 0, 0, 0, 0]));
}

#[test]
fn a_function_of_no_arguments_never_waits_on_stdin() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mrly-core"))
        .arg("core.Colorizer.heat")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .expect("the mrly-core binary runs");
    let open = child.stdin.take();
    let start = std::time::Instant::now();
    while child.try_wait().expect("mrly-core runs").is_none() {
        if start.elapsed().as_secs() > 20 {
            child.kill().ok();
            panic!("mrly-core waited on an open stdin");
        }
        std::thread::yield_now();
    }
    drop(open);
    assert!(child.wait().expect("mrly-core finishes").success());
}

#[test]
fn a_negative_number_is_an_argument_and_a_double_dash_ends_the_options() {
    for words in [
        &["core.cell.moore", "-1"][..],
        &["core.cell.moore", "--", "-1"],
    ] {
        let out = mrly(words);
        let said = text(&out.stderr);
        assert!(
            said.contains("core.cell.moore argument 1: invalid value: integer `-1`"),
            "{words:?}: {said}"
        );
    }
}

#[test]
fn help_on_a_module_lists_its_functions() {
    let out = mrly(&["help", "core.colors"]);
    assert!(out.status.success(), "{}", text(&out.stderr));
    let listed = String::from_utf8(out.stdout).unwrap();
    assert!(listed.lines().count() > 1);
    assert!(listed.lines().all(|line| line.starts_with("core.colors.")));
}

#[test]
fn raw_prints_a_string_unquoted() {
    let quoted = mrly(&["core.colors.to_hex", "[[255, 61, 64, 255]]"]);
    assert_eq!(text(&quoted.stdout), r##""#ff3d40""##);
    let raw = mrly(&["core.colors.to_hex", "[[255, 61, 64, 255]]", "--raw"]);
    assert!(raw.status.success());
    assert_eq!(String::from_utf8(raw.stdout).unwrap(), "#ff3d40\n");
}

#[test]
fn a_closed_pipe_ends_quietly() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mrly-core"))
        .arg("list")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the mrly-core binary runs");
    let mut first = [0u8; 64];
    child
        .stdout
        .take()
        .expect("a stdout")
        .read_exact(&mut first)
        .expect("list prints");
    let out = child.wait_with_output().expect("mrly-core finishes");
    assert!(out.status.success(), "{}", text(&out.stderr));
    assert!(out.stderr.is_empty(), "{}", text(&out.stderr));
}

#[test]
fn help_leaves_out_the_rust_examples() {
    let out = mrly(&["help", "core.colors.from_hex"]);
    assert!(out.status.success());
    let page = text(&out.stdout);
    assert!(page.contains("# Errors"), "{page}");
    assert!(
        !page.contains("```") && !page.contains("mrly_core"),
        "{page}"
    );
}
