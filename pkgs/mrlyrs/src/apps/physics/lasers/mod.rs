use crate::core::colors::{named, RED};
use crate::core::rng::Rng;
use crate::core::trig::{FracIndex, N as TRIG_N};
use crate::os::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use crate::physics::{Emitter, Lasers as Sim, LasersConfig, Mask};
use crate::ui::frame::{self, Frame};
use crate::ui::shaders;
use serde_json::{json, Value as Json};
use std::f32::consts::{PI, TAU};

const WALL: [u8; 4] = [90, 90, 94, 255];
const DESIGNS: [&str; 5] = ["carpet", "net", "htree", "vtree", "void"];
const NUMBERS: [i64; 4] = [3, 5, 7, 9];
const MASK_KEYS: [&str; 6] = ["design", "number", "level", "padding", "invert", "subpixel"];
const SUBPIXELS: [i64; 3] = [1, 2, 4];
const SPREADS: [(&str, f32); 5] = [
    ("none", 0.0),
    ("narrow", PI / 8.0),
    ("wide", PI / 2.0),
    ("half", PI),
    ("full", TAU),
];
const SPINS: [f64; 5] = [0.0, 0.1, 0.3, 0.8, 2.0];

fn too_many_cells(number: i64, level: i64, subpixel: i64) -> bool {
    match (number as u32).checked_pow(level as u32) {
        Some(cells) => (cells as i64) * subpixel > 256,
        None => true,
    }
}

fn radians_to_index(radians: f32) -> f32 {
    radians / TAU * TRIG_N as f32
}

fn bresenham(x0: i64, y0: i64, x1: i64, y1: i64) -> Vec<(i64, i64)> {
    let mut points = Vec::new();
    let (mut x, mut y) = (x0, y0);
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        points.push((x, y));
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
    points
}

struct Set {
    design: String,
    number: i64,
    level: i64,
    padding: i64,
    invert: bool,
    subpixel: i64,
    rays: i64,
    spread: String,
    bounces: i64,
    spin: f64,
    accent: String,
}

impl Set {
    fn new() -> Set {
        Set {
            design: "carpet".to_string(),
            number: 5,
            level: 2,
            padding: 8,
            invert: false,
            subpixel: 2,
            rays: 32,
            spread: "full".to_string(),
            bounces: 16,
            spin: 0.3,
            accent: "red".to_string(),
        }
    }
    fn apply(&mut self, key: &str, value: &Json) -> Result<Json, &'static str> {
        match key {
            "design" => {
                let d = value.as_str().ok_or("value must be a string")?;
                if !DESIGNS.contains(&d) {
                    return Err("no such option");
                }
                self.design = d.to_string();
                Ok(json!(d))
            }
            "number" => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                if !NUMBERS.contains(&n) {
                    return Err("no such option");
                }
                if too_many_cells(n, self.level, self.subpixel) {
                    return Err("too many cells");
                }
                self.number = n;
                Ok(json!(n))
            }
            "level" => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                if n < 1 {
                    return Err("out of range");
                }
                if too_many_cells(self.number, n, self.subpixel) {
                    return Err("too many cells");
                }
                self.level = n;
                Ok(json!(n))
            }
            "padding" => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                if !(0..=48).contains(&n) {
                    return Err("out of range");
                }
                self.padding = n;
                Ok(json!(n))
            }
            "invert" => {
                let on = value.as_bool().ok_or("value must be a bool")?;
                self.invert = on;
                Ok(json!(on))
            }
            "rays" => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                if !(1..=128).contains(&n) {
                    return Err("out of range");
                }
                self.rays = n;
                Ok(json!(n))
            }
            "bounces" => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                if !(1..=256).contains(&n) {
                    return Err("out of range");
                }
                self.bounces = n;
                Ok(json!(n))
            }
            "spread" => {
                let s = value.as_str().ok_or("value must be a string")?;
                if !SPREADS.iter().any(|(name, _)| *name == s) {
                    return Err("no such option");
                }
                self.spread = s.to_string();
                Ok(json!(s))
            }
            "spin" => {
                let n = value
                    .as_f64()
                    .or_else(|| value.as_str().and_then(|s| s.parse::<f64>().ok()))
                    .ok_or("value must be a number")?;
                if !SPINS.contains(&n) {
                    return Err("no such option");
                }
                self.spin = n;
                Ok(json!(n))
            }
            "subpixel" => {
                let n = value
                    .as_i64()
                    .or_else(|| value.as_str().and_then(|s| s.parse::<i64>().ok()))
                    .ok_or("value must be an integer")?;
                if !SUBPIXELS.contains(&n) {
                    return Err("no such option");
                }
                if too_many_cells(self.number, self.level, n) {
                    return Err("too many cells");
                }
                self.subpixel = n;
                Ok(json!(n))
            }
            "accent" => {
                let name = value.as_str().ok_or("value must be a string")?;
                named(name).map_err(|_| "unknown color")?;
                self.accent = name.to_string();
                Ok(json!(name))
            }
            _ => Err("no such key"),
        }
    }
    fn to_json(&self) -> Json {
        json!({
            "design": self.design,
            "number": self.number,
            "level": self.level,
            "padding": self.padding,
            "invert": self.invert,
            "subpixel": self.subpixel,
            "rays": self.rays,
            "spread": self.spread,
            "bounces": self.bounces,
            "spin": self.spin,
            "accent": self.accent,
        })
    }
    fn from_json(value: &Json) -> Set {
        let mut set = Set::new();
        if let Some(obj) = value.as_object() {
            for (key, val) in obj {
                let _ = set.apply(key, val);
            }
        }
        set
    }
    fn mask(&self) -> Mask {
        Mask::build(
            &self.design,
            self.number as usize,
            self.level as usize,
            self.padding as usize,
            self.subpixel as usize,
            self.invert,
        )
        .unwrap()
    }
    fn config(&self) -> LasersConfig {
        let radians = SPREADS
            .iter()
            .find(|(name, _)| *name == self.spread)
            .map(|(_, r)| *r)
            .unwrap_or(TAU);
        LasersConfig {
            rays: self.rays as usize,
            spread_idx: radians_to_index(radians),
            bounces: self.bounces as i32,
            omega_idx: radians_to_index(self.spin as f32),
        }
    }
}

