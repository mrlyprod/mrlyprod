//! The mrly binary, driven end to end.

use mrly::core::Image;
use mrly::core::name::sha;
use serde_json::{Value, json};
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Output, Stdio};

fn stored(module: &str, path: &str) -> Value {
    let file = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../crates/mrly/fixtures")
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

fn piped(words: &[&str], input: &str) -> Output {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mrly"))
        .args(words)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the mrly binary runs");
    child
        .stdin
        .take()
        .expect("a stdin")
        .write_all(input.as_bytes())
        .expect("stdin takes the input");
    child.wait_with_output().expect("mrly finishes")
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).trim().to_string()
}

fn printed(name: &str, args: &str) -> Value {
    let out = mrly(&[name, args]);
    assert!(out.status.success(), "{name} {args}: {}", text(&out.stderr));
    serde_json::from_slice(&out.stdout).expect("one json value")
}

fn pixels(value: &Value) -> Value {
    let image: Image = serde_json::from_value(value.clone()).expect("an image");
    json!({
        "shape": [image.height, image.width],
        "sha256": sha::hex(&image.colors.concat()),
    })
}

#[test]
fn every_module_replays_its_fixture() {
    let corners = r#"{"width":2,"height":2,"colors":[[255,0,0,255],[0,255,0,255],[0,0,255,255],[255,255,255,255]]}"#;
    let scale = format!("[{corners},3]");
    let cases: [(&str, &str, &str, &str, bool); 6] = [
        (
            "core.image.scale",
            &scale,
            "core",
            "core::Image::scale",
            true,
        ),
        (
            "num.factor.factorial",
            "[25]",
            "num",
            "num::factor::factorial",
            false,
        ),
        (
            "math.counts.fill",
            r#"["23",3,3,2,2]"#,
            "math",
            "math::counts::fill",
            false,
        ),
        (
            "tile.background",
            "[1,2,2]",
            "tile",
            "tile::background",
            true,
        ),
        (
            "life.counts",
            r#"["Primes",8,false,false]"#,
            "life",
            "life::counts",
            false,
        ),
        ("font.path", r#"["M"]"#, "font", "font::path", false),
    ];
    for (name, args, module, path, image) in cases {
        let value = printed(name, args);
        let seen = if image { pixels(&value) } else { value };
        assert_eq!(seen, stored(module, path), "{name}");
    }
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
    let out = mrly(&["num.factor.factorial", "[40]"]);
    assert_eq!(out.status.code(), Some(1));
    assert!(out.stdout.is_empty());
    assert_eq!(text(&out.stderr), "mrly: a factorial of 40 passes u128");
}

#[test]
fn list_names_every_function_with_its_types() {
    let out = mrly(&["list"]);
    assert!(out.status.success());
    let listed = String::from_utf8(out.stdout).unwrap();
    assert!(
        listed
            .lines()
            .any(|line| line == "num.factor.gcd(a: u128, b: u128) -> u128"),
        "list does not name num.factor.gcd"
    );
    assert_eq!(listed.lines().count(), mrly_cli::list().count());
}

#[test]
fn list_takes_a_prefix() {
    let out = mrly(&["list", "num.factor."]);
    assert!(out.status.success());
    let listed = String::from_utf8(out.stdout).unwrap();
    assert!(listed.lines().all(|line| line.starts_with("num.factor.")));
    assert!(listed.lines().count() > 1);
}

#[test]
fn help_and_version_print_and_exit_zero() {
    for words in [&[][..], &["help"], &["-h"], &["--help"]] {
        let out = mrly(words);
        assert!(out.status.success(), "{words:?}");
        let page = text(&out.stdout);
        assert!(
            page.contains("exit codes:") && page.contains("mrly list"),
            "{words:?}"
        );
    }
    let out = mrly(&["--version"]);
    assert!(out.status.success());
    assert_eq!(
        text(&out.stdout),
        format!("mrly {}", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn help_names_one_function_with_its_signature_and_docs() {
    let out = mrly(&["help", "num.factor.gcd"]);
    assert!(out.status.success());
    let page = text(&out.stdout);
    let mut lines = page.lines();
    assert_eq!(
        lines.next(),
        Some("num.factor.gcd(a: u128, b: u128) -> u128")
    );
    assert_eq!(lines.next(), Some(""));
    assert!(lines.next().is_some_and(|line| !line.is_empty()));
}

#[test]
fn an_unknown_name_suggests_close_ones() {
    let out = mrly(&["num.factor.factorise", "[5]"]);
    assert_eq!(out.status.code(), Some(2));
    let said = text(&out.stderr);
    assert!(
        said.contains("no function named \"num.factor.factorise\""),
        "{said}"
    );
    assert!(said.contains("num.factor.factorize"), "{said}");
    assert!(!said.contains("door"), "{said}");
}

#[test]
fn a_wrong_count_names_the_signature() {
    let out = mrly(&["num.factor.factorial", "[1, 2]"]);
    assert_eq!(out.status.code(), Some(2));
    let said = text(&out.stderr);
    assert!(
        said.contains("num.factor.factorial takes 1 argument, got 2"),
        "{said}"
    );
    assert!(said.contains("num.factor.factorial("), "{said}");
}

#[test]
fn stdin_holds_the_arguments_and_a_lone_value_is_the_one_argument() {
    let listed = piped(&["num.factor.gcd", "-"], r#"["1071", "462"]"#);
    assert!(listed.status.success(), "{}", text(&listed.stderr));
    assert_eq!(text(&listed.stdout), r#""21""#);
    let cell = mrly(&["math.two.carpet", "[3, 1]"]);
    let chained = piped(&["math.cell.models.invert"], &text(&cell.stdout));
    assert!(chained.status.success(), "{}", text(&chained.stderr));
    let inverted: Value = serde_json::from_slice(&chained.stdout).expect("one json value");
    assert_eq!(
        inverted["cell"]["types"]["data"]["U8"],
        json!([0, 0, 0, 0, 1, 0, 0, 0, 0])
    );
}

#[test]
fn a_function_of_no_arguments_never_waits_on_stdin() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mrly"))
        .arg("core.Colorizer.heat")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()
        .expect("the mrly binary runs");
    let open = child.stdin.take();
    let start = std::time::Instant::now();
    while child.try_wait().expect("mrly runs").is_none() {
        if start.elapsed().as_secs() > 20 {
            child.kill().ok();
            panic!("mrly waited on an open stdin");
        }
        std::thread::yield_now();
    }
    drop(open);
    assert!(child.wait().expect("mrly finishes").success());
}

#[test]
fn a_negative_number_is_an_argument_and_a_double_dash_ends_the_options() {
    for words in [
        &["math.cell.models.dtype_for", "-1"][..],
        &["math.cell.models.dtype_for", "--", "-1"],
    ] {
        let out = mrly(words);
        assert!(out.status.success(), "{words:?}: {}", text(&out.stderr));
        assert_eq!(text(&out.stdout), r#""U8""#);
    }
}

#[test]
fn help_on_a_module_lists_its_functions() {
    let out = mrly(&["help", "num.factor"]);
    assert!(out.status.success(), "{}", text(&out.stderr));
    let listed = String::from_utf8(out.stdout).unwrap();
    assert!(listed.lines().count() > 1);
    assert!(listed.lines().all(|line| line.starts_with("num.factor.")));
}

#[test]
fn raw_prints_a_string_unquoted() {
    let quoted = mrly(&["num.factor.gcd", r#"["1071", "462"]"#]);
    assert_eq!(text(&quoted.stdout), r#""21""#);
    let raw = mrly(&["num.factor.gcd", r#"["1071", "462"]"#, "--raw"]);
    assert!(raw.status.success());
    assert_eq!(String::from_utf8(raw.stdout).unwrap(), "21\n");
}

#[test]
fn a_closed_pipe_ends_quietly() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_mrly"))
        .arg("list")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the mrly binary runs");
    let mut first = [0u8; 64];
    child
        .stdout
        .take()
        .expect("a stdout")
        .read_exact(&mut first)
        .expect("list prints");
    let out = child.wait_with_output().expect("mrly finishes");
    assert!(out.status.success(), "{}", text(&out.stderr));
    assert!(out.stderr.is_empty(), "{}", text(&out.stderr));
}

#[test]
fn help_leaves_out_the_rust_examples() {
    let out = mrly(&["help", "math.two.carpet"]);
    assert!(out.status.success());
    let page = text(&out.stdout);
    assert!(page.contains("# Errors"), "{page}");
    assert!(
        !page.contains("```") && !page.contains("mrly_math"),
        "{page}"
    );
}
