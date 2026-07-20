use crate::core::colors::{named, CYAN, PINK};
use crate::core::rng::Rng;
use crate::os::kernel::{App, Call, Iden, Manifest, Outcome, Verb};
use crate::physics::{Mask, Waves as Sim, WavesConfig};
use crate::ui::frame::{self, Frame};
use crate::ui::shaders;
use serde_json::{json, Value as Json};

const WALL: [u8; 4] = [90, 90, 94, 255];
const DESIGNS: [&str; 5] = ["carpet", "net", "htree", "vtree", "void"];
const NUMBERS: [i64; 4] = [3, 5, 7, 9];
const MASK_KEYS: [&str; 6] = ["design", "number", "level", "padding", "invert", "subpixel"];
const SUBPIXELS: [i64; 3] = [1, 2, 4];

fn too_many_cells(number: i64, level: i64, subpixel: i64) -> bool {
    match (number as u32).checked_pow(level as u32) {
        Some(cells) => (cells as i64) * subpixel > 256,
        None => true,
    }
}

struct Set {
    design: String,
    number: i64,
    level: i64,
    padding: i64,
    invert: bool,
    subpixel: i64,
    speed: f64,
    damp: f64,
    freq: f64,
    sigma: f64,
    amp: f64,
    gain: i64,
    reflect: f64,
    accent: String,
    anti: String,
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
            speed: 0.2,
            damp: 0.001,
            freq: 1.0,
            sigma: 1.5,
            amp: 0.8,
            gain: 4,
            reflect: 1.0,
            accent: "cyan".to_string(),
            anti: "pink".to_string(),
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
            "gain" => {
                let n = value.as_i64().ok_or("value must be an integer")?;
                if !(1..=16).contains(&n) {
                    return Err("out of range");
                }
                self.gain = n;
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
            "accent" | "anti" => {
                let name = value.as_str().ok_or("value must be a string")?;
                named(name).map_err(|_| "unknown color")?;
                match key {
                    "accent" => self.accent = name.to_string(),
                    _ => self.anti = name.to_string(),
                }
                Ok(json!(name))
            }
            "speed" | "damp" | "freq" | "sigma" | "amp" | "reflect" => {
                let n = value.as_f64().ok_or("value must be a number")?;
                let (min, max) = match key {
                    "speed" => (0.05, 0.45),
                    "damp" => (0.0, 0.02),
                    "freq" => (0.3, 4.0),
                    "sigma" => (1.0, 6.0),
                    "amp" => (0.4, 3.0),
                    _ => (0.0, 1.0),
                };
                if !(min..=max).contains(&n) {
                    return Err("out of range");
                }
                match key {
                    "speed" => self.speed = n,
                    "damp" => self.damp = n,
                    "freq" => self.freq = n,
                    "sigma" => self.sigma = n,
                    "amp" => self.amp = n,
                    _ => self.reflect = n,
                }
                Ok(json!(n))
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
            "speed": self.speed,
            "damp": self.damp,
            "freq": self.freq,
            "sigma": self.sigma,
            "amp": self.amp,
            "gain": self.gain,
            "reflect": self.reflect,
            "accent": self.accent,
            "anti": self.anti,
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
    fn config(&self) -> WavesConfig {
        WavesConfig {
            c2: self.speed as f32,
            damp: self.damp as f32,
            freq: self.freq as f32,
            sigma: self.sigma as f32,
            amp: self.amp as f32,
            gain: self.gain as f32,
            reflect: self.reflect as f32,
        }
    }
}

pub struct Waves {
    set: Set,
    sim: Sim,
    play: bool,
    rng: Rng,
    seed: u64,
    dark: bool,
}

impl Default for Waves {
    fn default() -> Waves {
        Waves::new()
    }
}

impl Waves {
    pub fn new() -> Waves {
        let set = Set::new();
        let sim = Sim::new(set.mask(), set.config(), 0);
        let mut waves = Waves {
            set,
            sim,
            play: true,
            rng: Rng::new(0),
            seed: 0,
            dark: false,
        };
        waves.reset(0);
        waves
    }
    fn rebuild(&mut self) {
        self.sim = Sim::new(self.set.mask(), self.set.config(), self.seed);
    }
    fn reset(&mut self, seed: u64) {
        self.rng = Rng::new(seed);
        self.seed = seed;
        self.rebuild();
    }
    fn sources_json(&self) -> Json {
        let frame = self.sim.frame();
        let items: Vec<Json> = self
            .sim
            .sources()
            .iter()
            .map(|s| {
                json!({
                    "x": s.x,
                    "y": s.y,
                    "age": frame - s.born_frame,
                    "phase": s.phase.value,
                })
            })
            .collect();
        Json::Array(items)
    }
    fn render(&self) -> Frame {
        let mask = self.sim.mask();
        let w = mask.width();
        let h = mask.height();
        let field = self.sim.field();
        let gain = self.set.gain as f32;
        let c = named(&self.set.accent).unwrap_or(CYAN);
        let accent = [c.r, c.g, c.b, c.a];
        let p = named(&self.set.anti).unwrap_or(PINK);
        let anti = [p.r, p.g, p.b, p.a];
        let board = crate::ui::frame::board(self.dark);
        let mut colors = vec![board; w * h];
        for y in 0..h {
            for x in 0..w {
                let i = y * w + x;
                if mask.solid(x as f32, y as f32) {
                    colors[i] = WALL;
                    continue;
                }
                let v = (field.at(x as i64, y as i64) * gain) as f64;
                colors[i] = if v >= 0.0 {
                    frame::mix(board, accent, v.clamp(0.0, 1.0))
                } else {
                    frame::mix(board, anti, (-v).clamp(0.0, 1.0))
                };
            }
        }
        frame::field(w, h, colors, board)
    }
    fn shade(&self) -> Json {
        let mask = self.sim.mask();
        let p = shaders::linear(crate::ui::frame::board(self.dark));
        let c = named(&self.set.accent).unwrap_or(CYAN);
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
        json!({ "program": "waves", "uniforms": u })
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

impl App for Waves {
    fn route(&self) -> &str {
        "waves"
    }
    fn manifest(&self) -> Manifest {
        Manifest::new("waves").emoji("🌊").category("physics")
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
            "sources": self.sim.sources().len(),
            "grid": { "width": mask.width(), "height": mask.height() },
            "frame": self.render().fact(),
            "shade": self.shade(),
        })
    }
    fn actions(&self, _iden: &Iden) -> Vec<Verb> {
        vec![
            Verb::new("waves.drop", json!({ "x": "int", "y": "int" })),
            Verb::new("waves.step", json!({ "n": "int" })),
            Verb::new("waves.reset", json!({ "seed": "int" })),
            Verb::new("waves.set", json!({ "key": "string", "value": "any" })),
        ]
    }
    fn act(&mut self, _iden: &Iden, call: &Call) -> Outcome {
        match call.verb.as_str() {
            "waves.drop" => {
                let (Some(x), Some(y)) = (call.arg("x").as_i64(), call.arg("y").as_i64()) else {
                    return Outcome::fail("x and y must be integers");
                };
                if self.sim.mask().solid(x as f32, y as f32) {
                    return Outcome::fail("that cell is wall");
                }
                self.sim.drop(x as f32, y as f32);
                Outcome::ok(json!({ "x": x, "y": y }))
            }
            "waves.step" => {
                let n = match Waves::count(call, 64) {
                    Ok(n) => n,
                    Err(note) => return Outcome::fail(note),
                };
                for _ in 0..n {
                    self.sim.step();
                }
                Outcome::ok(json!({ "frame": self.sim.frame() }))
            }
            "waves.reset" => {
                let seed = call
                    .arg("seed")
                    .as_u64()
                    .unwrap_or(call.now.unwrap_or(0).max(0) as u64);
                self.reset(seed);
                Outcome::ok(json!({ "seed": seed }))
            }
            "waves.set" => {
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
            Some(Call::new("waves.step", json!({})))
        } else {
            None
        }
    }
    fn save(&self) -> Json {
        json!({
            "settings": self.set.to_json(),
            "play": self.play,
            "seed": self.seed,
            "pos": self.rng.pos() as u64,
            "sources": self.sources_json(),
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
        if let Some(arr) = state["sources"].as_array() {
            for s in arr {
                if let (Some(x), Some(y)) = (s["x"].as_f64(), s["y"].as_f64()) {
                    self.sim.drop(x as f32, y as f32);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::os::kernel::testkit::{iden, seeded, send};

    fn waves(seed: u64) -> Waves {
        seeded(Waves::new(), "waves.reset", seed)
    }

    #[test]
    fn seed_reproduces() {
        let mut a = waves(123);
        let mut b = waves(123);
        for w in [&mut a, &mut b] {
            send(w, "waves.drop", json!({ "x": 20, "y": 20 }));
            send(w, "waves.step", json!({ "n": 5 }));
        }
        assert_eq!(a.state(&iden()), b.state(&iden()));
        assert_eq!(a.save(), b.save());
    }
    #[test]
    fn save_load_continues() {
        let mut a = waves(11);
        send(&mut a, "waves.drop", json!({ "x": 20, "y": 20 }));
        send(&mut a, "waves.step", json!({ "n": 3 }));
        let saved = a.save();
        let mut b = Waves::new();
        b.load(&saved);
        assert_eq!(b.state(&iden())["settings"], a.state(&iden())["settings"]);
        assert_eq!(b.state(&iden())["sources"], a.state(&iden())["sources"]);
        let mut c = Waves::new();
        c.load(&saved);
        for w in [&mut b, &mut c] {
            send(w, "waves.step", json!({ "n": 4 }));
        }
        assert_eq!(b.state(&iden()), c.state(&iden()));
    }
    #[test]
    fn load_survives_garbage() {
        let mut w = Waves::new();
        w.load(&json!({ "seed": "soup", "settings": 7, "sources": "nope" }));
        assert_eq!(w.state(&iden())["seed"], json!(0));
        assert_eq!(w.state(&iden())["sources"], json!(0));
    }
    #[test]
    fn reset_seed_defaults_to_now() {
        let mut w = Waves::new();
        let out = w.act(&iden(), &Call::new("waves.reset", json!({})).at(5000));
        assert!(out.ok);
        assert_eq!(out.data["seed"], json!(5000));
    }
    #[test]
    fn actions_offer_the_natural_verbs() {
        let w = waves(3);
        let names: Vec<String> = w.actions(&iden()).iter().map(|v| v.name.clone()).collect();
        assert_eq!(
            names,
            vec!["waves.drop", "waves.step", "waves.reset", "waves.set"]
        );
    }
    #[test]
    fn drop_fails_on_wall() {
        let mut w = waves(1);
        assert!(!send(&mut w, "waves.drop", json!({ "x": -1, "y": 0 })).ok);
        assert!(!send(&mut w, "waves.drop", json!({ "x": "nope", "y": 0 })).ok);
    }
    #[test]
    fn drop_and_step_grows_sources() {
        let mut w = waves(1);
        assert!(send(&mut w, "waves.drop", json!({ "x": 20, "y": 20 })).ok);
        assert_eq!(w.state(&iden())["sources"], json!(1));
        send(&mut w, "waves.step", json!({ "n": 1 }));
        assert!(w.state(&iden())["sources"].as_u64().unwrap() <= 1);
    }
    #[test]
    fn set_validates_mask_and_config_keys() {
        let mut w = waves(1);
        assert!(send(&mut w, "waves.set", json!({ "key": "number", "value": 3 })).ok);
        assert!(!send(&mut w, "waves.set", json!({ "key": "number", "value": 4 })).ok);
        assert!(!send(&mut w, "waves.set", json!({ "key": "level", "value": 9 })).ok);
        assert!(
            !send(
                &mut w,
                "waves.set",
                json!({ "key": "design", "value": "spiral" })
            )
            .ok
        );
        assert!(!send(&mut w, "waves.set", json!({ "key": "volume", "value": 1 })).ok);
        assert!(send(&mut w, "waves.set", json!({ "key": "gain", "value": 8 })).ok);
        assert!(!send(&mut w, "waves.set", json!({ "key": "gain", "value": 99 })).ok);
        assert!(
            send(
                &mut w,
                "waves.set",
                json!({ "key": "play", "value": false })
            )
            .ok
        );
        assert!(!send(&mut w, "waves.set", json!({ "key": "play", "value": "no" })).ok);
    }
    #[test]
    fn accents_take_any_named_color() {
        let mut w = waves(1);
        for name in crate::core::colors::NAMES {
            assert!(
                send(
                    &mut w,
                    "waves.set",
                    json!({ "key": "accent", "value": name })
                )
                .ok
            );
            assert!(send(&mut w, "waves.set", json!({ "key": "anti", "value": name })).ok);
        }
        assert!(
            !send(
                &mut w,
                "waves.set",
                json!({ "key": "accent", "value": "chartreuse" })
            )
            .ok
        );
        assert!(!send(&mut w, "waves.set", json!({ "key": "anti", "value": 7 })).ok);
    }
    #[test]
    fn subpixel_validates_and_rebuilds() {
        let mut w = waves(1);
        send(&mut w, "waves.drop", json!({ "x": 20, "y": 20 }));
        assert_eq!(w.state(&iden())["sources"], json!(1));
        assert!(
            send(
                &mut w,
                "waves.set",
                json!({ "key": "subpixel", "value": 4 })
            )
            .ok
        );
        assert_eq!(w.state(&iden())["sources"], json!(0));
        assert_eq!(w.state(&iden())["settings"]["subpixel"], json!(4));
        assert!(
            send(
                &mut w,
                "waves.set",
                json!({ "key": "subpixel", "value": "1" })
            )
            .ok
        );
        assert!(
            !send(
                &mut w,
                "waves.set",
                json!({ "key": "subpixel", "value": 3 })
            )
            .ok
        );
        assert!(
            !send(
                &mut w,
                "waves.set",
                json!({ "key": "subpixel", "value": "loud" })
            )
            .ok
        );
    }
    #[test]
    fn accents_and_subpixel_round_trip() {
        let mut a = waves(2);
        send(
            &mut a,
            "waves.set",
            json!({ "key": "accent", "value": "teal" }),
        );
        send(
            &mut a,
            "waves.set",
            json!({ "key": "anti", "value": "orange" }),
        );
        send(
            &mut a,
            "waves.set",
            json!({ "key": "subpixel", "value": 4 }),
        );
        let mut b = Waves::new();
        b.load(&a.save());
        assert_eq!(b.state(&iden())["settings"], a.state(&iden())["settings"]);
    }
    #[test]
    fn mask_key_change_rebuilds_and_clears_sources() {
        let mut w = waves(1);
        send(&mut w, "waves.drop", json!({ "x": 20, "y": 20 }));
        assert_eq!(w.state(&iden())["sources"], json!(1));
        send(&mut w, "waves.set", json!({ "key": "padding", "value": 4 }));
        assert_eq!(w.state(&iden())["sources"], json!(0));
        assert_eq!(w.state(&iden())["settings"]["padding"], json!(4));
    }
    #[test]
    fn beat_gates_on_play() {
        let mut w = waves(1);
        assert_eq!(w.beat(), Some(Call::new("waves.step", json!({}))));
        send(
            &mut w,
            "waves.set",
            json!({ "key": "play", "value": false }),
        );
        assert_eq!(w.beat(), None);
    }
    #[test]
    fn step_out_of_range_fails() {
        let mut w = waves(1);
        assert!(!send(&mut w, "waves.step", json!({ "n": 0 })).ok);
        assert!(!send(&mut w, "waves.step", json!({ "n": 65 })).ok);
    }
}