pub struct Lasers {
    set: Set,
    sim: Sim,
    play: bool,
    rng: Rng,
    seed: u64,
    dark: bool,
}

impl Default for Lasers {
    fn default() -> Lasers {
        Lasers::new()
    }
}

impl Lasers {
    pub fn new() -> Lasers {
        let set = Set::new();
        let sim = Sim::new(set.mask(), set.config(), 0);
        let mut lasers = Lasers {
            set,
            sim,
            play: true,
            rng: Rng::new(0),
            seed: 0,
            dark: false,
        };
        lasers.reset(0);
        lasers
    }
    fn rebuild(&mut self) {
        self.sim = Sim::new(self.set.mask(), self.set.config(), self.seed);
    }
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.seed = seed;
        self.rebuild();
    }
    fn render(&self) -> Frame {
        let mask = self.sim.mask();
        let w = mask.width();
        let h = mask.height();
        let mut intensity = vec![0.0f32; w * h];
        let alpha = (8.0 / self.set.rays as f32).clamp(0.035, 0.95);
        for poly in self.sim.trace_all() {
            for pair in poly.windows(2) {
                let (x0, y0) = pair[0];
                let (x1, y1) = pair[1];
                for (px, py) in bresenham(
                    x0.round() as i64,
                    y0.round() as i64,
                    x1.round() as i64,
                    y1.round() as i64,
                ) {
                    if px < 0 || py < 0 || px >= w as i64 || py >= h as i64 {
                        continue;
                    }
                    intensity[py as usize * w + px as usize] += alpha;
                }
            }
        }
        let c = named(&self.set.accent).unwrap_or(RED);
        let accent = [c.r, c.g, c.b, c.a];
        let board = crate::ui::frame::board(self.dark);
        let mut colors = vec![board; w * h];
        for y in 0..h {
            for x in 0..w {
                let i = y * w + x;
                if mask.solid(x as f32, y as f32) {
                    colors[i] = WALL;
                    continue;
                }
                let t = intensity[i].clamp(0.0, 1.0) as f64;
                colors[i] = frame::mix(board, accent, t);
            }
        }
        frame::field(w, h, colors, board)
    }
    fn shade(&self) -> Json {
        let mask = self.sim.mask();
        let p = shaders::linear(crate::ui::frame::board(self.dark));
        let c = named(&self.set.accent).unwrap_or(RED);
        let a = shaders::linear([c.r, c.g, c.b, c.a]);
        let mut u = vec![0.0; 16];
        u[4] = p[0];
        u[5] = p[1];
        u[6] = p[2];
        u[8] = a[0];
        u[9] = a[1];
        u[10] = a[2];
        u[12] = mask.width() as f64;
        u[13] = mask.height() as f64;
        json!({ "program": "lasers", "uniforms": u })
    }
    fn count(call: &Call, max: i64) -> Result<i64, &'static str> {
        match call.arg("n") {
            Json::Null => Ok(1),
            given => match given.as_i64() {
                Some(n) if (1..=max).contains(&n) => Ok(n),
                _ => Err("n out of range"),
            },
        }
    }
}

