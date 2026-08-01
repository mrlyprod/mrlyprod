use crate::term::{self, Cell, Key, Raw, Screen};
use mrlycore::colors::ROLLABLE;
use mrlycore::{json, Json};
use mrlyos::kernel::{Call, Envelope, Os};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::sync::mpsc;
use std::time::{Duration, Instant};

const CURSOR: char = '\u{2588}';
const HELP: &str = "arrows move · 1-9 act · : verb · esc menu · q quit";

type Keys = HashMap<String, Vec<(String, Call)>>;
type Titles = HashMap<String, (String, String)>;

// LOOP

pub fn run() {
    let mut os = crate::build();
    let (keys, titles) = maps();
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
    let mut cmd: Option<String> = None;
    let mut beat = Instant::now();
    let mut dirty = true;
    let mut alive = true;
    while alive {
        let wait = Duration::from_millis(crate::BEAT as u64).saturating_sub(beat.elapsed());
        match rx.recv_timeout(wait) {
            Ok(chunk) => {
                for key in term::parse(&chunk) {
                    if !press(&mut os, &keys, &mut cmd, &mut dirty, key) {
                        alive = false;
                    }
                }
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                if let Some(call) = os.frame(None).view.and_then(|v| v.beat) {
                    os.act(call.at(crate::now_ms()));
                }
                beat = Instant::now();
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => alive = false,
        }
        let size = term::size();
        let env = os.frame(None);
        if dirty || last != Some(env.tick) || shape != size {
            let next = render(&env, size, cmd.as_deref(), &titles);
            let bytes = term::diff(screen.as_ref(), &next);
            if !bytes.is_empty() {
                let mut out = std::io::stdout();
                out.write_all(bytes.as_bytes()).ok();
                out.flush().ok();
            }
            screen = Some(next);
            last = Some(env.tick);
            shape = size;
            dirty = false;
        }
    }
    drop(raw);
}

fn maps() -> (Keys, Titles) {
    let mut keys = Keys::new();
    let mut titles = Titles::new();
    for app in mrlyweb::registry::catalogue() {
        let manifest = app.manifest();
        titles.insert(
            manifest.route.clone(),
            (manifest.emoji.clone(), manifest.title.clone()),
        );
        keys.insert(manifest.route, manifest.keys);
    }
    (keys, titles)
}

// INPUT

fn press(os: &mut Os, keys: &Keys, cmd: &mut Option<String>, dirty: &mut bool, key: Key) -> bool {
    if cmd.is_some() {
        typing(os, cmd, dirty, key);
        return true;
    }
    match key {
        Key::Ctrl('c') | Key::Char('q') => return false,
        Key::Char(':') => {
            *cmd = Some(String::new());
            *dirty = true;
        }
        Key::Esc => {
            os.act(Call::new("nav.open", json!({ "app": "menu" })).at(crate::now_ms()));
        }
        Key::Up | Key::Char('w') | Key::Char('W') => bound(os, keys, "up"),
        Key::Down | Key::Char('s') | Key::Char('S') => bound(os, keys, "down"),
        Key::Left | Key::Char('a') | Key::Char('A') => bound(os, keys, "left"),
        Key::Right | Key::Char('d') | Key::Char('D') => bound(os, keys, "right"),
        Key::Char(ch) if ('1'..='9').contains(&ch) => {
            nth(os, cmd, dirty, ch as usize - '1' as usize)
        }
        _ => {}
    }
    true
}

fn typing(os: &mut Os, cmd: &mut Option<String>, dirty: &mut bool, key: Key) {
    let Some(line) = cmd.as_mut() else { return };
    match key {
        Key::Char(ch) => line.push(ch),
        Key::Backspace => {
            line.pop();
        }
        Key::Esc | Key::Ctrl('c') => *cmd = None,
        Key::Enter => {
            if submit(os, &line.clone()) {
                *cmd = None;
            }
        }
        _ => return,
    }
    *dirty = true;
}

fn submit(os: &mut Os, line: &str) -> bool {
    let line = line.trim();
    if line.is_empty() {
        return true;
    }
    let mut it = line.splitn(2, char::is_whitespace);
    let verb = it.next().unwrap_or("");
    let rest = it.next().unwrap_or("").trim();
    let args = if rest.is_empty() {
        json!({})
    } else {
        match mrlycore::json::parse(rest) {
            Ok(value) => value,
            Err(_) => return false,
        }
    };
    os.act(Call::new(verb, args).at(crate::now_ms()));
    true
}

fn bound(os: &mut Os, keys: &Keys, dir: &str) {
    let Some(route) = os.frame(None).route.map(|r| r.app) else {
        return;
    };
    let Some(call) = keys
        .get(&route)
        .and_then(|binds| binds.iter().find(|(d, _)| d == dir))
        .map(|(_, call)| call.clone())
    else {
        return;
    };
    os.act(call.at(crate::now_ms()));
}

fn nth(os: &mut Os, cmd: &mut Option<String>, dirty: &mut bool, index: usize) {
    let Some(view) = os.frame(None).view else {
        return;
    };
    let Some(verb) = view.actions.get(index) else {
        return;
    };
    if verb.args.as_object().is_none_or(|args| args.is_empty()) {
        let call = Call::new(&verb.name, json!({})).at(crate::now_ms());
        os.act(call);
    } else {
        *cmd = Some(format!("{} ", verb.name));
        *dirty = true;
    }
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

fn render(env: &Envelope, size: (usize, usize), cmd: Option<&str>, titles: &Titles) -> Screen {
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
    let mut y = 0;
    if y < body {
        let plain = ("\u{2728}".to_string(), route.clone());
        let (emoji, title) = titles.get(&route).unwrap_or(&plain);
        let x = screen.text(0, y, emoji, Some(ink), false);
        let end = screen.text(x + 1, y, title, None, false);
        if let Some(beat) = env.view.as_ref().and_then(|v| v.beat.as_ref()) {
            let width = beat.verb.chars().count();
            if w > end + width + 1 {
                screen.text(w - width, y, &beat.verb, None, true);
            }
        }
        y += 1;
    }
    if let Some(view) = &env.view {
        if let Some(params) = view.params.as_object() {
            if !params.is_empty() && y < body {
                let text = params
                    .iter()
                    .map(|(k, v)| format!("{k}={}", brief(v)))
                    .collect::<Vec<_>>()
                    .join(" ");
                screen.text(0, y, &text, None, true);
                y += 1;
            }
        }
        y = canvas(&mut screen, &view.state["frame"], y, body);
        if let Some(state) = view.state.as_object() {
            for (key, value) in state.iter() {
                if y >= body {
                    break;
                }
                if key == "frame" || key == "shade" || key == "md" {
                    continue;
                }
                screen.text(0, y, &format!("{key}: {}", brief(value)), None, false);
                y += 1;
            }
            if let Some(md) = state.get("md").and_then(Json::as_str) {
                for line in md.lines() {
                    if y >= body {
                        break;
                    }
                    screen.text(0, y, line, None, false);
                    y += 1;
                }
            }
        }
    }
    for notice in &env.notices {
        if y >= body {
            break;
        }
        screen.text(
            0,
            y,
            &format!("! {} {}", notice.title, notice.body),
            None,
            true,
        );
        y += 1;
    }
    if let Some(note) = env
        .last
        .as_ref()
        .filter(|last| !last.ok)
        .and_then(|last| last.note.as_ref())
    {
        if y < body {
            screen.text(0, y, &format!("! {note}"), None, true);
        }
    }
    if h >= 2 {
        actions(&mut screen, env, h - 2, ink);
    }
    status(&mut screen, env, &route, cmd, h - 1);
    screen
}

fn canvas(screen: &mut Screen, frame: &Json, y: usize, body: usize) -> usize {
    let (Some(rows), Some(palette)) = (frame["rows"].as_array(), frame["palette"].as_array())
    else {
        return y;
    };
    if rows.is_empty() || y >= body {
        return y;
    }
    let colors: Vec<(u8, u8, u8)> = palette
        .iter()
        .map(|c| crate::rgb(c.as_str().unwrap_or("")))
        .collect();
    let ph = rows.len();
    let pw = rows
        .iter()
        .map(|r| r.as_array().map_or(0, Vec::len))
        .max()
        .unwrap_or(0);
    if pw == 0 {
        return y;
    }
    let scale = ph
        .div_ceil((body - y) * 2)
        .max(pw.div_ceil(screen.w))
        .max(1);
    let oh = ph.div_ceil(scale);
    let ow = pw.div_ceil(scale).min(screen.w);
    let at = |r: usize, c: usize| -> (u8, u8, u8) {
        let index = rows
            .get(r * scale)
            .and_then(|row| row.as_array())
            .and_then(|row| row.get(c * scale))
            .and_then(Json::as_u64)
            .unwrap_or(0) as usize;
        colors.get(index).copied().unwrap_or((0, 0, 0))
    };
    let left = (screen.w - ow) / 2;
    let mut cy = y;
    let mut r = 0;
    while r < oh && cy < body {
        for c in 0..ow {
            let top = at(r, c);
            let bottom = if r + 1 < oh { at(r + 1, c) } else { (0, 0, 0) };
            screen.put(left + c, cy, Cell::block(top, bottom));
        }
        r += 2;
        cy += 1;
    }
    cy
}

fn actions(screen: &mut Screen, env: &Envelope, y: usize, ink: (u8, u8, u8)) {
    let empty = Vec::new();
    let verbs = env.view.as_ref().map(|v| &v.actions).unwrap_or(&empty);
    if verbs.is_empty() {
        screen.text(0, y, "no actions", None, true);
        return;
    }
    let mut x = 0;
    for (i, verb) in verbs.iter().take(9).enumerate() {
        if x + 4 >= screen.w {
            break;
        }
        x = screen.text(x, y, &format!("[{}] ", i + 1), None, true);
        x = screen.text(x, y, &verb.name, Some(ink), false) + 1;
    }
    if verbs.len() > 9 {
        screen.text(x, y, &format!("+ {} more", verbs.len() - 9), None, true);
    }
}

fn status(screen: &mut Screen, env: &Envelope, route: &str, cmd: Option<&str>, y: usize) {
    if let Some(line) = cmd {
        let x = screen.text(0, y, &format!(":{line}"), None, false);
        screen.put(x, y, Cell::ink(CURSOR, None, true));
        return;
    }
    let left = format!("mrly · {route} · tick {}", env.tick);
    let end = screen.text(0, y, &left, None, true);
    let width = HELP.chars().count();
    if screen.w > end + width + 1 {
        screen.text(screen.w - width, y, HELP, None, true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_app_renders_at_any_size() {
        let (_, titles) = maps();
        let mut os = crate::build();
        let routes = os.catalogue();
        for route in routes {
            os.open(&route).unwrap();
            let env = os.frame(None);
            for size in [(80, 24), (20, 6), (1, 1)] {
                let screen = render(&env, size, None, &titles);
                assert_eq!((screen.w, screen.h), size);
            }
            let typed = render(&env, (80, 24), Some("snake.step "), &titles);
            assert!(typed.dump().contains(":snake.step"));
        }
    }

    #[test]
    fn snake_reads_its_title_and_actions() {
        let (_, titles) = maps();
        let mut os = crate::build();
        os.open("snake").unwrap();
        let env = os.frame(None);
        let text = render(&env, (80, 24), None, &titles).dump();
        assert!(text.contains("snake"));
        assert!(text.contains("[1] snake.turn"));
        assert!(text.contains("tick 0"));
    }

    #[test]
    fn keymaps_carry_the_snake_turns() {
        let (keys, _) = maps();
        let binds = keys.get("snake").unwrap();
        assert_eq!(binds.len(), 4);
        let mut os = crate::build();
        os.open("snake").unwrap();
        let before = os.frame(None).tick;
        bound(&mut os, &keys, "up");
        assert!(os.frame(None).tick > before);
    }
}
