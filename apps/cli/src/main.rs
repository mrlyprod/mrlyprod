use mrlycore::{json, Json};
use mrlyos::kernel::{Call, Goose, Os};
use std::io::{BufRead, Read};

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
        Some("repl") => repl(),
        Some("tui") => tui::run(),
        Some("run") => run(&args[2..]),
        Some("render") => render(args.get(2).map(String::as_str)),
        Some("shot") => shot(&args[2..]),
        Some("face") => face(&args[2..]),
        Some("peek") => peek(&args[2..]),
        Some("watch") => watch(&args[2..]),
        Some("goose") => goose(&args[2..]),
        Some("drive") => drive(&args[2..]),
        Some("monkey") => monkey(&args[2..]),
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
    eprintln!("  mrlycli               tui on a terminal, repl on a pipe (default)");
    eprintln!("  mrlycli tui           raw-mode terminal face");
    eprintln!("  mrlycli repl          line-by-line interactive session");
    eprintln!(
        "  mrlycli run [file]    replay a call script, print the final frame (--facts trims grids)"
    );
    eprintln!("  mrlycli render [file] draw the final frame as colored blocks");
    eprintln!("  mrlycli shot [file]   write the final frame as a PNG (--out path)");
    eprintln!("  mrlycli face [file]   write the default face as a PNG, any app (--out path)");
    eprintln!("  mrlycli peek <app[/path]> [file] [shape]");
    eprintln!("                        print an app's state, one field, or a shaped subtree");
    eprintln!("  mrlycli watch <app/path> [file]");
    eprintln!("                        poll the field and print it when it changes");
    eprintln!("  mrlycli goose <app> [--seed N] [--steps K] [--trace] [--peek path]");
    eprintln!("                        drive an app with random legal calls");
    eprintln!("  mrlycli drive [file]  play a gesture screenplay against the face");
    eprintln!("                        gestures: open call tap tap_at slide cell focus type");
    eprintln!("                        key wheel scroll beat shot hits assert");
    eprintln!("  mrlycli monkey <app> [--seed N] [--steps K]");
    eprintln!("                        mash random face hits with the pointer");
    eprintln!("  mrlycli describe      print the kernel surface as JSON");
    eprintln!("  mrlycli verbs [app]   list apps, or one app's verbs and args");
    eprintln!("  mrlycli help          show this message");
    eprintln!();
    eprintln!("repl:");
    eprintln!("  verb [json]          act, e.g. nav.open {{\"app\":\"snake\"}}");
    eprintln!(
        "  :help :frame :render :verbs :peek :shot :face :describe :apps :open <app> :reset :quit"
    );
}

// BOOT

pub(crate) fn build() -> Os {
    mrlyweb::registry::boot()
}

// DESCRIBE

fn describe() {
    println!("{}", build().describe(None).pretty());
}

// VERBS

