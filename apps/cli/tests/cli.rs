use std::io::Write;
use std::process::{Command, Stdio};

fn mrlycli() -> Command {
    Command::new(env!("CARGO_BIN_EXE_mrlycli"))
}

fn piped(args: &[&str], input: &str) -> Vec<u8> {
    let mut child = mrlycli()
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let out = child.wait_with_output().unwrap();
    assert!(out.status.success());
    out.stdout
}

#[test]
fn describe_lists_the_surface() {
    let out = mrlycli().arg("describe").output().unwrap();
    assert!(out.status.success());
    let surface = mrlycore::json::parse(std::str::from_utf8(&out.stdout).unwrap()).unwrap();
    assert!(surface["version"].is_string());
    assert!(surface["apps"]
        .as_array()
        .is_some_and(|apps| !apps.is_empty()));
}

#[test]
fn run_replays_to_the_final_frame() {
    let script = concat!(
        "{\"verb\":\"nav.open\",\"args\":{\"app\":\"snake\"}}\n",
        "{\"verb\":\"snake.reset\",\"args\":{\"seed\":7},\"now\":0}\n"
    );
    let stdout = piped(&["run"], script);
    let frame = mrlycore::json::parse(std::str::from_utf8(&stdout).unwrap()).unwrap();
    assert_eq!(frame["view"]["state"]["seed"], 7);
}

#[test]
fn repl_routes_and_prints_frames() {
    let input = concat!(
        "nav.open {\"app\":\"calculator\"}\n",
        "calculator.digit {\"d\":4}\n",
        ":quit\n"
    );
    let stdout = piped(&["repl"], input);
    let text = String::from_utf8_lossy(&stdout);
    assert!(text.contains("\"app\": \"calculator\""));
}

#[test]
fn render_draws_colored_blocks() {
    let script = concat!(
        "{\"verb\":\"nav.open\",\"args\":{\"app\":\"snake\"}}\n",
        "{\"verb\":\"snake.reset\",\"args\":{\"seed\":7},\"now\":0}\n"
    );
    let stdout = piped(&["render"], script);
    let text = String::from_utf8_lossy(&stdout);
    assert!(text.contains("snake"));
    assert!(text.contains('\u{2580}'));
    assert!(text.contains("\x1b[38;2;"));
}

#[test]
fn replay_surfaces_a_failed_call() {
    let mut child = mrlycli()
        .arg("run")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"{\"verb\":\"snake.ghost\",\"args\":{}}\n")
        .unwrap();
    let out = child.wait_with_output().unwrap();
    let err = String::from_utf8_lossy(&out.stderr);
    assert!(err.contains("snake.ghost"));
}

#[test]
fn run_facts_collapses_grids() {
    let script = concat!(
        "{\"verb\":\"nav.open\",\"args\":{\"app\":\"snake\"}}\n",
        "{\"verb\":\"snake.reset\",\"args\":{\"seed\":7},\"now\":0}\n"
    );
    let stdout = piped(&["run", "--facts"], script);
    let frame = mrlycore::json::parse(std::str::from_utf8(&stdout).unwrap()).unwrap();
    let state = &frame["view"]["state"];
    assert_eq!(state["seed"], 7);
    assert!(state["frame"]["rows"]
        .as_str()
        .is_some_and(|s| s.starts_with("grid ")));
}

#[test]
fn shot_writes_a_png_of_the_focused_frame() {
    let dir = std::env::temp_dir().join(format!("mrlycli_shot_{}.png", std::process::id()));
    let script = concat!(
        "{\"verb\":\"nav.open\",\"args\":{\"app\":\"snake\"}}\n",
        "{\"verb\":\"snake.reset\",\"args\":{\"seed\":7},\"now\":0}\n"
    );
    let mut child = mrlycli()
        .args(["shot", "--out", dir.to_str().unwrap()])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(script.as_bytes())
        .unwrap();
    assert!(child.wait().unwrap().success());
    let bytes = std::fs::read(&dir).unwrap();
    assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n");
    std::fs::remove_file(&dir).ok();
}

