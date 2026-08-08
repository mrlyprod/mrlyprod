use mrlycore::{json, Json};
use mrlyos::kernel::{Call, Os};
use mrlyweb::goose::Goose;
use std::io::{BufRead, Read};

mod mcp;
mod term;
mod tui;

const PROMPT: &str = "mrly> ";
const MAX_SIDE: usize = 64;
pub(crate) const BEAT: i64 = 125;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        None => {
            if term::tty() {
                tui::run()
            } else {
                repl()
            }
        }
        Some("mcp") => mcp::serve(),
        Some("repl") => repl(),
        Some("tui") => tui::run(),
        Some("run") => run(&args[2..]),
        Some("render") => render(args.get(2).map(String::as_str)),
        Some("shot") => shot(&args[2..]),
        Some("read") => read(&args[2..]),
        Some("watch") => watch(&args[2..]),
        Some("goose") => goose(&args[2..]),
        Some("drive") => drive(&args[2..]),
        Some("frame") => frame(&args[2..]),
        Some("list") => list(),
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
    eprintln!("  mrlycli               tui on a terminal, repl on a pipe (default)");
    eprintln!("  mrlycli tui           raw-mode terminal face");
    eprintln!("  mrlycli repl          line-by-line interactive session");
    eprintln!(
        "  mrlycli mcp           an MCP server over stdio: tools/list, tools/call, resources/read"
    );
    eprintln!(
        "  mrlycli run [file]    replay a call script, print the envelope (--facts trims grids)"
    );
    eprintln!("  mrlycli render [file] draw the final frame as colored blocks");
    eprintln!("  mrlycli shot [file]   write the final frame as a PNG (--out path)");
    eprintln!("  mrlycli read <app[/path]> [file] [shape]");
    eprintln!("                        print an app's view, one field, or a shaped subtree");
    eprintln!("  mrlycli watch <app/path> [file]");
    eprintln!("                        poll the field and print it when it changes");
    eprintln!("  mrlycli goose <app> [--seed N] [--steps K] [--trace] [--read path]");
    eprintln!("                        drive an app with random legal calls");
    eprintln!("  mrlycli drive [file]  play a wire screenplay: open, call, assert");
    eprintln!("  mrlycli frame [file] [WxH]");
    eprintln!("                        replay a screenplay and print the TUI screen as text");
    eprintln!("  mrlycli frame --record");
    eprintln!("                        re-pin every frame golden in tests/frames");
    eprintln!("  mrlycli list          print the kernel surface as JSON");
    eprintln!("  mrlycli verbs [app]   list apps, or one app's verbs and args");
    eprintln!("  mrlycli help          show this message");
    eprintln!();
    eprintln!("repl:");
    eprintln!("  verb [json]          call, e.g. nav.open {{\"app\":\"snake\"}}");
    eprintln!("  :help :read :render :verbs :shot :list :apps :open <app> :reset :quit");
}

// BOOT

pub(crate) fn build() -> Os {
    mrlyweb::registry::boot("full")
}

// LIST

fn list() {
    println!("{}", build().list(None).pretty());
}

// VERBS

fn verbs(app: Option<&str>) {
    if !list_verbs(&build().list(None), app) {
        eprintln!("! no such app: {}", app.unwrap_or(""));
        std::process::exit(1);
    }
}

