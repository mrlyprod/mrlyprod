use mrly::os::kernel::{Call, Iden, Os};
use serde_json::{json, to_string_pretty, Value};
use std::io::{BufRead, Read};

const PROMPT: &str = "mrly> ";
const MAX_SIDE: usize = 64;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        None | Some("repl") => repl(),
        Some("run") => run(&args[2..]),
        Some("render") => render(args.get(2).map(String::as_str)),
        Some("shot") => shot(&args[2..]),
        Some("describe") => describe(),
        Some("verbs") => verbs(args.get(2).map(String::as_str)),
        Some("help") | Some("-h") | Some("--help") => usage(),
        Some(_) => {
            usage();
            std::process::exit(2);
        }
    }
}

fn usage() {
    eprintln!("mrlycli - a terminal face for the mrly kernel");
    eprintln!();
    eprintln!("usage:");
    eprintln!("  mrlycli [repl]        interactive session (default)");
    eprintln!(
        "  mrlycli run [file]    replay a call script, print the final frame (--facts trims grids)"
    );
    eprintln!("  mrlycli render [file] draw the final frame as colored blocks");
    eprintln!("  mrlycli shot [file]   write the final frame as a PNG (--out path)");
    eprintln!("  mrlycli describe      print the kernel surface as JSON");
    eprintln!("  mrlycli verbs [app]   list apps, or one app's verbs and args");
    eprintln!("  mrlycli help          show this message");
    eprintln!();
    eprintln!("repl:");
    eprintln!("  verb [json]          act, e.g. nav.open {{\"app\":\"snake\"}}");
    eprintln!("  :help :frame :render :verbs :shot :describe :apps :open <app> :reset :quit");
}

// BOOT

fn build() -> Os {
    let mut os = Os::new(Iden::new("guest"));
    for app in mrly::net::registry::catalogue() {
        os = os.install(app);
    }
    os
}

// DESCRIBE

fn describe() {
    println!("{}", to_string_pretty(&build().describe()).unwrap());
}

// VERBS

fn verbs(app: Option<&str>) {
    if !list_verbs(&build().describe(), app) {
        eprintln!("! no such app: {}", app.unwrap_or(""));
        std::process::exit(1);
    }
}

fn list_verbs(surface: &Value, app: Option<&str>) -> bool {
    let empty = Vec::new();
    let groups = surface["verbs"].as_array().unwrap_or(&empty);
    match app {
        None => {
            for group in groups {
                let name = group["app"].as_str().unwrap_or("");
                let count = group["verbs"].as_array().map_or(0, Vec::len);
                println!("{name} ({count})");
            }
            true
        }
        Some(want) => match groups.iter().find(|g| g["app"].as_str() == Some(want)) {
            Some(group) => {
                for verb in group["verbs"].as_array().unwrap_or(&empty) {
                    println!("{}", verb_line(verb));
                }
                true
            }
            None => false,
        },
    }
}

fn verb_line(verb: &Value) -> String {
    let name = verb["verb"].as_str().unwrap_or("");
    let args = verb["args"]
        .as_object()
        .map(|m| {
            m.iter()
                .map(|(k, v)| format!("{k}:{}", v.as_str().unwrap_or("?")))
                .collect::<Vec<_>>()
                .join(" ")
        })
        .unwrap_or_default();
    if args.is_empty() {
        name.to_string()
    } else {
        format!("{name}  {{ {args} }}")
    }
}

// RUN

fn call_from(wire: &Value) -> Call {
    let verb = wire["verb"].as_str().unwrap_or("").to_string();
    let args = if wire["args"].is_object() {
        wire["args"].clone()
    } else {
        json!({})
    };
    let mut call = Call::new(&verb, args);
    if let Some(now) = wire["now"].as_i64() {
        call = call.at(now);
    }
    call
}

fn replay(path: Option<&str>) -> Os {
    let text = match path {
        Some(p) => std::fs::read_to_string(p).unwrap_or_default(),
        None => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf).ok();
            buf
        }
    };
    let wires: Vec<Value> = if text.trim_start().starts_with('[') {
        match serde_json::from_str::<Value>(&text) {
            Ok(Value::Array(items)) => items,
            _ => Vec::new(),
        }
    } else {
        text.lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .filter_map(|line| serde_json::from_str::<Value>(line).ok())
            .collect()
    };
    let mut os = build();
    for wire in &wires {
        let call = call_from(wire);
        let verb = call.verb.clone();
        let out = os.act(call);
        if !out.ok {
            let note = out.note.as_deref().unwrap_or("failed");
            eprintln!("! {verb}: {note}");
        }
    }
    os
}