fn verbs(app: Option<&str>) {
    if !list_verbs(&build().describe(None), app) {
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

fn call_from(wire: &Json) -> Call {
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
    let wires: Vec<Json> = if text.trim_start().starts_with('[') {
        match mrlycore::json::parse(&text) {
            Ok(Json::Arr(items)) => items,
            _ => Vec::new(),
        }
    } else {
        text.lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .filter_map(|line| mrlycore::json::parse(line).ok())
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
    let mut frame = replay(path).frame(None).to_json();
    if facts {
        collapse(&mut frame);
    }
    println!("{}", frame.pretty());
}

fn collapse(value: &mut Json) {
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
                "frame" => emit(&os, visual),
                "render" => {
                    visual = !visual;
                    eprintln!("render: {}", if visual { "visual" } else { "facts" });
                    emit(&os, visual);
                }
                "describe" => eprintln!("{}", os.describe(None).pretty()),
                "verbs" => {
                    let app = if arg.is_empty() {
                        os.frame(None).route.map(|r| r.app)
                    } else {
                        Some(arg.to_string())
                    };
                    if !list_verbs(&os.describe(None), app.as_deref()) {
                        eprintln!("! no such app: {arg}");
                    }
                }
                "peek" => {
                    let mut it = arg.splitn(2, char::is_whitespace);
                    let target = it.next().unwrap_or("");
                    let rest = it.next().unwrap_or("").trim();
                    if target.is_empty() {
                        eprintln!("! peek needs app[/path]");
                    } else if rest.is_empty() {
                        print_peek(&os, target, None);
                    } else {
                        match mrlycore::json::parse(rest) {
                            Ok(shape) => {
                                print_peek(&os, target, Some(&shape));
                            }
                            Err(e) => eprintln!("! bad shape: {e}"),
                        }
                    }
                }
                "shot" => {
                    let out = if arg.is_empty() { "shot.png" } else { arg };
                    let app = os.frame(None).route.map(|r| r.app).unwrap_or_default();
                    match os.snapshot(&app) {
                        Ok(bytes) => match std::fs::write(out, &bytes) {
                            Ok(()) => eprintln!("shot: {app} -> {out} ({} bytes)", bytes.len()),
                            Err(e) => eprintln!("! write failed: {e}"),
                        },
                        Err(e) => eprintln!("! {app}: {e}"),
                    }
                }
                "face" => {
                    let out = if arg.is_empty() { "face.png" } else { arg };
                    let app = os.frame(None).route.map(|r| r.app).unwrap_or_default();
                    match mrlyweb::face::face_png(&os, &app) {
                        Ok(bytes) => match std::fs::write(out, &bytes) {
                            Ok(()) => eprintln!("face: {app} -> {out} ({} bytes)", bytes.len()),
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
    eprintln!(":peek app[/path] [shape]   print state, one field, or a shaped subtree");
    eprintln!(":shot [path]       write the current frame as a PNG");
    eprintln!(":face [path]       write the default face as a PNG");
    eprintln!(":apps              list installed apps");
    eprintln!(":open <app>        open an app");
    eprintln!(":reset             boot a fresh session");
    eprintln!(":quit :q           exit");
    eprintln!("verb [json]        act, e.g. calculator.digit {{\"d\":4}}");
}

pub(crate) fn now_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

fn emit(os: &Os, visual: bool) {
    let frame = os.frame(None);
    let json = frame.to_json();
    if visual {
        println!("{}", paint(&json));
    } else {
        println!("{}", json.pretty());
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
    println!("{}", paint(&replay(path).frame(None).to_json()));
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
    let app = os.frame(None).route.map(|r| r.app).unwrap_or_default();
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

// FACE

fn face(args: &[String]) {
    let mut out = "face.png".to_string();
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
    let app = os.frame(None).route.map(|r| r.app).unwrap_or_default();
    match mrlyweb::face::face_png(&os, &app) {
        Ok(bytes) => match std::fs::write(&out, &bytes) {
            Ok(()) => eprintln!("face: {app} -> {out} ({} bytes)", bytes.len()),
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

// PEEK

fn split_target(target: &str) -> (String, Vec<String>) {
    let mut parts = target.split('/');
    let app = parts.next().unwrap_or("").to_string();
    (app, parts.map(str::to_string).collect())
}

fn drill<'a>(value: &'a Json, path: &[String]) -> Option<&'a Json> {
    let mut cur = value;
    for seg in path {
        cur = match cur {
            Json::Obj(map) => map.get(seg)?,
            Json::Arr(items) => items.get(seg.parse::<usize>().ok()?)?,
            _ => return None,
        };
    }
    Some(cur)
}

fn peek_value(os: &Os, target: &str, shape: Option<&Json>) -> Result<Json, String> {
    let (app, path) = split_target(target);
    let view = os.peek(&app, shape).ok_or(format!("no such app: {app}"))?;
    drill(&view.state, &path)
        .cloned()
        .ok_or(format!("no value at {target}"))
}

fn print_peek(os: &Os, target: &str, shape: Option<&Json>) -> bool {
    match peek_value(os, target, shape) {
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

fn peek(args: &[String]) {
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
        eprintln!("! peek needs app[/path]");
        std::process::exit(2);
    };
    let os = match file {
        Some(f) => replay(Some(f)),
        None => build(),
    };
    if !print_peek(&os, target, shape.as_ref()) {
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
    let (app, _) = split_target(target);
    if os.open(&app).is_err() {
        eprintln!("! no such app: {app}");
        std::process::exit(1);
    }
    if let Err(note) = peek_value(&os, target, None) {
        eprintln!("! {note}");
    }
    let mut last: Option<Json> = None;
    loop {
        if let Some(beat) = os.frame(None).view.and_then(|v| v.beat) {
            os.act(beat.at(now_ms()));
        }
        if let Ok(value) = peek_value(&os, target, None) {
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
            "--peek" => {
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
        if let Some(beat) = os.frame(None).view.and_then(|v| v.beat) {
            os.act(beat.at(now));
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
            if !print_peek(&os, &format!("{app}/{}", p.trim_start_matches('/')), None) {
                std::process::exit(1);
            }
        }
        None => println!("{}", os.frame(None).to_json().pretty()),
    }
}

fn paint(frame: &Json) -> String {
    let grid = &frame["view"]["state"]["frame"];
    match (grid["rows"].as_array(), grid["palette"].as_array()) {
        (Some(rows), Some(palette)) if !rows.is_empty() => {
            blocks(frame["view"]["app"].as_str().unwrap_or(""), rows, palette)
        }
        _ => frame.pretty(),
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
        text.lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .filter_map(|line| mrlycore::json::parse(line).ok())
            .collect()
    }
}

fn drive(args: &[String]) {
    let mut path: Option<&str> = None;
    for a in args {
        path = Some(a);
    }
    let gestures = script(path);
    if gestures.is_empty() {
        eprintln!("! empty screenplay");
        std::process::exit(2);
    }
    let mut driver = mrlyweb::drive::Driver::scripted(0);
    for (i, g) in gestures.iter().enumerate() {
        if let Err(note) = gesture(&mut driver, g) {
            eprintln!("! gesture {}: {note}", i + 1);
            std::process::exit(1);
        }
    }
}

fn gesture(driver: &mut mrlyweb::drive::Driver, g: &Json) -> Result<(), String> {
    use mrlyweb::drive::KeyPress;
    if let Some(app) = g["open"].as_str() {
        driver.open(app);
        return Ok(());
    }
    if !g["call"].is_null() {
        let verb = g["call"]["verb"].as_str().ok_or("call needs a verb")?;
        driver.act(verb, g["call"]["args"].clone());
        return Ok(());
    }
    if !g["tap"].is_null() {
        let verb = g["tap"]["verb"].as_str().ok_or("tap needs a verb")?;
        return driver.tap(verb, &g["tap"]["args"]);
    }
    if let Some(at) = g["tap_at"].as_array() {
        let x = at.first().and_then(Json::as_u64).unwrap_or(0) as usize;
        let y = at.get(1).and_then(Json::as_u64).unwrap_or(0) as usize;
        driver.tap_at(x, y);
        return Ok(());
    }
    if !g["slide"].is_null() {
        let verb = g["slide"]["verb"].as_str().ok_or("slide needs a verb")?;
        let value = g["slide"]["value"].as_i64().ok_or("slide needs a value")?;
        return driver.slide(verb, value);
    }
    if let Some(at) = g["cell"].as_array() {
        let col = at.first().and_then(Json::as_u64).unwrap_or(0) as usize;
        let row = at.get(1).and_then(Json::as_u64).unwrap_or(0) as usize;
        return driver.cell(col, row);
    }
    if !g["focus"].is_null() {
        let verb = g["focus"]["verb"].as_str().ok_or("focus needs a verb")?;
        return driver.focus(verb);
    }
    if let Some(text) = g["type"].as_str() {
        driver.type_str(text);
        return Ok(());
    }
    if let Some(key) = g["key"].as_str() {
        match key {
            "enter" => driver.key(KeyPress::Enter),
            "escape" => driver.key(KeyPress::Escape),
            "backspace" => driver.key(KeyPress::Backspace),
            other => return Err(format!("unknown key {other}")),
        }
        return Ok(());
    }
    if let Some(dy) = g["wheel"].as_i64() {
        driver.wheel(dy as f64, None);
        return Ok(());
    }
    if let Some(to) = g["scroll"].as_u64() {
        driver.scroll_to(to as usize);
        return Ok(());
    }
    if let Some(n) = g["beat"].as_u64() {
        driver.beat(n as usize);
        return Ok(());
    }
    if let Some(out) = g["shot"].as_str() {
        let png = driver.shot().map_err(str::to_string)?;
        return std::fs::write(out, png).map_err(|e| e.to_string());
    }
    if g["hits"] == json!(true) {
        println!("{}", driver.hits_json().pretty());
        return Ok(());
    }
    if !g["assert"].is_null() {
        return check(driver, &g["assert"]);
    }
    Err(format!("unknown gesture {g}"))
}

fn check(driver: &mrlyweb::drive::Driver, want: &Json) -> Result<(), String> {
    if let Some(route) = want["route"].as_str() {
        if driver.route() != route {
            return Err(format!("route is {}, wanted {route}", driver.route()));
        }
    }
    if let Some(verb) = want["verb"].as_str() {
        let hits = driver.hits_json();
        let found = hits["hits"].as_array().is_some_and(|list| {
            list.iter().any(|h| {
                let act = &h["act"];
                ["verb", "tap", "drag", "turn", "zoom", "pan"]
                    .iter()
                    .any(|k| act[*k].as_str() == Some(verb))
            })
        });
        if !found {
            return Err(format!("no hit carries {verb}"));
        }
    }
    if let Some(n) = want["hits"].as_u64() {
        let got = driver.scene().hits.len() as u64;
        if got != n {
            return Err(format!("{got} hits, wanted {n}"));
        }
    }
    if let Some(pin) = want["frame"].as_str() {
        let got = driver.frame_fnv().to_string();
        if got != pin {
            return Err(format!("frame is {got}, pinned {pin}"));
        }
    }
    if let Some(pair) = want["state"].as_array() {
        let target = pair
            .first()
            .and_then(Json::as_str)
            .ok_or("state needs a target")?;
        let wanted = pair.get(1).cloned().unwrap_or(Json::Null);
        let got = peek_value(driver.os(), target, None)?;
        if got != wanted {
            return Err(format!("{target} is {got}, wanted {wanted}"));
        }
    }
    Ok(())
}

// MONKEY

fn monkey(args: &[String]) {
    let mut app = None;
    let mut seed: u64 = 1;
    let mut steps: usize = 100;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--seed" => seed = it.next().and_then(|v| v.parse().ok()).unwrap_or(1),
            "--steps" => steps = it.next().and_then(|v| v.parse().ok()).unwrap_or(100),
            other => app = Some(other.to_string()),
        }
    }
    let Some(app) = app else {
        eprintln!("! monkey needs an app");
        std::process::exit(2);
    };
    let mut driver = mrlyweb::drive::Driver::scripted(0);
    driver.open(&app);
    let mut rng = mrlycore::Rng::new(seed);
    let mut acted = 0;
    for _ in 0..steps {
        let hits: Vec<mrlyweb::drive::Hit> = driver
            .scene()
            .hits
            .iter()
            .filter(|h| !matches!(h.act, mrlyweb::drive::Act::Mute))
            .cloned()
            .collect();
        if hits.is_empty() {
            driver.beat(1);
            continue;
        }
        let hit = &hits[rng.below(hits.len())];
        let cx = hit.x + hit.w / 2;
        let cy = hit.y + hit.h / 2;
        match &hit.act {
            mrlyweb::drive::Act::Slide { .. } => {
                driver.press(cx, cy);
                let to = hit.x + rng.below(hit.w.max(1));
                driver.release(to, cy);
            }
            mrlyweb::drive::Act::Board { .. } => {
                let px = hit.x + rng.below(hit.w.max(1));
                let py = hit.y + rng.below(hit.h.max(1));
                driver.tap_at(px, py);
            }
            mrlyweb::drive::Act::Edit { .. } => {
                driver.press(cx, cy);
                for _ in 0..3 {
                    let c = (b'a' + rng.below(26) as u8) as char;
                    driver.key(mrlyweb::drive::KeyPress::Char(c));
                }
                driver.key(mrlyweb::drive::KeyPress::Enter);
            }
            _ => driver.tap_at(cx, cy),
        }
        acted += 1;
    }
    println!(
        "{acted} gestures on {app} (seed {seed}), route now {}",
        driver.route()
    );
}
