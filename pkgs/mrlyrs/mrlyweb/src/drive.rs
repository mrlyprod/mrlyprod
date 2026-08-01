use crate::face::face_scene;
use mrlycore::ui::Call as Wish;
use mrlycore::{json, Json};
use mrlyos::kernel::{Call, Effect, Os};
pub use mrlyui::face::{Act, Hit, Scene};
use mrlyui::face::{Edit, UiState, HEIGHT, SCALE, WIDTH};

pub const BEAT: i64 = 125;
const EFFECTS: usize = 64;

pub enum KeyPress {
    Char(char),
    Backspace,
    Enter,
    Escape,
}

enum Drag {
    Slide {
        x: usize,
        w: usize,
        min: i64,
        max: i64,
        step: i64,
        call: Wish,
        arg: String,
    },
    Board {
        x: usize,
        y: usize,
        cols: usize,
        rows: usize,
        pw: usize,
        ph: usize,
        sunk: usize,
        tap: Option<Wish>,
        drag: Option<Wish>,
        turn: Option<Wish>,
        pan: Option<Wish>,
        start: (usize, usize),
        last: (usize, usize),
        points: Vec<(usize, usize)>,
    },
}

pub struct Driver {
    os: Os,
    route: String,
    ui: UiState,
    scene: Scene,
    clock: Option<i64>,
    drag: Option<Drag>,
    dirty: bool,
    effects: Vec<Effect>,
}

fn wall_ms() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_millis() as i64)
        .unwrap_or(0)
}

fn subset(args: &Json, of: &Json) -> bool {
    match args.as_object() {
        Some(map) => map.iter().all(|(k, v)| &of[k] == v),
        None => true,
    }
}

impl Driver {
    pub fn scripted(start: i64) -> Driver {
        Driver::boot(Some(start))
    }

    pub fn wall() -> Driver {
        Driver::boot(None)
    }

    fn boot(clock: Option<i64>) -> Driver {
        let mut driver = Driver {
            os: crate::registry::boot(),
            route: String::new(),
            ui: UiState::default(),
            scene: Scene {
                frame: mrlyui::frame::field(1, 1, vec![[0, 0, 0, 255]], [0, 0, 0, 255]),
                hits: Vec::new(),
                body: 0,
                window: 0,
            },
            clock,
            drag: None,
            dirty: false,
            effects: Vec::new(),
        };
        driver.act("nav.open", json!({ "app": "menu" }));
        driver
    }

    fn tick(&mut self) -> i64 {
        match &mut self.clock {
            Some(now) => {
                *now += BEAT;
                *now
            }
            None => wall_ms(),
        }
    }