fn list_verbs(surface: &Json, app: Option<&str>) -> bool {
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

fn verb_line(verb: &Json) -> String {
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

pub(crate) fn call_from(wire: &Json) -> Call {
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

pub(crate) fn wires(text: &str) -> Vec<Json> {
    if text.trim_start().starts_with('[') {
        match mrlycore::json::parse(text) {
            Ok(Json::Arr(items)) => items,
            _ => Vec::new(),
        }
    } else {
        text.lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .filter_map(|line| mrlycore::json::parse(line).ok())
            .collect()
    }
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
    let wires = wires(&text);
    let mut os = build();
    for wire in &wires {
        let call = call_from(wire);
        let verb = call.verb.clone();
        let out = os.call(call);
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
    let mut env = replay(path).read("", None).unwrap_or_default();
    if facts {
        collapse(&mut env);
    }
    println!("{}", env.pretty());
}

pub(crate) fn collapse(value: &mut Json) {
    match value {
        Json::Arr(items) => {
            if items.first().is_some_and(Json::is_array) {
                let rows = items.len();
                let cols = items[0].as_array().map_or(0, Vec::len);
                *value = Json::Str(format!("grid {rows}x{cols}"));
            } else {
                for item in items {
                    collapse(item);
                }
            }
        }
        Json::Obj(map) => {
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
                "render" => {
                    visual = !visual;
                    eprintln!("render: {}", if visual { "visual" } else { "facts" });
                    emit(&os, visual);
                }
                "list" => eprintln!("{}", os.list(None).pretty()),
                "verbs" => {
                    let app = if arg.is_empty() {
                        os.envelope(None).route.map(|r| r.app)
                    } else {
                        Some(arg.to_string())
                    };
                    if !list_verbs(&os.list(None), app.as_deref()) {
                        eprintln!("! no such app: {arg}");
                    }
                }
                "read" => {
                    let mut it = arg.splitn(2, char::is_whitespace);
                    let target = it.next().unwrap_or("");
                    let rest = it.next().unwrap_or("").trim();
                    if target.is_empty() {
                        emit(&os, visual);
                    } else if rest.is_empty() {
                        print_read(&os, target, None);
                    } else {
                        match mrlycore::json::parse(rest) {
                            Ok(shape) => {
                                print_read(&os, target, Some(&shape));
                            }
                            Err(e) => eprintln!("! bad shape: {e}"),
                        }
                    }
                }
                "shot" => {
                    let out = if arg.is_empty() { "shot.png" } else { arg };
                    let app = os.envelope(None).route.map(|r| r.app).unwrap_or_default();
                    match snap(&os, &app) {
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
                match mrlycore::json::parse(rest) {
                    Ok(v) => Some(v),
                    Err(e) => {
                        eprintln!("! bad args: {e}");
                        None
                    }
                }
            };
            if let Some(args) = args {
                os.call(Call::new(verb, args).at(now_ms()));
                emit(&os, visual);
            }
        }
        eprint!("{PROMPT}");
    }
    eprintln!();
}

fn meta_help() {
    eprintln!(":help              this message");
    eprintln!(":read [app/path] [shape]   reprint the envelope, or read one field");
    eprintln!(":render            toggle visual blocks / raw facts");
    eprintln!(":list              print the kernel surface");
    eprintln!(":verbs [app]       verbs and args (current app by default)");
    eprintln!(":shot [path]       write the current frame as a PNG");
    eprintln!(":apps              list installed apps");
    eprintln!(":open <app>        open an app");
    eprintln!(":reset             boot a fresh session");
    eprintln!(":quit :q           exit");
    eprintln!("verb [json]        call, e.g. calculator.digit {{\"d\":4}}");
}

pub(crate) fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

fn emit(os: &Os, visual: bool) {
    let env = os.envelope(None);
    let json = env.to_json();
    if visual {
        println!("{}", paint(&json));
    } else {
        println!("{}", json.pretty());
    }
    if let Some(last) = &env.last {
        if !last.ok {
            if let Some(note) = &last.note {
                eprintln!("! {note}");
            }
        }
    }
    if let Some(view) = &env.view {
        let names: Vec<&str> = view.actions.iter().map(|v| v.name.as_str()).collect();
        if !names.is_empty() {
            eprintln!("verbs: {}", names.join(", "));
        }
    }
}

// RENDER

fn render(path: Option<&str>) {
    let env = replay(path).read("", None).unwrap_or_default();
    println!("{}", paint(&env));
}

// SHOT

fn snap(os: &Os, app: &str) -> Result<Vec<u8>, &'static str> {
    let cells = os
        .read(&format!("{app}/cells"), None)
        .filter(|c| c.as_object().is_some())
        .ok_or("nothing to shoot here")?;
    let image = mrlyui::skin::raster(app, &cells, 8, true)
        .ok_or("nothing to shoot here")?
        .image();
    let scale = (512 / image.width.max(image.height).max(1)).max(1);
    image.png(scale).map_err(|_| "could not render frame")
}

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
    let app = os.envelope(None).route.map(|r| r.app).unwrap_or_default();
    match snap(&os, &app) {
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

// READ

fn read_value(os: &Os, target: &str, shape: Option<&Json>) -> Result<Json, String> {
    os.read(target, shape)
        .ok_or(format!("no value at {target}"))
}

fn print_read(os: &Os, target: &str, shape: Option<&Json>) -> bool {
    match read_value(os, target, shape) {
        Ok(value) => {
            println!("{}", value.pretty());
            true
        }
        Err(note) => {
            eprintln!("! {note}");
            false
        }
    }
}

fn read(args: &[String]) {
    let mut target: Option<&str> = None;
    let mut shape: Option<Json> = None;
    let mut file: Option<&str> = None;
    for a in args {
        if target.is_none() {
            target = Some(a);
        } else if a.trim_start().starts_with('{') {
            match mrlycore::json::parse(a) {
                Ok(v) => shape = Some(v),
                Err(e) => {
                    eprintln!("! bad shape: {e}");
                    std::process::exit(2);
                }
            }
        } else {
            file = Some(a);
        }
    }
    let Some(target) = target else {
        eprintln!("! read needs app[/path]");
        std::process::exit(2);
    };
    let os = match file {
        Some(f) => replay(Some(f)),
        None => build(),
    };
    if !print_read(&os, target, shape.as_ref()) {
        std::process::exit(1);
    }
}

// WATCH

fn watch(args: &[String]) {
    let mut target: Option<&str> = None;
    let mut file: Option<&str> = None;
    for a in args {
        if target.is_none() {
            target = Some(a);
        } else {
            file = Some(a);
        }
    }
    let Some(target) = target else {
        eprintln!("! watch needs app/path");
        std::process::exit(2);
    };
    let mut os = match file {
        Some(f) => replay(Some(f)),
        None => build(),
    };
    let app = target.split('/').next().unwrap_or("");
    if os.open(app).is_err() {
        eprintln!("! no such app: {app}");
        std::process::exit(1);
    }
    if let Err(note) = read_value(&os, target, None) {
        eprintln!("! {note}");
    }
    let mut last: Option<Json> = None;
    loop {
        if let Some(beat) = os.envelope(None).view.and_then(|v| v.beat) {
            os.call(beat.at(now_ms()));
        }
        if let Ok(value) = read_value(&os, target, None) {
            if last.as_ref() != Some(&value) {
                println!("{value}");
                last = Some(value);
            }
        }
        std::thread::sleep(std::time::Duration::from_millis(250));
    }
}

// GOOSE

fn goose(args: &[String]) {
    let mut app: Option<&str> = None;
    let mut seed: u64 = 7;
    let mut steps: usize = 100;
    let mut trace = false;
    let mut path: Option<&str> = None;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--seed" => {
                if let Some(v) = it.next() {
                    seed = v.parse().unwrap_or(seed);
                }
            }
            "--steps" => {
                if let Some(v) = it.next() {
                    steps = v.parse().unwrap_or(steps);
                }
            }
            "--trace" => trace = true,
            "--read" => {
                if let Some(v) = it.next() {
                    path = Some(v);
                }
            }
            other => app = Some(other),
        }
    }
    let Some(app) = app else {
        eprintln!("! goose needs an app");
        std::process::exit(2);
    };
    let mut os = build();
    if os.open(app).is_err() {
        eprintln!("! no such app: {app}");
        std::process::exit(1);
    }
    let mut goose = Goose::new(seed);
    let mut now = 0;
    for _ in 0..steps {
        if let Some(beat) = os.envelope(None).view.and_then(|v| v.beat) {
            os.call(beat.at(now));
        }
        if let Some(call) = goose.step(&mut os) {
            if trace {
                println!("{}", call.to_json());
            }
        }
        now += BEAT;
    }
    match path {
        Some(p) => {
            if !print_read(&os, &format!("{app}/{}", p.trim_start_matches('/')), None) {
                std::process::exit(1);
            }
        }
        None => println!("{}", os.read("", None).unwrap_or_default().pretty()),
    }
}

fn paint(env: &Json) -> String {
    let app = env["view"]["app"].as_str().unwrap_or("");
    let state = &env["view"]["state"];
    let grid = state
        .get("cells")
        .and_then(|cells| mrlyui::skin::raster(app, cells, 4, true))
        .map(|f| f.fact())
        .unwrap_or(Json::Null);
    match (grid["rows"].as_array(), grid["palette"].as_array()) {
        (Some(rows), Some(palette)) if !rows.is_empty() => blocks(app, rows, palette),
        _ => env.pretty(),
    }
}

fn blocks(app: &str, rows: &[Json], palette: &[Json]) -> String {
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
            .and_then(Json::as_u64)
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

pub(crate) fn rgb(hex: &str) -> (u8, u8, u8) {
    let hex = hex.strip_prefix('#').unwrap_or(hex);
    let byte = |i: usize| {
        hex.get(i..i + 2)
            .and_then(|s| u8::from_str_radix(s, 16).ok())
            .unwrap_or(0)
    };
    (byte(0), byte(2), byte(4))
}

// FRAME

fn frame(args: &[String]) {
    if args.first().map(String::as_str) == Some("--record") {
        let root = env!("CARGO_MANIFEST_DIR");
        let plays = format!("{root}/tests/screenplays");
        let frames = format!("{root}/tests/frames");
        std::fs::create_dir_all(&frames).ok();
        let mut paths: Vec<std::path::PathBuf> = std::fs::read_dir(&plays)
            .map(|it| {
                it.filter_map(Result::ok)
                    .map(|entry| entry.path())
                    .collect()
            })
            .unwrap_or_default();
        paths.sort();
        let mut count = 0;
        for path in paths {
            let Some(name) = path.file_stem().and_then(|stem| stem.to_str()) else {
                continue;
            };
            let text = std::fs::read_to_string(&path).unwrap_or_default();
            for (w, h) in [(80, 24), (20, 6)] {
                let screen = tui::frame(&text, (w, h));
                std::fs::write(format!("{frames}/{name}.{w}x{h}.txt"), screen).ok();
                count += 1;
            }
        }
        println!("{count} frames");
        return;
    }
    let Some(path) = args.first() else {
        usage();
        std::process::exit(2);
    };
    let size: (usize, usize) = args
        .get(1)
        .and_then(|arg| arg.split_once('x'))
        .and_then(|(w, h)| Some((w.parse().ok()?, h.parse().ok()?)))
        .unwrap_or((80, 24));
    let text = std::fs::read_to_string(path).unwrap_or_default();
    print!("{}", tui::frame(&text, size));
}

// DRIVE

fn script(path: Option<&str>) -> Vec<Json> {
    let text = match path {
        Some(p) => std::fs::read_to_string(p).unwrap_or_default(),
        None => {
            let mut buf = String::new();
            std::io::stdin().read_to_string(&mut buf).ok();
            buf
        }
    };
    if text.trim_start().starts_with('[') {
        match mrlycore::json::parse(&text) {
            Ok(Json::Arr(items)) => items,
            _ => Vec::new(),
        }
    } else {
        let mut out = Vec::new();
        for (i, line) in text.lines().enumerate() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            match mrlycore::json::parse(line) {
                Ok(gesture) => out.push(gesture),
                Err(_) => {
                    eprintln!("! line {}: not json", i + 1);
                    std::process::exit(2);
                }
            }
        }
        out
    }
}

fn drive(args: &[String]) {
    let mut path: Option<&str> = None;
    for a in args {
        path = Some(a);
    }
    let steps = script(path);
    if steps.is_empty() {
        eprintln!("! empty screenplay");
        std::process::exit(2);
    }
    let mut os = build();
    for (i, s) in steps.iter().enumerate() {
        if let Err(note) = step(&mut os, s) {
            eprintln!("! step {}: {note}", i + 1);
            std::process::exit(1);
        }
    }
}

fn step(os: &mut Os, s: &Json) -> Result<(), String> {
    if let Some(app) = s["open"].as_str() {
        return os.open(app).map_err(str::to_string);
    }
    if !s["call"].is_null() {
        let verb = s["call"]["verb"].as_str().ok_or("call needs a verb")?;
        let args = if s["call"]["args"].is_object() {
            s["call"]["args"].clone()
        } else {
            json!({})
        };
        let out = os.call(Call::new(verb, args));
        if !out.ok {
            let note = out.note.as_deref().unwrap_or("failed");
            return Err(format!("{verb}: {note}"));
        }
        return Ok(());
    }
    if !s["assert"].is_null() {
        return check(os, &s["assert"]);
    }
    Err("a step is open, call, or assert".to_string())
}

fn check(os: &Os, want: &Json) -> Result<(), String> {
    let env = os.envelope(None);
    if let Some(route) = want["route"].as_str() {
        let at = env.route.as_ref().map(|r| r.app.as_str()).unwrap_or("");
        if at != route {
            return Err(format!("route is {at}, wanted {route}"));
        }
    }
    if let Some(verb) = want["verb"].as_str() {
        let surface = os.list(None);
        let current = env
            .route
            .as_ref()
            .map(|r| r.app.clone())
            .unwrap_or_default();
        let named = |v: &Json| v["verb"].as_str() == Some(verb);
        let empty = Vec::new();
        let offered = surface["nav"]
            .as_array()
            .unwrap_or(&empty)
            .iter()
            .any(named)
            || surface["verbs"]
                .as_array()
                .unwrap_or(&empty)
                .iter()
                .any(|g| {
                    g["app"].as_str() == Some(current.as_str())
                        && g["verbs"].as_array().unwrap_or(&empty).iter().any(named)
                });
        if !offered {
            return Err(format!("{verb} is not offered here"));
        }
    }
    if let Some(pair) = want["state"].as_array() {
        let target = pair
            .first()
            .and_then(Json::as_str)
            .ok_or("state needs a target")?;
        let wanted = pair.get(1).cloned().unwrap_or(Json::Null);
        let got = read_value(os, target, None)?;
        if got != wanted {
            return Err(format!("{target} is {got}, wanted {wanted}"));
        }
    }
    Ok(())
}
