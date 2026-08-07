use crate::term::{self, Cell, Key, Raw, Screen};
use mrlycore::colors::ROLLABLE;
use mrlycore::{json, Json};
use mrlyos::kernel::{Call, Envelope, Os};
use std::io::{Read, Write};
use std::sync::mpsc;
use std::time::{Duration, Instant};

const CURSOR: char = '\u{2588}';
const HELP: &str = "arrows play · 1-9 act · / command · esc menu · q quit";
const SHELL: [&str; 5] = ["app", "json", "replay", "seed", "skin"];
const FEED: usize = 200;
const RECALL: usize = 100;
const WIDE: usize = 60;

// WIRE

struct App {
    route: String,
    emoji: String,
    title: String,
    keys: Vec<(String, Call)>,
}

struct Wire {
    apps: Vec<App>,
    verbs: Vec<(String, Json)>,
}

impl Wire {
    fn load(os: &Os) -> Wire {
        let list = os.list(None);
        let empty = Vec::new();
        let mut apps = Vec::new();
        for manifest in list["apps"].as_array().unwrap_or(&empty) {
            let mut keys = Vec::new();
            if let Some(map) = manifest["keys"].as_object() {
                for (dir, bind) in map.iter() {
                    let verb = bind["verb"].as_str().unwrap_or("");
                    keys.push((dir.clone(), Call::new(verb, bind["args"].clone())));
                }
            }
            apps.push(App {
                route: manifest["route"].as_str().unwrap_or("").to_string(),
                emoji: manifest["emoji"].as_str().unwrap_or("\u{2728}").to_string(),
                title: manifest["title"].as_str().unwrap_or("").to_string(),
                keys,
            });
        }
        let mut verbs = Vec::new();
        for group in list["verbs"].as_array().unwrap_or(&empty) {
            for verb in group["verbs"].as_array().unwrap_or(&empty) {
                if let Some(name) = verb["verb"].as_str() {
                    verbs.push((name.to_string(), verb["args"].clone()));
                }
            }
        }
        for verb in list["nav"].as_array().unwrap_or(&empty) {
            if let Some(name) = verb["verb"].as_str() {
                verbs.push((name.to_string(), verb["args"].clone()));
            }
        }
        Wire { apps, verbs }
    }

    fn face(&self, route: &str) -> (String, String) {
        self.apps
            .iter()
            .find(|app| app.route == route)
            .map(|app| (app.emoji.clone(), app.title.clone()))
            .unwrap_or(("\u{2728}".to_string(), route.to_string()))
    }
}

// SESSION

struct Entry {
    text: String,
    dim: bool,
}

struct Session {
    line: Option<String>,
    history: Vec<String>,
    reach: Option<usize>,
    stash: String,
    feed: Vec<Entry>,
    json: bool,
    dirty: bool,
}

impl Session {
    fn new() -> Session {
        Session {
            line: None,
            history: Vec::new(),
            reach: None,
            stash: String::new(),
            feed: Vec::new(),
            json: true,
            dirty: true,
        }
    }

    fn say(&mut self, text: String, dim: bool) {
        self.feed.push(Entry { text, dim });
        if self.feed.len() > FEED {
            self.feed.remove(0);
        }
        self.dirty = true;
    }
}

// LOOP