impl App for Lasers {
    fn route(&self) -> &str {
        "lasers"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("lasers").emoji("🔦").category("physics")
    }
    fn wear(&mut self, world: &Json) {
        self.dark = world["shared"]["settings"]["darkmode"] == true;
    }
    fn state(&self, _iden: &Iden) -> Json {
        let mask = self.sim.mask();
        json!({
            "settings": self.set.to_json(),
            "play": self.play,
            "seed": self.seed,
            "emitters": self.sim.emitters().len(),
            "grid": { "width": mask.width(), "height": mask.height() },
            "frame": self.render().fact(),
            "shade": self.shade(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("lasers.place", json!({ "x": "int", "y": "int" })),
            Verb::new("lasers.step", json!({ "n": "int" })),
            Verb::new("lasers.reset", json!({ "seed": "int" })),
            Verb::new("lasers.set", json!({ "key": "string", "value": "any" })),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "lasers.place" => {
                let (Some(x), Some(y)) = (call.arg("x").as_i64(), call.arg("y").as_i64()) else {
                    return Outcome::fail("x and y must be integers");
                };
                if self.sim.mask().solid(x as f32, y as f32) {
                    return Outcome::fail("that cell is wall");
                }
                self.sim.spawn(x as f32, y as f32);
                Outcome::ok(json!({ "x": x, "y": y, "emitters": self.sim.emitters().len() }))
            }
            "lasers.step" => {
                let n = match Lasers::count(call, 64) {
                    Ok(n) => n,
                    Err(note) => return Outcome::fail(note),
                };
                for _ in 0..n {
                    self.sim.step(1.0);
                }
                Outcome::ok(json!({ "emitters": self.sim.emitters().len() }))
            }
            "lasers.reset" => {
                let seed = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                self.reset(seed);
                Outcome::ok(json!({ "seed": seed }))
            }
            "lasers.set" => {
                let key = call.arg("key").as_str().unwrap_or("").to_string();
                if key == "play" {
                    return match call.arg("value").as_bool() {
                        Some(on) => {
                            self.play = on;
                            Outcome::ok(json!({ "key": "play", "value": on }))
                        }
                        None => Outcome::fail("value must be a bool"),
                    };
                }
                match self.set.apply(&key, call.arg("value")) {
                    Ok(value) => {
                        if MASK_KEYS.contains(&key.as_str()) {
                            self.rebuild();
                        } else {
                            self.sim.set_config(self.set.config());
                        }
                        Outcome::ok(json!({ "key": key, "value": value }))
                    }
                    Err(note) => Outcome::fail(note),
                }
            }
            _ => Outcome::fail("unknown verb"),
        }
    }
    fn beat(&self) -> Option<Call> {
        if self.play {
            Some(Call::new("lasers.step", json!({})))
        } else {
            None
        }
    }
    fn save(&self) -> Json {
        let emitters: Vec<Json> = self
            .sim
            .emitters()
            .iter()
            .map(|e| {
                json!({
                    "x": e.x,
                    "y": e.y,
                    "dir": e.dir.value,
                    "omega_idx": e.omega_idx,
                    "spread_idx": e.spread_idx,
                    "rays": e.rays,
                    "bounces": e.bounces,
                })
            })
            .collect();
        json!({
            "settings": self.set.to_json(),
            "play": self.play,
            "seed": self.seed,
            "pos": self.rng.pos() as u64,
            "emitters": emitters,
        })
    }
    fn load(&mut self, state: &Json) {
        self.set = Set::from_json(&state["settings"]);
        if let Some(on) = state["play"].as_bool() {
            self.play = on;
        }
        self.reset(state["seed"].as_u64().unwrap_or(0));
        if let Some(pos) = state["pos"].as_u64() {
            self.rng.seek(pos as u128);
        }
        if let Some(arr) = state["emitters"].as_array() {
            let emitters: Option<Vec<Emitter>> = arr
                .iter()
                .map(|e| {
                    Some(Emitter {
                        x: e["x"].as_f64()? as f32,
                        y: e["y"].as_f64()? as f32,
                        dir: FracIndex::new(e["dir"].as_f64()? as f32),
                        omega_idx: e["omega_idx"].as_f64()? as f32,
                        spread_idx: e["spread_idx"].as_f64()? as f32,
                        rays: e["rays"].as_u64()? as usize,
                        bounces: e["bounces"].as_i64()? as i32,
                    })
                })
                .collect();
            if let Some(emitters) = emitters {
                self.sim.load_emitters(emitters);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os::kernel::testkit::{iden, seeded, send};

    fn lasers(seed: u64) -> Lasers {
        seeded(Lasers::new(), "lasers.reset", seed)
    }

    #[test]
    fn seed_reproduces() {
        let mut a = lasers(123);
        let mut b = lasers(123);
        for l in [&mut a, &mut b] {
            send(l, "lasers.place", json!({ "x": 20, "y": 20 }));
            send(l, "lasers.step", json!({ "n": 5 }));
        }
        assert_eq!(a.state(&iden()), b.state(&iden()));
        assert_eq!(a.save(), b.save());
    }
    #[test]
    fn save_load_continues() {
        let mut a = lasers(11);
        send(&mut a, "lasers.place", json!({ "x": 20, "y": 20 }));
        send(&mut a, "lasers.step", json!({ "n": 4 }));
        let mut b = Lasers::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
        assert_eq!(b.save(), a.save());
        for l in [&mut a, &mut b] {
            send(l, "lasers.step", json!({ "n": 4 }));
        }
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut l = Lasers::new();
        l.load(&json!({ "seed": "soup", "settings": 7, "emitters": "nope" }));
        assert_eq!(l.state(&iden())["seed"], json!(0));
        assert_eq!(l.state(&iden())["emitters"], json!(0));
    }
    #[test]
    fn reset_seed_defaults_to_now() {
        let mut l = Lasers::new();
        let out = l.act(&iden(), &Call::new("lasers.reset", json!({})).at(5000));
        assert!(out.ok);
        assert_eq!(out.data["seed"], json!(5000));
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let l = lasers(3);
        let names: Vec<String> = l.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(
            names,
            vec!["lasers.place", "lasers.step", "lasers.reset", "lasers.set"]
        );
    }
    #[test]
    fn place_fails_on_wall() {
        let mut l = lasers(1);
        assert!(!send(&mut l, "lasers.place", json!({ "x": -1, "y": 0 })).ok);
        assert!(!send(&mut l, "lasers.place", json!({ "x": "nope", "y": 0 })).ok);
    }
    #[test]
    fn place_adds_an_emitter() {
        let mut l = lasers(1);
        assert!(send(&mut l, "lasers.place", json!({ "x": 20, "y": 20 })).ok);
        assert_eq!(l.state(&iden())["emitters"], json!(1));
    }
    #[test]
    fn set_validates_mask_and_config_keys() {
        let mut l = lasers(1);
        assert!(send(&mut l, "lasers.set", json!({ "key": "number", "value": 3 })).ok);
        assert!(!send(&mut l, "lasers.set", json!({ "key": "number", "value": 4 })).ok);
        assert!(
            !send(
                &mut l,
                "lasers.set",
                json!({ "key": "design", "value": "spiral" })
            )
            .ok
        );
        assert!(!send(&mut l, "lasers.set", json!({ "key": "volume", "value": 1 })).ok);
        assert!(
            send(
                &mut l,
                "lasers.set",
                json!({ "key": "spread", "value": "narrow" })
            )
            .ok
        );
        assert!(
            !send(
                &mut l,
                "lasers.set",
                json!({ "key": "spread", "value": "diagonal" })
            )
            .ok
        );
        assert!(send(&mut l, "lasers.set", json!({ "key": "spin", "value": 0.8 })).ok);
        assert!(!send(&mut l, "lasers.set", json!({ "key": "spin", "value": 0.5 })).ok);
        assert!(
            send(
                &mut l,
                "lasers.set",
                json!({ "key": "spin", "value": "0.3" })
            )
            .ok
        );
        assert!(
            !send(
                &mut l,
                "lasers.set",
                json!({ "key": "spin", "value": "loud" })
            )
            .ok
        );
        assert!(
            send(
                &mut l,
                "lasers.set",
                json!({ "key": "play", "value": false })
            )
            .ok
        );
        assert!(
            !send(
                &mut l,
                "lasers.set",
                json!({ "key": "play", "value": "no" })
            )
            .ok
        );
    }
    #[test]
    fn accent_takes_any_named_color() {
        let mut l = lasers(1);
        for name in crate::core::colors::NAMES {
            assert!(
                send(
                    &mut l,
                    "lasers.set",
                    json!({ "key": "accent", "value": name })
                )
                .ok
            );
        }
        assert!(
            !send(
                &mut l,
                "lasers.set",
                json!({ "key": "accent", "value": "chartreuse" })
            )
            .ok
        );
        assert!(!send(&mut l, "lasers.set", json!({ "key": "accent", "value": 7 })).ok);
    }
    #[test]
    fn subpixel_validates_and_rebuilds() {
        let mut l = lasers(1);
        send(&mut l, "lasers.place", json!({ "x": 20, "y": 20 }));
        assert_eq!(l.state(&iden())["emitters"], json!(1));
        assert!(
            send(
                &mut l,
                "lasers.set",
                json!({ "key": "subpixel", "value": 4 })
            )
            .ok
        );
        assert_eq!(l.state(&iden())["emitters"], json!(0));
        assert_eq!(l.state(&iden())["settings"]["subpixel"], json!(4));
        assert!(
            send(
                &mut l,
                "lasers.set",
                json!({ "key": "subpixel", "value": "1" })
            )
            .ok
        );
        assert!(
            !send(
                &mut l,
                "lasers.set",
                json!({ "key": "subpixel", "value": 3 })
            )
            .ok
        );
        assert!(
            !send(
                &mut l,
                "lasers.set",
                json!({ "key": "subpixel", "value": "loud" })
            )
            .ok
        );
    }
    #[test]
    fn accent_and_subpixel_round_trip() {
        let mut a = lasers(2);
        send(
            &mut a,
            "lasers.set",
            json!({ "key": "accent", "value": "teal" }),
        );
        send(
            &mut a,
            "lasers.set",
            json!({ "key": "subpixel", "value": 4 }),
        );
        let mut b = Lasers::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden()), a.state(&iden()));
    }
    #[test]
    fn mask_key_change_rebuilds_and_clears_emitters() {
        let mut l = lasers(1);
        send(&mut l, "lasers.place", json!({ "x": 20, "y": 20 }));
        assert_eq!(l.state(&iden())["emitters"], json!(1));
        send(
            &mut l,
            "lasers.set",
            json!({ "key": "padding", "value": 4 }),
        );
        assert_eq!(l.state(&iden())["emitters"], json!(0));
        assert_eq!(l.state(&iden())["settings"]["padding"], json!(4));
    }
    #[test]
    fn beat_gates_on_play() {
        let mut l = lasers(1);
        assert_eq!(l.beat(), Some(Call::new("lasers.step", json!({}))));
        send(
            &mut l,
            "lasers.set",
            json!({ "key": "play", "value": false }),
        );
        assert_eq!(l.beat(), None);
    }
    #[test]
    fn step_out_of_range_fails() {
        let mut l = lasers(1);
        assert!(!send(&mut l, "lasers.step", json!({ "n": 0 })).ok);
        assert!(!send(&mut l, "lasers.step", json!({ "n": 65 })).ok);
    }
}