    pub fn route(&self) -> &str {
        &self.route
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn ui(&self) -> &UiState {
        &self.ui
    }

    pub fn os(&self) -> &Os {
        &self.os
    }

    pub fn editing(&self) -> bool {
        self.ui.edit.is_some()
    }

    pub fn dirty(&mut self) -> bool {
        std::mem::take(&mut self.dirty)
    }

    pub fn act(&mut self, verb: &str, args: Json) {
        let at = self.tick();
        self.os.act(Call::new(verb, args).at(at));
        self.collect();
        let route = self.os.frame(None).route.map(|r| r.app).unwrap_or_default();
        if route != self.route {
            self.route = route;
            self.ui = UiState::default();
            self.drag = None;
        }
        self.refresh();
    }

    fn fire(&mut self, wish: &Wish) {
        self.ui.menu = None;
        self.act(&wish.verb.clone(), wish.args.clone());
    }

    fn refresh(&mut self) {
        if let Ok(scene) = face_scene(&self.os, &self.route, &self.ui) {
            self.scene = scene;
        }
        self.dirty = true;
    }

    pub fn open(&mut self, app: &str) {
        self.act("nav.open", json!({ "app": app }));
    }

    pub fn beat(&mut self, n: usize) {
        for _ in 0..n {
            let Some(call) = self.os.frame(None).view.and_then(|v| v.beat) else {
                return;
            };
            let at = self.tick();
            self.os.act(call.at(at));
            self.collect();
        }
        self.refresh();
    }

    fn collect(&mut self) {
        self.effects.extend_from_slice(self.os.effects());
        let overflow = self.effects.len().saturating_sub(EFFECTS);
        if overflow > 0 {
            self.effects.drain(..overflow);
        }
    }

    pub fn drain_effects(&mut self) -> Vec<Effect> {
        std::mem::take(&mut self.effects)
    }

    fn over(&self, x: usize, y: usize) -> Option<Hit> {
        self.scene
            .hits
            .iter()
            .rev()
            .find(|hit| hit.holds(x, y))
            .cloned()
    }

    fn commit(&mut self) {
        let Some(edit) = self.ui.edit.take() else {
            return;
        };
        let target = self.scene.hits.iter().find_map(|hit| match &hit.act {
            Act::Edit {
                id,
                value,
                call,
                arg,
                ..
            } if *id == edit.id => Some((value.clone(), call.clone(), arg.clone())),
            _ => None,
        });
        if let Some((value, call, arg)) = target {
            if value != edit.buffer {
                self.fire(&call.fill(&arg, Json::Str(edit.buffer)));
                return;
            }
        }
        self.refresh();
    }

    pub fn press(&mut self, x: usize, y: usize) {
        let Some(hit) = self.over(x, y) else {
            return;
        };
        match hit.act.clone() {
            Act::Tap { call } => {
                self.commit();
                self.fire(&call);
            }
            Act::Slide {
                call,
                arg,
                min,
                max,
                step,
            } => {
                self.commit();
                self.drag = Some(Drag::Slide {
                    x: hit.x,
                    w: hit.w,
                    min,
                    max,
                    step,
                    call,
                    arg,
                });
            }
            Act::Board {
                cols,
                rows,
                pw,
                ph,
                sunk,
                tap,
                drag,
                turn,
                pan,
                ..
            } => {
                self.commit();
                let cell = board_cell(hit.x, hit.y, cols, rows, pw, ph, sunk, x, y);
                self.drag = Some(Drag::Board {
                    x: hit.x,
                    y: hit.y,
                    cols,
                    rows,
                    pw,
                    ph,
                    sunk,
                    tap,
                    drag,
                    turn,
                    pan,
                    start: (x, y),
                    last: (x, y),
                    points: vec![cell],
                });
            }
            Act::Edit { id, value, .. } => {
                self.commit();
                self.ui.edit = Some(Edit { id, buffer: value });
                self.refresh();
            }
            Act::Menu { id } => {
                self.commit();
                self.ui.menu = Some(id);
                self.refresh();
            }
            Act::Shut => {
                self.ui.menu = None;
                self.refresh();
            }
            Act::Mute => {}
        }
    }

    pub fn move_to(&mut self, x: usize, y: usize) {
        let Some(Drag::Board {
            last,
            points,
            drag,
            x: bx,
            y: by,
            cols,
            rows,
            pw,
            ph,
            sunk,
            ..
        }) = &mut self.drag
        else {
            return;
        };
        *last = (x, y);
        if drag.is_none() {
            return;
        }
        let cell = board_cell(*bx, *by, *cols, *rows, *pw, *ph, *sunk, x, y);
        if points.last() != Some(&cell) {
            points.push(cell);
        }
    }

    pub fn release(&mut self, x: usize, _y: usize) {
        let Some(drag) = self.drag.take() else {
            return;
        };
        match drag {
            Drag::Slide {
                x: tx,
                w,
                min,
                max,
                step,
                call,
                arg,
            } => {
                let span = (max - min).max(0);
                let frac = (x.saturating_sub(tx)).min(w) as f64 / w.max(1) as f64;
                let raw = min + (frac * span as f64).round() as i64;
                let value = min + ((raw - min) / step.max(1)) * step.max(1);
                self.fire(&call.fill(&arg, Json::Int(value.clamp(min, max))));
            }
            Drag::Board {
                tap,
                drag,
                turn,
                pan,
                start,
                last,
                points,
                ..
            } => {
                let dx = last.0 as i64 - start.0 as i64;
                let dy = last.1 as i64 - start.1 as i64;
                let moved = dx.abs() + dy.abs() > 4;
                if let (Some(call), true) = (&drag, points.len() > 1) {
                    let list: Vec<Json> = points
                        .iter()
                        .map(|(c, r)| Json::Arr(vec![Json::Int(*c as i64), Json::Int(*r as i64)]))
                        .collect();
                    self.fire(&call.fill("points", Json::Arr(list)));
                } else if let (Some(call), true) = (&turn, moved) {
                    let mut wish = call.clone();
                    wish.args["dyaw"] = Json::Int(dx);
                    wish.args["dpitch"] = Json::Int(dy);
                    self.fire(&wish);
                } else if let (Some(call), true) = (&pan, moved) {
                    let mut wish = call.clone();
                    wish.args["dx"] = Json::Int(dx);
                    wish.args["dy"] = Json::Int(dy);
                    self.fire(&wish);
                } else if let (Some(call), Some((col, row))) = (&tap, points.first()) {
                    let mut wish = call.clone();
                    wish.args["x"] = Json::Int(*col as i64);
                    wish.args["y"] = Json::Int(*row as i64);
                    self.fire(&wish);
                }
            }
        }
    }

    pub fn wheel(&mut self, dy: f64, at: Option<(usize, usize)>) {
        if let Some((x, y)) = at {
            if let Some(hit) = self.over(x, y) {
                if let Act::Board {
                    zoom: Some(call), ..
                } = &hit.act
                {
                    let mut wish = call.clone();
                    wish.args["dir"] = Json::Str(if dy > 0.0 { "in" } else { "out" }.to_string());
                    wish.args["n"] = Json::Int(1);
                    self.fire(&wish);
                    return;
                }
            }
        }
        let cap = self.scene.body.saturating_sub(self.scene.window);
        let next = (self.ui.scroll as f64 - dy).clamp(0.0, cap as f64) as usize;
        if next != self.ui.scroll {
            self.ui.scroll = next;
            self.refresh();
        }
    }

    pub fn key(&mut self, press: KeyPress) {
        let Some(mut edit) = self.ui.edit.clone() else {
            return;
        };
        match press {
            KeyPress::Escape => {
                self.ui.edit = None;
                self.refresh();
                return;
            }
            KeyPress::Enter => {
                let enter = self.scene.hits.iter().find_map(|hit| match &hit.act {
                    Act::Edit { id, enter, .. } if *id == edit.id => enter.clone(),
                    _ => None,
                });
                self.commit();
                if let Some(call) = enter {
                    self.fire(&call);
                }
                return;
            }
            KeyPress::Backspace => {
                edit.buffer.pop();
            }
            KeyPress::Char(c) => {
                if c.is_ascii_graphic() || c == ' ' {
                    edit.buffer.push(c);
                }
            }
        }
        self.ui.edit = Some(edit.clone());
        let live = self.scene.hits.iter().find_map(|hit| match &hit.act {
            Act::Edit {
                id,
                live: true,
                call,
                arg,
                ..
            } if *id == edit.id => Some((call.clone(), arg.clone())),
            _ => None,
        });
        if let Some((call, arg)) = live {
            let keep = self.ui.edit.clone();
            self.fire(&call.fill(&arg, Json::Str(edit.buffer)));
            self.ui.edit = keep;
        }
        self.refresh();
    }

    pub fn escape(&mut self) {
        if self.ui.edit.is_some() {
            self.key(KeyPress::Escape);
        } else if self.ui.menu.is_some() {
            self.ui.menu = None;
            self.refresh();
        } else {
            self.open("menu");
        }
    }

    pub fn bound(&mut self, dir: &str) {
        let Some(manifest) = crate::face::manifest(&self.route.clone()) else {
            return;
        };
        let Some((_, call)) = manifest.keys.into_iter().find(|(d, _)| d == dir) else {
            return;
        };
        self.act(&call.verb.clone(), call.args.clone());
    }

    pub fn nth(&mut self, index: usize) {
        let Some(view) = self.os.frame(None).view else {
            return;
        };
        let Some(verb) = view.actions.get(index) else {
            return;
        };
        if verb.args.as_object().is_none_or(|args| args.is_empty()) {
            self.act(&verb.name.clone(), json!({}));
        }
    }

    fn find(&self, want: impl Fn(&Act) -> Option<Wish>) -> Option<Wish> {
        self.scene.hits.iter().find_map(|hit| want(&hit.act))
    }

    pub fn tap(&mut self, verb: &str, args: &Json) -> Result<(), String> {
        let found = self.find(|act| match act {
            Act::Tap { call } if call.verb == verb && subset(args, &call.args) => {
                Some(call.clone())
            }
            _ => None,
        });
        match found {
            Some(call) => {
                self.commit();
                self.fire(&call);
                Ok(())
            }
            None => Err(format!("no tap for {verb} {args}")),
        }
    }

    pub fn tap_at(&mut self, x: usize, y: usize) {
        self.press(x, y);
        self.release(x, y);
    }

    pub fn slide(&mut self, verb: &str, value: i64) -> Result<(), String> {
        let found = self.scene.hits.iter().find_map(|hit| match &hit.act {
            Act::Slide {
                call,
                arg,
                min,
                max,
                step,
            } if call.verb == verb => Some((call.clone(), arg.clone(), *min, *max, *step)),
            _ => None,
        });
        match found {
            Some((call, arg, min, max, step)) => {
                let stepped = min + ((value - min) / step.max(1)) * step.max(1);
                self.fire(&call.fill(&arg, Json::Int(stepped.clamp(min, max))));
                Ok(())
            }
            None => Err(format!("no slider for {verb}")),
        }
    }

    pub fn cell(&mut self, col: usize, row: usize) -> Result<(), String> {
        let found = self.find(|act| match act {
            Act::Board {
                tap: Some(call), ..
            } => Some(call.clone()),
            _ => None,
        });
        match found {
            Some(call) => {
                let mut wish = call;
                wish.args["x"] = Json::Int(col as i64);
                wish.args["y"] = Json::Int(row as i64);
                self.fire(&wish);
                Ok(())
            }
            None => Err("no board to tap".to_string()),
        }
    }

    pub fn focus(&mut self, verb: &str) -> Result<(), String> {
        let found = self.scene.hits.iter().find_map(|hit| match &hit.act {
            Act::Edit {
                id, value, call, ..
            } if call.verb == verb => Some((id.clone(), value.clone())),
            _ => None,
        });
        match found {
            Some((id, value)) => {
                self.commit();
                self.ui.edit = Some(Edit { id, buffer: value });
                self.refresh();
                Ok(())
            }
            None => Err(format!("no field for {verb}")),
        }
    }

    pub fn scroll_to(&mut self, to: usize) {
        let cap = self.scene.body.saturating_sub(self.scene.window);
        let next = to.min(cap);
        if next != self.ui.scroll {
            self.ui.scroll = next;
            self.refresh();
        }
    }

    pub fn type_str(&mut self, text: &str) {
        for c in text.chars() {
            self.key(KeyPress::Char(c));
        }
    }

    pub fn shot(&self) -> Result<Vec<u8>, &'static str> {
        let colors = self
            .scene
            .frame
            .composite()
            .cell
            .colors
            .ok_or("no pixels")?;
        mrlycore::png(&colors, WIDTH, HEIGHT, SCALE).map_err(|_| "could not encode")
    }