fn run(args: &[String]) {
    let mut facts = false;
    let mut path: Option<&str> = None;
    for a in args {
        match a.as_str() {
            "--facts" | "-f" => facts = true,
            other => path = Some(other),
        }
    }
    let mut frame = replay(path).frame().to_json();
    if facts {
        collapse(&mut frame);
    }
    println!("{}", to_string_pretty(&frame).unwrap());
}

fn collapse(value: &mut Value) {
    match value {
        Value::Array(items) => {
            if items.first().is_some_and(Value::is_array) {
                let rows = items.len();
                let cols = items[0].as_array().map_or(0, Vec::len);
                *value = Value::String(format!("grid {rows}x{cols}"));
            } else {
                for item in items {
                    collapse(item);
                }
            }
        }
        Value::Object(map) => {
            for (_, v) in map.iter_mut() {
                collapse(v);
            }
        }
        _ => {}
    }
}

// REPL

fn repl() {
    let mut os = build();
    let mut visual = false;
    emit(&os, visual);
    eprint!("{PROMPT}");
    let stdin = std::io::stdin();
    for line in stdin.lock().lines() {
        let Ok(line) = line else { break };
        let line = line.trim();
        if let Some(meta) = line.strip_prefix(':') {
            let mut it = meta.splitn(2, char::is_whitespace);
            let cmd = it.next().unwrap_or("");
            let arg = it.next().unwrap_or("").trim();
            match cmd {
                "quit" | "q" => break,
                "help" => meta_help(),
                "frame" => emit(&os, visual),
                "render" => {
                    visual = !visual;
                    eprintln!("render: {}", if visual { "visual" } else { "facts" });
                    emit(&os, visual);
                }
                "describe" => eprintln!("{}", to_string_pretty(&os.describe()).unwrap()),
                "verbs" => {
                    let app = if arg.is_empty() {
                        os.frame().route.map(|r| r.app)
                    } else {
                        Some(arg.to_string())
                    };
                    if !list_verbs(&os.describe(), app.as_deref()) {
                        eprintln!("! no such app: {arg}");
                    }
                }
                "shot" => {
                    let out = if arg.is_empty() { "shot.png" } else { arg };
                    let app = os.frame().route.map(|r| r.app).unwrap_or_default();
                    match os.snapshot(&app) {
                        Ok(bytes) => match std::fs::write(out, &bytes) {
                            Ok(()) => eprintln!("shot: {app} -> {out} ({} bytes)", bytes.len()),
                            Err(e) => eprintln!("! write failed: {e}"),
                        },
                        Err(e) => eprintln!("! {app}: {e}"),
                    }
                }
                "apps" => eprintln!("{}", os.catalogue().join(", ")),
                "open" => match os.open(arg) {
                    Ok(()) => emit(&os, visual),
                    Err(e) => eprintln!("! {e}"),
                },
                "reset" => {
                    os = build();
                    emit(&os, visual);
                }
                _ => eprintln!("? unknown :{cmd} (try :help)"),
            }
        } else if !line.is_empty() {
            let mut it = line.splitn(2, char::is_whitespace);
            let verb = it.next().unwrap_or("");
            let rest = it.next().unwrap_or("").trim();
            let args = if rest.is_empty() {
                Some(json!({}))
            } else {
                match serde_json::from_str::<Value>(rest) {
                    Ok(v) => Some(v),
                    Err(e) => {
                        eprintln!("! bad args: {e}");
                        None
                    }
                }
            };
            if let Some(args) = args {
                os.act(Call::new(verb, args).at(now_ms()));
                emit(&os, visual);
            }
        }
        eprint!("{PROMPT}");
    }
    eprintln!();
}

fn meta_help() {
    eprintln!(":help              this message");
    eprintln!(":frame             reprint the current frame");
    eprintln!(":render            toggle visual blocks / raw facts");
    eprintln!(":describe          print the kernel surface");
    eprintln!(":verbs [app]       verbs and args (current app by default)");
    eprintln!(":shot [path]       write the current frame as a PNG");
    eprintln!(":apps              list installed apps");
    eprintln!(":open <app>        open an app");
    eprintln!(":reset             boot a fresh session");
    eprintln!(":quit :q           exit");
    eprintln!("verb [json]        act, e.g. calculator.digit {{\"d\":4}}");
}

fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

fn emit(os: &Os, visual: bool) {
    let frame = os.frame();
    let json = frame.to_json();
    if visual {
        println!("{}", paint(&json));
    } else {
        println!("{}", to_string_pretty(&json).unwrap());
    }
    if let Some(last) = &frame.last {
        if !last.ok {
            if let Some(note) = &last.note {
                eprintln!("! {note}");
            }
        }
    }
    if let Some(view) = &frame.view {
        let names: Vec<&str> = view.actions.iter().map(|v| v.name.as_str()).collect();
        if !names.is_empty() {
            eprintln!("verbs: {}", names.join(", "));
        }
    }
}

// RENDER

fn render(path: Option<&str>) {
    println!("{}", paint(&replay(path).frame().to_json()));
}

// SHOT

fn shot(args: &[String]) {
    let mut out = "shot.png".to_string();
    let mut path: Option<&str> = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--out" | "-o" => {
                if let Some(p) = it.next() {
                    out = p.clone();
                }
            }
            other => path = Some(other),
        }
    }
    let os = replay(path);
    let app = os.frame().route.map(|r| r.app).unwrap_or_default();
    match os.snapshot(&app) {
        Ok(bytes) => match std::fs::write(&out, &bytes) {
            Ok(()) => eprintln!("shot: {app} -> {out} ({} bytes)", bytes.len()),
            Err(e) => {
                eprintln!("! write failed: {e}");
                std::process::exit(1);
            }
        },
        Err(e) => {
            eprintln!("! {app}: {e}");
            std::process::exit(1);
        }
    }
}

fn paint(frame: &Value) -> String {
    let grid = &frame["view"]["state"]["frame"];
    match (grid["rows"].as_array(), grid["palette"].as_array()) {
        (Some(rows), Some(palette)) if !rows.is_empty() => {
            blocks(frame["view"]["app"].as_str().unwrap_or(""), rows, palette)
        }
        _ => to_string_pretty(frame).unwrap(),
    }
}

fn blocks(app: &str, rows: &[Value], palette: &[Value]) -> String {
    let colors: Vec<(u8, u8, u8)> = palette
        .iter()
        .map(|c| rgb(c.as_str().unwrap_or("")))
        .collect();
    let h = rows.len();
    let w = rows
        .iter()
        .map(|r| r.as_array().map_or(0, |a| a.len()))
        .max()
        .unwrap_or(0);
    let scale = h.div_ceil(MAX_SIDE).max(w.div_ceil(MAX_SIDE)).max(1);
    let oh = h.div_ceil(scale);
    let ow = w.div_ceil(scale);
    let at = |r: usize, c: usize| -> (u8, u8, u8) {
        let idx = rows
            .get(r * scale)
            .and_then(|row| row.as_array())
            .and_then(|row| row.get(c * scale))
            .and_then(Value::as_u64)
            .unwrap_or(0) as usize;
        colors.get(idx).copied().unwrap_or((0, 0, 0))
    };
    let dims = if scale > 1 {
        format!("  {w}x{h} -> {ow}x{oh}")
    } else {
        format!("  {w}x{h}")
    };
    let mut out = format!("{app}{dims}\n");
    for r in (0..oh).step_by(2) {
        for c in 0..ow {
            let (tr, tg, tb) = at(r, c);
            let (br, bg, bb) = if r + 1 < oh { at(r + 1, c) } else { (0, 0, 0) };
            out.push_str(&format!(
                "\x1b[38;2;{tr};{tg};{tb};48;2;{br};{bg};{bb}m\u{2580}"
            ));
        }
        out.push_str("\x1b[0m\n");
    }
    out
}

fn rgb(hex: &str) -> (u8, u8, u8) {
    let hex = hex.strip_prefix('#').unwrap_or(hex);
    let byte = |i: usize| {
        hex.get(i..i + 2)
            .and_then(|s| u8::from_str_radix(s, 16).ok())
            .unwrap_or(0)
    };
    (byte(0), byte(2), byte(4))
}
