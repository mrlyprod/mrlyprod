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
fn list_shows_the_surface() {
    let out = mrlycli().arg("list").output().unwrap();
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
    assert!(state["cells"]["ids"]
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
fn drive_plays_the_twenty48_screenplay() {
    let status = mrlycli()
        .args(["drive", "tests/screenplays/twenty48.jsonl"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .unwrap();
    assert!(status.success());
}

#[test]
fn drive_plays_the_mandelbrot_screenplay() {
    let status = mrlycli()
        .args(["drive", "tests/screenplays/mandelbrot.jsonl"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .unwrap();
    assert!(status.success());
}

#[test]
fn drive_plays_the_settings_screenplay() {
    let status = mrlycli()
        .args(["drive", "tests/screenplays/settings.jsonl"])
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
fn drive_plays_the_chess_screenplay() {
    let status = mrlycli()
        .args(["drive", "tests/screenplays/chess.jsonl"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .unwrap();
    assert!(status.success());
}

#[test]
fn every_screenplay_is_in_the_gate() {
    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/screenplays");
    let gate = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/cli.rs"),
    )
    .unwrap();
    for entry in std::fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        assert!(
            gate.contains(&format!("tests/screenplays/{name}")),
            "{name} is not played by any test"
        );
    }
}

fn served(lines: &[&str]) -> Vec<mrlycore::Json> {
    let input = lines.join("\n") + "\n";
    let out = piped(&["mcp"], &input);
    std::str::from_utf8(&out)
        .unwrap()
        .lines()
        .map(|line| mrlycore::json::parse(line).unwrap())
        .collect()
}

#[test]
fn mcp_shakes_hands_and_stays_quiet_on_notifications() {
    let replies = served(&[
        r#"{"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2025-06-18"}}"#,
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#,
    ]);
    assert_eq!(replies.len(), 2);
    assert_eq!(
        replies[0]["result"]["protocolVersion"],
        mrlycore::json!("2025-06-18")
    );
    assert_eq!(
        replies[0]["result"]["serverInfo"]["name"],
        mrlycore::json!("mrly")
    );
    assert_eq!(replies[1]["result"], mrlycore::json!({}));
}

#[test]
fn mcp_lists_every_verb_as_a_tool_plus_the_read_door() {
    let replies = served(&[r#"{"jsonrpc":"2.0","id":0,"method":"tools/list"}"#]);
    let tools = replies[0]["result"]["tools"].as_array().unwrap();
    let names: Vec<&str> = tools.iter().filter_map(|t| t["name"].as_str()).collect();
    assert!(names.contains(&"nav_open"));
    assert!(names.contains(&"snake_turn"));
    assert!(names.contains(&"mrly_read"));
    assert!(names.iter().all(|n| !n.contains('.')));
    let turn = tools
        .iter()
        .find(|t| t["name"] == mrlycore::json!("snake_turn"))
        .unwrap();
    assert!(turn["inputSchema"]["properties"]["dir"]["enum"]
        .as_array()
        .is_some());
}

#[test]
fn mcp_calls_journal_and_errors_stay_inside_results() {
    let replies = served(&[
        r#"{"jsonrpc":"2.0","id":0,"method":"tools/call","params":{"name":"nav_open","arguments":{"app":"snake","now":1000}}}"#,
        r#"{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"nav_open","arguments":{"app":"ghost","now":2000}}}"#,
    ]);
    let fine: Vec<&mrlycore::Json> = replies.iter().filter(|r| !r["id"].is_null()).collect();
    assert_eq!(fine[0]["result"]["isError"], mrlycore::json!(false));
    assert_eq!(fine[1]["result"]["isError"], mrlycore::json!(true));
    let text = fine[1]["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("no such app"));
}

#[test]
fn mcp_announces_a_moving_surface() {
    let replies = served(&[
        r#"{"jsonrpc":"2.0","id":0,"method":"tools/call","params":{"name":"nav_open","arguments":{"app":"timer","now":1000}}}"#,
        r#"{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"timer_mode","arguments":{"mode":"stopwatch","now":2000}}}"#,
    ]);
    let moved = replies
        .iter()
        .any(|r| r["method"] == mrlycore::json!("notifications/tools/list_changed"));
    assert!(moved);
}

#[test]
fn mcp_reads_ride_resources_and_the_read_tool() {
    let replies = served(&[
        r#"{"jsonrpc":"2.0","id":0,"method":"resources/read","params":{"uri":"mrly://read/snake/dir"}}"#,
        r#"{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{"name":"mrly_read","arguments":{"path":"snake/dir"}}}"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"resources/list"}"#,
    ]);
    let by_resource = replies[0]["result"]["contents"][0]["text"]
        .as_str()
        .unwrap();
    let by_tool = replies[1]["result"]["content"][0]["text"].as_str().unwrap();
    assert_eq!(by_resource, by_tool);
    let listed = replies[2]["result"]["resources"].as_array().unwrap();
    assert!(listed
        .iter()
        .any(|r| r["uri"] == mrlycore::json!("mrly://read/snake")));
    assert!(!listed.iter().any(|r| r["name"] == mrlycore::json!("menu")));
}

#[test]
fn mcp_refuses_malformed_protocol_with_codes() {
    let replies = served(&[
        r#"{"jsonrpc":"2.0","id":0,"method":"prompts/list"}"#,
        r#"[{"jsonrpc":"2.0","id":1,"method":"ping"}]"#,
        r#"{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"snake_step","arguments":{"n":1.5}}}"#,
    ]);
    assert_eq!(replies[0]["error"]["code"], mrlycore::json!(-32601));
    assert_eq!(replies[1]["error"]["code"], mrlycore::json!(-32600));
    let parse = replies
        .iter()
        .find(|r| r["error"]["code"] == mrlycore::json!(-32700))
        .unwrap();
    assert!(parse["id"].is_null());
    assert!(parse["error"]["message"]
        .as_str()
        .unwrap()
        .contains("floats"));
}