#[test]
fn shot_fails_on_a_frameless_app() {
    let mut child = mrlycli()
        .args(["shot", "--out", "/dev/null"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"{\"verb\":\"nav.open\",\"args\":{\"app\":\"calculator\"}}\n")
        .unwrap();
    assert!(!child.wait().unwrap().success());
}

#[test]
fn face_writes_a_png_for_a_frameless_app() {
    let dir = std::env::temp_dir().join(format!("mrlycli_face_{}.png", std::process::id()));
    let mut child = mrlycli()
        .args(["face", "--out", dir.to_str().unwrap()])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"{\"verb\":\"nav.open\",\"args\":{\"app\":\"calculator\"}}\n")
        .unwrap();
    assert!(child.wait().unwrap().success());
    let bytes = std::fs::read(&dir).unwrap();
    assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n");
    std::fs::remove_file(&dir).ok();
}

#[test]
fn face_writes_a_png_with_a_canvas() {
    let dir = std::env::temp_dir().join(format!("mrlycli_face_live_{}.png", std::process::id()));
    let script = concat!(
        "{\"verb\":\"nav.open\",\"args\":{\"app\":\"snake\"}}\n",
        "{\"verb\":\"snake.reset\",\"args\":{\"seed\":7},\"now\":0}\n"
    );
    let mut child = mrlycli()
        .args(["face", "--out", dir.to_str().unwrap()])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(script.as_bytes())
        .unwrap();
    assert!(child.wait().unwrap().success());
    let bytes = std::fs::read(&dir).unwrap();
    assert_eq!(&bytes[..8], b"\x89PNG\r\n\x1a\n");
    std::fs::remove_file(&dir).ok();
}

#[test]
fn verbs_lists_one_app_with_args() {
    let out = mrlycli().args(["verbs", "snake"]).output().unwrap();
    assert!(out.status.success());
    let text = String::from_utf8_lossy(&out.stdout);
    assert!(text.contains("snake.step"));
    assert!(text.contains("n:int"));
}

#[test]
fn verbs_rejects_an_unknown_app() {
    let out = mrlycli().args(["verbs", "ghost"]).output().unwrap();
    assert!(!out.status.success());
}

#[test]
fn render_falls_back_to_json_without_a_grid() {
    let script = "{\"verb\":\"nav.open\",\"args\":{\"app\":\"calculator\"}}\n";
    let stdout = piped(&["render"], script);
    let text = String::from_utf8_lossy(&stdout);
    assert!(text.contains("\"tick\""));
    assert!(!text.contains('\u{2580}'));
}

#[test]
fn drive_plays_the_menu_screenplay() {
    let status = mrlycli()
        .args(["drive", "tests/screenplays/menu.jsonl"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .unwrap();
    assert!(status.success());
}

#[test]
fn drive_plays_the_snake_screenplay() {
    let status = mrlycli()
        .args(["drive", "tests/screenplays/snake.jsonl"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .unwrap();
    assert!(status.success());
}

#[test]
fn drive_refuses_a_wrong_route() {
    let mut child = mrlycli()
        .args(["drive"])
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    child
        .stdin
        .take()
        .unwrap()
        .write_all(b"{\"assert\": {\"route\": \"snake\"}}\n")
        .unwrap();
    assert!(!child.wait().unwrap().success());
}

#[test]
fn monkey_survives_a_menu_mash() {
    let status = mrlycli()
        .args(["monkey", "menu", "--seed", "3", "--steps", "40"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .unwrap();
    assert!(status.success());
}

#[test]
fn monkey_survives_a_snake_mash() {
    let status = mrlycli()
        .args(["monkey", "snake", "--seed", "3", "--steps", "40"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .unwrap();
    assert!(status.success());
}