pub fn run() {
    let mut os = crate::build();
    let wire = Wire::load(&os);
    let mut session = Session::new();
    let raw = Raw::enter();
    let (tx, rx) = mpsc::channel::<Vec<u8>>();
    std::thread::spawn(move || {
        let mut input = std::io::stdin();
        let mut buf = [0u8; 64];
        loop {
            match input.read(&mut buf) {
                Ok(0) | Err(_) => break,
                Ok(n) => {
                    if tx.send(buf[..n].to_vec()).is_err() {
                        break;
                    }
                }
            }
        }
    });
    let mut screen: Option<Screen> = None;
    let mut last: Option<u64> = None;
    let mut shape = (0, 0);
    let mut beat = Instant::now();
    let mut alive = true;
    while alive {
        let wait = Duration::from_millis(crate::BEAT as u64).saturating_sub(beat.elapsed());
        match rx.recv_timeout(wait) {
            Ok(chunk) => {
                for key in term::parse(&chunk) {
                    if !press(&mut os, &wire, &mut session, key) {
                        alive = false;
                    }
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                if let Some(call) = os.envelope(None).view.and_then(|v| v.beat) {
                    os.call(call.at(crate::now_ms()));
                }
                beat = Instant::now();
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => alive = false,
        }
        let size = term::size();
        let env = os.envelope(None);
        if session.dirty || last != Some(env.tick) || shape != size {
            let next = render(&env, size, &wire, &session);
            let bytes = term::diff(screen.as_ref(), &next);
            if !bytes.is_empty() {
                let mut out = std::io::stdout();
                out.write_all(bytes.as_bytes()).ok();
                out.flush().ok();
            }
            screen = Some(next);
            last = Some(env.tick);
            shape = size;
            session.dirty = false;
        }
    }
    drop(raw);
}

// INPUT

fn press(os: &mut Os, wire: &Wire, session: &mut Session, key: Key) -> bool {
    if session.line.is_some() {
        typing(os, wire, session, key);
        return true;
    }
    match key {
        Key::Ctrl('c') | Key::Char('q') => return false,
        Key::Char('/') => {
            session.line = Some("/".to_string());
            session.dirty = true;
        }
        Key::Esc => act(
            os,
            session,
            Call::new("nav.open", json!({ "app": "menu" })).at(crate::now_ms()),
        ),
        Key::Up | Key::Char('w') | Key::Char('W') => bound(os, wire, session, "up"),
        Key::Down | Key::Char('s') | Key::Char('S') => bound(os, wire, session, "down"),
        Key::Left | Key::Char('a') | Key::Char('A') => bound(os, wire, session, "left"),
        Key::Right | Key::Char('d') | Key::Char('D') => bound(os, wire, session, "right"),
        Key::Char(ch) if ('1'..='9').contains(&ch) => nth(os, session, ch as usize - '1' as usize),
        _ => {}
    }
    true
}

fn typing(os: &mut Os, wire: &Wire, session: &mut Session, key: Key) {
    match key {
        Key::Char(ch) => {
            if let Some(line) = session.line.as_mut() {
                line.push(ch);
            }
            session.reach = None;
        }
        Key::Backspace => {
            if let Some(line) = session.line.as_mut() {
                line.pop();
                if line.is_empty() {
                    session.line = None;
                }
            }
            session.reach = None;
        }
        Key::Esc | Key::Ctrl('c') => {
            session.line = None;
            session.reach = None;
        }
        Key::Ctrl('i') => complete(os, wire, session),
        Key::Up => recall(session, true),
        Key::Down => recall(session, false),
        Key::Enter => {
            let Some(text) = session.line.take() else {
                return;
            };
            session.reach = None;
            let text = text.trim().to_string();
            if text != "/" && !text.is_empty() {
                session.history.push(text.clone());
                if session.history.len() > RECALL {
                    session.history.remove(0);
                }
                submit(os, wire, session, &text);
            }
        }
        _ => return,
    }
    session.dirty = true;
}

fn recall(session: &mut Session, back: bool) {
    if session.history.is_empty() {
        return;
    }
    let at = match (session.reach, back) {
        (None, true) => {
            session.stash = session.line.clone().unwrap_or_default();
            session.history.len() - 1
        }
        (None, false) => return,
        (Some(0), true) => 0,
        (Some(i), true) => i - 1,
        (Some(i), false) => {
            if i + 1 < session.history.len() {
                i + 1
            } else {
                session.reach = None;
                session.line = Some(session.stash.clone());
                return;
            }
        }
    };
    session.reach = Some(at);
    session.line = Some(session.history[at].clone());
}

fn complete(os: &Os, wire: &Wire, session: &mut Session) {
    let Some(line) = session.line.clone() else {
        return;
    };
    let body = line.strip_prefix('/').unwrap_or(&line);
    if let Some((word, rest)) = body.split_once(char::is_whitespace) {
        if word == "app" {
            let routes: Vec<&str> = wire
                .apps
                .iter()
                .map(|app| app.route.as_str())
                .filter(|route| route.starts_with(rest.trim()))
                .collect();
            grow(session, "/app ", rest.trim(), &routes, "");
        }
        return;
    }
    let names = candidates(wire, &here(os), body);
    let names: Vec<&str> = names.iter().map(String::as_str).collect();
    grow(session, "/", body, &names, " ");
}

fn grow(session: &mut Session, head: &str, word: &str, names: &[&str], tail: &str) {
    if names.is_empty() {
        return;
    }
    let mut common = names[0].to_string();
    for name in &names[1..] {
        while !name.starts_with(&common) {
            common.pop();
        }
    }
    if names.len() == 1 {
        session.line = Some(format!("{head}{}{tail}", names[0]));
    } else if common.len() > word.len() {
        session.line = Some(format!("{head}{common}"));
    }
}

fn candidates(wire: &Wire, route: &str, word: &str) -> Vec<String> {
    let mut names: Vec<String> = SHELL
        .iter()
        .filter(|name| name.starts_with(word))
        .map(|name| name.to_string())
        .collect();
    let mine = format!("{route}.");
    for (name, _) in wire.verbs.iter().filter(|(n, _)| n.starts_with(word)) {
        if name.starts_with(&mine) {
            names.push(name.clone());
        }
    }
    for (name, _) in wire.verbs.iter().filter(|(n, _)| n.starts_with(word)) {
        if !name.starts_with(&mine) {
            names.push(name.clone());
        }
    }
    names
}

fn here(os: &Os) -> String {
    os.envelope(None).route.map(|r| r.app).unwrap_or_default()
}

// ACTS

fn act(os: &mut Os, session: &mut Session, call: Call) {
    let text = if call.args.as_object().is_none_or(|args| args.is_empty()) {
        format!("> {}", call.verb)
    } else {
        format!("> {} {}", call.verb, call.args)
    };
    session.say(text, false);
    let verb = call.verb.clone();
    let out = os.call(call);
    if out.ok {
        let env = os.envelope(None);
        let mut tail = format!("tick {}", env.tick);
        for effect in &env.effects {
            tail.push_str(&format!(" · {}", effect.kind));
        }
        session.say(tail, true);
    } else {
        let note = out.note.as_deref().unwrap_or("failed");
        session.say(format!("! {verb}: {note}"), false);
    }
}

fn bound(os: &mut Os, wire: &Wire, session: &mut Session, dir: &str) {
    let route = here(os);
    let Some(call) = wire
        .apps
        .iter()
        .find(|app| app.route == route)
        .and_then(|app| app.keys.iter().find(|(d, _)| d == dir))
        .map(|(_, call)| call.clone())
    else {
        return;
    };
    act(os, session, call.at(crate::now_ms()));
}

fn nth(os: &mut Os, session: &mut Session, index: usize) {
    let Some(view) = os.envelope(None).view else {
        return;
    };
    let Some(verb) = view.actions.get(index) else {
        return;
    };
    if verb.args.as_object().is_none_or(|args| args.is_empty()) {
        act(
            os,
            session,
            Call::new(&verb.name, json!({})).at(crate::now_ms()),
        );
    } else {
        session.line = Some(format!("/{} ", verb.name));
        session.dirty = true;
    }
}

// SHELL

fn submit(os: &mut Os, wire: &Wire, session: &mut Session, text: &str) {
    let body = text.strip_prefix('/').unwrap_or(text);
    let mut it = body.splitn(2, char::is_whitespace);
    let word = it.next().unwrap_or("");
    let rest = it.next().unwrap_or("").trim();
    match word {
        "json" => {
            session.json = !session.json;
            let state = if session.json { "on" } else { "off" };
            session.say(format!("json {state}"), true);
        }
        "app" => open(os, session, rest),
        "seed" => seed(os, wire, session, rest),
        "skin" => skin(os, wire, session, rest),
        "replay" => replay(os, session, rest),
        _ => call(os, session, word, rest),
    }
}

fn open(os: &mut Os, session: &mut Session, rest: &str) {
    if rest.is_empty() {
        session.say("! /app <route>".to_string(), false);
        return;
    }
    act(
        os,
        session,
        Call::new("nav.open", json!({ "app": rest })).at(crate::now_ms()),
    );
}

fn seed(os: &mut Os, wire: &Wire, session: &mut Session, rest: &str) {
    let Ok(n) = rest.parse::<i64>() else {
        session.say("! /seed <int>".to_string(), false);
        return;
    };
    act(
        os,
        session,
        Call::new("settings.set", json!({ "key": "seed", "value": n })).at(crate::now_ms()),
    );
    let reset = format!("{}.reset", here(os));
    if let Some((name, args)) = wire.verbs.iter().find(|(name, _)| name == &reset) {
        let args = if args["seed"].is_null() {
            json!({})
        } else {
            json!({ "seed": n })
        };
        act(os, session, Call::new(name, args).at(crate::now_ms()));
    }
}

fn skin(os: &mut Os, wire: &Wire, session: &mut Session, rest: &str) {
    if rest.is_empty() {
        session.say("! /skin <name>".to_string(), false);
        return;
    }
    let set = format!("{}.set", here(os));
    if wire.verbs.iter().any(|(name, _)| name == &set) {
        act(
            os,
            session,
            Call::new(&set, json!({ "key": "skin", "value": rest })).at(crate::now_ms()),
        );
    } else {
        session.say(format!("! no skin door in {}", here(os)), false);
    }
}

fn replay(os: &mut Os, session: &mut Session, rest: &str) {
    if rest.is_empty() {
        session.say("! /replay <file>".to_string(), false);
        return;
    }
    let Ok(text) = std::fs::read_to_string(rest) else {
        session.say(format!("! no such file: {rest}"), false);
        return;
    };
    let steps = crate::wires(&text);
    play(os, session, &steps);
    session.say(format!("replayed {} steps", steps.len()), true);
}

fn call(os: &mut Os, session: &mut Session, word: &str, rest: &str) {
    let args = if rest.is_empty() {
        json!({})
    } else {
        match mrlycore::json::parse(rest) {
            Ok(value) => value,
            Err(_) => {
                session.say(format!("! bad json: {rest}"), false);
                return;
            }
        }
    };
    act(os, session, Call::new(word, args).at(crate::now_ms()));
}

fn play(os: &mut Os, session: &mut Session, steps: &[Json]) {
    for step in steps {
        if let Some(app) = step["open"].as_str() {
            session.say(format!("> open {app}"), false);
            if let Err(note) = os.open(app) {
                session.say(format!("! {note}"), false);
            }
        } else if !step["call"].is_null() {
            act(os, session, crate::call_from(&step["call"]));
        } else if step["assert"].is_null() {
            act(os, session, crate::call_from(step));
        }
    }
}

// FRAME

pub(crate) fn frame(text: &str, size: (usize, usize)) -> String {
    let mut os = crate::build();
    let wire = Wire::load(&os);
    let mut session = Session::new();
    let steps = crate::wires(text);
    play(&mut os, &mut session, &steps);
    let env = os.envelope(None);
    render(&env, size, &wire, &session).dump()
}

// RENDER

fn accent(route: &str) -> (u8, u8, u8) {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in route.as_bytes() {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    let color = ROLLABLE[(hash % ROLLABLE.len() as u64) as usize];
    (color.r, color.g, color.b)
}

fn brief(value: &Json) -> String {
    match value {
        Json::Arr(items) if items.first().is_some_and(Json::is_array) => {
            let cols = items[0].as_array().map_or(0, Vec::len);
            format!("grid {}x{}", items.len(), cols)
        }
        Json::Arr(items) if items.first().is_some_and(Json::is_object) => {
            format!("{} items", items.len())
        }
        Json::Str(text) if text.chars().count() > 64 => {
            format!("{}..", text.chars().take(64).collect::<String>())
        }
        other => other.to_string(),
    }
}

fn clip(text: &str, width: usize) -> String {
    let mut out = String::new();
    let mut used = 0;
    for ch in text.chars() {
        if term::zero(ch) {
            out.push(ch);
            continue;
        }
        let need = if term::wide(ch) { 2 } else { 1 };
        if used + need > width {
            break;
        }
        out.push(ch);
        used += need;
    }
    out
}

fn render(env: &Envelope, size: (usize, usize), wire: &Wire, session: &Session) -> Screen {
    let (w, h) = size;
    let mut screen = Screen::new(w, h);
    if w == 0 || h == 0 {
        return screen;
    }
    let route = env
        .route
        .as_ref()
        .map(|r| r.app.clone())
        .unwrap_or_default();
    let ink = accent(&route);
    let body = h.saturating_sub(2);
    let split = body.div_ceil(2);
    let paned = session.json && w >= WIDE;
    let lw = if paned { w / 2 - 1 } else { w };
    face(&mut screen, env, wire, &route, ink, lw);
    pane(&mut screen, env, ink, lw, split);
    if paned {
        wired(&mut screen, env, w / 2 + 1, split);
    }
    feed(&mut screen, session, split, body);
    input(&mut screen, session);
    status(&mut screen, env, wire, session, &route);
    screen
}

fn face(
    screen: &mut Screen,
    env: &Envelope,
    wire: &Wire,
    route: &str,
    ink: (u8, u8, u8),
    lw: usize,
) {
    let (emoji, title) = wire.face(route);
    let x = screen.text(0, 0, &emoji, Some(ink), false);
    let end = screen.text(
        x + 1,
        0,
        &clip(&title, lw.saturating_sub(x + 1)),
        None,
        false,
    );
    if let Some(beat) = env.view.as_ref().and_then(|v| v.beat.as_ref()) {
        let width = beat.verb.chars().count();
        if lw > end + width + 1 {
            screen.text(lw - width, 0, &beat.verb, None, true);
        }
    }
}

fn pane(screen: &mut Screen, env: &Envelope, ink: (u8, u8, u8), lw: usize, split: usize) {
    if split < 2 {
        return;
    }
    let stop = split - 1;
    let mut y = 1;
    if let Some(view) = &env.view {
        if let Some(params) = view.params.as_object() {
            if !params.is_empty() && y < stop {
                let text = params
                    .iter()
                    .map(|(k, v)| format!("{k}={}", brief(v)))
                    .collect::<Vec<_>>()
                    .join(" ");
                screen.text(0, y, &clip(&text, lw), None, true);
                y += 1;
            }
        }
        y = sketch(screen, &view.state, lw, y, stop);
        if let Some(state) = view.state.as_object() {
            for (key, value) in state.iter() {
                if y >= stop {
                    break;
                }
                if key == "cells" || key == "shade" || key == "md" || key == "tris" {
                    continue;
                }
                let text = format!("{key}: {}", brief(value));
                screen.text(0, y, &clip(&text, lw), None, false);
                y += 1;
            }
            if let Some(md) = state.get("md").and_then(Json::as_str) {
                for line in md.lines() {
                    if y >= stop {
                        break;
                    }
                    screen.text(0, y, &clip(line, lw), None, false);
                    y += 1;
                }
            }
        }
    }
    for notice in &env.notices {
        if y >= stop {
            break;
        }
        let text = format!("! {} {}", notice.title, notice.body);
        screen.text(0, y, &clip(&text, lw), None, true);
        y += 1;
    }
    if let Some(note) = env
        .last
        .as_ref()
        .filter(|last| !last.ok)
        .and_then(|last| last.note.as_ref())
    {
        if y < stop {
            screen.text(0, y, &clip(&format!("! {note}"), lw), None, true);
        }
    }
    actions(screen, env, stop, ink, lw);
}

fn sketch(screen: &mut Screen, state: &Json, lw: usize, y: usize, stop: usize) -> usize {
    let cells = &state["cells"];
    let Some(grid) = cells["ids"].as_array() else {
        return y;
    };
    if grid.is_empty() || y >= stop || lw == 0 {
        return y;
    }
    let gh = grid.len();
    let gw = grid
        .iter()
        .map(|row| row.as_array().map_or(0, Vec::len))
        .max()
        .unwrap_or(0);
    if gw == 0 {
        return y;
    }
    let empty = Vec::new();
    let pens: Vec<(u8, u8, u8)> = cells["pens"]
        .as_array()
        .unwrap_or(&empty)
        .iter()
        .map(|pen| crate::rgb(pen.as_str().unwrap_or("")))
        .collect();
    let id = |r: usize, c: usize| -> usize {
        grid.get(r)
            .and_then(|row| row.as_array())
            .and_then(|row| row.get(c))
            .and_then(Json::as_u64)
            .unwrap_or(0) as usize
    };
    let pen = |n: usize| -> Option<(u8, u8, u8)> {
        if n == 0 || pens.is_empty() {
            None
        } else {
            Some(pens[(n - 1) % pens.len()])
        }
    };
    let room = stop - y;
    if cells["skin"].as_str() == Some("digits") {
        let scale = gh.div_ceil(room).max(gw.div_ceil(lw)).max(1);
        let oh = gh.div_ceil(scale);
        let ow = gw.div_ceil(scale).min(lw);
        let left = (lw - ow) / 2;
        for r in 0..oh {
            for c in 0..ow {
                let n = id(r * scale, c * scale);
                let cell = if n == 0 {
                    Cell::ink('\u{00b7}', None, true)
                } else {
                    let ch = char::from_digit((n % 36) as u32, 36).unwrap_or('#');
                    Cell::ink(ch, pen(n), false)
                };
                screen.put(left + c, y + r, cell);
            }
        }
        y + oh
    } else {
        if pens.is_empty() {
            return y;
        }
        let scale = gh.div_ceil(room * 2).max(gw.div_ceil(lw)).max(1);
        let oh = gh.div_ceil(scale);
        let ow = gw.div_ceil(scale).min(lw);
        let left = (lw - ow) / 2;
        let rows = oh.div_ceil(2);
        for tr in 0..rows {
            for c in 0..ow {
                let top = pen(id(tr * 2 * scale, c * scale));
                let low = if tr * 2 + 1 < oh {
                    pen(id((tr * 2 + 1) * scale, c * scale))
                } else {
                    None
                };
                let cell = match (top, low) {
                    (None, None) => continue,
                    (Some(t), Some(b)) => Cell::block(t, b),
                    (Some(t), None) => Cell::ink('\u{2580}', Some(t), false),
                    (None, Some(b)) => Cell::ink('\u{2584}', Some(b), false),
                };
                screen.put(left + c, y + tr, cell);
            }
        }
        y + rows
    }
}

fn wired(screen: &mut Screen, env: &Envelope, x0: usize, split: usize) {
    let width = screen.w.saturating_sub(x0);
    if width == 0 {
        return;
    }
    let mut doc = env.to_json();
    crate::collapse(&mut doc);
    let text = doc.pretty();
    for (y, line) in text.lines().enumerate() {
        if y >= split {
            break;
        }
        screen.text(x0, y, &clip(line, width), None, true);
    }
}

fn feed(screen: &mut Screen, session: &Session, top: usize, bottom: usize) {
    if bottom <= top {
        return;
    }
    let rows = bottom - top;
    let shown = session.feed.len().min(rows);
    let skip = session.feed.len() - shown;
    let start = bottom - shown;
    for (i, entry) in session.feed.iter().skip(skip).enumerate() {
        screen.text(0, start + i, &clip(&entry.text, screen.w), None, entry.dim);
    }
}

fn input(screen: &mut Screen, session: &Session) {
    if screen.h < 2 {
        return;
    }
    let y = screen.h - 2;
    match &session.line {
        Some(line) => {
            let x = screen.text(0, y, &clip(line, screen.w.saturating_sub(1)), None, false);
            screen.put(x, y, Cell::ink(CURSOR, None, true));
        }
        None => {
            screen.text(0, y, ">", None, true);
        }
    }
}

fn actions(screen: &mut Screen, env: &Envelope, y: usize, ink: (u8, u8, u8), lw: usize) {
    let empty = Vec::new();
    let verbs = env.view.as_ref().map(|v| &v.actions).unwrap_or(&empty);
    if verbs.is_empty() {
        screen.text(0, y, "no actions", None, true);
        return;
    }
    let mut x = 0;
    for (i, verb) in verbs.iter().take(9).enumerate() {
        if x + 4 >= lw {
            break;
        }
        x = screen.text(x, y, &format!("[{}] ", i + 1), None, true);
        x = screen.text(
            x,
            y,
            &clip(&verb.name, lw.saturating_sub(x)),
            Some(ink),
            false,
        ) + 1;
    }
    if verbs.len() > 9 && x < lw {
        screen.text(x, y, &format!("+ {} more", verbs.len() - 9), None, true);
    }
}

fn status(screen: &mut Screen, env: &Envelope, wire: &Wire, session: &Session, route: &str) {
    if screen.h < 1 {
        return;
    }
    let y = screen.h - 1;
    if let Some(line) = &session.line {
        let text = guide(wire, route, line);
        screen.text(0, y, &clip(&text, screen.w), None, true);
        return;
    }
    let left = format!("mrly · {route} · tick {}", env.tick);
    let end = screen.text(0, y, &left, None, true);
    let width = HELP.chars().count();
    if screen.w > end + width + 1 {
        screen.text(screen.w - width, y, HELP, None, true);
    }
}

fn guide(wire: &Wire, route: &str, line: &str) -> String {
    let body = line.strip_prefix('/').unwrap_or(line);
    let Some((word, rest)) = body.split_once(char::is_whitespace) else {
        return candidates(wire, route, body).join("  ");
    };
    let rest = rest.trim();
    match word {
        "app" => wire
            .apps
            .iter()
            .map(|app| app.route.as_str())
            .filter(|route| route.starts_with(rest))
            .collect::<Vec<_>>()
            .join("  "),
        "json" => "toggle the wire pane".to_string(),
        "seed" => "/seed <int>".to_string(),
        "skin" => "/skin <name>".to_string(),
        "replay" => "/replay <file>".to_string(),
        _ => wire
            .verbs
            .iter()
            .find(|(name, _)| name == word)
            .map(|(_, args)| hint(args))
            .unwrap_or_default(),
    }
}

fn hint(args: &Json) -> String {
    args.as_object()
        .map(|map| {
            map.iter()
                .map(|(k, v)| format!("{k}: {}", v.as_str().unwrap_or("?")))
                .collect::<Vec<_>>()
                .join(" · ")
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn boot(app: &str) -> (Os, Wire, Session) {
        let mut os = crate::build();
        let wire = Wire::load(&os);
        os.open(app).unwrap();
        (os, wire, Session::new())
    }

    #[test]
    fn every_app_renders_at_any_size() {
        let mut os = crate::build();
        let wire = Wire::load(&os);
        let session = Session::new();
        for route in os.catalogue() {
            os.open(&route).unwrap();
            let env = os.envelope(None);
            for size in [(80, 24), (20, 6), (1, 1)] {
                let screen = render(&env, size, &wire, &session);
                assert_eq!((screen.w, screen.h), size);
            }
        }
    }

    #[test]
    fn snake_reads_its_title_actions_and_wire() {
        let (os, wire, session) = boot("snake");
        let env = os.envelope(None);
        let text = render(&env, (80, 24), &wire, &session).dump();
        assert!(text.contains("snake"));
        assert!(text.contains("[1] snake.turn"));
        assert!(text.contains("tick 0"));
        assert!(text.contains("\"tick\": 0"));
    }

    #[test]
    fn json_toggles_the_wire_pane() {
        let (mut os, wire, mut session) = boot("snake");
        submit(&mut os, &wire, &mut session, "/json");
        assert!(!session.json);
        let env = os.envelope(None);
        let text = render(&env, (80, 24), &wire, &session).dump();
        assert!(!text.contains("\"tick\""));
    }

    #[test]
    fn the_line_opens_submits_and_remembers() {
        let (mut os, wire, mut session) = boot("snake");
        press(&mut os, &wire, &mut session, Key::Char('/'));
        for ch in "app menu".chars() {
            press(&mut os, &wire, &mut session, Key::Char(ch));
        }
        press(&mut os, &wire, &mut session, Key::Enter);
        assert_eq!(here(&os), "menu");
        assert!(session.line.is_none());
        press(&mut os, &wire, &mut session, Key::Char('/'));
        press(&mut os, &wire, &mut session, Key::Up);
        assert_eq!(session.line.as_deref(), Some("/app menu"));
        press(&mut os, &wire, &mut session, Key::Down);
        assert_eq!(session.line.as_deref(), Some("/"));
    }

    #[test]
    fn tab_completes_from_the_listing() {
        let (os, wire, mut session) = boot("snake");
        session.line = Some("/snake.t".to_string());
        complete(&os, &wire, &mut session);
        assert_eq!(session.line.as_deref(), Some("/snake.turn "));
        session.line = Some("/app sna".to_string());
        complete(&os, &wire, &mut session);
        assert_eq!(session.line.as_deref(), Some("/app snake"));
    }

    #[test]
    fn seed_rides_the_listing() {
        let (mut os, wire, mut session) = boot("snake");
        submit(&mut os, &wire, &mut session, "/seed 7");
        let env = os.envelope(None);
        assert_eq!(env.view.unwrap().state["seed"].as_i64(), Some(7));
        assert!(session.feed.iter().any(|e| e.text.contains("snake.reset")));
    }

    #[test]
    fn keymaps_carry_the_snake_turns() {
        let (mut os, wire, mut session) = boot("snake");
        let before = os.envelope(None).tick;
        bound(&mut os, &wire, &mut session, "up");
        assert!(os.envelope(None).tick > before);
        assert!(session.feed.iter().any(|e| e.text.contains("snake.turn")));
    }

    #[test]
    fn a_cells_grid_sketches_as_half_blocks() {
        let state =
            json!({ "cells": { "ids": [[1, 0], [0, 1]], "skin": "tiles", "pens": ["#ff0000"] } });
        let mut screen = Screen::new(10, 4);
        let y = sketch(&mut screen, &state, 10, 0, 4);
        assert_eq!(y, 1);
        let text = screen.dump();
        assert!(text.contains('\u{2580}'));
        assert!(text.contains('\u{2584}'));
    }

    #[test]
    fn a_digits_grid_sketches_as_glyphs() {
        let state = json!({ "cells": { "ids": [[0, 5], [12, 0]], "skin": "digits", "pens": [] } });
        let mut screen = Screen::new(10, 4);
        let y = sketch(&mut screen, &state, 10, 0, 4);
        assert_eq!(y, 2);
        let text = screen.dump();
        assert!(text.contains('5'));
        assert!(text.contains('c'));
        assert!(text.contains('\u{00b7}'));
    }
}