    pub fn frame_fnv(&self) -> u64 {
        let colors = self.scene.frame.composite().cell.colors.unwrap_or_default();
        let mut h: u64 = 0xcbf29ce484222325;
        for px in &colors {
            for byte in px {
                h ^= *byte as u64;
                h = h.wrapping_mul(0x100000001b3);
            }
        }
        h
    }

    pub fn hits_json(&self) -> Json {
        let hits: Vec<Json> = self.scene.hits.iter().map(hit_json).collect();
        json!({
            "route": &self.route,
            "hits": hits,
            "body": self.scene.body,
            "window": self.scene.window,
            "scroll": self.ui.scroll,
        })
    }
}

#[allow(clippy::too_many_arguments)]
fn board_cell(
    hx: usize,
    hy: usize,
    cols: usize,
    rows: usize,
    pw: usize,
    ph: usize,
    sunk: usize,
    sx: usize,
    sy: usize,
) -> (usize, usize) {
    let col = (sx.saturating_sub(hx) * cols / pw.max(1)).min(cols.saturating_sub(1));
    let row = ((sy.saturating_sub(hy) + sunk) * rows / ph.max(1)).min(rows.saturating_sub(1));
    (col, row)
}

fn hit_json(hit: &Hit) -> Json {
    let (kind, detail) = match &hit.act {
        Act::Tap { call } => ("tap", json!({ "verb": &call.verb, "args": &call.args })),
        Act::Slide {
            call,
            min,
            max,
            step,
            ..
        } => (
            "slide",
            json!({ "verb": &call.verb, "min": *min, "max": *max, "step": *step }),
        ),
        Act::Board {
            cols,
            rows,
            tap,
            drag,
            turn,
            zoom,
            pan,
            ..
        } => (
            "board",
            json!({
                "cols": *cols,
                "rows": *rows,
                "tap": tap.as_ref().map(|c| c.verb.clone()),
                "drag": drag.as_ref().map(|c| c.verb.clone()),
                "turn": turn.as_ref().map(|c| c.verb.clone()),
                "zoom": zoom.as_ref().map(|c| c.verb.clone()),
                "pan": pan.as_ref().map(|c| c.verb.clone()),
            }),
        ),
        Act::Edit { call, live, .. } => ("field", json!({ "verb": &call.verb, "live": *live })),
        Act::Menu { id } => ("menu", json!({ "id": id })),
        Act::Shut => ("shut", json!({})),
        Act::Mute => ("mute", json!({})),
    };
    json!({ "x": hit.x, "y": hit.y, "w": hit.w, "h": hit.h, "kind": kind, "act": detail })
}
